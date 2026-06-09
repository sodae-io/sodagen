use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum TokenMillProgramIx {
    CreateConfig(CreateConfigIxArgs),
    CreateMarket(CreateMarketIxArgs),
    ForceRemoveFeeReserve,
    RemoveSwapAuthority,
    Swap(SwapIxArgs),
    SwapWithPriceLimit(SwapWithPriceLimitIxArgs),
    TransferConfigOwnership(TransferConfigOwnershipIxArgs),
    UpdateConfigSettings(UpdateConfigSettingsIxArgs),
    UpdateFeeReserve,
    UpdateMarketDefaults(UpdateMarketDefaultsIxArgs),
}
impl TokenMillProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CREATE_CONFIG_IX_DISCM) {
            let mut reader = &buf[CREATE_CONFIG_IX_DISCM.len()..];
            let quote_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let protocol_fee_share: u32 = crate::borsh_de_or_default(&mut reader)?;
            let protocol_fee_token_account: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let kotm_fee_token_account: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let fee_recipient_change_cooldown: u32 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let market_settings = if reader.is_empty() {
                Default::default()
            } else {
                <MarketSettingsInput>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateConfig(CreateConfigIxArgs {
                    quote_token_mint,
                    protocol_fee_share,
                    protocol_fee_token_account,
                    kotm_fee_token_account,
                    fee_recipient_change_cooldown,
                    market_settings,
                }),
            );
        }
        if buf.starts_with(&CREATE_MARKET_IX_DISCM) {
            let mut reader = &buf[CREATE_MARKET_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            let swap_authority: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::CreateMarket(CreateMarketIxArgs {
                    name,
                    symbol,
                    uri,
                    swap_authority,
                }),
            );
        }
        if buf.starts_with(&FORCE_REMOVE_FEE_RESERVE_IX_DISCM) {
            return Ok(Self::ForceRemoveFeeReserve);
        }
        if buf.starts_with(&REMOVE_SWAP_AUTHORITY_IX_DISCM) {
            return Ok(Self::RemoveSwapAuthority);
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let swap_parameters = <SwapParameters as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            return Ok(Self::Swap(SwapIxArgs { swap_parameters }));
        }
        if buf.starts_with(&SWAP_WITH_PRICE_LIMIT_IX_DISCM) {
            let mut reader = &buf[SWAP_WITH_PRICE_LIMIT_IX_DISCM.len()..];
            let zero_for_one: bool = crate::borsh_de_or_default(&mut reader)?;
            let delta_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
            let sqrt_price_limit_x96: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapWithPriceLimit(SwapWithPriceLimitIxArgs {
                    zero_for_one,
                    delta_amount,
                    sqrt_price_limit_x96,
                }),
            );
        }
        if buf.starts_with(&TRANSFER_CONFIG_OWNERSHIP_IX_DISCM) {
            let mut reader = &buf[TRANSFER_CONFIG_OWNERSHIP_IX_DISCM.len()..];
            let new_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TransferConfigOwnership(TransferConfigOwnershipIxArgs {
                    new_admin,
                }),
            );
        }
        if buf.starts_with(&UPDATE_CONFIG_SETTINGS_IX_DISCM) {
            let mut reader = &buf[UPDATE_CONFIG_SETTINGS_IX_DISCM.len()..];
            let new_protocol_fee_share: u32 = crate::borsh_de_or_default(&mut reader)?;
            let new_fee_recipient_change_cooldown: u32 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateConfigSettings(UpdateConfigSettingsIxArgs {
                    new_protocol_fee_share,
                    new_fee_recipient_change_cooldown,
                }),
            );
        }
        if buf.starts_with(&UPDATE_FEE_RESERVE_IX_DISCM) {
            return Ok(Self::UpdateFeeReserve);
        }
        if buf.starts_with(&UPDATE_MARKET_DEFAULTS_IX_DISCM) {
            let mut reader = &buf[UPDATE_MARKET_DEFAULTS_IX_DISCM.len()..];
            let market_settings = if reader.is_empty() {
                Default::default()
            } else {
                <MarketSettingsInput>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateMarketDefaults(UpdateMarketDefaultsIxArgs {
                    market_settings,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateConfig(args) => {
                writer.write_all(&CREATE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.quote_token_mint, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.protocol_fee_share, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.protocol_fee_token_account,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.kotm_fee_token_account,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.fee_recipient_change_cooldown,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.market_settings, &mut writer)?;
                Ok(())
            }
            Self::CreateMarket(args) => {
                writer.write_all(&CREATE_MARKET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.swap_authority, &mut writer)?;
                Ok(())
            }
            Self::ForceRemoveFeeReserve => {
                writer.write_all(&FORCE_REMOVE_FEE_RESERVE_IX_DISCM)
            }
            Self::RemoveSwapAuthority => {
                writer.write_all(&REMOVE_SWAP_AUTHORITY_IX_DISCM)
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.swap_parameters, &mut writer)?;
                Ok(())
            }
            Self::SwapWithPriceLimit(args) => {
                writer.write_all(&SWAP_WITH_PRICE_LIMIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.zero_for_one, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.delta_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.sqrt_price_limit_x96,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::TransferConfigOwnership(args) => {
                writer.write_all(&TRANSFER_CONFIG_OWNERSHIP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_admin, &mut writer)?;
                Ok(())
            }
            Self::UpdateConfigSettings(args) => {
                writer.write_all(&UPDATE_CONFIG_SETTINGS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_protocol_fee_share,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.new_fee_recipient_change_cooldown,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateFeeReserve => writer.write_all(&UPDATE_FEE_RESERVE_IX_DISCM),
            Self::UpdateMarketDefaults(args) => {
                writer.write_all(&UPDATE_MARKET_DEFAULTS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.market_settings, &mut writer)?;
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
pub const CREATE_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateConfigAccounts<'me, 'info> {
    pub token_mill_config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateConfigKeys {
    pub token_mill_config: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateConfigAccounts<'_, '_>> for CreateConfigKeys {
    fn from(accounts: CreateConfigAccounts) -> Self {
        Self {
            token_mill_config: *accounts.token_mill_config.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateConfigKeys> for [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_mill_config,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]> for CreateConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_mill_config: pubkeys[0],
            admin: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.token_mill_config.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]>
for CreateConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_mill_config: &arr[0],
            admin: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CREATE_CONFIG_IX_DISCM: [u8; 8usize] = [201, 207, 243, 114, 75, 111, 47, 189];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateConfigIxArgs {
    pub quote_token_mint: Pubkey,
    pub protocol_fee_share: u32,
    pub protocol_fee_token_account: Pubkey,
    pub kotm_fee_token_account: Pubkey,
    pub fee_recipient_change_cooldown: u32,
    pub market_settings: MarketSettingsInput,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateConfigIxData(pub CreateConfigIxArgs);
impl From<CreateConfigIxArgs> for CreateConfigIxData {
    fn from(args: CreateConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CreateConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let quote_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_share: u32 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_token_account: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let kotm_fee_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_recipient_change_cooldown: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let market_settings = if reader.is_empty() {
            Default::default()
        } else {
            <MarketSettingsInput>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateConfigIxArgs {
                quote_token_mint,
                protocol_fee_share,
                protocol_fee_token_account,
                kotm_fee_token_account,
                fee_recipient_change_cooldown,
                market_settings,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.quote_token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.protocol_fee_share, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.protocol_fee_token_account,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.kotm_fee_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.fee_recipient_change_cooldown,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.market_settings, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateConfigKeys,
    args: CreateConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_config_ix(
    keys: CreateConfigKeys,
    args: CreateConfigIxArgs,
) -> std::io::Result<Instruction> {
    create_config_ix_with_program_id(TOKEN_MILL_PROGRAM_ID, keys, args)
}
pub fn create_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
) -> ProgramResult {
    let keys: CreateConfigKeys = accounts.into();
    let ix = create_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_config_invoke(
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
) -> ProgramResult {
    create_config_invoke_with_program_id(TOKEN_MILL_PROGRAM_ID, accounts, args)
}
pub fn create_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateConfigKeys = accounts.into();
    let ix = create_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_config_invoke_signed(
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_config_invoke_signed_with_program_id(
        TOKEN_MILL_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_config_verify_account_keys(
    accounts: CreateConfigAccounts<'_, '_>,
    keys: CreateConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_mill_config.key, keys.token_mill_config),
        (*accounts.admin.key, keys.admin),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_config_verify_writable_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_mill_config, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.token_mill_config, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_config_verify_account_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_config_verify_writable_privileges(accounts)?;
    create_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_MARKET_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct CreateMarketAccounts<'me, 'info> {
    pub token_mill_config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub token_mint_0: &'me AccountInfo<'info>,
    pub market_reserve_0: &'me AccountInfo<'info>,
    pub token_0_metadata: &'me AccountInfo<'info>,
    pub token_mint_1: &'me AccountInfo<'info>,
    pub market_reserve_1: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMarketKeys {
    pub token_mill_config: Pubkey,
    pub market: Pubkey,
    pub token_mint_0: Pubkey,
    pub market_reserve_0: Pubkey,
    pub token_0_metadata: Pubkey,
    pub token_mint_1: Pubkey,
    pub market_reserve_1: Pubkey,
    pub creator: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub token_metadata_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateMarketAccounts<'_, '_>> for CreateMarketKeys {
    fn from(accounts: CreateMarketAccounts) -> Self {
        Self {
            token_mill_config: *accounts.token_mill_config.key,
            market: *accounts.market.key,
            token_mint_0: *accounts.token_mint_0.key,
            market_reserve_0: *accounts.market_reserve_0.key,
            token_0_metadata: *accounts.token_0_metadata.key,
            token_mint_1: *accounts.token_mint_1.key,
            market_reserve_1: *accounts.market_reserve_1.key,
            creator: *accounts.creator.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            token_metadata_program: *accounts.token_metadata_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateMarketKeys> for [AccountMeta; CREATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_mill_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_0,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_reserve_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_reserve_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
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
                pubkey: keys.token_metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_MARKET_IX_ACCOUNTS_LEN]> for CreateMarketKeys {
    fn from(pubkeys: [Pubkey; CREATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_mill_config: pubkeys[0],
            market: pubkeys[1],
            token_mint_0: pubkeys[2],
            market_reserve_0: pubkeys[3],
            token_0_metadata: pubkeys[4],
            token_mint_1: pubkeys[5],
            market_reserve_1: pubkeys[6],
            creator: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            token_metadata_program: pubkeys[10],
            associated_token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<CreateMarketAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.token_mill_config.clone(),
            accounts.market.clone(),
            accounts.token_mint_0.clone(),
            accounts.market_reserve_0.clone(),
            accounts.token_0_metadata.clone(),
            accounts.token_mint_1.clone(),
            accounts.market_reserve_1.clone(),
            accounts.creator.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.token_metadata_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN]>
for CreateMarketAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_mill_config: &arr[0],
            market: &arr[1],
            token_mint_0: &arr[2],
            market_reserve_0: &arr[3],
            token_0_metadata: &arr[4],
            token_mint_1: &arr[5],
            market_reserve_1: &arr[6],
            creator: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            token_metadata_program: &arr[10],
            associated_token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const CREATE_MARKET_IX_DISCM: [u8; 8usize] = [103, 226, 97, 235, 200, 188, 251, 254];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMarketIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub swap_authority: Option<Pubkey>,
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
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        let swap_authority: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateMarketIxArgs {
                name,
                symbol,
                uri,
                swap_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_MARKET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.swap_authority, &mut writer)?;
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
    create_market_ix_with_program_id(TOKEN_MILL_PROGRAM_ID, keys, args)
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
    create_market_invoke_with_program_id(TOKEN_MILL_PROGRAM_ID, accounts, args)
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
        TOKEN_MILL_PROGRAM_ID,
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
        (*accounts.token_mill_config.key, keys.token_mill_config),
        (*accounts.market.key, keys.market),
        (*accounts.token_mint_0.key, keys.token_mint_0),
        (*accounts.market_reserve_0.key, keys.market_reserve_0),
        (*accounts.token_0_metadata.key, keys.token_0_metadata),
        (*accounts.token_mint_1.key, keys.token_mint_1),
        (*accounts.market_reserve_1.key, keys.market_reserve_1),
        (*accounts.creator.key, keys.creator),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_metadata_program.key, keys.token_metadata_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
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
        accounts.market,
        accounts.token_mint_0,
        accounts.market_reserve_0,
        accounts.token_0_metadata,
        accounts.market_reserve_1,
        accounts.creator,
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
    for should_be_signer in [accounts.token_mint_0, accounts.creator] {
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
pub const FORCE_REMOVE_FEE_RESERVE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ForceRemoveFeeReserveAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ForceRemoveFeeReserveKeys {
    pub config: Pubkey,
    pub market: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ForceRemoveFeeReserveAccounts<'_, '_>> for ForceRemoveFeeReserveKeys {
    fn from(accounts: ForceRemoveFeeReserveAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            market: *accounts.market.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ForceRemoveFeeReserveKeys>
for [AccountMeta; FORCE_REMOVE_FEE_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(keys: ForceRemoveFeeReserveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FORCE_REMOVE_FEE_RESERVE_IX_ACCOUNTS_LEN]>
for ForceRemoveFeeReserveKeys {
    fn from(pubkeys: [Pubkey; FORCE_REMOVE_FEE_RESERVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            market: pubkeys[1],
            admin: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<ForceRemoveFeeReserveAccounts<'_, 'info>>
for [AccountInfo<'info>; FORCE_REMOVE_FEE_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ForceRemoveFeeReserveAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.market.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; FORCE_REMOVE_FEE_RESERVE_IX_ACCOUNTS_LEN]>
for ForceRemoveFeeReserveAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; FORCE_REMOVE_FEE_RESERVE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            market: &arr[1],
            admin: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const FORCE_REMOVE_FEE_RESERVE_IX_DISCM: [u8; 8usize] = [
    171, 249, 234, 88, 247, 69, 109, 249,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ForceRemoveFeeReserveIxData;
impl ForceRemoveFeeReserveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FORCE_REMOVE_FEE_RESERVE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FORCE_REMOVE_FEE_RESERVE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn force_remove_fee_reserve_ix_with_program_id(
    program_id: Pubkey,
    keys: ForceRemoveFeeReserveKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FORCE_REMOVE_FEE_RESERVE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ForceRemoveFeeReserveIxData.try_to_vec()?,
    })
}
pub fn force_remove_fee_reserve_ix(
    keys: ForceRemoveFeeReserveKeys,
) -> std::io::Result<Instruction> {
    force_remove_fee_reserve_ix_with_program_id(TOKEN_MILL_PROGRAM_ID, keys)
}
pub fn force_remove_fee_reserve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ForceRemoveFeeReserveAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ForceRemoveFeeReserveKeys = accounts.into();
    let ix = force_remove_fee_reserve_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn force_remove_fee_reserve_invoke(
    accounts: ForceRemoveFeeReserveAccounts<'_, '_>,
) -> ProgramResult {
    force_remove_fee_reserve_invoke_with_program_id(TOKEN_MILL_PROGRAM_ID, accounts)
}
pub fn force_remove_fee_reserve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ForceRemoveFeeReserveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ForceRemoveFeeReserveKeys = accounts.into();
    let ix = force_remove_fee_reserve_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn force_remove_fee_reserve_invoke_signed(
    accounts: ForceRemoveFeeReserveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    force_remove_fee_reserve_invoke_signed_with_program_id(
        TOKEN_MILL_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn force_remove_fee_reserve_verify_account_keys(
    accounts: ForceRemoveFeeReserveAccounts<'_, '_>,
    keys: ForceRemoveFeeReserveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.market.key, keys.market),
        (*accounts.admin.key, keys.admin),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn force_remove_fee_reserve_verify_writable_privileges<'me, 'info>(
    accounts: ForceRemoveFeeReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn force_remove_fee_reserve_verify_signer_privileges<'me, 'info>(
    accounts: ForceRemoveFeeReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn force_remove_fee_reserve_verify_account_privileges<'me, 'info>(
    accounts: ForceRemoveFeeReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    force_remove_fee_reserve_verify_writable_privileges(accounts)?;
    force_remove_fee_reserve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_SWAP_AUTHORITY_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RemoveSwapAuthorityAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub new_fee_reserve: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveSwapAuthorityKeys {
    pub config: Pubkey,
    pub market: Pubkey,
    pub new_fee_reserve: Pubkey,
    pub creator: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveSwapAuthorityAccounts<'_, '_>> for RemoveSwapAuthorityKeys {
    fn from(accounts: RemoveSwapAuthorityAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            market: *accounts.market.key,
            new_fee_reserve: *accounts.new_fee_reserve.key,
            creator: *accounts.creator.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveSwapAuthorityKeys>
for [AccountMeta; REMOVE_SWAP_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveSwapAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_fee_reserve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_SWAP_AUTHORITY_IX_ACCOUNTS_LEN]> for RemoveSwapAuthorityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_SWAP_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            market: pubkeys[1],
            new_fee_reserve: pubkeys[2],
            creator: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<RemoveSwapAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_SWAP_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveSwapAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.market.clone(),
            accounts.new_fee_reserve.clone(),
            accounts.creator.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_SWAP_AUTHORITY_IX_ACCOUNTS_LEN]>
for RemoveSwapAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_SWAP_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            market: &arr[1],
            new_fee_reserve: &arr[2],
            creator: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const REMOVE_SWAP_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    128, 249, 213, 153, 65, 171, 76, 171,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveSwapAuthorityIxData;
impl RemoveSwapAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_SWAP_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_SWAP_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_swap_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveSwapAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_SWAP_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveSwapAuthorityIxData.try_to_vec()?,
    })
}
pub fn remove_swap_authority_ix(
    keys: RemoveSwapAuthorityKeys,
) -> std::io::Result<Instruction> {
    remove_swap_authority_ix_with_program_id(TOKEN_MILL_PROGRAM_ID, keys)
}
pub fn remove_swap_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveSwapAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveSwapAuthorityKeys = accounts.into();
    let ix = remove_swap_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_swap_authority_invoke(
    accounts: RemoveSwapAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    remove_swap_authority_invoke_with_program_id(TOKEN_MILL_PROGRAM_ID, accounts)
}
pub fn remove_swap_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveSwapAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveSwapAuthorityKeys = accounts.into();
    let ix = remove_swap_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_swap_authority_invoke_signed(
    accounts: RemoveSwapAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_swap_authority_invoke_signed_with_program_id(
        TOKEN_MILL_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn remove_swap_authority_verify_account_keys(
    accounts: RemoveSwapAuthorityAccounts<'_, '_>,
    keys: RemoveSwapAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.market.key, keys.market),
        (*accounts.new_fee_reserve.key, keys.new_fee_reserve),
        (*accounts.creator.key, keys.creator),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_swap_authority_verify_writable_privileges<'me, 'info>(
    accounts: RemoveSwapAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_swap_authority_verify_signer_privileges<'me, 'info>(
    accounts: RemoveSwapAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_swap_authority_verify_account_privileges<'me, 'info>(
    accounts: RemoveSwapAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_swap_authority_verify_writable_privileges(accounts)?;
    remove_swap_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub market_reserve_0: &'me AccountInfo<'info>,
    pub user_token_account_0: &'me AccountInfo<'info>,
    pub market_reserve_1: &'me AccountInfo<'info>,
    pub user_token_account_1: &'me AccountInfo<'info>,
    pub fee_reserve: &'me AccountInfo<'info>,
    pub protocol_fee_reserve: &'me AccountInfo<'info>,
    pub creator_fee_pool: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub swap_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub config: Pubkey,
    pub market: Pubkey,
    pub market_reserve_0: Pubkey,
    pub user_token_account_0: Pubkey,
    pub market_reserve_1: Pubkey,
    pub user_token_account_1: Pubkey,
    pub fee_reserve: Pubkey,
    pub protocol_fee_reserve: Pubkey,
    pub creator_fee_pool: Pubkey,
    pub user: Pubkey,
    pub swap_authority: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            market: *accounts.market.key,
            market_reserve_0: *accounts.market_reserve_0.key,
            user_token_account_0: *accounts.user_token_account_0.key,
            market_reserve_1: *accounts.market_reserve_1.key,
            user_token_account_1: *accounts.user_token_account_1.key,
            fee_reserve: *accounts.fee_reserve.key,
            protocol_fee_reserve: *accounts.protocol_fee_reserve.key,
            creator_fee_pool: *accounts.creator_fee_pool.key,
            user: *accounts.user.key,
            swap_authority: *accounts.swap_authority.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_reserve_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_reserve_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_fee_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            market: pubkeys[1],
            market_reserve_0: pubkeys[2],
            user_token_account_0: pubkeys[3],
            market_reserve_1: pubkeys[4],
            user_token_account_1: pubkeys[5],
            fee_reserve: pubkeys[6],
            protocol_fee_reserve: pubkeys[7],
            creator_fee_pool: pubkeys[8],
            user: pubkeys[9],
            swap_authority: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.market.clone(),
            accounts.market_reserve_0.clone(),
            accounts.user_token_account_0.clone(),
            accounts.market_reserve_1.clone(),
            accounts.user_token_account_1.clone(),
            accounts.fee_reserve.clone(),
            accounts.protocol_fee_reserve.clone(),
            accounts.creator_fee_pool.clone(),
            accounts.user.clone(),
            accounts.swap_authority.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            market: &arr[1],
            market_reserve_0: &arr[2],
            user_token_account_0: &arr[3],
            market_reserve_1: &arr[4],
            user_token_account_1: &arr[5],
            fee_reserve: &arr[6],
            protocol_fee_reserve: &arr[7],
            creator_fee_pool: &arr[8],
            user: &arr[9],
            swap_authority: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub swap_parameters: SwapParameters,
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
        let swap_parameters = <SwapParameters as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        Ok(Self(SwapIxArgs { swap_parameters }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.swap_parameters, &mut writer)?;
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
    swap_ix_with_program_id(TOKEN_MILL_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(TOKEN_MILL_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(TOKEN_MILL_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.market.key, keys.market),
        (*accounts.market_reserve_0.key, keys.market_reserve_0),
        (*accounts.user_token_account_0.key, keys.user_token_account_0),
        (*accounts.market_reserve_1.key, keys.market_reserve_1),
        (*accounts.user_token_account_1.key, keys.user_token_account_1),
        (*accounts.fee_reserve.key, keys.fee_reserve),
        (*accounts.protocol_fee_reserve.key, keys.protocol_fee_reserve),
        (*accounts.creator_fee_pool.key, keys.creator_fee_pool),
        (*accounts.user.key, keys.user),
        (*accounts.swap_authority.key, keys.swap_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
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
        accounts.market,
        accounts.market_reserve_0,
        accounts.user_token_account_0,
        accounts.market_reserve_1,
        accounts.user_token_account_1,
        accounts.fee_reserve,
        accounts.protocol_fee_reserve,
        accounts.creator_fee_pool,
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
    for should_be_signer in [accounts.user, accounts.swap_authority] {
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
pub const SWAP_WITH_PRICE_LIMIT_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SwapWithPriceLimitAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub market_reserve_0: &'me AccountInfo<'info>,
    pub user_token_account_0: &'me AccountInfo<'info>,
    pub market_reserve_1: &'me AccountInfo<'info>,
    pub user_token_account_1: &'me AccountInfo<'info>,
    pub fee_reserve: &'me AccountInfo<'info>,
    pub protocol_fee_reserve: &'me AccountInfo<'info>,
    pub creator_fee_pool: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub swap_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapWithPriceLimitKeys {
    pub config: Pubkey,
    pub market: Pubkey,
    pub market_reserve_0: Pubkey,
    pub user_token_account_0: Pubkey,
    pub market_reserve_1: Pubkey,
    pub user_token_account_1: Pubkey,
    pub fee_reserve: Pubkey,
    pub protocol_fee_reserve: Pubkey,
    pub creator_fee_pool: Pubkey,
    pub user: Pubkey,
    pub swap_authority: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapWithPriceLimitAccounts<'_, '_>> for SwapWithPriceLimitKeys {
    fn from(accounts: SwapWithPriceLimitAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            market: *accounts.market.key,
            market_reserve_0: *accounts.market_reserve_0.key,
            user_token_account_0: *accounts.user_token_account_0.key,
            market_reserve_1: *accounts.market_reserve_1.key,
            user_token_account_1: *accounts.user_token_account_1.key,
            fee_reserve: *accounts.fee_reserve.key,
            protocol_fee_reserve: *accounts.protocol_fee_reserve.key,
            creator_fee_pool: *accounts.creator_fee_pool.key,
            user: *accounts.user.key,
            swap_authority: *accounts.swap_authority.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapWithPriceLimitKeys>
for [AccountMeta; SWAP_WITH_PRICE_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapWithPriceLimitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_reserve_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_reserve_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_fee_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_WITH_PRICE_LIMIT_IX_ACCOUNTS_LEN]> for SwapWithPriceLimitKeys {
    fn from(pubkeys: [Pubkey; SWAP_WITH_PRICE_LIMIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            market: pubkeys[1],
            market_reserve_0: pubkeys[2],
            user_token_account_0: pubkeys[3],
            market_reserve_1: pubkeys[4],
            user_token_account_1: pubkeys[5],
            fee_reserve: pubkeys[6],
            protocol_fee_reserve: pubkeys[7],
            creator_fee_pool: pubkeys[8],
            user: pubkeys[9],
            swap_authority: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<SwapWithPriceLimitAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_WITH_PRICE_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapWithPriceLimitAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.market.clone(),
            accounts.market_reserve_0.clone(),
            accounts.user_token_account_0.clone(),
            accounts.market_reserve_1.clone(),
            accounts.user_token_account_1.clone(),
            accounts.fee_reserve.clone(),
            accounts.protocol_fee_reserve.clone(),
            accounts.creator_fee_pool.clone(),
            accounts.user.clone(),
            accounts.swap_authority.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_WITH_PRICE_LIMIT_IX_ACCOUNTS_LEN]>
for SwapWithPriceLimitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_WITH_PRICE_LIMIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            market: &arr[1],
            market_reserve_0: &arr[2],
            user_token_account_0: &arr[3],
            market_reserve_1: &arr[4],
            user_token_account_1: &arr[5],
            fee_reserve: &arr[6],
            protocol_fee_reserve: &arr[7],
            creator_fee_pool: &arr[8],
            user: &arr[9],
            swap_authority: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const SWAP_WITH_PRICE_LIMIT_IX_DISCM: [u8; 8usize] = [
    54, 23, 76, 40, 64, 202, 5, 69,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapWithPriceLimitIxArgs {
    pub zero_for_one: bool,
    pub delta_amount: i64,
    pub sqrt_price_limit_x96: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapWithPriceLimitIxData(pub SwapWithPriceLimitIxArgs);
impl From<SwapWithPriceLimitIxArgs> for SwapWithPriceLimitIxData {
    fn from(args: SwapWithPriceLimitIxArgs) -> Self {
        Self(args)
    }
}
impl SwapWithPriceLimitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_WITH_PRICE_LIMIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let zero_for_one: bool = crate::borsh_de_or_default(&mut reader)?;
        let delta_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_limit_x96: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapWithPriceLimitIxArgs {
                zero_for_one,
                delta_amount,
                sqrt_price_limit_x96,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_WITH_PRICE_LIMIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.zero_for_one, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.delta_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price_limit_x96, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_with_price_limit_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapWithPriceLimitKeys,
    args: SwapWithPriceLimitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_WITH_PRICE_LIMIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapWithPriceLimitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_with_price_limit_ix(
    keys: SwapWithPriceLimitKeys,
    args: SwapWithPriceLimitIxArgs,
) -> std::io::Result<Instruction> {
    swap_with_price_limit_ix_with_program_id(TOKEN_MILL_PROGRAM_ID, keys, args)
}
pub fn swap_with_price_limit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithPriceLimitAccounts<'_, '_>,
    args: SwapWithPriceLimitIxArgs,
) -> ProgramResult {
    let keys: SwapWithPriceLimitKeys = accounts.into();
    let ix = swap_with_price_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_with_price_limit_invoke(
    accounts: SwapWithPriceLimitAccounts<'_, '_>,
    args: SwapWithPriceLimitIxArgs,
) -> ProgramResult {
    swap_with_price_limit_invoke_with_program_id(TOKEN_MILL_PROGRAM_ID, accounts, args)
}
pub fn swap_with_price_limit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithPriceLimitAccounts<'_, '_>,
    args: SwapWithPriceLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapWithPriceLimitKeys = accounts.into();
    let ix = swap_with_price_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_with_price_limit_invoke_signed(
    accounts: SwapWithPriceLimitAccounts<'_, '_>,
    args: SwapWithPriceLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_with_price_limit_invoke_signed_with_program_id(
        TOKEN_MILL_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_with_price_limit_verify_account_keys(
    accounts: SwapWithPriceLimitAccounts<'_, '_>,
    keys: SwapWithPriceLimitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.market.key, keys.market),
        (*accounts.market_reserve_0.key, keys.market_reserve_0),
        (*accounts.user_token_account_0.key, keys.user_token_account_0),
        (*accounts.market_reserve_1.key, keys.market_reserve_1),
        (*accounts.user_token_account_1.key, keys.user_token_account_1),
        (*accounts.fee_reserve.key, keys.fee_reserve),
        (*accounts.protocol_fee_reserve.key, keys.protocol_fee_reserve),
        (*accounts.creator_fee_pool.key, keys.creator_fee_pool),
        (*accounts.user.key, keys.user),
        (*accounts.swap_authority.key, keys.swap_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_with_price_limit_verify_writable_privileges<'me, 'info>(
    accounts: SwapWithPriceLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.market_reserve_0,
        accounts.user_token_account_0,
        accounts.market_reserve_1,
        accounts.user_token_account_1,
        accounts.fee_reserve,
        accounts.protocol_fee_reserve,
        accounts.creator_fee_pool,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_with_price_limit_verify_signer_privileges<'me, 'info>(
    accounts: SwapWithPriceLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user, accounts.swap_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_with_price_limit_verify_account_privileges<'me, 'info>(
    accounts: SwapWithPriceLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_with_price_limit_verify_writable_privileges(accounts)?;
    swap_with_price_limit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct TransferConfigOwnershipAccounts<'me, 'info> {
    pub token_mill_config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferConfigOwnershipKeys {
    pub token_mill_config: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<TransferConfigOwnershipAccounts<'_, '_>> for TransferConfigOwnershipKeys {
    fn from(accounts: TransferConfigOwnershipAccounts) -> Self {
        Self {
            token_mill_config: *accounts.token_mill_config.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<TransferConfigOwnershipKeys>
for [AccountMeta; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferConfigOwnershipKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_mill_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN]>
for TransferConfigOwnershipKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_mill_config: pubkeys[0],
            admin: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<TransferConfigOwnershipAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferConfigOwnershipAccounts<'_, 'info>) -> Self {
        [
            accounts.token_mill_config.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN]>
for TransferConfigOwnershipAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_mill_config: &arr[0],
            admin: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const TRANSFER_CONFIG_OWNERSHIP_IX_DISCM: [u8; 8usize] = [
    53, 124, 67, 226, 108, 130, 19, 12,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferConfigOwnershipIxArgs {
    pub new_admin: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferConfigOwnershipIxData(pub TransferConfigOwnershipIxArgs);
impl From<TransferConfigOwnershipIxArgs> for TransferConfigOwnershipIxData {
    fn from(args: TransferConfigOwnershipIxArgs) -> Self {
        Self(args)
    }
}
impl TransferConfigOwnershipIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_CONFIG_OWNERSHIP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TransferConfigOwnershipIxArgs {
                new_admin,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_CONFIG_OWNERSHIP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_admin, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_config_ownership_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferConfigOwnershipKeys,
    args: TransferConfigOwnershipIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] = keys.into();
    let data: TransferConfigOwnershipIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_config_ownership_ix(
    keys: TransferConfigOwnershipKeys,
    args: TransferConfigOwnershipIxArgs,
) -> std::io::Result<Instruction> {
    transfer_config_ownership_ix_with_program_id(TOKEN_MILL_PROGRAM_ID, keys, args)
}
pub fn transfer_config_ownership_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferConfigOwnershipAccounts<'_, '_>,
    args: TransferConfigOwnershipIxArgs,
) -> ProgramResult {
    let keys: TransferConfigOwnershipKeys = accounts.into();
    let ix = transfer_config_ownership_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_config_ownership_invoke(
    accounts: TransferConfigOwnershipAccounts<'_, '_>,
    args: TransferConfigOwnershipIxArgs,
) -> ProgramResult {
    transfer_config_ownership_invoke_with_program_id(
        TOKEN_MILL_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn transfer_config_ownership_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferConfigOwnershipAccounts<'_, '_>,
    args: TransferConfigOwnershipIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferConfigOwnershipKeys = accounts.into();
    let ix = transfer_config_ownership_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_config_ownership_invoke_signed(
    accounts: TransferConfigOwnershipAccounts<'_, '_>,
    args: TransferConfigOwnershipIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_config_ownership_invoke_signed_with_program_id(
        TOKEN_MILL_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn transfer_config_ownership_verify_account_keys(
    accounts: TransferConfigOwnershipAccounts<'_, '_>,
    keys: TransferConfigOwnershipKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_mill_config.key, keys.token_mill_config),
        (*accounts.admin.key, keys.admin),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_config_ownership_verify_writable_privileges<'me, 'info>(
    accounts: TransferConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_mill_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_config_ownership_verify_signer_privileges<'me, 'info>(
    accounts: TransferConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_config_ownership_verify_account_privileges<'me, 'info>(
    accounts: TransferConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_config_ownership_verify_writable_privileges(accounts)?;
    transfer_config_ownership_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CONFIG_SETTINGS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateConfigSettingsAccounts<'me, 'info> {
    pub token_mill_config: &'me AccountInfo<'info>,
    pub new_protocol_fee_reserve: &'me AccountInfo<'info>,
    pub new_creator_fee_pool: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateConfigSettingsKeys {
    pub token_mill_config: Pubkey,
    pub new_protocol_fee_reserve: Pubkey,
    pub new_creator_fee_pool: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateConfigSettingsAccounts<'_, '_>> for UpdateConfigSettingsKeys {
    fn from(accounts: UpdateConfigSettingsAccounts) -> Self {
        Self {
            token_mill_config: *accounts.token_mill_config.key,
            new_protocol_fee_reserve: *accounts.new_protocol_fee_reserve.key,
            new_creator_fee_pool: *accounts.new_creator_fee_pool.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateConfigSettingsKeys>
for [AccountMeta; UPDATE_CONFIG_SETTINGS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateConfigSettingsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_mill_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_protocol_fee_reserve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_creator_fee_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_CONFIG_SETTINGS_IX_ACCOUNTS_LEN]>
for UpdateConfigSettingsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CONFIG_SETTINGS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_mill_config: pubkeys[0],
            new_protocol_fee_reserve: pubkeys[1],
            new_creator_fee_pool: pubkeys[2],
            admin: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateConfigSettingsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CONFIG_SETTINGS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateConfigSettingsAccounts<'_, 'info>) -> Self {
        [
            accounts.token_mill_config.clone(),
            accounts.new_protocol_fee_reserve.clone(),
            accounts.new_creator_fee_pool.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_CONFIG_SETTINGS_IX_ACCOUNTS_LEN]>
for UpdateConfigSettingsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_CONFIG_SETTINGS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_mill_config: &arr[0],
            new_protocol_fee_reserve: &arr[1],
            new_creator_fee_pool: &arr[2],
            admin: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_CONFIG_SETTINGS_IX_DISCM: [u8; 8usize] = [
    222, 242, 103, 173, 124, 98, 180, 244,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateConfigSettingsIxArgs {
    pub new_protocol_fee_share: u32,
    pub new_fee_recipient_change_cooldown: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateConfigSettingsIxData(pub UpdateConfigSettingsIxArgs);
impl From<UpdateConfigSettingsIxArgs> for UpdateConfigSettingsIxData {
    fn from(args: UpdateConfigSettingsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateConfigSettingsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CONFIG_SETTINGS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_protocol_fee_share: u32 = crate::borsh_de_or_default(&mut reader)?;
        let new_fee_recipient_change_cooldown: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UpdateConfigSettingsIxArgs {
                new_protocol_fee_share,
                new_fee_recipient_change_cooldown,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CONFIG_SETTINGS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_protocol_fee_share, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_fee_recipient_change_cooldown,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_config_settings_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateConfigSettingsKeys,
    args: UpdateConfigSettingsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CONFIG_SETTINGS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateConfigSettingsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_config_settings_ix(
    keys: UpdateConfigSettingsKeys,
    args: UpdateConfigSettingsIxArgs,
) -> std::io::Result<Instruction> {
    update_config_settings_ix_with_program_id(TOKEN_MILL_PROGRAM_ID, keys, args)
}
pub fn update_config_settings_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConfigSettingsAccounts<'_, '_>,
    args: UpdateConfigSettingsIxArgs,
) -> ProgramResult {
    let keys: UpdateConfigSettingsKeys = accounts.into();
    let ix = update_config_settings_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_config_settings_invoke(
    accounts: UpdateConfigSettingsAccounts<'_, '_>,
    args: UpdateConfigSettingsIxArgs,
) -> ProgramResult {
    update_config_settings_invoke_with_program_id(TOKEN_MILL_PROGRAM_ID, accounts, args)
}
pub fn update_config_settings_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConfigSettingsAccounts<'_, '_>,
    args: UpdateConfigSettingsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateConfigSettingsKeys = accounts.into();
    let ix = update_config_settings_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_config_settings_invoke_signed(
    accounts: UpdateConfigSettingsAccounts<'_, '_>,
    args: UpdateConfigSettingsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_config_settings_invoke_signed_with_program_id(
        TOKEN_MILL_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_config_settings_verify_account_keys(
    accounts: UpdateConfigSettingsAccounts<'_, '_>,
    keys: UpdateConfigSettingsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_mill_config.key, keys.token_mill_config),
        (*accounts.new_protocol_fee_reserve.key, keys.new_protocol_fee_reserve),
        (*accounts.new_creator_fee_pool.key, keys.new_creator_fee_pool),
        (*accounts.admin.key, keys.admin),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_config_settings_verify_writable_privileges<'me, 'info>(
    accounts: UpdateConfigSettingsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_mill_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_config_settings_verify_signer_privileges<'me, 'info>(
    accounts: UpdateConfigSettingsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_config_settings_verify_account_privileges<'me, 'info>(
    accounts: UpdateConfigSettingsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_config_settings_verify_writable_privileges(accounts)?;
    update_config_settings_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FEE_RESERVE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeeReserveAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub new_fee_reserve: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeeReserveKeys {
    pub config: Pubkey,
    pub market: Pubkey,
    pub new_fee_reserve: Pubkey,
    pub creator: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateFeeReserveAccounts<'_, '_>> for UpdateFeeReserveKeys {
    fn from(accounts: UpdateFeeReserveAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            market: *accounts.market.key,
            new_fee_reserve: *accounts.new_fee_reserve.key,
            creator: *accounts.creator.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateFeeReserveKeys> for [AccountMeta; UPDATE_FEE_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeeReserveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_fee_reserve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FEE_RESERVE_IX_ACCOUNTS_LEN]> for UpdateFeeReserveKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FEE_RESERVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            market: pubkeys[1],
            new_fee_reserve: pubkeys[2],
            creator: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateFeeReserveAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FEE_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeeReserveAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.market.clone(),
            accounts.new_fee_reserve.clone(),
            accounts.creator.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FEE_RESERVE_IX_ACCOUNTS_LEN]>
for UpdateFeeReserveAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_FEE_RESERVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            market: &arr[1],
            new_fee_reserve: &arr[2],
            creator: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_FEE_RESERVE_IX_DISCM: [u8; 8usize] = [
    129, 136, 189, 198, 155, 88, 145, 199,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeeReserveIxData;
impl UpdateFeeReserveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEE_RESERVE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEE_RESERVE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fee_reserve_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeeReserveKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEE_RESERVE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateFeeReserveIxData.try_to_vec()?,
    })
}
pub fn update_fee_reserve_ix(
    keys: UpdateFeeReserveKeys,
) -> std::io::Result<Instruction> {
    update_fee_reserve_ix_with_program_id(TOKEN_MILL_PROGRAM_ID, keys)
}
pub fn update_fee_reserve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeReserveAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateFeeReserveKeys = accounts.into();
    let ix = update_fee_reserve_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fee_reserve_invoke(
    accounts: UpdateFeeReserveAccounts<'_, '_>,
) -> ProgramResult {
    update_fee_reserve_invoke_with_program_id(TOKEN_MILL_PROGRAM_ID, accounts)
}
pub fn update_fee_reserve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeReserveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeeReserveKeys = accounts.into();
    let ix = update_fee_reserve_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fee_reserve_invoke_signed(
    accounts: UpdateFeeReserveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fee_reserve_invoke_signed_with_program_id(
        TOKEN_MILL_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_fee_reserve_verify_account_keys(
    accounts: UpdateFeeReserveAccounts<'_, '_>,
    keys: UpdateFeeReserveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.market.key, keys.market),
        (*accounts.new_fee_reserve.key, keys.new_fee_reserve),
        (*accounts.creator.key, keys.creator),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fee_reserve_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFeeReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fee_reserve_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFeeReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fee_reserve_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeeReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fee_reserve_verify_writable_privileges(accounts)?;
    update_fee_reserve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_MARKET_DEFAULTS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateMarketDefaultsAccounts<'me, 'info> {
    pub token_mill_config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateMarketDefaultsKeys {
    pub token_mill_config: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateMarketDefaultsAccounts<'_, '_>> for UpdateMarketDefaultsKeys {
    fn from(accounts: UpdateMarketDefaultsAccounts) -> Self {
        Self {
            token_mill_config: *accounts.token_mill_config.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateMarketDefaultsKeys>
for [AccountMeta; UPDATE_MARKET_DEFAULTS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateMarketDefaultsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_mill_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_MARKET_DEFAULTS_IX_ACCOUNTS_LEN]>
for UpdateMarketDefaultsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_MARKET_DEFAULTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_mill_config: pubkeys[0],
            admin: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateMarketDefaultsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_MARKET_DEFAULTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateMarketDefaultsAccounts<'_, 'info>) -> Self {
        [
            accounts.token_mill_config.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_MARKET_DEFAULTS_IX_ACCOUNTS_LEN]>
for UpdateMarketDefaultsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_MARKET_DEFAULTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_mill_config: &arr[0],
            admin: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_MARKET_DEFAULTS_IX_DISCM: [u8; 8usize] = [
    38, 26, 191, 18, 30, 234, 178, 41,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateMarketDefaultsIxArgs {
    pub market_settings: MarketSettingsInput,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMarketDefaultsIxData(pub UpdateMarketDefaultsIxArgs);
impl From<UpdateMarketDefaultsIxArgs> for UpdateMarketDefaultsIxData {
    fn from(args: UpdateMarketDefaultsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateMarketDefaultsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_MARKET_DEFAULTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let market_settings = if reader.is_empty() {
            Default::default()
        } else {
            <MarketSettingsInput>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateMarketDefaultsIxArgs {
                market_settings,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_MARKET_DEFAULTS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.market_settings, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_market_defaults_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateMarketDefaultsKeys,
    args: UpdateMarketDefaultsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_MARKET_DEFAULTS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateMarketDefaultsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_market_defaults_ix(
    keys: UpdateMarketDefaultsKeys,
    args: UpdateMarketDefaultsIxArgs,
) -> std::io::Result<Instruction> {
    update_market_defaults_ix_with_program_id(TOKEN_MILL_PROGRAM_ID, keys, args)
}
pub fn update_market_defaults_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateMarketDefaultsAccounts<'_, '_>,
    args: UpdateMarketDefaultsIxArgs,
) -> ProgramResult {
    let keys: UpdateMarketDefaultsKeys = accounts.into();
    let ix = update_market_defaults_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_market_defaults_invoke(
    accounts: UpdateMarketDefaultsAccounts<'_, '_>,
    args: UpdateMarketDefaultsIxArgs,
) -> ProgramResult {
    update_market_defaults_invoke_with_program_id(TOKEN_MILL_PROGRAM_ID, accounts, args)
}
pub fn update_market_defaults_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateMarketDefaultsAccounts<'_, '_>,
    args: UpdateMarketDefaultsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateMarketDefaultsKeys = accounts.into();
    let ix = update_market_defaults_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_market_defaults_invoke_signed(
    accounts: UpdateMarketDefaultsAccounts<'_, '_>,
    args: UpdateMarketDefaultsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_market_defaults_invoke_signed_with_program_id(
        TOKEN_MILL_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_market_defaults_verify_account_keys(
    accounts: UpdateMarketDefaultsAccounts<'_, '_>,
    keys: UpdateMarketDefaultsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_mill_config.key, keys.token_mill_config),
        (*accounts.admin.key, keys.admin),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_market_defaults_verify_writable_privileges<'me, 'info>(
    accounts: UpdateMarketDefaultsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_mill_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_market_defaults_verify_signer_privileges<'me, 'info>(
    accounts: UpdateMarketDefaultsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_market_defaults_verify_account_privileges<'me, 'info>(
    accounts: UpdateMarketDefaultsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_market_defaults_verify_writable_privileges(accounts)?;
    update_market_defaults_verify_signer_privileges(accounts)?;
    Ok(())
}
