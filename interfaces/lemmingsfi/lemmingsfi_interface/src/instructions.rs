use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum LemmingsfiProgramIx {
    CreateMarket(CreateMarketIxArgs),
    Deposit(DepositIxArgs),
    Initialize(InitializeIxArgs),
    MigrateMarket,
    MigrateMarketV2,
    SetConfigAuthority(SetConfigAuthorityIxArgs),
    SetGlobalPaused(SetGlobalPausedIxArgs),
    SetMarketAuthority(SetMarketAuthorityIxArgs),
    SetOracleAuthority(SetOracleAuthorityIxArgs),
    Swap(SwapIxArgs),
    Swap2(Swap2IxArgs),
    UpdateOracle(UpdateOracleIxArgs),
    UpdateParams(UpdateParamsIxArgs),
    Withdraw(WithdrawIxArgs),
}
impl LemmingsfiProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CREATE_MARKET_IX_DISCM) {
            let mut reader = &buf[CREATE_MARKET_IX_DISCM.len()..];
            let field_0: u16 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u16 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u16 = crate::borsh_de_or_default(&mut reader)?;
            let field_3: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_5: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_6: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_7: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateMarket(CreateMarketIxArgs {
                    field_0,
                    field_1,
                    field_2,
                    field_3,
                    field_4,
                    field_5,
                    field_6,
                    field_7,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Deposit(DepositIxArgs { field_0, field_1 }));
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_IX_DISCM.len()..];
            let field_0: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Initialize(InitializeIxArgs { field_0 }));
        }
        if buf.starts_with(&MIGRATE_MARKET_IX_DISCM) {
            return Ok(Self::MigrateMarket);
        }
        if buf.starts_with(&MIGRATE_MARKET_V2_IX_DISCM) {
            return Ok(Self::MigrateMarketV2);
        }
        if buf.starts_with(&SET_CONFIG_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[SET_CONFIG_AUTHORITY_IX_DISCM.len()..];
            let field_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetConfigAuthority(SetConfigAuthorityIxArgs {
                    field_0,
                }),
            );
        }
        if buf.starts_with(&SET_GLOBAL_PAUSED_IX_DISCM) {
            let mut reader = &buf[SET_GLOBAL_PAUSED_IX_DISCM.len()..];
            let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetGlobalPaused(SetGlobalPausedIxArgs { field_0 }));
        }
        if buf.starts_with(&SET_MARKET_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[SET_MARKET_AUTHORITY_IX_DISCM.len()..];
            let field_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetMarketAuthority(SetMarketAuthorityIxArgs {
                    field_0,
                }),
            );
        }
        if buf.starts_with(&SET_ORACLE_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[SET_ORACLE_AUTHORITY_IX_DISCM.len()..];
            let field_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetOracleAuthority(SetOracleAuthorityIxArgs {
                    field_0,
                }),
            );
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    field_0,
                    field_1,
                    field_2,
                }),
            );
        }
        if buf.starts_with(&SWAP2_IX_DISCM) {
            let mut reader = &buf[SWAP2_IX_DISCM.len()..];
            let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_3: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap2(Swap2IxArgs {
                    field_0,
                    field_1,
                    field_2,
                    field_3,
                }),
            );
        }
        if buf.starts_with(&UPDATE_ORACLE_IX_DISCM) {
            let mut reader = &buf[UPDATE_ORACLE_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u16 = crate::borsh_de_or_default(&mut reader)?;
            let field_3: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateOracle(UpdateOracleIxArgs {
                    field_0,
                    field_1,
                    field_2,
                    field_3,
                }),
            );
        }
        if buf.starts_with(&UPDATE_PARAMS_IX_DISCM) {
            let mut reader = &buf[UPDATE_PARAMS_IX_DISCM.len()..];
            let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u8 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u8 = crate::borsh_de_or_default(&mut reader)?;
            let field_3: u8 = crate::borsh_de_or_default(&mut reader)?;
            let field_4: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateParams(UpdateParamsIxArgs {
                    field_0,
                    field_1,
                    field_2,
                    field_3,
                    field_4,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Withdraw(WithdrawIxArgs { field_0, field_1 }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateMarket(args) => {
                writer.write_all(&CREATE_MARKET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_3, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_4, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_5, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_6, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_7, &mut writer)?;
                Ok(())
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                Ok(())
            }
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                Ok(())
            }
            Self::MigrateMarket => writer.write_all(&MIGRATE_MARKET_IX_DISCM),
            Self::MigrateMarketV2 => writer.write_all(&MIGRATE_MARKET_V2_IX_DISCM),
            Self::SetConfigAuthority(args) => {
                writer.write_all(&SET_CONFIG_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                Ok(())
            }
            Self::SetGlobalPaused(args) => {
                writer.write_all(&SET_GLOBAL_PAUSED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                Ok(())
            }
            Self::SetMarketAuthority(args) => {
                writer.write_all(&SET_MARKET_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                Ok(())
            }
            Self::SetOracleAuthority(args) => {
                writer.write_all(&SET_ORACLE_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                Ok(())
            }
            Self::Swap2(args) => {
                writer.write_all(&SWAP2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_3, &mut writer)?;
                Ok(())
            }
            Self::UpdateOracle(args) => {
                writer.write_all(&UPDATE_ORACLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_3, &mut writer)?;
                Ok(())
            }
            Self::UpdateParams(args) => {
                writer.write_all(&UPDATE_PARAMS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_3, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_4, &mut writer)?;
                Ok(())
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
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
pub const CREATE_MARKET_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CreateMarketAccounts<'me, 'info> {
    pub vault_quote: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub vault_base: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMarketKeys {
    pub vault_quote: Pubkey,
    pub market: Pubkey,
    pub vault_base: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub authority: Pubkey,
    pub global_config: Pubkey,
}
impl From<CreateMarketAccounts<'_, '_>> for CreateMarketKeys {
    fn from(accounts: CreateMarketAccounts) -> Self {
        Self {
            vault_quote: *accounts.vault_quote.key,
            market: *accounts.market.key,
            vault_base: *accounts.vault_base.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            authority: *accounts.authority.key,
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<CreateMarketKeys> for [AccountMeta; CREATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_base,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_MARKET_IX_ACCOUNTS_LEN]> for CreateMarketKeys {
    fn from(pubkeys: [Pubkey; CREATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault_quote: pubkeys[0],
            market: pubkeys[1],
            vault_base: pubkeys[2],
            token_program: pubkeys[3],
            system_program: pubkeys[4],
            rent: pubkeys[5],
            authority: pubkeys[6],
            global_config: pubkeys[7],
        }
    }
}
impl<'info> From<CreateMarketAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.vault_quote.clone(),
            accounts.market.clone(),
            accounts.vault_base.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.authority.clone(),
            accounts.global_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN]>
for CreateMarketAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault_quote: &arr[0],
            market: &arr[1],
            vault_base: &arr[2],
            token_program: &arr[3],
            system_program: &arr[4],
            rent: &arr[5],
            authority: &arr[6],
            global_config: &arr[7],
        }
    }
}
pub const CREATE_MARKET_IX_DISCM: [u8; 8usize] = [103, 226, 97, 235, 200, 188, 251, 254];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMarketIxArgs {
    pub field_0: u16,
    pub field_1: u16,
    pub field_2: u16,
    pub field_3: u64,
    pub field_4: u64,
    pub field_5: u64,
    pub field_6: u64,
    pub field_7: u16,
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
        let field_0: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_3: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_5: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_6: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_7: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateMarketIxArgs {
                field_0,
                field_1,
                field_2,
                field_3,
                field_4,
                field_5,
                field_6,
                field_7,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_MARKET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_6, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_7, &mut writer)?;
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
    create_market_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys, args)
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
    create_market_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args)
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
    create_market_invoke_signed_with_program_id(
        LEMMINGSFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_market_verify_account_keys(
    accounts: CreateMarketAccounts<'_, '_>,
    keys: CreateMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault_quote.key, keys.vault_quote),
        (*accounts.market.key, keys.market),
        (*accounts.vault_base.key, keys.vault_base),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.authority.key, keys.authority),
        (*accounts.global_config.key, keys.global_config),
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
    for should_be_writable in [accounts.market, accounts.vault_base] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_market_verify_account_privileges<'me, 'info>(
    accounts: CreateMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_market_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub authority_quote: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub vault_base: &'me AccountInfo<'info>,
    pub vault_quote: &'me AccountInfo<'info>,
    pub authority_base: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub authority_quote: Pubkey,
    pub token_program: Pubkey,
    pub authority: Pubkey,
    pub market: Pubkey,
    pub vault_base: Pubkey,
    pub vault_quote: Pubkey,
    pub authority_base: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            authority_quote: *accounts.authority_quote.key,
            token_program: *accounts.token_program.key,
            authority: *accounts.authority.key,
            market: *accounts.market.key,
            vault_base: *accounts.vault_base.key,
            vault_quote: *accounts.vault_quote.key,
            authority_base: *accounts.authority_base.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_base,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority_quote: pubkeys[0],
            token_program: pubkeys[1],
            authority: pubkeys[2],
            market: pubkeys[3],
            vault_base: pubkeys[4],
            vault_quote: pubkeys[5],
            authority_base: pubkeys[6],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.authority_quote.clone(),
            accounts.token_program.clone(),
            accounts.authority.clone(),
            accounts.market.clone(),
            accounts.vault_base.clone(),
            accounts.vault_quote.clone(),
            accounts.authority_base.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority_quote: &arr[0],
            token_program: &arr[1],
            authority: &arr[2],
            market: &arr[3],
            vault_base: &arr[4],
            vault_quote: &arr[5],
            authority_base: &arr[6],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub field_0: u64,
    pub field_1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositIxData(pub DepositIxArgs);
impl From<DepositIxArgs> for DepositIxData {
    fn from(args: DepositIxArgs) -> Self {
        Self(args)
    }
}
impl DepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DepositIxArgs { field_0, field_1 }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositKeys,
    args: DepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_ix(
    keys: DepositKeys,
    args: DepositIxArgs,
) -> std::io::Result<Instruction> {
    deposit_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys, args)
}
pub fn deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
) -> ProgramResult {
    let keys: DepositKeys = accounts.into();
    let ix = deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_invoke(
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
) -> ProgramResult {
    deposit_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args)
}
pub fn deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositKeys = accounts.into();
    let ix = deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_invoke_signed(
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_invoke_signed_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority_quote.key, keys.authority_quote),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.authority.key, keys.authority),
        (*accounts.market.key, keys.market),
        (*accounts.vault_base.key, keys.vault_base),
        (*accounts.vault_quote.key, keys.vault_quote),
        (*accounts.authority_base.key, keys.authority_base),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_verify_writable_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority_quote] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_verify_account_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub vault_quote: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub vault_base: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub vault_quote: Pubkey,
    pub market: Pubkey,
    pub vault_base: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub authority: Pubkey,
    pub global_config: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            vault_quote: *accounts.vault_quote.key,
            market: *accounts.market.key,
            vault_base: *accounts.vault_base.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            authority: *accounts.authority.key,
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_base,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault_quote: pubkeys[0],
            market: pubkeys[1],
            vault_base: pubkeys[2],
            token_program: pubkeys[3],
            system_program: pubkeys[4],
            rent: pubkeys[5],
            authority: pubkeys[6],
            global_config: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.vault_quote.clone(),
            accounts.market.clone(),
            accounts.vault_base.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.authority.clone(),
            accounts.global_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault_quote: &arr[0],
            market: &arr[1],
            vault_base: &arr[2],
            token_program: &arr[3],
            system_program: &arr[4],
            rent: &arr[5],
            authority: &arr[6],
            global_config: &arr[7],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8usize] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeIxArgs {
    pub field_0: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData(pub InitializeIxArgs);
impl From<InitializeIxArgs> for InitializeIxData {
    fn from(args: InitializeIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(InitializeIxArgs { field_0 }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_ix(
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys, args)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    initialize_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(
        LEMMINGSFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault_quote.key, keys.vault_quote),
        (*accounts.market.key, keys.market),
        (*accounts.vault_base.key, keys.vault_base),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.authority.key, keys.authority),
        (*accounts.global_config.key, keys.global_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market, accounts.vault_base] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_MARKET_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct MigrateMarketAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateMarketKeys {
    pub payer: Pubkey,
    pub global_config: Pubkey,
    pub market: Pubkey,
}
impl From<MigrateMarketAccounts<'_, '_>> for MigrateMarketKeys {
    fn from(accounts: MigrateMarketAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            global_config: *accounts.global_config.key,
            market: *accounts.market.key,
        }
    }
}
impl From<MigrateMarketKeys> for [AccountMeta; MIGRATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MIGRATE_MARKET_IX_ACCOUNTS_LEN]> for MigrateMarketKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            global_config: pubkeys[1],
            market: pubkeys[2],
        }
    }
}
impl<'info> From<MigrateMarketAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateMarketAccounts<'_, 'info>) -> Self {
        [accounts.payer.clone(), accounts.global_config.clone(), accounts.market.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_MARKET_IX_ACCOUNTS_LEN]>
for MigrateMarketAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            global_config: &arr[1],
            market: &arr[2],
        }
    }
}
pub const MIGRATE_MARKET_IX_DISCM: [u8; 8usize] = [
    201, 113, 181, 120, 217, 60, 109, 203,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateMarketIxData;
impl MigrateMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_MARKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_MARKET_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_market_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateMarketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_MARKET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateMarketIxData.try_to_vec()?,
    })
}
pub fn migrate_market_ix(keys: MigrateMarketKeys) -> std::io::Result<Instruction> {
    migrate_market_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys)
}
pub fn migrate_market_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateMarketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateMarketKeys = accounts.into();
    let ix = migrate_market_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_market_invoke(accounts: MigrateMarketAccounts<'_, '_>) -> ProgramResult {
    migrate_market_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts)
}
pub fn migrate_market_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateMarketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateMarketKeys = accounts.into();
    let ix = migrate_market_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_market_invoke_signed(
    accounts: MigrateMarketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_market_invoke_signed_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, seeds)
}
pub fn migrate_market_verify_account_keys(
    accounts: MigrateMarketAccounts<'_, '_>,
    keys: MigrateMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.market.key, keys.market),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const MIGRATE_MARKET_V2_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct MigrateMarketV2Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateMarketV2Keys {
    pub payer: Pubkey,
    pub global_config: Pubkey,
    pub market: Pubkey,
}
impl From<MigrateMarketV2Accounts<'_, '_>> for MigrateMarketV2Keys {
    fn from(accounts: MigrateMarketV2Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            global_config: *accounts.global_config.key,
            market: *accounts.market.key,
        }
    }
}
impl From<MigrateMarketV2Keys> for [AccountMeta; MIGRATE_MARKET_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateMarketV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MIGRATE_MARKET_V2_IX_ACCOUNTS_LEN]> for MigrateMarketV2Keys {
    fn from(pubkeys: [Pubkey; MIGRATE_MARKET_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            global_config: pubkeys[1],
            market: pubkeys[2],
        }
    }
}
impl<'info> From<MigrateMarketV2Accounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_MARKET_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateMarketV2Accounts<'_, 'info>) -> Self {
        [accounts.payer.clone(), accounts.global_config.clone(), accounts.market.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_MARKET_V2_IX_ACCOUNTS_LEN]>
for MigrateMarketV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_MARKET_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            global_config: &arr[1],
            market: &arr[2],
        }
    }
}
pub const MIGRATE_MARKET_V2_IX_DISCM: [u8; 8usize] = [
    53, 150, 38, 89, 165, 103, 103, 59,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateMarketV2IxData;
impl MigrateMarketV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_MARKET_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_MARKET_V2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_market_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateMarketV2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_MARKET_V2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateMarketV2IxData.try_to_vec()?,
    })
}
pub fn migrate_market_v2_ix(keys: MigrateMarketV2Keys) -> std::io::Result<Instruction> {
    migrate_market_v2_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys)
}
pub fn migrate_market_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateMarketV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateMarketV2Keys = accounts.into();
    let ix = migrate_market_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_market_v2_invoke(
    accounts: MigrateMarketV2Accounts<'_, '_>,
) -> ProgramResult {
    migrate_market_v2_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts)
}
pub fn migrate_market_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateMarketV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateMarketV2Keys = accounts.into();
    let ix = migrate_market_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_market_v2_invoke_signed(
    accounts: MigrateMarketV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_market_v2_invoke_signed_with_program_id(
        LEMMINGSFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn migrate_market_v2_verify_account_keys(
    accounts: MigrateMarketV2Accounts<'_, '_>,
    keys: MigrateMarketV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.market.key, keys.market),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const SET_CONFIG_AUTHORITY_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct SetConfigAuthorityAccounts<'me, 'info> {
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetConfigAuthorityKeys {
    pub global_config: Pubkey,
}
impl From<SetConfigAuthorityAccounts<'_, '_>> for SetConfigAuthorityKeys {
    fn from(accounts: SetConfigAuthorityAccounts) -> Self {
        Self {
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<SetConfigAuthorityKeys>
for [AccountMeta; SET_CONFIG_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetConfigAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_CONFIG_AUTHORITY_IX_ACCOUNTS_LEN]> for SetConfigAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_CONFIG_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self { global_config: pubkeys[0] }
    }
}
impl<'info> From<SetConfigAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_CONFIG_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetConfigAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.global_config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_CONFIG_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetConfigAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_CONFIG_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self { global_config: &arr[0] }
    }
}
pub const SET_CONFIG_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    16, 200, 212, 18, 95, 43, 107, 89,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetConfigAuthorityIxArgs {
    pub field_0: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetConfigAuthorityIxData(pub SetConfigAuthorityIxArgs);
impl From<SetConfigAuthorityIxArgs> for SetConfigAuthorityIxData {
    fn from(args: SetConfigAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl SetConfigAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_CONFIG_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetConfigAuthorityIxArgs {
                field_0,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_CONFIG_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_config_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetConfigAuthorityKeys,
    args: SetConfigAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_CONFIG_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetConfigAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_config_authority_ix(
    keys: SetConfigAuthorityKeys,
    args: SetConfigAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    set_config_authority_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys, args)
}
pub fn set_config_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetConfigAuthorityAccounts<'_, '_>,
    args: SetConfigAuthorityIxArgs,
) -> ProgramResult {
    let keys: SetConfigAuthorityKeys = accounts.into();
    let ix = set_config_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_config_authority_invoke(
    accounts: SetConfigAuthorityAccounts<'_, '_>,
    args: SetConfigAuthorityIxArgs,
) -> ProgramResult {
    set_config_authority_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args)
}
pub fn set_config_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetConfigAuthorityAccounts<'_, '_>,
    args: SetConfigAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetConfigAuthorityKeys = accounts.into();
    let ix = set_config_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_config_authority_invoke_signed(
    accounts: SetConfigAuthorityAccounts<'_, '_>,
    args: SetConfigAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_config_authority_invoke_signed_with_program_id(
        LEMMINGSFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_config_authority_verify_account_keys(
    accounts: SetConfigAuthorityAccounts<'_, '_>,
    keys: SetConfigAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.global_config.key, keys.global_config)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const SET_GLOBAL_PAUSED_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct SetGlobalPausedAccounts<'me, 'info> {
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetGlobalPausedKeys {
    pub global_config: Pubkey,
}
impl From<SetGlobalPausedAccounts<'_, '_>> for SetGlobalPausedKeys {
    fn from(accounts: SetGlobalPausedAccounts) -> Self {
        Self {
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<SetGlobalPausedKeys> for [AccountMeta; SET_GLOBAL_PAUSED_IX_ACCOUNTS_LEN] {
    fn from(keys: SetGlobalPausedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_GLOBAL_PAUSED_IX_ACCOUNTS_LEN]> for SetGlobalPausedKeys {
    fn from(pubkeys: [Pubkey; SET_GLOBAL_PAUSED_IX_ACCOUNTS_LEN]) -> Self {
        Self { global_config: pubkeys[0] }
    }
}
impl<'info> From<SetGlobalPausedAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_GLOBAL_PAUSED_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetGlobalPausedAccounts<'_, 'info>) -> Self {
        [accounts.global_config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_GLOBAL_PAUSED_IX_ACCOUNTS_LEN]>
for SetGlobalPausedAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_GLOBAL_PAUSED_IX_ACCOUNTS_LEN]) -> Self {
        Self { global_config: &arr[0] }
    }
}
pub const SET_GLOBAL_PAUSED_IX_DISCM: [u8; 8usize] = [
    148, 120, 202, 2, 211, 211, 10, 109,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetGlobalPausedIxArgs {
    pub field_0: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetGlobalPausedIxData(pub SetGlobalPausedIxArgs);
impl From<SetGlobalPausedIxArgs> for SetGlobalPausedIxData {
    fn from(args: SetGlobalPausedIxArgs) -> Self {
        Self(args)
    }
}
impl SetGlobalPausedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_GLOBAL_PAUSED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetGlobalPausedIxArgs { field_0 }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_GLOBAL_PAUSED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_global_paused_ix_with_program_id(
    program_id: Pubkey,
    keys: SetGlobalPausedKeys,
    args: SetGlobalPausedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_GLOBAL_PAUSED_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetGlobalPausedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_global_paused_ix(
    keys: SetGlobalPausedKeys,
    args: SetGlobalPausedIxArgs,
) -> std::io::Result<Instruction> {
    set_global_paused_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys, args)
}
pub fn set_global_paused_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetGlobalPausedAccounts<'_, '_>,
    args: SetGlobalPausedIxArgs,
) -> ProgramResult {
    let keys: SetGlobalPausedKeys = accounts.into();
    let ix = set_global_paused_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_global_paused_invoke(
    accounts: SetGlobalPausedAccounts<'_, '_>,
    args: SetGlobalPausedIxArgs,
) -> ProgramResult {
    set_global_paused_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args)
}
pub fn set_global_paused_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetGlobalPausedAccounts<'_, '_>,
    args: SetGlobalPausedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetGlobalPausedKeys = accounts.into();
    let ix = set_global_paused_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_global_paused_invoke_signed(
    accounts: SetGlobalPausedAccounts<'_, '_>,
    args: SetGlobalPausedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_global_paused_invoke_signed_with_program_id(
        LEMMINGSFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_global_paused_verify_account_keys(
    accounts: SetGlobalPausedAccounts<'_, '_>,
    keys: SetGlobalPausedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.global_config.key, keys.global_config)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const SET_MARKET_AUTHORITY_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct SetMarketAuthorityAccounts<'me, 'info> {
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMarketAuthorityKeys {
    pub global_config: Pubkey,
}
impl From<SetMarketAuthorityAccounts<'_, '_>> for SetMarketAuthorityKeys {
    fn from(accounts: SetMarketAuthorityAccounts) -> Self {
        Self {
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<SetMarketAuthorityKeys>
for [AccountMeta; SET_MARKET_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMarketAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_MARKET_AUTHORITY_IX_ACCOUNTS_LEN]> for SetMarketAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_MARKET_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self { global_config: pubkeys[0] }
    }
}
impl<'info> From<SetMarketAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_MARKET_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMarketAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.global_config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MARKET_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetMarketAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_MARKET_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self { global_config: &arr[0] }
    }
}
pub const SET_MARKET_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    124, 100, 155, 213, 122, 121, 177, 115,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetMarketAuthorityIxArgs {
    pub field_0: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetMarketAuthorityIxData(pub SetMarketAuthorityIxArgs);
impl From<SetMarketAuthorityIxArgs> for SetMarketAuthorityIxData {
    fn from(args: SetMarketAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl SetMarketAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_MARKET_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetMarketAuthorityIxArgs {
                field_0,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_MARKET_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_market_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMarketAuthorityKeys,
    args: SetMarketAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MARKET_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetMarketAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_market_authority_ix(
    keys: SetMarketAuthorityKeys,
    args: SetMarketAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    set_market_authority_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys, args)
}
pub fn set_market_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMarketAuthorityAccounts<'_, '_>,
    args: SetMarketAuthorityIxArgs,
) -> ProgramResult {
    let keys: SetMarketAuthorityKeys = accounts.into();
    let ix = set_market_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_market_authority_invoke(
    accounts: SetMarketAuthorityAccounts<'_, '_>,
    args: SetMarketAuthorityIxArgs,
) -> ProgramResult {
    set_market_authority_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args)
}
pub fn set_market_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMarketAuthorityAccounts<'_, '_>,
    args: SetMarketAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetMarketAuthorityKeys = accounts.into();
    let ix = set_market_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_market_authority_invoke_signed(
    accounts: SetMarketAuthorityAccounts<'_, '_>,
    args: SetMarketAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_market_authority_invoke_signed_with_program_id(
        LEMMINGSFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_market_authority_verify_account_keys(
    accounts: SetMarketAuthorityAccounts<'_, '_>,
    keys: SetMarketAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.global_config.key, keys.global_config)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const SET_ORACLE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct SetOracleAuthorityAccounts<'me, 'info> {
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetOracleAuthorityKeys {
    pub global_config: Pubkey,
}
impl From<SetOracleAuthorityAccounts<'_, '_>> for SetOracleAuthorityKeys {
    fn from(accounts: SetOracleAuthorityAccounts) -> Self {
        Self {
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<SetOracleAuthorityKeys>
for [AccountMeta; SET_ORACLE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetOracleAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_ORACLE_AUTHORITY_IX_ACCOUNTS_LEN]> for SetOracleAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_ORACLE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self { global_config: pubkeys[0] }
    }
}
impl<'info> From<SetOracleAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_ORACLE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetOracleAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.global_config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_ORACLE_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetOracleAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_ORACLE_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self { global_config: &arr[0] }
    }
}
pub const SET_ORACLE_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    39, 155, 66, 106, 213, 226, 114, 174,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetOracleAuthorityIxArgs {
    pub field_0: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetOracleAuthorityIxData(pub SetOracleAuthorityIxArgs);
impl From<SetOracleAuthorityIxArgs> for SetOracleAuthorityIxData {
    fn from(args: SetOracleAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl SetOracleAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_ORACLE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetOracleAuthorityIxArgs {
                field_0,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_ORACLE_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_oracle_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetOracleAuthorityKeys,
    args: SetOracleAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_ORACLE_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetOracleAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_oracle_authority_ix(
    keys: SetOracleAuthorityKeys,
    args: SetOracleAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    set_oracle_authority_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys, args)
}
pub fn set_oracle_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetOracleAuthorityAccounts<'_, '_>,
    args: SetOracleAuthorityIxArgs,
) -> ProgramResult {
    let keys: SetOracleAuthorityKeys = accounts.into();
    let ix = set_oracle_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_oracle_authority_invoke(
    accounts: SetOracleAuthorityAccounts<'_, '_>,
    args: SetOracleAuthorityIxArgs,
) -> ProgramResult {
    set_oracle_authority_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args)
}
pub fn set_oracle_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetOracleAuthorityAccounts<'_, '_>,
    args: SetOracleAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetOracleAuthorityKeys = accounts.into();
    let ix = set_oracle_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_oracle_authority_invoke_signed(
    accounts: SetOracleAuthorityAccounts<'_, '_>,
    args: SetOracleAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_oracle_authority_invoke_signed_with_program_id(
        LEMMINGSFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_oracle_authority_verify_account_keys(
    accounts: SetOracleAuthorityAccounts<'_, '_>,
    keys: SetOracleAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.global_config.key, keys.global_config)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub user_base: &'me AccountInfo<'info>,
    pub user_quote: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub score_signer: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub vault_base: &'me AccountInfo<'info>,
    pub vault_quote: &'me AccountInfo<'info>,
    pub authority_quote: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_base: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub user_base: Pubkey,
    pub user_quote: Pubkey,
    pub token_program: Pubkey,
    pub score_signer: Pubkey,
    pub global_config: Pubkey,
    pub market: Pubkey,
    pub vault_base: Pubkey,
    pub vault_quote: Pubkey,
    pub authority_quote: Pubkey,
    pub authority: Pubkey,
    pub authority_base: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            user_base: *accounts.user_base.key,
            user_quote: *accounts.user_quote.key,
            token_program: *accounts.token_program.key,
            score_signer: *accounts.score_signer.key,
            global_config: *accounts.global_config.key,
            market: *accounts.market.key,
            vault_base: *accounts.vault_base.key,
            vault_quote: *accounts.vault_quote.key,
            authority_quote: *accounts.authority_quote.key,
            authority: *accounts.authority.key,
            authority_base: *accounts.authority_base.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.score_signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_base,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_base: pubkeys[0],
            user_quote: pubkeys[1],
            token_program: pubkeys[2],
            score_signer: pubkeys[3],
            global_config: pubkeys[4],
            market: pubkeys[5],
            vault_base: pubkeys[6],
            vault_quote: pubkeys[7],
            authority_quote: pubkeys[8],
            authority: pubkeys[9],
            authority_base: pubkeys[10],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.user_base.clone(),
            accounts.user_quote.clone(),
            accounts.token_program.clone(),
            accounts.score_signer.clone(),
            accounts.global_config.clone(),
            accounts.market.clone(),
            accounts.vault_base.clone(),
            accounts.vault_quote.clone(),
            accounts.authority_quote.clone(),
            accounts.authority.clone(),
            accounts.authority_base.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_base: &arr[0],
            user_quote: &arr[1],
            token_program: &arr[2],
            score_signer: &arr[3],
            global_config: &arr[4],
            market: &arr[5],
            vault_base: &arr[6],
            vault_quote: &arr[7],
            authority_quote: &arr[8],
            authority: &arr[9],
            authority_base: &arr[10],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub field_0: u8,
    pub field_1: u64,
    pub field_2: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapIxData(pub SwapIxArgs);
impl From<SwapIxArgs> for SwapIxData {
    fn from(args: SwapIxArgs) -> Self {
        Self(args)
    }
}
impl SwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                field_0,
                field_1,
                field_2,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapKeys,
    args: SwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_ix(keys: SwapKeys, args: SwapIxArgs) -> std::io::Result<Instruction> {
    swap_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys, args)
}
pub fn swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_invoke(accounts: SwapAccounts<'_, '_>, args: SwapIxArgs) -> ProgramResult {
    swap_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args)
}
pub fn swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_invoke_signed(
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_invoke_signed_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_base.key, keys.user_base),
        (*accounts.user_quote.key, keys.user_quote),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.score_signer.key, keys.score_signer),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.market.key, keys.market),
        (*accounts.vault_base.key, keys.vault_base),
        (*accounts.vault_quote.key, keys.vault_quote),
        (*accounts.authority_quote.key, keys.authority_quote),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_base.key, keys.authority_base),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_verify_writable_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_base,
        accounts.user_quote,
        accounts.vault_base,
        accounts.vault_quote,
        accounts.authority_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_verify_signer_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.score_signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_verify_account_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_verify_writable_privileges(accounts)?;
    swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP2_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct Swap2Accounts<'me, 'info> {
    pub user_base: &'me AccountInfo<'info>,
    pub user_quote: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub score_signer: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub vault_base: &'me AccountInfo<'info>,
    pub vault_quote: &'me AccountInfo<'info>,
    pub authority_quote: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_base: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Swap2Keys {
    pub user_base: Pubkey,
    pub user_quote: Pubkey,
    pub token_program: Pubkey,
    pub score_signer: Pubkey,
    pub global_config: Pubkey,
    pub market: Pubkey,
    pub vault_base: Pubkey,
    pub vault_quote: Pubkey,
    pub authority_quote: Pubkey,
    pub authority: Pubkey,
    pub authority_base: Pubkey,
}
impl From<Swap2Accounts<'_, '_>> for Swap2Keys {
    fn from(accounts: Swap2Accounts) -> Self {
        Self {
            user_base: *accounts.user_base.key,
            user_quote: *accounts.user_quote.key,
            token_program: *accounts.token_program.key,
            score_signer: *accounts.score_signer.key,
            global_config: *accounts.global_config.key,
            market: *accounts.market.key,
            vault_base: *accounts.vault_base.key,
            vault_quote: *accounts.vault_quote.key,
            authority_quote: *accounts.authority_quote.key,
            authority: *accounts.authority.key,
            authority_base: *accounts.authority_base.key,
        }
    }
}
impl From<Swap2Keys> for [AccountMeta; SWAP2_IX_ACCOUNTS_LEN] {
    fn from(keys: Swap2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.score_signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_base,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP2_IX_ACCOUNTS_LEN]> for Swap2Keys {
    fn from(pubkeys: [Pubkey; SWAP2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_base: pubkeys[0],
            user_quote: pubkeys[1],
            token_program: pubkeys[2],
            score_signer: pubkeys[3],
            global_config: pubkeys[4],
            market: pubkeys[5],
            vault_base: pubkeys[6],
            vault_quote: pubkeys[7],
            authority_quote: pubkeys[8],
            authority: pubkeys[9],
            authority_base: pubkeys[10],
        }
    }
}
impl<'info> From<Swap2Accounts<'_, 'info>>
for [AccountInfo<'info>; SWAP2_IX_ACCOUNTS_LEN] {
    fn from(accounts: Swap2Accounts<'_, 'info>) -> Self {
        [
            accounts.user_base.clone(),
            accounts.user_quote.clone(),
            accounts.token_program.clone(),
            accounts.score_signer.clone(),
            accounts.global_config.clone(),
            accounts.market.clone(),
            accounts.vault_base.clone(),
            accounts.vault_quote.clone(),
            accounts.authority_quote.clone(),
            accounts.authority.clone(),
            accounts.authority_base.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP2_IX_ACCOUNTS_LEN]>
for Swap2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_base: &arr[0],
            user_quote: &arr[1],
            token_program: &arr[2],
            score_signer: &arr[3],
            global_config: &arr[4],
            market: &arr[5],
            vault_base: &arr[6],
            vault_quote: &arr[7],
            authority_quote: &arr[8],
            authority: &arr[9],
            authority_base: &arr[10],
        }
    }
}
pub const SWAP2_IX_DISCM: [u8; 8usize] = [65, 75, 63, 76, 235, 91, 91, 136];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Swap2IxArgs {
    pub field_0: u8,
    pub field_1: u64,
    pub field_2: u64,
    pub field_3: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Swap2IxData(pub Swap2IxArgs);
impl From<Swap2IxArgs> for Swap2IxData {
    fn from(args: Swap2IxArgs) -> Self {
        Self(args)
    }
}
impl Swap2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_3: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(Swap2IxArgs {
                field_0,
                field_1,
                field_2,
                field_3,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_3, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap2_ix_with_program_id(
    program_id: Pubkey,
    keys: Swap2Keys,
    args: Swap2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP2_IX_ACCOUNTS_LEN] = keys.into();
    let data: Swap2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap2_ix(keys: Swap2Keys, args: Swap2IxArgs) -> std::io::Result<Instruction> {
    swap2_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys, args)
}
pub fn swap2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: Swap2Accounts<'_, '_>,
    args: Swap2IxArgs,
) -> ProgramResult {
    let keys: Swap2Keys = accounts.into();
    let ix = swap2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap2_invoke(
    accounts: Swap2Accounts<'_, '_>,
    args: Swap2IxArgs,
) -> ProgramResult {
    swap2_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args)
}
pub fn swap2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: Swap2Accounts<'_, '_>,
    args: Swap2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: Swap2Keys = accounts.into();
    let ix = swap2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap2_invoke_signed(
    accounts: Swap2Accounts<'_, '_>,
    args: Swap2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap2_invoke_signed_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap2_verify_account_keys(
    accounts: Swap2Accounts<'_, '_>,
    keys: Swap2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_base.key, keys.user_base),
        (*accounts.user_quote.key, keys.user_quote),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.score_signer.key, keys.score_signer),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.market.key, keys.market),
        (*accounts.vault_base.key, keys.vault_base),
        (*accounts.vault_quote.key, keys.vault_quote),
        (*accounts.authority_quote.key, keys.authority_quote),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_base.key, keys.authority_base),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap2_verify_writable_privileges<'me, 'info>(
    accounts: Swap2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_base,
        accounts.user_quote,
        accounts.vault_base,
        accounts.vault_quote,
        accounts.authority_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap2_verify_signer_privileges<'me, 'info>(
    accounts: Swap2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.score_signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap2_verify_account_privileges<'me, 'info>(
    accounts: Swap2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap2_verify_writable_privileges(accounts)?;
    swap2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_ORACLE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct UpdateOracleAccounts<'me, 'info> {
    pub user_base: &'me AccountInfo<'info>,
    pub user_quote: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub vault_base: &'me AccountInfo<'info>,
    pub vault_quote: &'me AccountInfo<'info>,
    pub score_signer: &'me AccountInfo<'info>,
    pub authority_quote: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_base: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateOracleKeys {
    pub user_base: Pubkey,
    pub user_quote: Pubkey,
    pub token_program: Pubkey,
    pub global_config: Pubkey,
    pub market: Pubkey,
    pub vault_base: Pubkey,
    pub vault_quote: Pubkey,
    pub score_signer: Pubkey,
    pub authority_quote: Pubkey,
    pub authority: Pubkey,
    pub authority_base: Pubkey,
}
impl From<UpdateOracleAccounts<'_, '_>> for UpdateOracleKeys {
    fn from(accounts: UpdateOracleAccounts) -> Self {
        Self {
            user_base: *accounts.user_base.key,
            user_quote: *accounts.user_quote.key,
            token_program: *accounts.token_program.key,
            global_config: *accounts.global_config.key,
            market: *accounts.market.key,
            vault_base: *accounts.vault_base.key,
            vault_quote: *accounts.vault_quote.key,
            score_signer: *accounts.score_signer.key,
            authority_quote: *accounts.authority_quote.key,
            authority: *accounts.authority.key,
            authority_base: *accounts.authority_base.key,
        }
    }
}
impl From<UpdateOracleKeys> for [AccountMeta; UPDATE_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateOracleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.score_signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_base,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_ORACLE_IX_ACCOUNTS_LEN]> for UpdateOracleKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_base: pubkeys[0],
            user_quote: pubkeys[1],
            token_program: pubkeys[2],
            global_config: pubkeys[3],
            market: pubkeys[4],
            vault_base: pubkeys[5],
            vault_quote: pubkeys[6],
            score_signer: pubkeys[7],
            authority_quote: pubkeys[8],
            authority: pubkeys[9],
            authority_base: pubkeys[10],
        }
    }
}
impl<'info> From<UpdateOracleAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateOracleAccounts<'_, 'info>) -> Self {
        [
            accounts.user_base.clone(),
            accounts.user_quote.clone(),
            accounts.token_program.clone(),
            accounts.global_config.clone(),
            accounts.market.clone(),
            accounts.vault_base.clone(),
            accounts.vault_quote.clone(),
            accounts.score_signer.clone(),
            accounts.authority_quote.clone(),
            accounts.authority.clone(),
            accounts.authority_base.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ORACLE_IX_ACCOUNTS_LEN]>
for UpdateOracleAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_base: &arr[0],
            user_quote: &arr[1],
            token_program: &arr[2],
            global_config: &arr[3],
            market: &arr[4],
            vault_base: &arr[5],
            vault_quote: &arr[6],
            score_signer: &arr[7],
            authority_quote: &arr[8],
            authority: &arr[9],
            authority_base: &arr[10],
        }
    }
}
pub const UPDATE_ORACLE_IX_DISCM: [u8; 8usize] = [112, 41, 209, 18, 248, 226, 252, 188];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOracleIxArgs {
    pub field_0: u64,
    pub field_1: u64,
    pub field_2: u16,
    pub field_3: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOracleIxData(pub UpdateOracleIxArgs);
impl From<UpdateOracleIxArgs> for UpdateOracleIxData {
    fn from(args: UpdateOracleIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateOracleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ORACLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_3: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateOracleIxArgs {
                field_0,
                field_1,
                field_2,
                field_3,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ORACLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_3, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_oracle_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateOracleKeys,
    args: UpdateOracleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_ORACLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateOracleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_oracle_ix(
    keys: UpdateOracleKeys,
    args: UpdateOracleIxArgs,
) -> std::io::Result<Instruction> {
    update_oracle_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys, args)
}
pub fn update_oracle_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOracleAccounts<'_, '_>,
    args: UpdateOracleIxArgs,
) -> ProgramResult {
    let keys: UpdateOracleKeys = accounts.into();
    let ix = update_oracle_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_oracle_invoke(
    accounts: UpdateOracleAccounts<'_, '_>,
    args: UpdateOracleIxArgs,
) -> ProgramResult {
    update_oracle_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args)
}
pub fn update_oracle_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOracleAccounts<'_, '_>,
    args: UpdateOracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateOracleKeys = accounts.into();
    let ix = update_oracle_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_oracle_invoke_signed(
    accounts: UpdateOracleAccounts<'_, '_>,
    args: UpdateOracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_oracle_invoke_signed_with_program_id(
        LEMMINGSFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_oracle_verify_account_keys(
    accounts: UpdateOracleAccounts<'_, '_>,
    keys: UpdateOracleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_base.key, keys.user_base),
        (*accounts.user_quote.key, keys.user_quote),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.market.key, keys.market),
        (*accounts.vault_base.key, keys.vault_base),
        (*accounts.vault_quote.key, keys.vault_quote),
        (*accounts.score_signer.key, keys.score_signer),
        (*accounts.authority_quote.key, keys.authority_quote),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_base.key, keys.authority_base),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_oracle_verify_writable_privileges<'me, 'info>(
    accounts: UpdateOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_base,
        accounts.user_quote,
        accounts.market,
        accounts.vault_base,
        accounts.vault_quote,
        accounts.authority_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_oracle_verify_signer_privileges<'me, 'info>(
    accounts: UpdateOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.score_signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_oracle_verify_account_privileges<'me, 'info>(
    accounts: UpdateOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_oracle_verify_writable_privileges(accounts)?;
    update_oracle_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PARAMS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateParamsAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateParamsKeys {
    pub payer: Pubkey,
    pub global_config: Pubkey,
    pub market: Pubkey,
}
impl From<UpdateParamsAccounts<'_, '_>> for UpdateParamsKeys {
    fn from(accounts: UpdateParamsAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            global_config: *accounts.global_config.key,
            market: *accounts.market.key,
        }
    }
}
impl From<UpdateParamsKeys> for [AccountMeta; UPDATE_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateParamsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_PARAMS_IX_ACCOUNTS_LEN]> for UpdateParamsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            global_config: pubkeys[1],
            market: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateParamsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateParamsAccounts<'_, 'info>) -> Self {
        [accounts.payer.clone(), accounts.global_config.clone(), accounts.market.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PARAMS_IX_ACCOUNTS_LEN]>
for UpdateParamsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            global_config: &arr[1],
            market: &arr[2],
        }
    }
}
pub const UPDATE_PARAMS_IX_DISCM: [u8; 8usize] = [108, 178, 190, 95, 94, 203, 116, 20];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateParamsIxArgs {
    pub field_0: u8,
    pub field_1: u8,
    pub field_2: u8,
    pub field_3: u8,
    pub field_4: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateParamsIxData(pub UpdateParamsIxArgs);
impl From<UpdateParamsIxArgs> for UpdateParamsIxData {
    fn from(args: UpdateParamsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateParamsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PARAMS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_3: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_4: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateParamsIxArgs {
                field_0,
                field_1,
                field_2,
                field_3,
                field_4,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PARAMS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_4, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_params_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateParamsKeys,
    args: UpdateParamsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PARAMS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateParamsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_params_ix(
    keys: UpdateParamsKeys,
    args: UpdateParamsIxArgs,
) -> std::io::Result<Instruction> {
    update_params_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys, args)
}
pub fn update_params_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateParamsAccounts<'_, '_>,
    args: UpdateParamsIxArgs,
) -> ProgramResult {
    let keys: UpdateParamsKeys = accounts.into();
    let ix = update_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_params_invoke(
    accounts: UpdateParamsAccounts<'_, '_>,
    args: UpdateParamsIxArgs,
) -> ProgramResult {
    update_params_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args)
}
pub fn update_params_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateParamsAccounts<'_, '_>,
    args: UpdateParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateParamsKeys = accounts.into();
    let ix = update_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_params_invoke_signed(
    accounts: UpdateParamsAccounts<'_, '_>,
    args: UpdateParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_params_invoke_signed_with_program_id(
        LEMMINGSFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_params_verify_account_keys(
    accounts: UpdateParamsAccounts<'_, '_>,
    keys: UpdateParamsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.market.key, keys.market),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub authority_quote: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub vault_base: &'me AccountInfo<'info>,
    pub vault_quote: &'me AccountInfo<'info>,
    pub authority_base: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub authority_quote: Pubkey,
    pub token_program: Pubkey,
    pub authority: Pubkey,
    pub market: Pubkey,
    pub vault_base: Pubkey,
    pub vault_quote: Pubkey,
    pub authority_base: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            authority_quote: *accounts.authority_quote.key,
            token_program: *accounts.token_program.key,
            authority: *accounts.authority.key,
            market: *accounts.market.key,
            vault_base: *accounts.vault_base.key,
            vault_quote: *accounts.vault_quote.key,
            authority_base: *accounts.authority_base.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_base,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority_quote: pubkeys[0],
            token_program: pubkeys[1],
            authority: pubkeys[2],
            market: pubkeys[3],
            vault_base: pubkeys[4],
            vault_quote: pubkeys[5],
            authority_base: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.authority_quote.clone(),
            accounts.token_program.clone(),
            accounts.authority.clone(),
            accounts.market.clone(),
            accounts.vault_base.clone(),
            accounts.vault_quote.clone(),
            accounts.authority_base.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority_quote: &arr[0],
            token_program: &arr[1],
            authority: &arr[2],
            market: &arr[3],
            vault_base: &arr[4],
            vault_quote: &arr[5],
            authority_base: &arr[6],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8usize] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIxArgs {
    pub field_0: u64,
    pub field_1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawIxData(pub WithdrawIxArgs);
impl From<WithdrawIxArgs> for WithdrawIxData {
    fn from(args: WithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawIxArgs { field_0, field_1 }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawKeys,
    args: WithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_ix(
    keys: WithdrawKeys,
    args: WithdrawIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_ix_with_program_id(LEMMINGSFI_PROGRAM_ID, keys, args)
}
pub fn withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
) -> ProgramResult {
    let keys: WithdrawKeys = accounts.into();
    let ix = withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_invoke(
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
) -> ProgramResult {
    withdraw_invoke_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args)
}
pub fn withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawKeys = accounts.into();
    let ix = withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_invoke_signed(
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_invoke_signed_with_program_id(LEMMINGSFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority_quote.key, keys.authority_quote),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.authority.key, keys.authority),
        (*accounts.market.key, keys.market),
        (*accounts.vault_base.key, keys.vault_base),
        (*accounts.vault_quote.key, keys.vault_quote),
        (*accounts.authority_base.key, keys.authority_base),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority_quote] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_verify_account_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_verify_writable_privileges(accounts)?;
    Ok(())
}
