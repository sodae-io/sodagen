use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum NumeraireProgramIx {
    AddLiquidity(AddLiquidityIxArgs),
    Compound,
    CreatePool(CreatePoolIxArgs),
    InitVirtualStablePair(InitVirtualStablePairIxArgs),
    RemoveAllLiquidity,
    RemoveLiquidity(RemoveLiquidityIxArgs),
    SetFee(SetFeeIxArgs),
    SetInvTMax(SetInvTMaxIxArgs),
    SetLpTokenMetadata(SetLpTokenMetadataIxArgs),
    SetNumeraireOwner(SetNumeraireOwnerIxArgs),
    SetNumeraireStatus(SetNumeraireStatusIxArgs),
    SetNumeraireWhitelistedPoolCreator(SetNumeraireWhitelistedPoolCreatorIxArgs),
    SetOwner(SetOwnerIxArgs),
    SetRate(SetRateIxArgs),
    SetStatus(SetStatusIxArgs),
    SetWhitelistedAdder(SetWhitelistedAdderIxArgs),
    Skim,
    SwapExactIn(SwapExactInIxArgs),
    SwapExactInHinted(SwapExactInHintedIxArgs),
    SwapExactInQuote(SwapExactInQuoteIxArgs),
    SwapExactOut(SwapExactOutIxArgs),
    SwapExactOutHinted(SwapExactOutHintedIxArgs),
}
impl NumeraireProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ADD_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <AddLiquidityData>::deserialize(&mut reader)?
            };
            return Ok(Self::AddLiquidity(AddLiquidityIxArgs { data }));
        }
        if buf.starts_with(&COMPOUND_IX_DISCM) {
            return Ok(Self::Compound);
        }
        if buf.starts_with(&CREATE_POOL_IX_DISCM) {
            let mut reader = &buf[CREATE_POOL_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <CreateStablePoolData>::deserialize(&mut reader)?
            };
            return Ok(Self::CreatePool(CreatePoolIxArgs { data }));
        }
        if buf.starts_with(&INIT_VIRTUAL_STABLE_PAIR_IX_DISCM) {
            let mut reader = &buf[INIT_VIRTUAL_STABLE_PAIR_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <InitVirtualStablePairData>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitVirtualStablePair(InitVirtualStablePairIxArgs {
                    data,
                }),
            );
        }
        if buf.starts_with(&REMOVE_ALL_LIQUIDITY_IX_DISCM) {
            return Ok(Self::RemoveAllLiquidity);
        }
        if buf.starts_with(&REMOVE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[REMOVE_LIQUIDITY_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <RemoveLiquidityData>::deserialize(&mut reader)?
            };
            return Ok(Self::RemoveLiquidity(RemoveLiquidityIxArgs { data }));
        }
        if buf.starts_with(&SET_FEE_IX_DISCM) {
            let mut reader = &buf[SET_FEE_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SetFeeData>::deserialize(&mut reader)?
            };
            return Ok(Self::SetFee(SetFeeIxArgs { data }));
        }
        if buf.starts_with(&SET_INV_T_MAX_IX_DISCM) {
            let mut reader = &buf[SET_INV_T_MAX_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SetInvTMaxData>::deserialize(&mut reader)?
            };
            return Ok(Self::SetInvTMax(SetInvTMaxIxArgs { data }));
        }
        if buf.starts_with(&SET_LP_TOKEN_METADATA_IX_DISCM) {
            let mut reader = &buf[SET_LP_TOKEN_METADATA_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SetMetadataData>::deserialize(&mut reader)?
            };
            return Ok(Self::SetLpTokenMetadata(SetLpTokenMetadataIxArgs { data }));
        }
        if buf.starts_with(&SET_NUMERAIRE_OWNER_IX_DISCM) {
            let mut reader = &buf[SET_NUMERAIRE_OWNER_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SetOwnerData>::deserialize(&mut reader)?
            };
            return Ok(Self::SetNumeraireOwner(SetNumeraireOwnerIxArgs { data }));
        }
        if buf.starts_with(&SET_NUMERAIRE_STATUS_IX_DISCM) {
            let mut reader = &buf[SET_NUMERAIRE_STATUS_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SetStatusData>::deserialize(&mut reader)?
            };
            return Ok(Self::SetNumeraireStatus(SetNumeraireStatusIxArgs { data }));
        }
        if buf.starts_with(&SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_DISCM) {
            let mut reader = &buf[SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_DISCM
                .len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SetWhilelistedAddrData>::deserialize(&mut reader)?
            };
            return Ok(
                Self::SetNumeraireWhitelistedPoolCreator(SetNumeraireWhitelistedPoolCreatorIxArgs {
                    data,
                }),
            );
        }
        if buf.starts_with(&SET_OWNER_IX_DISCM) {
            let mut reader = &buf[SET_OWNER_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SetOwnerData>::deserialize(&mut reader)?
            };
            return Ok(Self::SetOwner(SetOwnerIxArgs { data }));
        }
        if buf.starts_with(&SET_RATE_IX_DISCM) {
            let mut reader = &buf[SET_RATE_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SetRateData>::deserialize(&mut reader)?
            };
            return Ok(Self::SetRate(SetRateIxArgs { data }));
        }
        if buf.starts_with(&SET_STATUS_IX_DISCM) {
            let mut reader = &buf[SET_STATUS_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SetStatusData>::deserialize(&mut reader)?
            };
            return Ok(Self::SetStatus(SetStatusIxArgs { data }));
        }
        if buf.starts_with(&SET_WHITELISTED_ADDER_IX_DISCM) {
            let mut reader = &buf[SET_WHITELISTED_ADDER_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SetWhilelistedAddrData>::deserialize(&mut reader)?
            };
            return Ok(Self::SetWhitelistedAdder(SetWhitelistedAdderIxArgs { data }));
        }
        if buf.starts_with(&SKIM_IX_DISCM) {
            return Ok(Self::Skim);
        }
        if buf.starts_with(&SWAP_EXACT_IN_IX_DISCM) {
            let mut reader = &buf[SWAP_EXACT_IN_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SwapExactInHintlessData>::deserialize(&mut reader)?
            };
            return Ok(Self::SwapExactIn(SwapExactInIxArgs { data }));
        }
        if buf.starts_with(&SWAP_EXACT_IN_HINTED_IX_DISCM) {
            let mut reader = &buf[SWAP_EXACT_IN_HINTED_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SwapExactInData>::deserialize(&mut reader)?
            };
            return Ok(Self::SwapExactInHinted(SwapExactInHintedIxArgs { data }));
        }
        if buf.starts_with(&SWAP_EXACT_IN_QUOTE_IX_DISCM) {
            let mut reader = &buf[SWAP_EXACT_IN_QUOTE_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SwapExactInData>::deserialize(&mut reader)?
            };
            return Ok(Self::SwapExactInQuote(SwapExactInQuoteIxArgs { data }));
        }
        if buf.starts_with(&SWAP_EXACT_OUT_IX_DISCM) {
            let mut reader = &buf[SWAP_EXACT_OUT_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SwapExactOutHintlessData>::deserialize(&mut reader)?
            };
            return Ok(Self::SwapExactOut(SwapExactOutIxArgs { data }));
        }
        if buf.starts_with(&SWAP_EXACT_OUT_HINTED_IX_DISCM) {
            let mut reader = &buf[SWAP_EXACT_OUT_HINTED_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <SwapExactOutData>::deserialize(&mut reader)?
            };
            return Ok(Self::SwapExactOutHinted(SwapExactOutHintedIxArgs { data }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AddLiquidity(args) => {
                writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::Compound => writer.write_all(&COMPOUND_IX_DISCM),
            Self::CreatePool(args) => {
                writer.write_all(&CREATE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::InitVirtualStablePair(args) => {
                writer.write_all(&INIT_VIRTUAL_STABLE_PAIR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::RemoveAllLiquidity => writer.write_all(&REMOVE_ALL_LIQUIDITY_IX_DISCM),
            Self::RemoveLiquidity(args) => {
                writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SetFee(args) => {
                writer.write_all(&SET_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SetInvTMax(args) => {
                writer.write_all(&SET_INV_T_MAX_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SetLpTokenMetadata(args) => {
                writer.write_all(&SET_LP_TOKEN_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SetNumeraireOwner(args) => {
                writer.write_all(&SET_NUMERAIRE_OWNER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SetNumeraireStatus(args) => {
                writer.write_all(&SET_NUMERAIRE_STATUS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SetNumeraireWhitelistedPoolCreator(args) => {
                writer.write_all(&SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SetOwner(args) => {
                writer.write_all(&SET_OWNER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SetRate(args) => {
                writer.write_all(&SET_RATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SetStatus(args) => {
                writer.write_all(&SET_STATUS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SetWhitelistedAdder(args) => {
                writer.write_all(&SET_WHITELISTED_ADDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::Skim => writer.write_all(&SKIM_IX_DISCM),
            Self::SwapExactIn(args) => {
                writer.write_all(&SWAP_EXACT_IN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SwapExactInHinted(args) => {
                writer.write_all(&SWAP_EXACT_IN_HINTED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SwapExactInQuote(args) => {
                writer.write_all(&SWAP_EXACT_IN_QUOTE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SwapExactOut(args) => {
                writer.write_all(&SWAP_EXACT_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::SwapExactOutHinted(args) => {
                writer.write_all(&SWAP_EXACT_OUT_HINTED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
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
pub const ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub payer_lp_account: &'me AccountInfo<'info>,
    pub numeraire_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub payer_lp_account: Pubkey,
    pub numeraire_config: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<AddLiquidityAccounts<'_, '_>> for AddLiquidityKeys {
    fn from(accounts: AddLiquidityAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            payer_lp_account: *accounts.payer_lp_account.key,
            numeraire_config: *accounts.numeraire_config.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<AddLiquidityKeys> for [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_lp_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for AddLiquidityKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            payer_lp_account: pubkeys[2],
            numeraire_config: pubkeys[3],
            payer: pubkeys[4],
            token_program: pubkeys[5],
            token_2022_program: pubkeys[6],
        }
    }
}
impl<'info> From<AddLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.payer_lp_account.clone(),
            accounts.numeraire_config.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]>
for AddLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            payer_lp_account: &arr[2],
            numeraire_config: &arr[3],
            payer: &arr[4],
            token_program: &arr[5],
            token_2022_program: &arr[6],
        }
    }
}
pub const ADD_LIQUIDITY_IX_DISCM: [u8; 8usize] = [181, 157, 89, 67, 143, 182, 52, 72];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityIxArgs {
    pub data: AddLiquidityData,
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
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <AddLiquidityData>::deserialize(&mut reader)?
        };
        Ok(Self(AddLiquidityIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
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
    add_liquidity_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
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
    add_liquidity_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
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
    add_liquidity_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_verify_account_keys(
    accounts: AddLiquidityAccounts<'_, '_>,
    keys: AddLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.payer_lp_account.key, keys.payer_lp_account),
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
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
        accounts.pool,
        accounts.lp_mint,
        accounts.payer_lp_account,
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
    for should_be_signer in [accounts.payer] {
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
pub const COMPOUND_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CompoundAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub payer_lp_account: &'me AccountInfo<'info>,
    pub numeraire_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CompoundKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub payer_lp_account: Pubkey,
    pub numeraire_config: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<CompoundAccounts<'_, '_>> for CompoundKeys {
    fn from(accounts: CompoundAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            payer_lp_account: *accounts.payer_lp_account.key,
            numeraire_config: *accounts.numeraire_config.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<CompoundKeys> for [AccountMeta; COMPOUND_IX_ACCOUNTS_LEN] {
    fn from(keys: CompoundKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_lp_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COMPOUND_IX_ACCOUNTS_LEN]> for CompoundKeys {
    fn from(pubkeys: [Pubkey; COMPOUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            payer_lp_account: pubkeys[2],
            numeraire_config: pubkeys[3],
            payer: pubkeys[4],
            token_program: pubkeys[5],
            token_2022_program: pubkeys[6],
        }
    }
}
impl<'info> From<CompoundAccounts<'_, 'info>>
for [AccountInfo<'info>; COMPOUND_IX_ACCOUNTS_LEN] {
    fn from(accounts: CompoundAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.payer_lp_account.clone(),
            accounts.numeraire_config.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COMPOUND_IX_ACCOUNTS_LEN]>
for CompoundAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COMPOUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            payer_lp_account: &arr[2],
            numeraire_config: &arr[3],
            payer: &arr[4],
            token_program: &arr[5],
            token_2022_program: &arr[6],
        }
    }
}
pub const COMPOUND_IX_DISCM: [u8; 8usize] = [165, 208, 251, 78, 242, 160, 141, 47];
#[derive(Clone, Debug, PartialEq)]
pub struct CompoundIxData;
impl CompoundIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COMPOUND_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COMPOUND_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn compound_ix_with_program_id(
    program_id: Pubkey,
    keys: CompoundKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COMPOUND_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CompoundIxData.try_to_vec()?,
    })
}
pub fn compound_ix(keys: CompoundKeys) -> std::io::Result<Instruction> {
    compound_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys)
}
pub fn compound_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CompoundAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CompoundKeys = accounts.into();
    let ix = compound_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn compound_invoke(accounts: CompoundAccounts<'_, '_>) -> ProgramResult {
    compound_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts)
}
pub fn compound_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CompoundAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CompoundKeys = accounts.into();
    let ix = compound_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn compound_invoke_signed(
    accounts: CompoundAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    compound_invoke_signed_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, seeds)
}
pub fn compound_verify_account_keys(
    accounts: CompoundAccounts<'_, '_>,
    keys: CompoundKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.payer_lp_account.key, keys.payer_lp_account),
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn compound_verify_writable_privileges<'me, 'info>(
    accounts: CompoundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.payer_lp_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn compound_verify_signer_privileges<'me, 'info>(
    accounts: CompoundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn compound_verify_account_privileges<'me, 'info>(
    accounts: CompoundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    compound_verify_writable_privileges(accounts)?;
    compound_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_POOL_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreatePoolAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub creator_lp_account: &'me AccountInfo<'info>,
    pub numeraire_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePoolKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub creator_lp_account: Pubkey,
    pub numeraire_config: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePoolAccounts<'_, '_>> for CreatePoolKeys {
    fn from(accounts: CreatePoolAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            creator_lp_account: *accounts.creator_lp_account.key,
            numeraire_config: *accounts.numeraire_config.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePoolKeys> for [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_lp_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
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
impl From<[Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]> for CreatePoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            creator_lp_account: pubkeys[2],
            numeraire_config: pubkeys[3],
            payer: pubkeys[4],
            token_program: pubkeys[5],
            token_program_2022: pubkeys[6],
            associated_token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<CreatePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.creator_lp_account.clone(),
            accounts.numeraire_config.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]>
for CreatePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            creator_lp_account: &arr[2],
            numeraire_config: &arr[3],
            payer: &arr[4],
            token_program: &arr[5],
            token_program_2022: &arr[6],
            associated_token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const CREATE_POOL_IX_DISCM: [u8; 8usize] = [233, 146, 209, 142, 207, 104, 64, 188];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePoolIxArgs {
    pub data: CreateStablePoolData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePoolIxData(pub CreatePoolIxArgs);
impl From<CreatePoolIxArgs> for CreatePoolIxData {
    fn from(args: CreatePoolIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <CreateStablePoolData>::deserialize(&mut reader)?
        };
        Ok(Self(CreatePoolIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePoolKeys,
    args: CreatePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_pool_ix(
    keys: CreatePoolKeys,
    args: CreatePoolIxArgs,
) -> std::io::Result<Instruction> {
    create_pool_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn create_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_pool_invoke(
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
) -> ProgramResult {
    create_pool_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn create_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_pool_invoke_signed(
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_pool_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_pool_verify_account_keys(
    accounts: CreatePoolAccounts<'_, '_>,
    keys: CreatePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.creator_lp_account.key, keys.creator_lp_account),
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_pool_verify_writable_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.creator_lp_account,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_pool_verify_signer_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_pool_verify_account_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_pool_verify_writable_privileges(accounts)?;
    create_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_VIRTUAL_STABLE_PAIR_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitVirtualStablePairAccounts<'me, 'info> {
    pub x_mint: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub pair_authority: &'me AccountInfo<'info>,
    pub x_vault: &'me AccountInfo<'info>,
    pub x_adder: &'me AccountInfo<'info>,
    pub numeraire_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitVirtualStablePairKeys {
    pub x_mint: Pubkey,
    pub pair: Pubkey,
    pub pair_authority: Pubkey,
    pub x_vault: Pubkey,
    pub x_adder: Pubkey,
    pub numeraire_config: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitVirtualStablePairAccounts<'_, '_>> for InitVirtualStablePairKeys {
    fn from(accounts: InitVirtualStablePairAccounts) -> Self {
        Self {
            x_mint: *accounts.x_mint.key,
            pair: *accounts.pair.key,
            pair_authority: *accounts.pair_authority.key,
            x_vault: *accounts.x_vault.key,
            x_adder: *accounts.x_adder.key,
            numeraire_config: *accounts.numeraire_config.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitVirtualStablePairKeys>
for [AccountMeta; INIT_VIRTUAL_STABLE_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitVirtualStablePairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pair_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.x_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.x_adder,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
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
impl From<[Pubkey; INIT_VIRTUAL_STABLE_PAIR_IX_ACCOUNTS_LEN]>
for InitVirtualStablePairKeys {
    fn from(pubkeys: [Pubkey; INIT_VIRTUAL_STABLE_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            x_mint: pubkeys[0],
            pair: pubkeys[1],
            pair_authority: pubkeys[2],
            x_vault: pubkeys[3],
            x_adder: pubkeys[4],
            numeraire_config: pubkeys[5],
            payer: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<InitVirtualStablePairAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_VIRTUAL_STABLE_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitVirtualStablePairAccounts<'_, 'info>) -> Self {
        [
            accounts.x_mint.clone(),
            accounts.pair.clone(),
            accounts.pair_authority.clone(),
            accounts.x_vault.clone(),
            accounts.x_adder.clone(),
            accounts.numeraire_config.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INIT_VIRTUAL_STABLE_PAIR_IX_ACCOUNTS_LEN]>
for InitVirtualStablePairAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_VIRTUAL_STABLE_PAIR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            x_mint: &arr[0],
            pair: &arr[1],
            pair_authority: &arr[2],
            x_vault: &arr[3],
            x_adder: &arr[4],
            numeraire_config: &arr[5],
            payer: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const INIT_VIRTUAL_STABLE_PAIR_IX_DISCM: [u8; 8usize] = [
    228, 18, 163, 161, 101, 204, 106, 172,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitVirtualStablePairIxArgs {
    pub data: InitVirtualStablePairData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitVirtualStablePairIxData(pub InitVirtualStablePairIxArgs);
impl From<InitVirtualStablePairIxArgs> for InitVirtualStablePairIxData {
    fn from(args: InitVirtualStablePairIxArgs) -> Self {
        Self(args)
    }
}
impl InitVirtualStablePairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_VIRTUAL_STABLE_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <InitVirtualStablePairData>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitVirtualStablePairIxArgs {
                data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_VIRTUAL_STABLE_PAIR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_virtual_stable_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: InitVirtualStablePairKeys,
    args: InitVirtualStablePairIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_VIRTUAL_STABLE_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitVirtualStablePairIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_virtual_stable_pair_ix(
    keys: InitVirtualStablePairKeys,
    args: InitVirtualStablePairIxArgs,
) -> std::io::Result<Instruction> {
    init_virtual_stable_pair_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn init_virtual_stable_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitVirtualStablePairAccounts<'_, '_>,
    args: InitVirtualStablePairIxArgs,
) -> ProgramResult {
    let keys: InitVirtualStablePairKeys = accounts.into();
    let ix = init_virtual_stable_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_virtual_stable_pair_invoke(
    accounts: InitVirtualStablePairAccounts<'_, '_>,
    args: InitVirtualStablePairIxArgs,
) -> ProgramResult {
    init_virtual_stable_pair_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn init_virtual_stable_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitVirtualStablePairAccounts<'_, '_>,
    args: InitVirtualStablePairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitVirtualStablePairKeys = accounts.into();
    let ix = init_virtual_stable_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_virtual_stable_pair_invoke_signed(
    accounts: InitVirtualStablePairAccounts<'_, '_>,
    args: InitVirtualStablePairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_virtual_stable_pair_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_virtual_stable_pair_verify_account_keys(
    accounts: InitVirtualStablePairAccounts<'_, '_>,
    keys: InitVirtualStablePairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.x_mint.key, keys.x_mint),
        (*accounts.pair.key, keys.pair),
        (*accounts.pair_authority.key, keys.pair_authority),
        (*accounts.x_vault.key, keys.x_vault),
        (*accounts.x_adder.key, keys.x_adder),
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_virtual_stable_pair_verify_writable_privileges<'me, 'info>(
    accounts: InitVirtualStablePairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.x_vault,
        accounts.x_adder,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_virtual_stable_pair_verify_signer_privileges<'me, 'info>(
    accounts: InitVirtualStablePairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_virtual_stable_pair_verify_account_privileges<'me, 'info>(
    accounts: InitVirtualStablePairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_virtual_stable_pair_verify_writable_privileges(accounts)?;
    init_virtual_stable_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct RemoveAllLiquidityAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub payer_lp_account: &'me AccountInfo<'info>,
    pub numeraire_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveAllLiquidityKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub payer_lp_account: Pubkey,
    pub numeraire_config: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<RemoveAllLiquidityAccounts<'_, '_>> for RemoveAllLiquidityKeys {
    fn from(accounts: RemoveAllLiquidityAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            payer_lp_account: *accounts.payer_lp_account.key,
            numeraire_config: *accounts.numeraire_config.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<RemoveAllLiquidityKeys>
for [AccountMeta; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveAllLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_lp_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveAllLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            payer_lp_account: pubkeys[2],
            numeraire_config: pubkeys[3],
            payer: pubkeys[4],
            token_program: pubkeys[5],
            token_2022_program: pubkeys[6],
        }
    }
}
impl<'info> From<RemoveAllLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveAllLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.payer_lp_account.clone(),
            accounts.numeraire_config.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RemoveAllLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            payer_lp_account: &arr[2],
            numeraire_config: &arr[3],
            payer: &arr[4],
            token_program: &arr[5],
            token_2022_program: &arr[6],
        }
    }
}
pub const REMOVE_ALL_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    10, 51, 61, 35, 112, 105, 24, 85,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveAllLiquidityIxData;
impl RemoveAllLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_ALL_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_ALL_LIQUIDITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_all_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveAllLiquidityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveAllLiquidityIxData.try_to_vec()?,
    })
}
pub fn remove_all_liquidity_ix(
    keys: RemoveAllLiquidityKeys,
) -> std::io::Result<Instruction> {
    remove_all_liquidity_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys)
}
pub fn remove_all_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveAllLiquidityKeys = accounts.into();
    let ix = remove_all_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_all_liquidity_invoke(
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    remove_all_liquidity_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts)
}
pub fn remove_all_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveAllLiquidityKeys = accounts.into();
    let ix = remove_all_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_all_liquidity_invoke_signed(
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_all_liquidity_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn remove_all_liquidity_verify_account_keys(
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    keys: RemoveAllLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.payer_lp_account.key, keys.payer_lp_account),
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_all_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RemoveAllLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.payer_lp_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_all_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: RemoveAllLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_all_liquidity_verify_account_privileges<'me, 'info>(
    accounts: RemoveAllLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_all_liquidity_verify_writable_privileges(accounts)?;
    remove_all_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub payer_lp_account: &'me AccountInfo<'info>,
    pub numeraire_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidityKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub payer_lp_account: Pubkey,
    pub numeraire_config: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<RemoveLiquidityAccounts<'_, '_>> for RemoveLiquidityKeys {
    fn from(accounts: RemoveLiquidityAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            payer_lp_account: *accounts.payer_lp_account.key,
            numeraire_config: *accounts.numeraire_config.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<RemoveLiquidityKeys> for [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_lp_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            payer_lp_account: pubkeys[2],
            numeraire_config: pubkeys[3],
            payer: pubkeys[4],
            token_program: pubkeys[5],
            token_2022_program: pubkeys[6],
        }
    }
}
impl<'info> From<RemoveLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.payer_lp_account.clone(),
            accounts.numeraire_config.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RemoveLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            payer_lp_account: &arr[2],
            numeraire_config: &arr[3],
            payer: &arr[4],
            token_program: &arr[5],
            token_2022_program: &arr[6],
        }
    }
}
pub const REMOVE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [80, 85, 209, 72, 24, 206, 177, 108];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidityIxArgs {
    pub data: RemoveLiquidityData,
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
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <RemoveLiquidityData>::deserialize(&mut reader)?
        };
        Ok(Self(RemoveLiquidityIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
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
    remove_liquidity_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
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
    remove_liquidity_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
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
        NUMERAIRE_PROGRAM_ID,
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
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.payer_lp_account.key, keys.payer_lp_account),
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
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
        accounts.pool,
        accounts.lp_mint,
        accounts.payer_lp_account,
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
    for should_be_signer in [accounts.payer] {
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
pub const SET_FEE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetFeeKeys {
    pub pool: Pubkey,
    pub payer: Pubkey,
}
impl From<SetFeeAccounts<'_, '_>> for SetFeeKeys {
    fn from(accounts: SetFeeAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            payer: *accounts.payer.key,
        }
    }
}
impl From<SetFeeKeys> for [AccountMeta; SET_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_FEE_IX_ACCOUNTS_LEN]> for SetFeeKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            payer: pubkeys[1],
        }
    }
}
impl<'info> From<SetFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetFeeAccounts<'_, 'info>) -> Self {
        [accounts.pool.clone(), accounts.payer.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN]>
for SetFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            payer: &arr[1],
        }
    }
}
pub const SET_FEE_IX_DISCM: [u8; 8usize] = [18, 154, 24, 18, 237, 214, 19, 80];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetFeeIxArgs {
    pub data: SetFeeData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeIxData(pub SetFeeIxArgs);
impl From<SetFeeIxArgs> for SetFeeIxData {
    fn from(args: SetFeeIxArgs) -> Self {
        Self(args)
    }
}
impl SetFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SetFeeData>::deserialize(&mut reader)?
        };
        Ok(Self(SetFeeIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFeeKeys,
    args: SetFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fee_ix(keys: SetFeeKeys, args: SetFeeIxArgs) -> std::io::Result<Instruction> {
    set_fee_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn set_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeAccounts<'_, '_>,
    args: SetFeeIxArgs,
) -> ProgramResult {
    let keys: SetFeeKeys = accounts.into();
    let ix = set_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_fee_invoke(
    accounts: SetFeeAccounts<'_, '_>,
    args: SetFeeIxArgs,
) -> ProgramResult {
    set_fee_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn set_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeAccounts<'_, '_>,
    args: SetFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFeeKeys = accounts.into();
    let ix = set_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_fee_invoke_signed(
    accounts: SetFeeAccounts<'_, '_>,
    args: SetFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_fee_invoke_signed_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_fee_verify_account_keys(
    accounts: SetFeeAccounts<'_, '_>,
    keys: SetFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.payer.key, keys.payer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_fee_verify_writable_privileges<'me, 'info>(
    accounts: SetFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_fee_verify_signer_privileges<'me, 'info>(
    accounts: SetFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_fee_verify_account_privileges<'me, 'info>(
    accounts: SetFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_fee_verify_writable_privileges(accounts)?;
    set_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_INV_T_MAX_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetInvTMaxAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetInvTMaxKeys {
    pub pool: Pubkey,
    pub payer: Pubkey,
}
impl From<SetInvTMaxAccounts<'_, '_>> for SetInvTMaxKeys {
    fn from(accounts: SetInvTMaxAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            payer: *accounts.payer.key,
        }
    }
}
impl From<SetInvTMaxKeys> for [AccountMeta; SET_INV_T_MAX_IX_ACCOUNTS_LEN] {
    fn from(keys: SetInvTMaxKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_INV_T_MAX_IX_ACCOUNTS_LEN]> for SetInvTMaxKeys {
    fn from(pubkeys: [Pubkey; SET_INV_T_MAX_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            payer: pubkeys[1],
        }
    }
}
impl<'info> From<SetInvTMaxAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_INV_T_MAX_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetInvTMaxAccounts<'_, 'info>) -> Self {
        [accounts.pool.clone(), accounts.payer.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_INV_T_MAX_IX_ACCOUNTS_LEN]>
for SetInvTMaxAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_INV_T_MAX_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            payer: &arr[1],
        }
    }
}
pub const SET_INV_T_MAX_IX_DISCM: [u8; 8usize] = [218, 209, 244, 237, 211, 236, 98, 58];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetInvTMaxIxArgs {
    pub data: SetInvTMaxData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetInvTMaxIxData(pub SetInvTMaxIxArgs);
impl From<SetInvTMaxIxArgs> for SetInvTMaxIxData {
    fn from(args: SetInvTMaxIxArgs) -> Self {
        Self(args)
    }
}
impl SetInvTMaxIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_INV_T_MAX_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SetInvTMaxData>::deserialize(&mut reader)?
        };
        Ok(Self(SetInvTMaxIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_INV_T_MAX_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_inv_t_max_ix_with_program_id(
    program_id: Pubkey,
    keys: SetInvTMaxKeys,
    args: SetInvTMaxIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_INV_T_MAX_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetInvTMaxIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_inv_t_max_ix(
    keys: SetInvTMaxKeys,
    args: SetInvTMaxIxArgs,
) -> std::io::Result<Instruction> {
    set_inv_t_max_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn set_inv_t_max_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetInvTMaxAccounts<'_, '_>,
    args: SetInvTMaxIxArgs,
) -> ProgramResult {
    let keys: SetInvTMaxKeys = accounts.into();
    let ix = set_inv_t_max_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_inv_t_max_invoke(
    accounts: SetInvTMaxAccounts<'_, '_>,
    args: SetInvTMaxIxArgs,
) -> ProgramResult {
    set_inv_t_max_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn set_inv_t_max_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetInvTMaxAccounts<'_, '_>,
    args: SetInvTMaxIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetInvTMaxKeys = accounts.into();
    let ix = set_inv_t_max_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_inv_t_max_invoke_signed(
    accounts: SetInvTMaxAccounts<'_, '_>,
    args: SetInvTMaxIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_inv_t_max_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_inv_t_max_verify_account_keys(
    accounts: SetInvTMaxAccounts<'_, '_>,
    keys: SetInvTMaxKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.payer.key, keys.payer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_inv_t_max_verify_writable_privileges<'me, 'info>(
    accounts: SetInvTMaxAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_inv_t_max_verify_signer_privileges<'me, 'info>(
    accounts: SetInvTMaxAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_inv_t_max_verify_account_privileges<'me, 'info>(
    accounts: SetInvTMaxAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_inv_t_max_verify_writable_privileges(accounts)?;
    set_inv_t_max_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_LP_TOKEN_METADATA_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct SetLpTokenMetadataAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub instructions: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetLpTokenMetadataKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub metadata_account: Pubkey,
    pub payer: Pubkey,
    pub token_metadata_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub instructions: Pubkey,
}
impl From<SetLpTokenMetadataAccounts<'_, '_>> for SetLpTokenMetadataKeys {
    fn from(accounts: SetLpTokenMetadataAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            metadata_account: *accounts.metadata_account.key,
            payer: *accounts.payer.key,
            token_metadata_program: *accounts.token_metadata_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            instructions: *accounts.instructions.key,
        }
    }
}
impl From<SetLpTokenMetadataKeys>
for [AccountMeta; SET_LP_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLpTokenMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_LP_TOKEN_METADATA_IX_ACCOUNTS_LEN]> for SetLpTokenMetadataKeys {
    fn from(pubkeys: [Pubkey; SET_LP_TOKEN_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            metadata_account: pubkeys[2],
            payer: pubkeys[3],
            token_metadata_program: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
            instructions: pubkeys[7],
        }
    }
}
impl<'info> From<SetLpTokenMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_LP_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetLpTokenMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.metadata_account.clone(),
            accounts.payer.clone(),
            accounts.token_metadata_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.instructions.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LP_TOKEN_METADATA_IX_ACCOUNTS_LEN]>
for SetLpTokenMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_LP_TOKEN_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            metadata_account: &arr[2],
            payer: &arr[3],
            token_metadata_program: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
            instructions: &arr[7],
        }
    }
}
pub const SET_LP_TOKEN_METADATA_IX_DISCM: [u8; 8usize] = [
    71, 73, 56, 155, 202, 142, 100, 150,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetLpTokenMetadataIxArgs {
    pub data: SetMetadataData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLpTokenMetadataIxData(pub SetLpTokenMetadataIxArgs);
impl From<SetLpTokenMetadataIxArgs> for SetLpTokenMetadataIxData {
    fn from(args: SetLpTokenMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl SetLpTokenMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_LP_TOKEN_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SetMetadataData>::deserialize(&mut reader)?
        };
        Ok(Self(SetLpTokenMetadataIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_LP_TOKEN_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_lp_token_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLpTokenMetadataKeys,
    args: SetLpTokenMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_LP_TOKEN_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetLpTokenMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_lp_token_metadata_ix(
    keys: SetLpTokenMetadataKeys,
    args: SetLpTokenMetadataIxArgs,
) -> std::io::Result<Instruction> {
    set_lp_token_metadata_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn set_lp_token_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLpTokenMetadataAccounts<'_, '_>,
    args: SetLpTokenMetadataIxArgs,
) -> ProgramResult {
    let keys: SetLpTokenMetadataKeys = accounts.into();
    let ix = set_lp_token_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_lp_token_metadata_invoke(
    accounts: SetLpTokenMetadataAccounts<'_, '_>,
    args: SetLpTokenMetadataIxArgs,
) -> ProgramResult {
    set_lp_token_metadata_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn set_lp_token_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLpTokenMetadataAccounts<'_, '_>,
    args: SetLpTokenMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLpTokenMetadataKeys = accounts.into();
    let ix = set_lp_token_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_lp_token_metadata_invoke_signed(
    accounts: SetLpTokenMetadataAccounts<'_, '_>,
    args: SetLpTokenMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_lp_token_metadata_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_lp_token_metadata_verify_account_keys(
    accounts: SetLpTokenMetadataAccounts<'_, '_>,
    keys: SetLpTokenMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_metadata_program.key, keys.token_metadata_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.instructions.key, keys.instructions),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_lp_token_metadata_verify_writable_privileges<'me, 'info>(
    accounts: SetLpTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.metadata_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_lp_token_metadata_verify_signer_privileges<'me, 'info>(
    accounts: SetLpTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_lp_token_metadata_verify_account_privileges<'me, 'info>(
    accounts: SetLpTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_lp_token_metadata_verify_writable_privileges(accounts)?;
    set_lp_token_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_NUMERAIRE_OWNER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetNumeraireOwnerAccounts<'me, 'info> {
    pub numeraire_config: &'me AccountInfo<'info>,
    pub pair_mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetNumeraireOwnerKeys {
    pub numeraire_config: Pubkey,
    pub pair_mint: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<SetNumeraireOwnerAccounts<'_, '_>> for SetNumeraireOwnerKeys {
    fn from(accounts: SetNumeraireOwnerAccounts) -> Self {
        Self {
            numeraire_config: *accounts.numeraire_config.key,
            pair_mint: *accounts.pair_mint.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SetNumeraireOwnerKeys> for [AccountMeta; SET_NUMERAIRE_OWNER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetNumeraireOwnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pair_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
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
impl From<[Pubkey; SET_NUMERAIRE_OWNER_IX_ACCOUNTS_LEN]> for SetNumeraireOwnerKeys {
    fn from(pubkeys: [Pubkey; SET_NUMERAIRE_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            numeraire_config: pubkeys[0],
            pair_mint: pubkeys[1],
            payer: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<SetNumeraireOwnerAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_NUMERAIRE_OWNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetNumeraireOwnerAccounts<'_, 'info>) -> Self {
        [
            accounts.numeraire_config.clone(),
            accounts.pair_mint.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_NUMERAIRE_OWNER_IX_ACCOUNTS_LEN]>
for SetNumeraireOwnerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_NUMERAIRE_OWNER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            numeraire_config: &arr[0],
            pair_mint: &arr[1],
            payer: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const SET_NUMERAIRE_OWNER_IX_DISCM: [u8; 8usize] = [
    6, 199, 177, 104, 86, 61, 93, 253,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetNumeraireOwnerIxArgs {
    pub data: SetOwnerData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetNumeraireOwnerIxData(pub SetNumeraireOwnerIxArgs);
impl From<SetNumeraireOwnerIxArgs> for SetNumeraireOwnerIxData {
    fn from(args: SetNumeraireOwnerIxArgs) -> Self {
        Self(args)
    }
}
impl SetNumeraireOwnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_NUMERAIRE_OWNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SetOwnerData>::deserialize(&mut reader)?
        };
        Ok(Self(SetNumeraireOwnerIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_NUMERAIRE_OWNER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_numeraire_owner_ix_with_program_id(
    program_id: Pubkey,
    keys: SetNumeraireOwnerKeys,
    args: SetNumeraireOwnerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_NUMERAIRE_OWNER_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetNumeraireOwnerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_numeraire_owner_ix(
    keys: SetNumeraireOwnerKeys,
    args: SetNumeraireOwnerIxArgs,
) -> std::io::Result<Instruction> {
    set_numeraire_owner_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn set_numeraire_owner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetNumeraireOwnerAccounts<'_, '_>,
    args: SetNumeraireOwnerIxArgs,
) -> ProgramResult {
    let keys: SetNumeraireOwnerKeys = accounts.into();
    let ix = set_numeraire_owner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_numeraire_owner_invoke(
    accounts: SetNumeraireOwnerAccounts<'_, '_>,
    args: SetNumeraireOwnerIxArgs,
) -> ProgramResult {
    set_numeraire_owner_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn set_numeraire_owner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetNumeraireOwnerAccounts<'_, '_>,
    args: SetNumeraireOwnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetNumeraireOwnerKeys = accounts.into();
    let ix = set_numeraire_owner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_numeraire_owner_invoke_signed(
    accounts: SetNumeraireOwnerAccounts<'_, '_>,
    args: SetNumeraireOwnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_numeraire_owner_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_numeraire_owner_verify_account_keys(
    accounts: SetNumeraireOwnerAccounts<'_, '_>,
    keys: SetNumeraireOwnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.pair_mint.key, keys.pair_mint),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_numeraire_owner_verify_writable_privileges<'me, 'info>(
    accounts: SetNumeraireOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.numeraire_config, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_numeraire_owner_verify_signer_privileges<'me, 'info>(
    accounts: SetNumeraireOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_numeraire_owner_verify_account_privileges<'me, 'info>(
    accounts: SetNumeraireOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_numeraire_owner_verify_writable_privileges(accounts)?;
    set_numeraire_owner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_NUMERAIRE_STATUS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetNumeraireStatusAccounts<'me, 'info> {
    pub numeraire_config: &'me AccountInfo<'info>,
    pub pair_mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetNumeraireStatusKeys {
    pub numeraire_config: Pubkey,
    pub pair_mint: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<SetNumeraireStatusAccounts<'_, '_>> for SetNumeraireStatusKeys {
    fn from(accounts: SetNumeraireStatusAccounts) -> Self {
        Self {
            numeraire_config: *accounts.numeraire_config.key,
            pair_mint: *accounts.pair_mint.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SetNumeraireStatusKeys>
for [AccountMeta; SET_NUMERAIRE_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetNumeraireStatusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pair_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
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
impl From<[Pubkey; SET_NUMERAIRE_STATUS_IX_ACCOUNTS_LEN]> for SetNumeraireStatusKeys {
    fn from(pubkeys: [Pubkey; SET_NUMERAIRE_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            numeraire_config: pubkeys[0],
            pair_mint: pubkeys[1],
            payer: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<SetNumeraireStatusAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_NUMERAIRE_STATUS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetNumeraireStatusAccounts<'_, 'info>) -> Self {
        [
            accounts.numeraire_config.clone(),
            accounts.pair_mint.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_NUMERAIRE_STATUS_IX_ACCOUNTS_LEN]>
for SetNumeraireStatusAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_NUMERAIRE_STATUS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            numeraire_config: &arr[0],
            pair_mint: &arr[1],
            payer: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const SET_NUMERAIRE_STATUS_IX_DISCM: [u8; 8usize] = [
    10, 17, 5, 71, 204, 171, 126, 173,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetNumeraireStatusIxArgs {
    pub data: SetStatusData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetNumeraireStatusIxData(pub SetNumeraireStatusIxArgs);
impl From<SetNumeraireStatusIxArgs> for SetNumeraireStatusIxData {
    fn from(args: SetNumeraireStatusIxArgs) -> Self {
        Self(args)
    }
}
impl SetNumeraireStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_NUMERAIRE_STATUS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SetStatusData>::deserialize(&mut reader)?
        };
        Ok(Self(SetNumeraireStatusIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_NUMERAIRE_STATUS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_numeraire_status_ix_with_program_id(
    program_id: Pubkey,
    keys: SetNumeraireStatusKeys,
    args: SetNumeraireStatusIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_NUMERAIRE_STATUS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetNumeraireStatusIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_numeraire_status_ix(
    keys: SetNumeraireStatusKeys,
    args: SetNumeraireStatusIxArgs,
) -> std::io::Result<Instruction> {
    set_numeraire_status_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn set_numeraire_status_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetNumeraireStatusAccounts<'_, '_>,
    args: SetNumeraireStatusIxArgs,
) -> ProgramResult {
    let keys: SetNumeraireStatusKeys = accounts.into();
    let ix = set_numeraire_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_numeraire_status_invoke(
    accounts: SetNumeraireStatusAccounts<'_, '_>,
    args: SetNumeraireStatusIxArgs,
) -> ProgramResult {
    set_numeraire_status_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn set_numeraire_status_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetNumeraireStatusAccounts<'_, '_>,
    args: SetNumeraireStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetNumeraireStatusKeys = accounts.into();
    let ix = set_numeraire_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_numeraire_status_invoke_signed(
    accounts: SetNumeraireStatusAccounts<'_, '_>,
    args: SetNumeraireStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_numeraire_status_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_numeraire_status_verify_account_keys(
    accounts: SetNumeraireStatusAccounts<'_, '_>,
    keys: SetNumeraireStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.pair_mint.key, keys.pair_mint),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_numeraire_status_verify_writable_privileges<'me, 'info>(
    accounts: SetNumeraireStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.numeraire_config, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_numeraire_status_verify_signer_privileges<'me, 'info>(
    accounts: SetNumeraireStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_numeraire_status_verify_account_privileges<'me, 'info>(
    accounts: SetNumeraireStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_numeraire_status_verify_writable_privileges(accounts)?;
    set_numeraire_status_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetNumeraireWhitelistedPoolCreatorAccounts<'me, 'info> {
    pub numeraire_config: &'me AccountInfo<'info>,
    pub pair_mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetNumeraireWhitelistedPoolCreatorKeys {
    pub numeraire_config: Pubkey,
    pub pair_mint: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<SetNumeraireWhitelistedPoolCreatorAccounts<'_, '_>>
for SetNumeraireWhitelistedPoolCreatorKeys {
    fn from(accounts: SetNumeraireWhitelistedPoolCreatorAccounts) -> Self {
        Self {
            numeraire_config: *accounts.numeraire_config.key,
            pair_mint: *accounts.pair_mint.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SetNumeraireWhitelistedPoolCreatorKeys>
for [AccountMeta; SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: SetNumeraireWhitelistedPoolCreatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pair_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
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
impl From<[Pubkey; SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_ACCOUNTS_LEN]>
for SetNumeraireWhitelistedPoolCreatorKeys {
    fn from(
        pubkeys: [Pubkey; SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            numeraire_config: pubkeys[0],
            pair_mint: pubkeys[1],
            payer: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<SetNumeraireWhitelistedPoolCreatorAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetNumeraireWhitelistedPoolCreatorAccounts<'_, 'info>) -> Self {
        [
            accounts.numeraire_config.clone(),
            accounts.pair_mint.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_ACCOUNTS_LEN]>
for SetNumeraireWhitelistedPoolCreatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            numeraire_config: &arr[0],
            pair_mint: &arr[1],
            payer: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_DISCM: [u8; 8usize] = [
    147, 51, 31, 255, 111, 2, 189, 173,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetNumeraireWhitelistedPoolCreatorIxArgs {
    pub data: SetWhilelistedAddrData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetNumeraireWhitelistedPoolCreatorIxData(
    pub SetNumeraireWhitelistedPoolCreatorIxArgs,
);
impl From<SetNumeraireWhitelistedPoolCreatorIxArgs>
for SetNumeraireWhitelistedPoolCreatorIxData {
    fn from(args: SetNumeraireWhitelistedPoolCreatorIxArgs) -> Self {
        Self(args)
    }
}
impl SetNumeraireWhitelistedPoolCreatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SetWhilelistedAddrData>::deserialize(&mut reader)?
        };
        Ok(
            Self(SetNumeraireWhitelistedPoolCreatorIxArgs {
                data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_numeraire_whitelisted_pool_creator_ix_with_program_id(
    program_id: Pubkey,
    keys: SetNumeraireWhitelistedPoolCreatorKeys,
    args: SetNumeraireWhitelistedPoolCreatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_NUMERAIRE_WHITELISTED_POOL_CREATOR_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetNumeraireWhitelistedPoolCreatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_numeraire_whitelisted_pool_creator_ix(
    keys: SetNumeraireWhitelistedPoolCreatorKeys,
    args: SetNumeraireWhitelistedPoolCreatorIxArgs,
) -> std::io::Result<Instruction> {
    set_numeraire_whitelisted_pool_creator_ix_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn set_numeraire_whitelisted_pool_creator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetNumeraireWhitelistedPoolCreatorAccounts<'_, '_>,
    args: SetNumeraireWhitelistedPoolCreatorIxArgs,
) -> ProgramResult {
    let keys: SetNumeraireWhitelistedPoolCreatorKeys = accounts.into();
    let ix = set_numeraire_whitelisted_pool_creator_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn set_numeraire_whitelisted_pool_creator_invoke(
    accounts: SetNumeraireWhitelistedPoolCreatorAccounts<'_, '_>,
    args: SetNumeraireWhitelistedPoolCreatorIxArgs,
) -> ProgramResult {
    set_numeraire_whitelisted_pool_creator_invoke_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_numeraire_whitelisted_pool_creator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetNumeraireWhitelistedPoolCreatorAccounts<'_, '_>,
    args: SetNumeraireWhitelistedPoolCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetNumeraireWhitelistedPoolCreatorKeys = accounts.into();
    let ix = set_numeraire_whitelisted_pool_creator_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_numeraire_whitelisted_pool_creator_invoke_signed(
    accounts: SetNumeraireWhitelistedPoolCreatorAccounts<'_, '_>,
    args: SetNumeraireWhitelistedPoolCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_numeraire_whitelisted_pool_creator_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_numeraire_whitelisted_pool_creator_verify_account_keys(
    accounts: SetNumeraireWhitelistedPoolCreatorAccounts<'_, '_>,
    keys: SetNumeraireWhitelistedPoolCreatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.pair_mint.key, keys.pair_mint),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_numeraire_whitelisted_pool_creator_verify_writable_privileges<'me, 'info>(
    accounts: SetNumeraireWhitelistedPoolCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.numeraire_config, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_numeraire_whitelisted_pool_creator_verify_signer_privileges<'me, 'info>(
    accounts: SetNumeraireWhitelistedPoolCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_numeraire_whitelisted_pool_creator_verify_account_privileges<'me, 'info>(
    accounts: SetNumeraireWhitelistedPoolCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_numeraire_whitelisted_pool_creator_verify_writable_privileges(accounts)?;
    set_numeraire_whitelisted_pool_creator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_OWNER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetOwnerAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetOwnerKeys {
    pub pool: Pubkey,
    pub payer: Pubkey,
}
impl From<SetOwnerAccounts<'_, '_>> for SetOwnerKeys {
    fn from(accounts: SetOwnerAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            payer: *accounts.payer.key,
        }
    }
}
impl From<SetOwnerKeys> for [AccountMeta; SET_OWNER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetOwnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_OWNER_IX_ACCOUNTS_LEN]> for SetOwnerKeys {
    fn from(pubkeys: [Pubkey; SET_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            payer: pubkeys[1],
        }
    }
}
impl<'info> From<SetOwnerAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_OWNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetOwnerAccounts<'_, 'info>) -> Self {
        [accounts.pool.clone(), accounts.payer.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_OWNER_IX_ACCOUNTS_LEN]>
for SetOwnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            payer: &arr[1],
        }
    }
}
pub const SET_OWNER_IX_DISCM: [u8; 8usize] = [72, 202, 120, 52, 77, 128, 96, 197];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetOwnerIxArgs {
    pub data: SetOwnerData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetOwnerIxData(pub SetOwnerIxArgs);
impl From<SetOwnerIxArgs> for SetOwnerIxData {
    fn from(args: SetOwnerIxArgs) -> Self {
        Self(args)
    }
}
impl SetOwnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_OWNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SetOwnerData>::deserialize(&mut reader)?
        };
        Ok(Self(SetOwnerIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_OWNER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_owner_ix_with_program_id(
    program_id: Pubkey,
    keys: SetOwnerKeys,
    args: SetOwnerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_OWNER_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetOwnerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_owner_ix(
    keys: SetOwnerKeys,
    args: SetOwnerIxArgs,
) -> std::io::Result<Instruction> {
    set_owner_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn set_owner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetOwnerAccounts<'_, '_>,
    args: SetOwnerIxArgs,
) -> ProgramResult {
    let keys: SetOwnerKeys = accounts.into();
    let ix = set_owner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_owner_invoke(
    accounts: SetOwnerAccounts<'_, '_>,
    args: SetOwnerIxArgs,
) -> ProgramResult {
    set_owner_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn set_owner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetOwnerAccounts<'_, '_>,
    args: SetOwnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetOwnerKeys = accounts.into();
    let ix = set_owner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_owner_invoke_signed(
    accounts: SetOwnerAccounts<'_, '_>,
    args: SetOwnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_owner_invoke_signed_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_owner_verify_account_keys(
    accounts: SetOwnerAccounts<'_, '_>,
    keys: SetOwnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.payer.key, keys.payer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_owner_verify_writable_privileges<'me, 'info>(
    accounts: SetOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_owner_verify_signer_privileges<'me, 'info>(
    accounts: SetOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_owner_verify_account_privileges<'me, 'info>(
    accounts: SetOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_owner_verify_writable_privileges(accounts)?;
    set_owner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_RATE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetRateAccounts<'me, 'info> {
    pub numeraire_config: &'me AccountInfo<'info>,
    pub pair_mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetRateKeys {
    pub numeraire_config: Pubkey,
    pub pair_mint: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<SetRateAccounts<'_, '_>> for SetRateKeys {
    fn from(accounts: SetRateAccounts) -> Self {
        Self {
            numeraire_config: *accounts.numeraire_config.key,
            pair_mint: *accounts.pair_mint.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SetRateKeys> for [AccountMeta; SET_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetRateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pair_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
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
impl From<[Pubkey; SET_RATE_IX_ACCOUNTS_LEN]> for SetRateKeys {
    fn from(pubkeys: [Pubkey; SET_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            numeraire_config: pubkeys[0],
            pair_mint: pubkeys[1],
            payer: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<SetRateAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_RATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetRateAccounts<'_, 'info>) -> Self {
        [
            accounts.numeraire_config.clone(),
            accounts.pair_mint.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_RATE_IX_ACCOUNTS_LEN]>
for SetRateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            numeraire_config: &arr[0],
            pair_mint: &arr[1],
            payer: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const SET_RATE_IX_DISCM: [u8; 8usize] = [99, 58, 170, 238, 160, 120, 74, 11];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetRateIxArgs {
    pub data: SetRateData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetRateIxData(pub SetRateIxArgs);
impl From<SetRateIxArgs> for SetRateIxData {
    fn from(args: SetRateIxArgs) -> Self {
        Self(args)
    }
}
impl SetRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_RATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SetRateData>::deserialize(&mut reader)?
        };
        Ok(Self(SetRateIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_RATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_rate_ix_with_program_id(
    program_id: Pubkey,
    keys: SetRateKeys,
    args: SetRateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_RATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetRateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_rate_ix(
    keys: SetRateKeys,
    args: SetRateIxArgs,
) -> std::io::Result<Instruction> {
    set_rate_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn set_rate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetRateAccounts<'_, '_>,
    args: SetRateIxArgs,
) -> ProgramResult {
    let keys: SetRateKeys = accounts.into();
    let ix = set_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_rate_invoke(
    accounts: SetRateAccounts<'_, '_>,
    args: SetRateIxArgs,
) -> ProgramResult {
    set_rate_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn set_rate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetRateAccounts<'_, '_>,
    args: SetRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetRateKeys = accounts.into();
    let ix = set_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_rate_invoke_signed(
    accounts: SetRateAccounts<'_, '_>,
    args: SetRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_rate_invoke_signed_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_rate_verify_account_keys(
    accounts: SetRateAccounts<'_, '_>,
    keys: SetRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.pair_mint.key, keys.pair_mint),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_rate_verify_writable_privileges<'me, 'info>(
    accounts: SetRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.numeraire_config, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_rate_verify_signer_privileges<'me, 'info>(
    accounts: SetRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_rate_verify_account_privileges<'me, 'info>(
    accounts: SetRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_rate_verify_writable_privileges(accounts)?;
    set_rate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_STATUS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetStatusAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetStatusKeys {
    pub pool: Pubkey,
    pub payer: Pubkey,
}
impl From<SetStatusAccounts<'_, '_>> for SetStatusKeys {
    fn from(accounts: SetStatusAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            payer: *accounts.payer.key,
        }
    }
}
impl From<SetStatusKeys> for [AccountMeta; SET_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetStatusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_STATUS_IX_ACCOUNTS_LEN]> for SetStatusKeys {
    fn from(pubkeys: [Pubkey; SET_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            payer: pubkeys[1],
        }
    }
}
impl<'info> From<SetStatusAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_STATUS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetStatusAccounts<'_, 'info>) -> Self {
        [accounts.pool.clone(), accounts.payer.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_STATUS_IX_ACCOUNTS_LEN]>
for SetStatusAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            payer: &arr[1],
        }
    }
}
pub const SET_STATUS_IX_DISCM: [u8; 8usize] = [181, 184, 224, 203, 193, 29, 177, 224];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetStatusIxArgs {
    pub data: SetStatusData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetStatusIxData(pub SetStatusIxArgs);
impl From<SetStatusIxArgs> for SetStatusIxData {
    fn from(args: SetStatusIxArgs) -> Self {
        Self(args)
    }
}
impl SetStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_STATUS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SetStatusData>::deserialize(&mut reader)?
        };
        Ok(Self(SetStatusIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_STATUS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_status_ix_with_program_id(
    program_id: Pubkey,
    keys: SetStatusKeys,
    args: SetStatusIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_STATUS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetStatusIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_status_ix(
    keys: SetStatusKeys,
    args: SetStatusIxArgs,
) -> std::io::Result<Instruction> {
    set_status_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn set_status_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetStatusAccounts<'_, '_>,
    args: SetStatusIxArgs,
) -> ProgramResult {
    let keys: SetStatusKeys = accounts.into();
    let ix = set_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_status_invoke(
    accounts: SetStatusAccounts<'_, '_>,
    args: SetStatusIxArgs,
) -> ProgramResult {
    set_status_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn set_status_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetStatusAccounts<'_, '_>,
    args: SetStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetStatusKeys = accounts.into();
    let ix = set_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_status_invoke_signed(
    accounts: SetStatusAccounts<'_, '_>,
    args: SetStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_status_invoke_signed_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_status_verify_account_keys(
    accounts: SetStatusAccounts<'_, '_>,
    keys: SetStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.payer.key, keys.payer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_status_verify_writable_privileges<'me, 'info>(
    accounts: SetStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_status_verify_signer_privileges<'me, 'info>(
    accounts: SetStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_status_verify_account_privileges<'me, 'info>(
    accounts: SetStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_status_verify_writable_privileges(accounts)?;
    set_status_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_WHITELISTED_ADDER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetWhitelistedAdderAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetWhitelistedAdderKeys {
    pub pool: Pubkey,
    pub payer: Pubkey,
}
impl From<SetWhitelistedAdderAccounts<'_, '_>> for SetWhitelistedAdderKeys {
    fn from(accounts: SetWhitelistedAdderAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            payer: *accounts.payer.key,
        }
    }
}
impl From<SetWhitelistedAdderKeys>
for [AccountMeta; SET_WHITELISTED_ADDER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetWhitelistedAdderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_WHITELISTED_ADDER_IX_ACCOUNTS_LEN]> for SetWhitelistedAdderKeys {
    fn from(pubkeys: [Pubkey; SET_WHITELISTED_ADDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            payer: pubkeys[1],
        }
    }
}
impl<'info> From<SetWhitelistedAdderAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_WHITELISTED_ADDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetWhitelistedAdderAccounts<'_, 'info>) -> Self {
        [accounts.pool.clone(), accounts.payer.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_WHITELISTED_ADDER_IX_ACCOUNTS_LEN]>
for SetWhitelistedAdderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_WHITELISTED_ADDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            payer: &arr[1],
        }
    }
}
pub const SET_WHITELISTED_ADDER_IX_DISCM: [u8; 8usize] = [
    74, 38, 140, 129, 228, 73, 236, 105,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetWhitelistedAdderIxArgs {
    pub data: SetWhilelistedAddrData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetWhitelistedAdderIxData(pub SetWhitelistedAdderIxArgs);
impl From<SetWhitelistedAdderIxArgs> for SetWhitelistedAdderIxData {
    fn from(args: SetWhitelistedAdderIxArgs) -> Self {
        Self(args)
    }
}
impl SetWhitelistedAdderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_WHITELISTED_ADDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SetWhilelistedAddrData>::deserialize(&mut reader)?
        };
        Ok(Self(SetWhitelistedAdderIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_WHITELISTED_ADDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_whitelisted_adder_ix_with_program_id(
    program_id: Pubkey,
    keys: SetWhitelistedAdderKeys,
    args: SetWhitelistedAdderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_WHITELISTED_ADDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetWhitelistedAdderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_whitelisted_adder_ix(
    keys: SetWhitelistedAdderKeys,
    args: SetWhitelistedAdderIxArgs,
) -> std::io::Result<Instruction> {
    set_whitelisted_adder_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn set_whitelisted_adder_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetWhitelistedAdderAccounts<'_, '_>,
    args: SetWhitelistedAdderIxArgs,
) -> ProgramResult {
    let keys: SetWhitelistedAdderKeys = accounts.into();
    let ix = set_whitelisted_adder_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_whitelisted_adder_invoke(
    accounts: SetWhitelistedAdderAccounts<'_, '_>,
    args: SetWhitelistedAdderIxArgs,
) -> ProgramResult {
    set_whitelisted_adder_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn set_whitelisted_adder_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetWhitelistedAdderAccounts<'_, '_>,
    args: SetWhitelistedAdderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetWhitelistedAdderKeys = accounts.into();
    let ix = set_whitelisted_adder_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_whitelisted_adder_invoke_signed(
    accounts: SetWhitelistedAdderAccounts<'_, '_>,
    args: SetWhitelistedAdderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_whitelisted_adder_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_whitelisted_adder_verify_account_keys(
    accounts: SetWhitelistedAdderAccounts<'_, '_>,
    keys: SetWhitelistedAdderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.payer.key, keys.payer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_whitelisted_adder_verify_writable_privileges<'me, 'info>(
    accounts: SetWhitelistedAdderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_whitelisted_adder_verify_signer_privileges<'me, 'info>(
    accounts: SetWhitelistedAdderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_whitelisted_adder_verify_account_privileges<'me, 'info>(
    accounts: SetWhitelistedAdderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_whitelisted_adder_verify_writable_privileges(accounts)?;
    set_whitelisted_adder_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SKIM_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct SkimAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub payer_lp_account: &'me AccountInfo<'info>,
    pub numeraire_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SkimKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub payer_lp_account: Pubkey,
    pub numeraire_config: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<SkimAccounts<'_, '_>> for SkimKeys {
    fn from(accounts: SkimAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            payer_lp_account: *accounts.payer_lp_account.key,
            numeraire_config: *accounts.numeraire_config.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<SkimKeys> for [AccountMeta; SKIM_IX_ACCOUNTS_LEN] {
    fn from(keys: SkimKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_lp_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SKIM_IX_ACCOUNTS_LEN]> for SkimKeys {
    fn from(pubkeys: [Pubkey; SKIM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            payer_lp_account: pubkeys[2],
            numeraire_config: pubkeys[3],
            payer: pubkeys[4],
            token_program: pubkeys[5],
            token_2022_program: pubkeys[6],
        }
    }
}
impl<'info> From<SkimAccounts<'_, 'info>>
for [AccountInfo<'info>; SKIM_IX_ACCOUNTS_LEN] {
    fn from(accounts: SkimAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.payer_lp_account.clone(),
            accounts.numeraire_config.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SKIM_IX_ACCOUNTS_LEN]>
for SkimAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SKIM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            payer_lp_account: &arr[2],
            numeraire_config: &arr[3],
            payer: &arr[4],
            token_program: &arr[5],
            token_2022_program: &arr[6],
        }
    }
}
pub const SKIM_IX_DISCM: [u8; 8usize] = [238, 120, 221, 138, 82, 60, 100, 218];
#[derive(Clone, Debug, PartialEq)]
pub struct SkimIxData;
impl SkimIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SKIM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SKIM_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn skim_ix_with_program_id(
    program_id: Pubkey,
    keys: SkimKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SKIM_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SkimIxData.try_to_vec()?,
    })
}
pub fn skim_ix(keys: SkimKeys) -> std::io::Result<Instruction> {
    skim_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys)
}
pub fn skim_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SkimAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SkimKeys = accounts.into();
    let ix = skim_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn skim_invoke(accounts: SkimAccounts<'_, '_>) -> ProgramResult {
    skim_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts)
}
pub fn skim_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SkimAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SkimKeys = accounts.into();
    let ix = skim_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn skim_invoke_signed(
    accounts: SkimAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    skim_invoke_signed_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, seeds)
}
pub fn skim_verify_account_keys(
    accounts: SkimAccounts<'_, '_>,
    keys: SkimKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.payer_lp_account.key, keys.payer_lp_account),
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn skim_verify_writable_privileges<'me, 'info>(
    accounts: SkimAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.payer_lp_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn skim_verify_signer_privileges<'me, 'info>(
    accounts: SkimAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn skim_verify_account_privileges<'me, 'info>(
    accounts: SkimAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    skim_verify_writable_privileges(accounts)?;
    skim_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_EXACT_IN_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct SwapExactInAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub in_mint: &'me AccountInfo<'info>,
    pub out_mint: &'me AccountInfo<'info>,
    pub in_trader: &'me AccountInfo<'info>,
    pub out_trader: &'me AccountInfo<'info>,
    pub in_vault: &'me AccountInfo<'info>,
    pub out_vault: &'me AccountInfo<'info>,
    pub numeraire_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapExactInKeys {
    pub pool: Pubkey,
    pub in_mint: Pubkey,
    pub out_mint: Pubkey,
    pub in_trader: Pubkey,
    pub out_trader: Pubkey,
    pub in_vault: Pubkey,
    pub out_vault: Pubkey,
    pub numeraire_config: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<SwapExactInAccounts<'_, '_>> for SwapExactInKeys {
    fn from(accounts: SwapExactInAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            in_mint: *accounts.in_mint.key,
            out_mint: *accounts.out_mint.key,
            in_trader: *accounts.in_trader.key,
            out_trader: *accounts.out_trader.key,
            in_vault: *accounts.in_vault.key,
            out_vault: *accounts.out_vault.key,
            numeraire_config: *accounts.numeraire_config.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<SwapExactInKeys> for [AccountMeta; SWAP_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapExactInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_trader,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_trader,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_EXACT_IN_IX_ACCOUNTS_LEN]> for SwapExactInKeys {
    fn from(pubkeys: [Pubkey; SWAP_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            in_mint: pubkeys[1],
            out_mint: pubkeys[2],
            in_trader: pubkeys[3],
            out_trader: pubkeys[4],
            in_vault: pubkeys[5],
            out_vault: pubkeys[6],
            numeraire_config: pubkeys[7],
            payer: pubkeys[8],
            token_program: pubkeys[9],
            token_2022_program: pubkeys[10],
        }
    }
}
impl<'info> From<SwapExactInAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapExactInAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.in_mint.clone(),
            accounts.out_mint.clone(),
            accounts.in_trader.clone(),
            accounts.out_trader.clone(),
            accounts.in_vault.clone(),
            accounts.out_vault.clone(),
            accounts.numeraire_config.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXACT_IN_IX_ACCOUNTS_LEN]>
for SwapExactInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            in_mint: &arr[1],
            out_mint: &arr[2],
            in_trader: &arr[3],
            out_trader: &arr[4],
            in_vault: &arr[5],
            out_vault: &arr[6],
            numeraire_config: &arr[7],
            payer: &arr[8],
            token_program: &arr[9],
            token_2022_program: &arr[10],
        }
    }
}
pub const SWAP_EXACT_IN_IX_DISCM: [u8; 8usize] = [104, 104, 131, 86, 161, 189, 180, 216];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExactInIxArgs {
    pub data: SwapExactInHintlessData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactInIxData(pub SwapExactInIxArgs);
impl From<SwapExactInIxArgs> for SwapExactInIxData {
    fn from(args: SwapExactInIxArgs) -> Self {
        Self(args)
    }
}
impl SwapExactInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_IN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SwapExactInHintlessData>::deserialize(&mut reader)?
        };
        Ok(Self(SwapExactInIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_IN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_exact_in_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapExactInKeys,
    args: SwapExactInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_EXACT_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapExactInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_in_ix(
    keys: SwapExactInKeys,
    args: SwapExactInIxArgs,
) -> std::io::Result<Instruction> {
    swap_exact_in_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn swap_exact_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactInAccounts<'_, '_>,
    args: SwapExactInIxArgs,
) -> ProgramResult {
    let keys: SwapExactInKeys = accounts.into();
    let ix = swap_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_exact_in_invoke(
    accounts: SwapExactInAccounts<'_, '_>,
    args: SwapExactInIxArgs,
) -> ProgramResult {
    swap_exact_in_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn swap_exact_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactInAccounts<'_, '_>,
    args: SwapExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapExactInKeys = accounts.into();
    let ix = swap_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_exact_in_invoke_signed(
    accounts: SwapExactInAccounts<'_, '_>,
    args: SwapExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exact_in_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_exact_in_verify_account_keys(
    accounts: SwapExactInAccounts<'_, '_>,
    keys: SwapExactInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.in_mint.key, keys.in_mint),
        (*accounts.out_mint.key, keys.out_mint),
        (*accounts.in_trader.key, keys.in_trader),
        (*accounts.out_trader.key, keys.out_trader),
        (*accounts.in_vault.key, keys.in_vault),
        (*accounts.out_vault.key, keys.out_vault),
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_exact_in_verify_writable_privileges<'me, 'info>(
    accounts: SwapExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.in_mint,
        accounts.out_mint,
        accounts.in_trader,
        accounts.out_trader,
        accounts.in_vault,
        accounts.out_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_exact_in_verify_signer_privileges<'me, 'info>(
    accounts: SwapExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_exact_in_verify_account_privileges<'me, 'info>(
    accounts: SwapExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_exact_in_verify_writable_privileges(accounts)?;
    swap_exact_in_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_EXACT_IN_HINTED_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct SwapExactInHintedAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub in_mint: &'me AccountInfo<'info>,
    pub out_mint: &'me AccountInfo<'info>,
    pub in_trader: &'me AccountInfo<'info>,
    pub out_trader: &'me AccountInfo<'info>,
    pub in_vault: &'me AccountInfo<'info>,
    pub out_vault: &'me AccountInfo<'info>,
    pub numeraire_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapExactInHintedKeys {
    pub pool: Pubkey,
    pub in_mint: Pubkey,
    pub out_mint: Pubkey,
    pub in_trader: Pubkey,
    pub out_trader: Pubkey,
    pub in_vault: Pubkey,
    pub out_vault: Pubkey,
    pub numeraire_config: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<SwapExactInHintedAccounts<'_, '_>> for SwapExactInHintedKeys {
    fn from(accounts: SwapExactInHintedAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            in_mint: *accounts.in_mint.key,
            out_mint: *accounts.out_mint.key,
            in_trader: *accounts.in_trader.key,
            out_trader: *accounts.out_trader.key,
            in_vault: *accounts.in_vault.key,
            out_vault: *accounts.out_vault.key,
            numeraire_config: *accounts.numeraire_config.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<SwapExactInHintedKeys>
for [AccountMeta; SWAP_EXACT_IN_HINTED_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapExactInHintedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_trader,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_trader,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_EXACT_IN_HINTED_IX_ACCOUNTS_LEN]> for SwapExactInHintedKeys {
    fn from(pubkeys: [Pubkey; SWAP_EXACT_IN_HINTED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            in_mint: pubkeys[1],
            out_mint: pubkeys[2],
            in_trader: pubkeys[3],
            out_trader: pubkeys[4],
            in_vault: pubkeys[5],
            out_vault: pubkeys[6],
            numeraire_config: pubkeys[7],
            payer: pubkeys[8],
            token_program: pubkeys[9],
            token_2022_program: pubkeys[10],
        }
    }
}
impl<'info> From<SwapExactInHintedAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_EXACT_IN_HINTED_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapExactInHintedAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.in_mint.clone(),
            accounts.out_mint.clone(),
            accounts.in_trader.clone(),
            accounts.out_trader.clone(),
            accounts.in_vault.clone(),
            accounts.out_vault.clone(),
            accounts.numeraire_config.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXACT_IN_HINTED_IX_ACCOUNTS_LEN]>
for SwapExactInHintedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_EXACT_IN_HINTED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            in_mint: &arr[1],
            out_mint: &arr[2],
            in_trader: &arr[3],
            out_trader: &arr[4],
            in_vault: &arr[5],
            out_vault: &arr[6],
            numeraire_config: &arr[7],
            payer: &arr[8],
            token_program: &arr[9],
            token_2022_program: &arr[10],
        }
    }
}
pub const SWAP_EXACT_IN_HINTED_IX_DISCM: [u8; 8usize] = [
    98, 239, 244, 233, 16, 236, 40, 49,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExactInHintedIxArgs {
    pub data: SwapExactInData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactInHintedIxData(pub SwapExactInHintedIxArgs);
impl From<SwapExactInHintedIxArgs> for SwapExactInHintedIxData {
    fn from(args: SwapExactInHintedIxArgs) -> Self {
        Self(args)
    }
}
impl SwapExactInHintedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_IN_HINTED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SwapExactInData>::deserialize(&mut reader)?
        };
        Ok(Self(SwapExactInHintedIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_IN_HINTED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_exact_in_hinted_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapExactInHintedKeys,
    args: SwapExactInHintedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_EXACT_IN_HINTED_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapExactInHintedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_in_hinted_ix(
    keys: SwapExactInHintedKeys,
    args: SwapExactInHintedIxArgs,
) -> std::io::Result<Instruction> {
    swap_exact_in_hinted_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn swap_exact_in_hinted_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactInHintedAccounts<'_, '_>,
    args: SwapExactInHintedIxArgs,
) -> ProgramResult {
    let keys: SwapExactInHintedKeys = accounts.into();
    let ix = swap_exact_in_hinted_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_exact_in_hinted_invoke(
    accounts: SwapExactInHintedAccounts<'_, '_>,
    args: SwapExactInHintedIxArgs,
) -> ProgramResult {
    swap_exact_in_hinted_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn swap_exact_in_hinted_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactInHintedAccounts<'_, '_>,
    args: SwapExactInHintedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapExactInHintedKeys = accounts.into();
    let ix = swap_exact_in_hinted_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_exact_in_hinted_invoke_signed(
    accounts: SwapExactInHintedAccounts<'_, '_>,
    args: SwapExactInHintedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exact_in_hinted_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_exact_in_hinted_verify_account_keys(
    accounts: SwapExactInHintedAccounts<'_, '_>,
    keys: SwapExactInHintedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.in_mint.key, keys.in_mint),
        (*accounts.out_mint.key, keys.out_mint),
        (*accounts.in_trader.key, keys.in_trader),
        (*accounts.out_trader.key, keys.out_trader),
        (*accounts.in_vault.key, keys.in_vault),
        (*accounts.out_vault.key, keys.out_vault),
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_exact_in_hinted_verify_writable_privileges<'me, 'info>(
    accounts: SwapExactInHintedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.in_mint,
        accounts.out_mint,
        accounts.in_trader,
        accounts.out_trader,
        accounts.in_vault,
        accounts.out_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_exact_in_hinted_verify_signer_privileges<'me, 'info>(
    accounts: SwapExactInHintedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_exact_in_hinted_verify_account_privileges<'me, 'info>(
    accounts: SwapExactInHintedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_exact_in_hinted_verify_writable_privileges(accounts)?;
    swap_exact_in_hinted_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_EXACT_IN_QUOTE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct SwapExactInQuoteAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub in_mint: &'me AccountInfo<'info>,
    pub out_mint: &'me AccountInfo<'info>,
    pub in_trader: &'me AccountInfo<'info>,
    pub out_trader: &'me AccountInfo<'info>,
    pub in_vault: &'me AccountInfo<'info>,
    pub out_vault: &'me AccountInfo<'info>,
    pub numeraire_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapExactInQuoteKeys {
    pub pool: Pubkey,
    pub in_mint: Pubkey,
    pub out_mint: Pubkey,
    pub in_trader: Pubkey,
    pub out_trader: Pubkey,
    pub in_vault: Pubkey,
    pub out_vault: Pubkey,
    pub numeraire_config: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<SwapExactInQuoteAccounts<'_, '_>> for SwapExactInQuoteKeys {
    fn from(accounts: SwapExactInQuoteAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            in_mint: *accounts.in_mint.key,
            out_mint: *accounts.out_mint.key,
            in_trader: *accounts.in_trader.key,
            out_trader: *accounts.out_trader.key,
            in_vault: *accounts.in_vault.key,
            out_vault: *accounts.out_vault.key,
            numeraire_config: *accounts.numeraire_config.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<SwapExactInQuoteKeys> for [AccountMeta; SWAP_EXACT_IN_QUOTE_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapExactInQuoteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_trader,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_trader,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_EXACT_IN_QUOTE_IX_ACCOUNTS_LEN]> for SwapExactInQuoteKeys {
    fn from(pubkeys: [Pubkey; SWAP_EXACT_IN_QUOTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            in_mint: pubkeys[1],
            out_mint: pubkeys[2],
            in_trader: pubkeys[3],
            out_trader: pubkeys[4],
            in_vault: pubkeys[5],
            out_vault: pubkeys[6],
            numeraire_config: pubkeys[7],
            payer: pubkeys[8],
            token_program: pubkeys[9],
            token_2022_program: pubkeys[10],
        }
    }
}
impl<'info> From<SwapExactInQuoteAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_EXACT_IN_QUOTE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapExactInQuoteAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.in_mint.clone(),
            accounts.out_mint.clone(),
            accounts.in_trader.clone(),
            accounts.out_trader.clone(),
            accounts.in_vault.clone(),
            accounts.out_vault.clone(),
            accounts.numeraire_config.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXACT_IN_QUOTE_IX_ACCOUNTS_LEN]>
for SwapExactInQuoteAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_EXACT_IN_QUOTE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            in_mint: &arr[1],
            out_mint: &arr[2],
            in_trader: &arr[3],
            out_trader: &arr[4],
            in_vault: &arr[5],
            out_vault: &arr[6],
            numeraire_config: &arr[7],
            payer: &arr[8],
            token_program: &arr[9],
            token_2022_program: &arr[10],
        }
    }
}
pub const SWAP_EXACT_IN_QUOTE_IX_DISCM: [u8; 8usize] = [
    68, 209, 177, 170, 185, 100, 29, 191,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExactInQuoteIxArgs {
    pub data: SwapExactInData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactInQuoteIxData(pub SwapExactInQuoteIxArgs);
impl From<SwapExactInQuoteIxArgs> for SwapExactInQuoteIxData {
    fn from(args: SwapExactInQuoteIxArgs) -> Self {
        Self(args)
    }
}
impl SwapExactInQuoteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_IN_QUOTE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SwapExactInData>::deserialize(&mut reader)?
        };
        Ok(Self(SwapExactInQuoteIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_IN_QUOTE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_exact_in_quote_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapExactInQuoteKeys,
    args: SwapExactInQuoteIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_EXACT_IN_QUOTE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapExactInQuoteIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_in_quote_ix(
    keys: SwapExactInQuoteKeys,
    args: SwapExactInQuoteIxArgs,
) -> std::io::Result<Instruction> {
    swap_exact_in_quote_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn swap_exact_in_quote_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactInQuoteAccounts<'_, '_>,
    args: SwapExactInQuoteIxArgs,
) -> ProgramResult {
    let keys: SwapExactInQuoteKeys = accounts.into();
    let ix = swap_exact_in_quote_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_exact_in_quote_invoke(
    accounts: SwapExactInQuoteAccounts<'_, '_>,
    args: SwapExactInQuoteIxArgs,
) -> ProgramResult {
    swap_exact_in_quote_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn swap_exact_in_quote_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactInQuoteAccounts<'_, '_>,
    args: SwapExactInQuoteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapExactInQuoteKeys = accounts.into();
    let ix = swap_exact_in_quote_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_exact_in_quote_invoke_signed(
    accounts: SwapExactInQuoteAccounts<'_, '_>,
    args: SwapExactInQuoteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exact_in_quote_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_exact_in_quote_verify_account_keys(
    accounts: SwapExactInQuoteAccounts<'_, '_>,
    keys: SwapExactInQuoteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.in_mint.key, keys.in_mint),
        (*accounts.out_mint.key, keys.out_mint),
        (*accounts.in_trader.key, keys.in_trader),
        (*accounts.out_trader.key, keys.out_trader),
        (*accounts.in_vault.key, keys.in_vault),
        (*accounts.out_vault.key, keys.out_vault),
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_exact_in_quote_verify_writable_privileges<'me, 'info>(
    accounts: SwapExactInQuoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.in_mint,
        accounts.out_mint,
        accounts.in_trader,
        accounts.out_trader,
        accounts.in_vault,
        accounts.out_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_exact_in_quote_verify_signer_privileges<'me, 'info>(
    accounts: SwapExactInQuoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_exact_in_quote_verify_account_privileges<'me, 'info>(
    accounts: SwapExactInQuoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_exact_in_quote_verify_writable_privileges(accounts)?;
    swap_exact_in_quote_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct SwapExactOutAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub in_mint: &'me AccountInfo<'info>,
    pub out_mint: &'me AccountInfo<'info>,
    pub in_trader: &'me AccountInfo<'info>,
    pub out_trader: &'me AccountInfo<'info>,
    pub in_vault: &'me AccountInfo<'info>,
    pub out_vault: &'me AccountInfo<'info>,
    pub numeraire_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapExactOutKeys {
    pub pool: Pubkey,
    pub in_mint: Pubkey,
    pub out_mint: Pubkey,
    pub in_trader: Pubkey,
    pub out_trader: Pubkey,
    pub in_vault: Pubkey,
    pub out_vault: Pubkey,
    pub numeraire_config: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<SwapExactOutAccounts<'_, '_>> for SwapExactOutKeys {
    fn from(accounts: SwapExactOutAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            in_mint: *accounts.in_mint.key,
            out_mint: *accounts.out_mint.key,
            in_trader: *accounts.in_trader.key,
            out_trader: *accounts.out_trader.key,
            in_vault: *accounts.in_vault.key,
            out_vault: *accounts.out_vault.key,
            numeraire_config: *accounts.numeraire_config.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<SwapExactOutKeys> for [AccountMeta; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapExactOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_trader,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_trader,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]> for SwapExactOutKeys {
    fn from(pubkeys: [Pubkey; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            in_mint: pubkeys[1],
            out_mint: pubkeys[2],
            in_trader: pubkeys[3],
            out_trader: pubkeys[4],
            in_vault: pubkeys[5],
            out_vault: pubkeys[6],
            numeraire_config: pubkeys[7],
            payer: pubkeys[8],
            token_program: pubkeys[9],
            token_2022_program: pubkeys[10],
        }
    }
}
impl<'info> From<SwapExactOutAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.in_mint.clone(),
            accounts.out_mint.clone(),
            accounts.in_trader.clone(),
            accounts.out_trader.clone(),
            accounts.in_vault.clone(),
            accounts.out_vault.clone(),
            accounts.numeraire_config.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]>
for SwapExactOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            in_mint: &arr[1],
            out_mint: &arr[2],
            in_trader: &arr[3],
            out_trader: &arr[4],
            in_vault: &arr[5],
            out_vault: &arr[6],
            numeraire_config: &arr[7],
            payer: &arr[8],
            token_program: &arr[9],
            token_2022_program: &arr[10],
        }
    }
}
pub const SWAP_EXACT_OUT_IX_DISCM: [u8; 8usize] = [250, 73, 101, 33, 38, 207, 75, 184];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExactOutIxArgs {
    pub data: SwapExactOutHintlessData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactOutIxData(pub SwapExactOutIxArgs);
impl From<SwapExactOutIxArgs> for SwapExactOutIxData {
    fn from(args: SwapExactOutIxArgs) -> Self {
        Self(args)
    }
}
impl SwapExactOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SwapExactOutHintlessData>::deserialize(&mut reader)?
        };
        Ok(Self(SwapExactOutIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_exact_out_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapExactOutKeys,
    args: SwapExactOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapExactOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_out_ix(
    keys: SwapExactOutKeys,
    args: SwapExactOutIxArgs,
) -> std::io::Result<Instruction> {
    swap_exact_out_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn swap_exact_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
) -> ProgramResult {
    let keys: SwapExactOutKeys = accounts.into();
    let ix = swap_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_exact_out_invoke(
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
) -> ProgramResult {
    swap_exact_out_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn swap_exact_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapExactOutKeys = accounts.into();
    let ix = swap_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_exact_out_invoke_signed(
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exact_out_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_exact_out_verify_account_keys(
    accounts: SwapExactOutAccounts<'_, '_>,
    keys: SwapExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.in_mint.key, keys.in_mint),
        (*accounts.out_mint.key, keys.out_mint),
        (*accounts.in_trader.key, keys.in_trader),
        (*accounts.out_trader.key, keys.out_trader),
        (*accounts.in_vault.key, keys.in_vault),
        (*accounts.out_vault.key, keys.out_vault),
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_writable_privileges<'me, 'info>(
    accounts: SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.in_mint,
        accounts.out_mint,
        accounts.in_trader,
        accounts.out_trader,
        accounts.in_vault,
        accounts.out_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_signer_privileges<'me, 'info>(
    accounts: SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_account_privileges<'me, 'info>(
    accounts: SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_exact_out_verify_writable_privileges(accounts)?;
    swap_exact_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_EXACT_OUT_HINTED_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct SwapExactOutHintedAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub in_mint: &'me AccountInfo<'info>,
    pub out_mint: &'me AccountInfo<'info>,
    pub in_trader: &'me AccountInfo<'info>,
    pub out_trader: &'me AccountInfo<'info>,
    pub in_vault: &'me AccountInfo<'info>,
    pub out_vault: &'me AccountInfo<'info>,
    pub numeraire_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapExactOutHintedKeys {
    pub pool: Pubkey,
    pub in_mint: Pubkey,
    pub out_mint: Pubkey,
    pub in_trader: Pubkey,
    pub out_trader: Pubkey,
    pub in_vault: Pubkey,
    pub out_vault: Pubkey,
    pub numeraire_config: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<SwapExactOutHintedAccounts<'_, '_>> for SwapExactOutHintedKeys {
    fn from(accounts: SwapExactOutHintedAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            in_mint: *accounts.in_mint.key,
            out_mint: *accounts.out_mint.key,
            in_trader: *accounts.in_trader.key,
            out_trader: *accounts.out_trader.key,
            in_vault: *accounts.in_vault.key,
            out_vault: *accounts.out_vault.key,
            numeraire_config: *accounts.numeraire_config.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<SwapExactOutHintedKeys>
for [AccountMeta; SWAP_EXACT_OUT_HINTED_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapExactOutHintedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_trader,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_trader,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.numeraire_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_EXACT_OUT_HINTED_IX_ACCOUNTS_LEN]> for SwapExactOutHintedKeys {
    fn from(pubkeys: [Pubkey; SWAP_EXACT_OUT_HINTED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            in_mint: pubkeys[1],
            out_mint: pubkeys[2],
            in_trader: pubkeys[3],
            out_trader: pubkeys[4],
            in_vault: pubkeys[5],
            out_vault: pubkeys[6],
            numeraire_config: pubkeys[7],
            payer: pubkeys[8],
            token_program: pubkeys[9],
            token_2022_program: pubkeys[10],
        }
    }
}
impl<'info> From<SwapExactOutHintedAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_EXACT_OUT_HINTED_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapExactOutHintedAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.in_mint.clone(),
            accounts.out_mint.clone(),
            accounts.in_trader.clone(),
            accounts.out_trader.clone(),
            accounts.in_vault.clone(),
            accounts.out_vault.clone(),
            accounts.numeraire_config.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXACT_OUT_HINTED_IX_ACCOUNTS_LEN]>
for SwapExactOutHintedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_EXACT_OUT_HINTED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            in_mint: &arr[1],
            out_mint: &arr[2],
            in_trader: &arr[3],
            out_trader: &arr[4],
            in_vault: &arr[5],
            out_vault: &arr[6],
            numeraire_config: &arr[7],
            payer: &arr[8],
            token_program: &arr[9],
            token_2022_program: &arr[10],
        }
    }
}
pub const SWAP_EXACT_OUT_HINTED_IX_DISCM: [u8; 8usize] = [
    153, 208, 206, 70, 62, 234, 98, 182,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExactOutHintedIxArgs {
    pub data: SwapExactOutData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactOutHintedIxData(pub SwapExactOutHintedIxArgs);
impl From<SwapExactOutHintedIxArgs> for SwapExactOutHintedIxData {
    fn from(args: SwapExactOutHintedIxArgs) -> Self {
        Self(args)
    }
}
impl SwapExactOutHintedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_OUT_HINTED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <SwapExactOutData>::deserialize(&mut reader)?
        };
        Ok(Self(SwapExactOutHintedIxArgs { data }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_OUT_HINTED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_exact_out_hinted_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapExactOutHintedKeys,
    args: SwapExactOutHintedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_EXACT_OUT_HINTED_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapExactOutHintedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_out_hinted_ix(
    keys: SwapExactOutHintedKeys,
    args: SwapExactOutHintedIxArgs,
) -> std::io::Result<Instruction> {
    swap_exact_out_hinted_ix_with_program_id(NUMERAIRE_PROGRAM_ID, keys, args)
}
pub fn swap_exact_out_hinted_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactOutHintedAccounts<'_, '_>,
    args: SwapExactOutHintedIxArgs,
) -> ProgramResult {
    let keys: SwapExactOutHintedKeys = accounts.into();
    let ix = swap_exact_out_hinted_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_exact_out_hinted_invoke(
    accounts: SwapExactOutHintedAccounts<'_, '_>,
    args: SwapExactOutHintedIxArgs,
) -> ProgramResult {
    swap_exact_out_hinted_invoke_with_program_id(NUMERAIRE_PROGRAM_ID, accounts, args)
}
pub fn swap_exact_out_hinted_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactOutHintedAccounts<'_, '_>,
    args: SwapExactOutHintedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapExactOutHintedKeys = accounts.into();
    let ix = swap_exact_out_hinted_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_exact_out_hinted_invoke_signed(
    accounts: SwapExactOutHintedAccounts<'_, '_>,
    args: SwapExactOutHintedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exact_out_hinted_invoke_signed_with_program_id(
        NUMERAIRE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_exact_out_hinted_verify_account_keys(
    accounts: SwapExactOutHintedAccounts<'_, '_>,
    keys: SwapExactOutHintedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.in_mint.key, keys.in_mint),
        (*accounts.out_mint.key, keys.out_mint),
        (*accounts.in_trader.key, keys.in_trader),
        (*accounts.out_trader.key, keys.out_trader),
        (*accounts.in_vault.key, keys.in_vault),
        (*accounts.out_vault.key, keys.out_vault),
        (*accounts.numeraire_config.key, keys.numeraire_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_exact_out_hinted_verify_writable_privileges<'me, 'info>(
    accounts: SwapExactOutHintedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.in_mint,
        accounts.out_mint,
        accounts.in_trader,
        accounts.out_trader,
        accounts.in_vault,
        accounts.out_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_exact_out_hinted_verify_signer_privileges<'me, 'info>(
    accounts: SwapExactOutHintedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_exact_out_hinted_verify_account_privileges<'me, 'info>(
    accounts: SwapExactOutHintedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_exact_out_hinted_verify_writable_privileges(accounts)?;
    swap_exact_out_hinted_verify_signer_privileges(accounts)?;
    Ok(())
}
