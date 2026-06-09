use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum LimoProgramIx {
    InitializeGlobalConfig,
    InitializeVault,
    CreateOrder(CreateOrderIxArgs),
    UpdateOrder(UpdateOrderIxArgs),
    CloseOrderAndClaimTip,
    TakeOrder(TakeOrderIxArgs),
    FlashTakeOrderStart(FlashTakeOrderStartIxArgs),
    FlashTakeOrderEnd(FlashTakeOrderEndIxArgs),
    UpdateGlobalConfig(UpdateGlobalConfigIxArgs),
    UpdateGlobalConfigAdmin,
    WithdrawHostTip,
    LogUserSwapBalancesStart,
    LogUserSwapBalancesEnd(LogUserSwapBalancesEndIxArgs),
    AssertUserSwapBalancesStart,
    AssertUserSwapBalancesEnd(AssertUserSwapBalancesEndIxArgs),
}
impl LimoProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_GLOBAL_CONFIG_IX_DISCM) {
            return Ok(Self::InitializeGlobalConfig);
        }
        if buf.starts_with(&INITIALIZE_VAULT_IX_DISCM) {
            return Ok(Self::InitializeVault);
        }
        if buf.starts_with(&CREATE_ORDER_IX_DISCM) {
            let mut reader = &buf[CREATE_ORDER_IX_DISCM.len()..];
            let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let order_type: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateOrder(CreateOrderIxArgs {
                    input_amount,
                    output_amount,
                    order_type,
                }),
            );
        }
        if buf.starts_with(&UPDATE_ORDER_IX_DISCM) {
            let mut reader = &buf[UPDATE_ORDER_IX_DISCM.len()..];
            let mode: u16 = crate::borsh_de_or_default(&mut reader)?;
            let value: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::UpdateOrder(UpdateOrderIxArgs { mode, value }));
        }
        if buf.starts_with(&CLOSE_ORDER_AND_CLAIM_TIP_IX_DISCM) {
            return Ok(Self::CloseOrderAndClaimTip);
        }
        if buf.starts_with(&TAKE_ORDER_IX_DISCM) {
            let mut reader = &buf[TAKE_ORDER_IX_DISCM.len()..];
            let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let tip_amount_permissionless_taking: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::TakeOrder(TakeOrderIxArgs {
                    input_amount,
                    min_output_amount,
                    tip_amount_permissionless_taking,
                }),
            );
        }
        if buf.starts_with(&FLASH_TAKE_ORDER_START_IX_DISCM) {
            let mut reader = &buf[FLASH_TAKE_ORDER_START_IX_DISCM.len()..];
            let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let tip_amount_permissionless_taking: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::FlashTakeOrderStart(FlashTakeOrderStartIxArgs {
                    input_amount,
                    min_output_amount,
                    tip_amount_permissionless_taking,
                }),
            );
        }
        if buf.starts_with(&FLASH_TAKE_ORDER_END_IX_DISCM) {
            let mut reader = &buf[FLASH_TAKE_ORDER_END_IX_DISCM.len()..];
            let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let tip_amount_permissionless_taking: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::FlashTakeOrderEnd(FlashTakeOrderEndIxArgs {
                    input_amount,
                    min_output_amount,
                    tip_amount_permissionless_taking,
                }),
            );
        }
        if buf.starts_with(&UPDATE_GLOBAL_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_GLOBAL_CONFIG_IX_DISCM.len()..];
            let mode: u16 = crate::borsh_de_or_default(&mut reader)?;
            let value = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateGlobalConfig(UpdateGlobalConfigIxArgs {
                    mode,
                    value,
                }),
            );
        }
        if buf.starts_with(&UPDATE_GLOBAL_CONFIG_ADMIN_IX_DISCM) {
            return Ok(Self::UpdateGlobalConfigAdmin);
        }
        if buf.starts_with(&WITHDRAW_HOST_TIP_IX_DISCM) {
            return Ok(Self::WithdrawHostTip);
        }
        if buf.starts_with(&LOG_USER_SWAP_BALANCES_START_IX_DISCM) {
            return Ok(Self::LogUserSwapBalancesStart);
        }
        if buf.starts_with(&LOG_USER_SWAP_BALANCES_END_IX_DISCM) {
            let mut reader = &buf[LOG_USER_SWAP_BALANCES_END_IX_DISCM.len()..];
            let simulated_swap_amount_out: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let simulated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            let swap_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let simulated_amount_out_next_best: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let aggregator: u8 = crate::borsh_de_or_default(&mut reader)?;
            let next_best_aggregator: u8 = crate::borsh_de_or_default(&mut reader)?;
            let padding: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LogUserSwapBalancesEnd(LogUserSwapBalancesEndIxArgs {
                    simulated_swap_amount_out,
                    simulated_ts,
                    minimum_amount_out,
                    swap_amount_in,
                    simulated_amount_out_next_best,
                    aggregator,
                    next_best_aggregator,
                    padding,
                }),
            );
        }
        if buf.starts_with(&ASSERT_USER_SWAP_BALANCES_START_IX_DISCM) {
            return Ok(Self::AssertUserSwapBalancesStart);
        }
        if buf.starts_with(&ASSERT_USER_SWAP_BALANCES_END_IX_DISCM) {
            let mut reader = &buf[ASSERT_USER_SWAP_BALANCES_END_IX_DISCM.len()..];
            let max_input_amount_change: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_output_amount_change: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AssertUserSwapBalancesEnd(AssertUserSwapBalancesEndIxArgs {
                    max_input_amount_change,
                    min_output_amount_change,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeGlobalConfig => {
                writer.write_all(&INITIALIZE_GLOBAL_CONFIG_IX_DISCM)
            }
            Self::InitializeVault => writer.write_all(&INITIALIZE_VAULT_IX_DISCM),
            Self::CreateOrder(args) => {
                writer.write_all(&CREATE_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.input_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.output_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.order_type, &mut writer)?;
                Ok(())
            }
            Self::UpdateOrder(args) => {
                writer.write_all(&UPDATE_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.mode, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.value, &mut writer)?;
                Ok(())
            }
            Self::CloseOrderAndClaimTip => {
                writer.write_all(&CLOSE_ORDER_AND_CLAIM_TIP_IX_DISCM)
            }
            Self::TakeOrder(args) => {
                writer.write_all(&TAKE_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.input_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_output_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.tip_amount_permissionless_taking,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::FlashTakeOrderStart(args) => {
                writer.write_all(&FLASH_TAKE_ORDER_START_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.input_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_output_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.tip_amount_permissionless_taking,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::FlashTakeOrderEnd(args) => {
                writer.write_all(&FLASH_TAKE_ORDER_END_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.input_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_output_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.tip_amount_permissionless_taking,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateGlobalConfig(args) => {
                writer.write_all(&UPDATE_GLOBAL_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.mode, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.value, &mut writer)?;
                Ok(())
            }
            Self::UpdateGlobalConfigAdmin => {
                writer.write_all(&UPDATE_GLOBAL_CONFIG_ADMIN_IX_DISCM)
            }
            Self::WithdrawHostTip => writer.write_all(&WITHDRAW_HOST_TIP_IX_DISCM),
            Self::LogUserSwapBalancesStart => {
                writer.write_all(&LOG_USER_SWAP_BALANCES_START_IX_DISCM)
            }
            Self::LogUserSwapBalancesEnd(args) => {
                writer.write_all(&LOG_USER_SWAP_BALANCES_END_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.simulated_swap_amount_out,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.simulated_ts, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_amount_out, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.swap_amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.simulated_amount_out_next_best,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.aggregator, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.next_best_aggregator,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.padding, &mut writer)?;
                Ok(())
            }
            Self::AssertUserSwapBalancesStart => {
                writer.write_all(&ASSERT_USER_SWAP_BALANCES_START_IX_DISCM)
            }
            Self::AssertUserSwapBalancesEnd(args) => {
                writer.write_all(&ASSERT_USER_SWAP_BALANCES_END_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.max_input_amount_change,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.min_output_amount_change,
                    &mut writer,
                )?;
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
pub const INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeGlobalConfigAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub pda_authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeGlobalConfigKeys {
    pub admin_authority: Pubkey,
    pub pda_authority: Pubkey,
    pub global_config: Pubkey,
}
impl From<InitializeGlobalConfigAccounts<'_, '_>> for InitializeGlobalConfigKeys {
    fn from(accounts: InitializeGlobalConfigAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            pda_authority: *accounts.pda_authority.key,
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<InitializeGlobalConfigKeys>
for [AccountMeta; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeGlobalConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeGlobalConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            pda_authority: pubkeys[1],
            global_config: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeGlobalConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeGlobalConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.pda_authority.clone(),
            accounts.global_config.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeGlobalConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            pda_authority: &arr[1],
            global_config: &arr[2],
        }
    }
}
pub const INITIALIZE_GLOBAL_CONFIG_IX_DISCM: [u8; 8usize] = [
    113, 216, 122, 131, 225, 209, 22, 55,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeGlobalConfigIxData;
impl InitializeGlobalConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_GLOBAL_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_GLOBAL_CONFIG_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_global_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeGlobalConfigKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeGlobalConfigIxData.try_to_vec()?,
    })
}
pub fn initialize_global_config_ix(
    keys: InitializeGlobalConfigKeys,
) -> std::io::Result<Instruction> {
    initialize_global_config_ix_with_program_id(LIMO_PROGRAM_ID, keys)
}
pub fn initialize_global_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeGlobalConfigKeys = accounts.into();
    let ix = initialize_global_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_global_config_invoke(
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
) -> ProgramResult {
    initialize_global_config_invoke_with_program_id(LIMO_PROGRAM_ID, accounts)
}
pub fn initialize_global_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeGlobalConfigKeys = accounts.into();
    let ix = initialize_global_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_global_config_invoke_signed(
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_global_config_invoke_signed_with_program_id(
        LIMO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_global_config_verify_account_keys(
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
    keys: InitializeGlobalConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.pda_authority.key, keys.pda_authority),
        (*accounts.global_config.key, keys.global_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_global_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_authority,
        accounts.pda_authority,
        accounts.global_config,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_global_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_global_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_global_config_verify_writable_privileges(accounts)?;
    initialize_global_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VAULT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pda_authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeVaultKeys {
    pub payer: Pubkey,
    pub global_config: Pubkey,
    pub pda_authority: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeVaultAccounts<'_, '_>> for InitializeVaultKeys {
    fn from(accounts: InitializeVaultAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            global_config: *accounts.global_config.key,
            pda_authority: *accounts.pda_authority.key,
            mint: *accounts.mint.key,
            vault: *accounts.vault.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeVaultKeys> for [AccountMeta; INITIALIZE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
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
        ]
    }
}
impl From<[Pubkey; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]> for InitializeVaultKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            global_config: pubkeys[1],
            pda_authority: pubkeys[2],
            mint: pubkeys[3],
            vault: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.global_config.clone(),
            accounts.pda_authority.clone(),
            accounts.mint.clone(),
            accounts.vault.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]>
for InitializeVaultAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            global_config: &arr[1],
            pda_authority: &arr[2],
            mint: &arr[3],
            vault: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const INITIALIZE_VAULT_IX_DISCM: [u8; 8usize] = [48, 191, 163, 44, 71, 129, 63, 164];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultIxData;
impl InitializeVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_VAULT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVaultKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeVaultIxData.try_to_vec()?,
    })
}
pub fn initialize_vault_ix(keys: InitializeVaultKeys) -> std::io::Result<Instruction> {
    initialize_vault_ix_with_program_id(LIMO_PROGRAM_ID, keys)
}
pub fn initialize_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeVaultKeys = accounts.into();
    let ix = initialize_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_vault_invoke(
    accounts: InitializeVaultAccounts<'_, '_>,
) -> ProgramResult {
    initialize_vault_invoke_with_program_id(LIMO_PROGRAM_ID, accounts)
}
pub fn initialize_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVaultKeys = accounts.into();
    let ix = initialize_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_vault_invoke_signed(
    accounts: InitializeVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_vault_invoke_signed_with_program_id(LIMO_PROGRAM_ID, accounts, seeds)
}
pub fn initialize_vault_verify_account_keys(
    accounts: InitializeVaultAccounts<'_, '_>,
    keys: InitializeVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pda_authority.key, keys.pda_authority),
        (*accounts.mint.key, keys.mint),
        (*accounts.vault.key, keys.vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_vault_verify_writable_privileges<'me, 'info>(
    accounts: InitializeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.global_config, accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_vault_verify_signer_privileges<'me, 'info>(
    accounts: InitializeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_vault_verify_account_privileges<'me, 'info>(
    accounts: InitializeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_vault_verify_writable_privileges(accounts)?;
    initialize_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_ORDER_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct CreateOrderAccounts<'me, 'info> {
    pub maker: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pda_authority: &'me AccountInfo<'info>,
    pub order: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub maker_ata: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub input_token_program: &'me AccountInfo<'info>,
    pub output_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateOrderKeys {
    pub maker: Pubkey,
    pub global_config: Pubkey,
    pub pda_authority: Pubkey,
    pub order: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub maker_ata: Pubkey,
    pub input_vault: Pubkey,
    pub input_token_program: Pubkey,
    pub output_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateOrderAccounts<'_, '_>> for CreateOrderKeys {
    fn from(accounts: CreateOrderAccounts) -> Self {
        Self {
            maker: *accounts.maker.key,
            global_config: *accounts.global_config.key,
            pda_authority: *accounts.pda_authority.key,
            order: *accounts.order.key,
            input_mint: *accounts.input_mint.key,
            output_mint: *accounts.output_mint.key,
            maker_ata: *accounts.maker_ata.key,
            input_vault: *accounts.input_vault.key,
            input_token_program: *accounts.input_token_program.key,
            output_token_program: *accounts.output_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateOrderKeys> for [AccountMeta; CREATE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.maker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.maker_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_token_program,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; CREATE_ORDER_IX_ACCOUNTS_LEN]> for CreateOrderKeys {
    fn from(pubkeys: [Pubkey; CREATE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            maker: pubkeys[0],
            global_config: pubkeys[1],
            pda_authority: pubkeys[2],
            order: pubkeys[3],
            input_mint: pubkeys[4],
            output_mint: pubkeys[5],
            maker_ata: pubkeys[6],
            input_vault: pubkeys[7],
            input_token_program: pubkeys[8],
            output_token_program: pubkeys[9],
            system_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<CreateOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.maker.clone(),
            accounts.global_config.clone(),
            accounts.pda_authority.clone(),
            accounts.order.clone(),
            accounts.input_mint.clone(),
            accounts.output_mint.clone(),
            accounts.maker_ata.clone(),
            accounts.input_vault.clone(),
            accounts.input_token_program.clone(),
            accounts.output_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_ORDER_IX_ACCOUNTS_LEN]>
for CreateOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            maker: &arr[0],
            global_config: &arr[1],
            pda_authority: &arr[2],
            order: &arr[3],
            input_mint: &arr[4],
            output_mint: &arr[5],
            maker_ata: &arr[6],
            input_vault: &arr[7],
            input_token_program: &arr[8],
            output_token_program: &arr[9],
            system_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const CREATE_ORDER_IX_DISCM: [u8; 8usize] = [141, 54, 37, 207, 237, 210, 250, 215];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderIxArgs {
    pub input_amount: u64,
    pub output_amount: u64,
    pub order_type: u8,
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
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateOrderIxArgs {
                input_amount,
                output_amount,
                order_type,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.order_type, &mut writer)?;
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
    create_order_ix_with_program_id(LIMO_PROGRAM_ID, keys, args)
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
    create_order_invoke_with_program_id(LIMO_PROGRAM_ID, accounts, args)
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
    create_order_invoke_signed_with_program_id(LIMO_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_order_verify_account_keys(
    accounts: CreateOrderAccounts<'_, '_>,
    keys: CreateOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.maker.key, keys.maker),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pda_authority.key, keys.pda_authority),
        (*accounts.order.key, keys.order),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.maker_ata.key, keys.maker_ata),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.input_token_program.key, keys.input_token_program),
        (*accounts.output_token_program.key, keys.output_token_program),
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
pub fn create_order_verify_writable_privileges<'me, 'info>(
    accounts: CreateOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.maker,
        accounts.global_config,
        accounts.order,
        accounts.maker_ata,
        accounts.input_vault,
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
    for should_be_signer in [accounts.maker] {
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
pub const UPDATE_ORDER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateOrderAccounts<'me, 'info> {
    pub maker: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub order: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateOrderKeys {
    pub maker: Pubkey,
    pub global_config: Pubkey,
    pub order: Pubkey,
}
impl From<UpdateOrderAccounts<'_, '_>> for UpdateOrderKeys {
    fn from(accounts: UpdateOrderAccounts) -> Self {
        Self {
            maker: *accounts.maker.key,
            global_config: *accounts.global_config.key,
            order: *accounts.order.key,
        }
    }
}
impl From<UpdateOrderKeys> for [AccountMeta; UPDATE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.maker,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_ORDER_IX_ACCOUNTS_LEN]> for UpdateOrderKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            maker: pubkeys[0],
            global_config: pubkeys[1],
            order: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateOrderAccounts<'_, 'info>) -> Self {
        [accounts.maker.clone(), accounts.global_config.clone(), accounts.order.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ORDER_IX_ACCOUNTS_LEN]>
for UpdateOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            maker: &arr[0],
            global_config: &arr[1],
            order: &arr[2],
        }
    }
}
pub const UPDATE_ORDER_IX_DISCM: [u8; 8usize] = [54, 8, 208, 207, 34, 134, 239, 168];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOrderIxArgs {
    pub mode: u16,
    pub value: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOrderIxData(pub UpdateOrderIxArgs);
impl From<UpdateOrderIxArgs> for UpdateOrderIxData {
    fn from(args: UpdateOrderIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let mode: u16 = crate::borsh_de_or_default(&mut reader)?;
        let value: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(UpdateOrderIxArgs { mode, value }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.value, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_order_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateOrderKeys,
    args: UpdateOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_order_ix(
    keys: UpdateOrderKeys,
    args: UpdateOrderIxArgs,
) -> std::io::Result<Instruction> {
    update_order_ix_with_program_id(LIMO_PROGRAM_ID, keys, args)
}
pub fn update_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOrderAccounts<'_, '_>,
    args: UpdateOrderIxArgs,
) -> ProgramResult {
    let keys: UpdateOrderKeys = accounts.into();
    let ix = update_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_order_invoke(
    accounts: UpdateOrderAccounts<'_, '_>,
    args: UpdateOrderIxArgs,
) -> ProgramResult {
    update_order_invoke_with_program_id(LIMO_PROGRAM_ID, accounts, args)
}
pub fn update_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOrderAccounts<'_, '_>,
    args: UpdateOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateOrderKeys = accounts.into();
    let ix = update_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_order_invoke_signed(
    accounts: UpdateOrderAccounts<'_, '_>,
    args: UpdateOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_order_invoke_signed_with_program_id(LIMO_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_order_verify_account_keys(
    accounts: UpdateOrderAccounts<'_, '_>,
    keys: UpdateOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.maker.key, keys.maker),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.order.key, keys.order),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_order_verify_writable_privileges<'me, 'info>(
    accounts: UpdateOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.order] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_order_verify_signer_privileges<'me, 'info>(
    accounts: UpdateOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.maker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_order_verify_account_privileges<'me, 'info>(
    accounts: UpdateOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_order_verify_writable_privileges(accounts)?;
    update_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_ORDER_AND_CLAIM_TIP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CloseOrderAndClaimTipAccounts<'me, 'info> {
    pub maker: &'me AccountInfo<'info>,
    pub order: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pda_authority: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub maker_input_ata: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub input_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseOrderAndClaimTipKeys {
    pub maker: Pubkey,
    pub order: Pubkey,
    pub global_config: Pubkey,
    pub pda_authority: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub maker_input_ata: Pubkey,
    pub input_vault: Pubkey,
    pub input_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CloseOrderAndClaimTipAccounts<'_, '_>> for CloseOrderAndClaimTipKeys {
    fn from(accounts: CloseOrderAndClaimTipAccounts) -> Self {
        Self {
            maker: *accounts.maker.key,
            order: *accounts.order.key,
            global_config: *accounts.global_config.key,
            pda_authority: *accounts.pda_authority.key,
            input_mint: *accounts.input_mint.key,
            output_mint: *accounts.output_mint.key,
            maker_input_ata: *accounts.maker_input_ata.key,
            input_vault: *accounts.input_vault.key,
            input_token_program: *accounts.input_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CloseOrderAndClaimTipKeys>
for [AccountMeta; CLOSE_ORDER_AND_CLAIM_TIP_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseOrderAndClaimTipKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.maker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.maker_input_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_program,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; CLOSE_ORDER_AND_CLAIM_TIP_IX_ACCOUNTS_LEN]>
for CloseOrderAndClaimTipKeys {
    fn from(pubkeys: [Pubkey; CLOSE_ORDER_AND_CLAIM_TIP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            maker: pubkeys[0],
            order: pubkeys[1],
            global_config: pubkeys[2],
            pda_authority: pubkeys[3],
            input_mint: pubkeys[4],
            output_mint: pubkeys[5],
            maker_input_ata: pubkeys[6],
            input_vault: pubkeys[7],
            input_token_program: pubkeys[8],
            system_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<CloseOrderAndClaimTipAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_ORDER_AND_CLAIM_TIP_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseOrderAndClaimTipAccounts<'_, 'info>) -> Self {
        [
            accounts.maker.clone(),
            accounts.order.clone(),
            accounts.global_config.clone(),
            accounts.pda_authority.clone(),
            accounts.input_mint.clone(),
            accounts.output_mint.clone(),
            accounts.maker_input_ata.clone(),
            accounts.input_vault.clone(),
            accounts.input_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_ORDER_AND_CLAIM_TIP_IX_ACCOUNTS_LEN]>
for CloseOrderAndClaimTipAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_ORDER_AND_CLAIM_TIP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            maker: &arr[0],
            order: &arr[1],
            global_config: &arr[2],
            pda_authority: &arr[3],
            input_mint: &arr[4],
            output_mint: &arr[5],
            maker_input_ata: &arr[6],
            input_vault: &arr[7],
            input_token_program: &arr[8],
            system_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const CLOSE_ORDER_AND_CLAIM_TIP_IX_DISCM: [u8; 8usize] = [
    244, 27, 12, 226, 45, 247, 230, 43,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseOrderAndClaimTipIxData;
impl CloseOrderAndClaimTipIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_ORDER_AND_CLAIM_TIP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_ORDER_AND_CLAIM_TIP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_order_and_claim_tip_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseOrderAndClaimTipKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_ORDER_AND_CLAIM_TIP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseOrderAndClaimTipIxData.try_to_vec()?,
    })
}
pub fn close_order_and_claim_tip_ix(
    keys: CloseOrderAndClaimTipKeys,
) -> std::io::Result<Instruction> {
    close_order_and_claim_tip_ix_with_program_id(LIMO_PROGRAM_ID, keys)
}
pub fn close_order_and_claim_tip_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseOrderAndClaimTipAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseOrderAndClaimTipKeys = accounts.into();
    let ix = close_order_and_claim_tip_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_order_and_claim_tip_invoke(
    accounts: CloseOrderAndClaimTipAccounts<'_, '_>,
) -> ProgramResult {
    close_order_and_claim_tip_invoke_with_program_id(LIMO_PROGRAM_ID, accounts)
}
pub fn close_order_and_claim_tip_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseOrderAndClaimTipAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseOrderAndClaimTipKeys = accounts.into();
    let ix = close_order_and_claim_tip_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_order_and_claim_tip_invoke_signed(
    accounts: CloseOrderAndClaimTipAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_order_and_claim_tip_invoke_signed_with_program_id(
        LIMO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_order_and_claim_tip_verify_account_keys(
    accounts: CloseOrderAndClaimTipAccounts<'_, '_>,
    keys: CloseOrderAndClaimTipKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.maker.key, keys.maker),
        (*accounts.order.key, keys.order),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pda_authority.key, keys.pda_authority),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.maker_input_ata.key, keys.maker_input_ata),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.input_token_program.key, keys.input_token_program),
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
pub fn close_order_and_claim_tip_verify_writable_privileges<'me, 'info>(
    accounts: CloseOrderAndClaimTipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.maker,
        accounts.order,
        accounts.global_config,
        accounts.pda_authority,
        accounts.maker_input_ata,
        accounts.input_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_order_and_claim_tip_verify_signer_privileges<'me, 'info>(
    accounts: CloseOrderAndClaimTipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.maker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_order_and_claim_tip_verify_account_privileges<'me, 'info>(
    accounts: CloseOrderAndClaimTipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_order_and_claim_tip_verify_writable_privileges(accounts)?;
    close_order_and_claim_tip_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TAKE_ORDER_IX_ACCOUNTS_LEN: usize = 23;
#[derive(Copy, Clone, Debug)]
pub struct TakeOrderAccounts<'me, 'info> {
    pub taker: &'me AccountInfo<'info>,
    pub maker: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pda_authority: &'me AccountInfo<'info>,
    pub order: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub taker_input_ata: &'me AccountInfo<'info>,
    pub taker_output_ata: &'me AccountInfo<'info>,
    pub intermediary_output_token_account: &'me AccountInfo<'info>,
    pub maker_output_ata: &'me AccountInfo<'info>,
    pub express_relay: &'me AccountInfo<'info>,
    pub express_relay_metadata: &'me AccountInfo<'info>,
    pub sysvar_instructions: &'me AccountInfo<'info>,
    pub permission: &'me AccountInfo<'info>,
    pub config_router: &'me AccountInfo<'info>,
    pub input_token_program: &'me AccountInfo<'info>,
    pub output_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeOrderKeys {
    pub taker: Pubkey,
    pub maker: Pubkey,
    pub global_config: Pubkey,
    pub pda_authority: Pubkey,
    pub order: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub input_vault: Pubkey,
    pub taker_input_ata: Pubkey,
    pub taker_output_ata: Pubkey,
    pub intermediary_output_token_account: Pubkey,
    pub maker_output_ata: Pubkey,
    pub express_relay: Pubkey,
    pub express_relay_metadata: Pubkey,
    pub sysvar_instructions: Pubkey,
    pub permission: Pubkey,
    pub config_router: Pubkey,
    pub input_token_program: Pubkey,
    pub output_token_program: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<TakeOrderAccounts<'_, '_>> for TakeOrderKeys {
    fn from(accounts: TakeOrderAccounts) -> Self {
        Self {
            taker: *accounts.taker.key,
            maker: *accounts.maker.key,
            global_config: *accounts.global_config.key,
            pda_authority: *accounts.pda_authority.key,
            order: *accounts.order.key,
            input_mint: *accounts.input_mint.key,
            output_mint: *accounts.output_mint.key,
            input_vault: *accounts.input_vault.key,
            taker_input_ata: *accounts.taker_input_ata.key,
            taker_output_ata: *accounts.taker_output_ata.key,
            intermediary_output_token_account: *accounts
                .intermediary_output_token_account
                .key,
            maker_output_ata: *accounts.maker_output_ata.key,
            express_relay: *accounts.express_relay.key,
            express_relay_metadata: *accounts.express_relay_metadata.key,
            sysvar_instructions: *accounts.sysvar_instructions.key,
            permission: *accounts.permission.key,
            config_router: *accounts.config_router.key,
            input_token_program: *accounts.input_token_program.key,
            output_token_program: *accounts.output_token_program.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<TakeOrderKeys> for [AccountMeta; TAKE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: TakeOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.taker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker_input_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker_output_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.intermediary_output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker_output_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.express_relay,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.express_relay_metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sysvar_instructions,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.permission,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config_router,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; TAKE_ORDER_IX_ACCOUNTS_LEN]> for TakeOrderKeys {
    fn from(pubkeys: [Pubkey; TAKE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            taker: pubkeys[0],
            maker: pubkeys[1],
            global_config: pubkeys[2],
            pda_authority: pubkeys[3],
            order: pubkeys[4],
            input_mint: pubkeys[5],
            output_mint: pubkeys[6],
            input_vault: pubkeys[7],
            taker_input_ata: pubkeys[8],
            taker_output_ata: pubkeys[9],
            intermediary_output_token_account: pubkeys[10],
            maker_output_ata: pubkeys[11],
            express_relay: pubkeys[12],
            express_relay_metadata: pubkeys[13],
            sysvar_instructions: pubkeys[14],
            permission: pubkeys[15],
            config_router: pubkeys[16],
            input_token_program: pubkeys[17],
            output_token_program: pubkeys[18],
            rent: pubkeys[19],
            system_program: pubkeys[20],
            event_authority: pubkeys[21],
            program: pubkeys[22],
        }
    }
}
impl<'info> From<TakeOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; TAKE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: TakeOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.taker.clone(),
            accounts.maker.clone(),
            accounts.global_config.clone(),
            accounts.pda_authority.clone(),
            accounts.order.clone(),
            accounts.input_mint.clone(),
            accounts.output_mint.clone(),
            accounts.input_vault.clone(),
            accounts.taker_input_ata.clone(),
            accounts.taker_output_ata.clone(),
            accounts.intermediary_output_token_account.clone(),
            accounts.maker_output_ata.clone(),
            accounts.express_relay.clone(),
            accounts.express_relay_metadata.clone(),
            accounts.sysvar_instructions.clone(),
            accounts.permission.clone(),
            accounts.config_router.clone(),
            accounts.input_token_program.clone(),
            accounts.output_token_program.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TAKE_ORDER_IX_ACCOUNTS_LEN]>
for TakeOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TAKE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            taker: &arr[0],
            maker: &arr[1],
            global_config: &arr[2],
            pda_authority: &arr[3],
            order: &arr[4],
            input_mint: &arr[5],
            output_mint: &arr[6],
            input_vault: &arr[7],
            taker_input_ata: &arr[8],
            taker_output_ata: &arr[9],
            intermediary_output_token_account: &arr[10],
            maker_output_ata: &arr[11],
            express_relay: &arr[12],
            express_relay_metadata: &arr[13],
            sysvar_instructions: &arr[14],
            permission: &arr[15],
            config_router: &arr[16],
            input_token_program: &arr[17],
            output_token_program: &arr[18],
            rent: &arr[19],
            system_program: &arr[20],
            event_authority: &arr[21],
            program: &arr[22],
        }
    }
}
pub const TAKE_ORDER_IX_DISCM: [u8; 8usize] = [163, 208, 20, 172, 223, 65, 255, 228];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TakeOrderIxArgs {
    pub input_amount: u64,
    pub min_output_amount: u64,
    pub tip_amount_permissionless_taking: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TakeOrderIxData(pub TakeOrderIxArgs);
impl From<TakeOrderIxArgs> for TakeOrderIxData {
    fn from(args: TakeOrderIxArgs) -> Self {
        Self(args)
    }
}
impl TakeOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TAKE_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tip_amount_permissionless_taking: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(TakeOrderIxArgs {
                input_amount,
                min_output_amount,
                tip_amount_permissionless_taking,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TAKE_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.tip_amount_permissionless_taking,
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
pub fn take_order_ix_with_program_id(
    program_id: Pubkey,
    keys: TakeOrderKeys,
    args: TakeOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TAKE_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: TakeOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn take_order_ix(
    keys: TakeOrderKeys,
    args: TakeOrderIxArgs,
) -> std::io::Result<Instruction> {
    take_order_ix_with_program_id(LIMO_PROGRAM_ID, keys, args)
}
pub fn take_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TakeOrderAccounts<'_, '_>,
    args: TakeOrderIxArgs,
) -> ProgramResult {
    let keys: TakeOrderKeys = accounts.into();
    let ix = take_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn take_order_invoke(
    accounts: TakeOrderAccounts<'_, '_>,
    args: TakeOrderIxArgs,
) -> ProgramResult {
    take_order_invoke_with_program_id(LIMO_PROGRAM_ID, accounts, args)
}
pub fn take_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TakeOrderAccounts<'_, '_>,
    args: TakeOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TakeOrderKeys = accounts.into();
    let ix = take_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn take_order_invoke_signed(
    accounts: TakeOrderAccounts<'_, '_>,
    args: TakeOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    take_order_invoke_signed_with_program_id(LIMO_PROGRAM_ID, accounts, args, seeds)
}
pub fn take_order_verify_account_keys(
    accounts: TakeOrderAccounts<'_, '_>,
    keys: TakeOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.taker.key, keys.taker),
        (*accounts.maker.key, keys.maker),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pda_authority.key, keys.pda_authority),
        (*accounts.order.key, keys.order),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.taker_input_ata.key, keys.taker_input_ata),
        (*accounts.taker_output_ata.key, keys.taker_output_ata),
        (
            *accounts.intermediary_output_token_account.key,
            keys.intermediary_output_token_account,
        ),
        (*accounts.maker_output_ata.key, keys.maker_output_ata),
        (*accounts.express_relay.key, keys.express_relay),
        (*accounts.express_relay_metadata.key, keys.express_relay_metadata),
        (*accounts.sysvar_instructions.key, keys.sysvar_instructions),
        (*accounts.permission.key, keys.permission),
        (*accounts.config_router.key, keys.config_router),
        (*accounts.input_token_program.key, keys.input_token_program),
        (*accounts.output_token_program.key, keys.output_token_program),
        (*accounts.rent.key, keys.rent),
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
pub fn take_order_verify_writable_privileges<'me, 'info>(
    accounts: TakeOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.taker,
        accounts.maker,
        accounts.global_config,
        accounts.pda_authority,
        accounts.order,
        accounts.input_vault,
        accounts.taker_input_ata,
        accounts.taker_output_ata,
        accounts.intermediary_output_token_account,
        accounts.maker_output_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn take_order_verify_signer_privileges<'me, 'info>(
    accounts: TakeOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.taker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn take_order_verify_account_privileges<'me, 'info>(
    accounts: TakeOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    take_order_verify_writable_privileges(accounts)?;
    take_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FLASH_TAKE_ORDER_START_IX_ACCOUNTS_LEN: usize = 23;
#[derive(Copy, Clone, Debug)]
pub struct FlashTakeOrderStartAccounts<'me, 'info> {
    pub taker: &'me AccountInfo<'info>,
    pub maker: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pda_authority: &'me AccountInfo<'info>,
    pub order: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub taker_input_ata: &'me AccountInfo<'info>,
    pub taker_output_ata: &'me AccountInfo<'info>,
    pub intermediary_output_token_account: &'me AccountInfo<'info>,
    pub maker_output_ata: &'me AccountInfo<'info>,
    pub express_relay: &'me AccountInfo<'info>,
    pub express_relay_metadata: &'me AccountInfo<'info>,
    pub sysvar_instructions: &'me AccountInfo<'info>,
    pub permission: &'me AccountInfo<'info>,
    pub config_router: &'me AccountInfo<'info>,
    pub input_token_program: &'me AccountInfo<'info>,
    pub output_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlashTakeOrderStartKeys {
    pub taker: Pubkey,
    pub maker: Pubkey,
    pub global_config: Pubkey,
    pub pda_authority: Pubkey,
    pub order: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub input_vault: Pubkey,
    pub taker_input_ata: Pubkey,
    pub taker_output_ata: Pubkey,
    pub intermediary_output_token_account: Pubkey,
    pub maker_output_ata: Pubkey,
    pub express_relay: Pubkey,
    pub express_relay_metadata: Pubkey,
    pub sysvar_instructions: Pubkey,
    pub permission: Pubkey,
    pub config_router: Pubkey,
    pub input_token_program: Pubkey,
    pub output_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FlashTakeOrderStartAccounts<'_, '_>> for FlashTakeOrderStartKeys {
    fn from(accounts: FlashTakeOrderStartAccounts) -> Self {
        Self {
            taker: *accounts.taker.key,
            maker: *accounts.maker.key,
            global_config: *accounts.global_config.key,
            pda_authority: *accounts.pda_authority.key,
            order: *accounts.order.key,
            input_mint: *accounts.input_mint.key,
            output_mint: *accounts.output_mint.key,
            input_vault: *accounts.input_vault.key,
            taker_input_ata: *accounts.taker_input_ata.key,
            taker_output_ata: *accounts.taker_output_ata.key,
            intermediary_output_token_account: *accounts
                .intermediary_output_token_account
                .key,
            maker_output_ata: *accounts.maker_output_ata.key,
            express_relay: *accounts.express_relay.key,
            express_relay_metadata: *accounts.express_relay_metadata.key,
            sysvar_instructions: *accounts.sysvar_instructions.key,
            permission: *accounts.permission.key,
            config_router: *accounts.config_router.key,
            input_token_program: *accounts.input_token_program.key,
            output_token_program: *accounts.output_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FlashTakeOrderStartKeys>
for [AccountMeta; FLASH_TAKE_ORDER_START_IX_ACCOUNTS_LEN] {
    fn from(keys: FlashTakeOrderStartKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.taker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker_input_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker_output_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.intermediary_output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker_output_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.express_relay,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.express_relay_metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sysvar_instructions,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.permission,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config_router,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_token_program,
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
impl From<[Pubkey; FLASH_TAKE_ORDER_START_IX_ACCOUNTS_LEN]> for FlashTakeOrderStartKeys {
    fn from(pubkeys: [Pubkey; FLASH_TAKE_ORDER_START_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            taker: pubkeys[0],
            maker: pubkeys[1],
            global_config: pubkeys[2],
            pda_authority: pubkeys[3],
            order: pubkeys[4],
            input_mint: pubkeys[5],
            output_mint: pubkeys[6],
            input_vault: pubkeys[7],
            taker_input_ata: pubkeys[8],
            taker_output_ata: pubkeys[9],
            intermediary_output_token_account: pubkeys[10],
            maker_output_ata: pubkeys[11],
            express_relay: pubkeys[12],
            express_relay_metadata: pubkeys[13],
            sysvar_instructions: pubkeys[14],
            permission: pubkeys[15],
            config_router: pubkeys[16],
            input_token_program: pubkeys[17],
            output_token_program: pubkeys[18],
            system_program: pubkeys[19],
            rent: pubkeys[20],
            event_authority: pubkeys[21],
            program: pubkeys[22],
        }
    }
}
impl<'info> From<FlashTakeOrderStartAccounts<'_, 'info>>
for [AccountInfo<'info>; FLASH_TAKE_ORDER_START_IX_ACCOUNTS_LEN] {
    fn from(accounts: FlashTakeOrderStartAccounts<'_, 'info>) -> Self {
        [
            accounts.taker.clone(),
            accounts.maker.clone(),
            accounts.global_config.clone(),
            accounts.pda_authority.clone(),
            accounts.order.clone(),
            accounts.input_mint.clone(),
            accounts.output_mint.clone(),
            accounts.input_vault.clone(),
            accounts.taker_input_ata.clone(),
            accounts.taker_output_ata.clone(),
            accounts.intermediary_output_token_account.clone(),
            accounts.maker_output_ata.clone(),
            accounts.express_relay.clone(),
            accounts.express_relay_metadata.clone(),
            accounts.sysvar_instructions.clone(),
            accounts.permission.clone(),
            accounts.config_router.clone(),
            accounts.input_token_program.clone(),
            accounts.output_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FLASH_TAKE_ORDER_START_IX_ACCOUNTS_LEN]>
for FlashTakeOrderStartAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; FLASH_TAKE_ORDER_START_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            taker: &arr[0],
            maker: &arr[1],
            global_config: &arr[2],
            pda_authority: &arr[3],
            order: &arr[4],
            input_mint: &arr[5],
            output_mint: &arr[6],
            input_vault: &arr[7],
            taker_input_ata: &arr[8],
            taker_output_ata: &arr[9],
            intermediary_output_token_account: &arr[10],
            maker_output_ata: &arr[11],
            express_relay: &arr[12],
            express_relay_metadata: &arr[13],
            sysvar_instructions: &arr[14],
            permission: &arr[15],
            config_router: &arr[16],
            input_token_program: &arr[17],
            output_token_program: &arr[18],
            system_program: &arr[19],
            rent: &arr[20],
            event_authority: &arr[21],
            program: &arr[22],
        }
    }
}
pub const FLASH_TAKE_ORDER_START_IX_DISCM: [u8; 8usize] = [
    126, 53, 176, 15, 39, 103, 97, 243,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlashTakeOrderStartIxArgs {
    pub input_amount: u64,
    pub min_output_amount: u64,
    pub tip_amount_permissionless_taking: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FlashTakeOrderStartIxData(pub FlashTakeOrderStartIxArgs);
impl From<FlashTakeOrderStartIxArgs> for FlashTakeOrderStartIxData {
    fn from(args: FlashTakeOrderStartIxArgs) -> Self {
        Self(args)
    }
}
impl FlashTakeOrderStartIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FLASH_TAKE_ORDER_START_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tip_amount_permissionless_taking: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(FlashTakeOrderStartIxArgs {
                input_amount,
                min_output_amount,
                tip_amount_permissionless_taking,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FLASH_TAKE_ORDER_START_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.tip_amount_permissionless_taking,
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
pub fn flash_take_order_start_ix_with_program_id(
    program_id: Pubkey,
    keys: FlashTakeOrderStartKeys,
    args: FlashTakeOrderStartIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FLASH_TAKE_ORDER_START_IX_ACCOUNTS_LEN] = keys.into();
    let data: FlashTakeOrderStartIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn flash_take_order_start_ix(
    keys: FlashTakeOrderStartKeys,
    args: FlashTakeOrderStartIxArgs,
) -> std::io::Result<Instruction> {
    flash_take_order_start_ix_with_program_id(LIMO_PROGRAM_ID, keys, args)
}
pub fn flash_take_order_start_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FlashTakeOrderStartAccounts<'_, '_>,
    args: FlashTakeOrderStartIxArgs,
) -> ProgramResult {
    let keys: FlashTakeOrderStartKeys = accounts.into();
    let ix = flash_take_order_start_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn flash_take_order_start_invoke(
    accounts: FlashTakeOrderStartAccounts<'_, '_>,
    args: FlashTakeOrderStartIxArgs,
) -> ProgramResult {
    flash_take_order_start_invoke_with_program_id(LIMO_PROGRAM_ID, accounts, args)
}
pub fn flash_take_order_start_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FlashTakeOrderStartAccounts<'_, '_>,
    args: FlashTakeOrderStartIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FlashTakeOrderStartKeys = accounts.into();
    let ix = flash_take_order_start_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn flash_take_order_start_invoke_signed(
    accounts: FlashTakeOrderStartAccounts<'_, '_>,
    args: FlashTakeOrderStartIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    flash_take_order_start_invoke_signed_with_program_id(
        LIMO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn flash_take_order_start_verify_account_keys(
    accounts: FlashTakeOrderStartAccounts<'_, '_>,
    keys: FlashTakeOrderStartKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.taker.key, keys.taker),
        (*accounts.maker.key, keys.maker),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pda_authority.key, keys.pda_authority),
        (*accounts.order.key, keys.order),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.taker_input_ata.key, keys.taker_input_ata),
        (*accounts.taker_output_ata.key, keys.taker_output_ata),
        (
            *accounts.intermediary_output_token_account.key,
            keys.intermediary_output_token_account,
        ),
        (*accounts.maker_output_ata.key, keys.maker_output_ata),
        (*accounts.express_relay.key, keys.express_relay),
        (*accounts.express_relay_metadata.key, keys.express_relay_metadata),
        (*accounts.sysvar_instructions.key, keys.sysvar_instructions),
        (*accounts.permission.key, keys.permission),
        (*accounts.config_router.key, keys.config_router),
        (*accounts.input_token_program.key, keys.input_token_program),
        (*accounts.output_token_program.key, keys.output_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn flash_take_order_start_verify_writable_privileges<'me, 'info>(
    accounts: FlashTakeOrderStartAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.taker,
        accounts.maker,
        accounts.global_config,
        accounts.pda_authority,
        accounts.order,
        accounts.input_vault,
        accounts.taker_input_ata,
        accounts.taker_output_ata,
        accounts.intermediary_output_token_account,
        accounts.maker_output_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn flash_take_order_start_verify_signer_privileges<'me, 'info>(
    accounts: FlashTakeOrderStartAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.taker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn flash_take_order_start_verify_account_privileges<'me, 'info>(
    accounts: FlashTakeOrderStartAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    flash_take_order_start_verify_writable_privileges(accounts)?;
    flash_take_order_start_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FLASH_TAKE_ORDER_END_IX_ACCOUNTS_LEN: usize = 23;
#[derive(Copy, Clone, Debug)]
pub struct FlashTakeOrderEndAccounts<'me, 'info> {
    pub taker: &'me AccountInfo<'info>,
    pub maker: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pda_authority: &'me AccountInfo<'info>,
    pub order: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub taker_input_ata: &'me AccountInfo<'info>,
    pub taker_output_ata: &'me AccountInfo<'info>,
    pub intermediary_output_token_account: &'me AccountInfo<'info>,
    pub maker_output_ata: &'me AccountInfo<'info>,
    pub express_relay: &'me AccountInfo<'info>,
    pub express_relay_metadata: &'me AccountInfo<'info>,
    pub sysvar_instructions: &'me AccountInfo<'info>,
    pub permission: &'me AccountInfo<'info>,
    pub config_router: &'me AccountInfo<'info>,
    pub input_token_program: &'me AccountInfo<'info>,
    pub output_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlashTakeOrderEndKeys {
    pub taker: Pubkey,
    pub maker: Pubkey,
    pub global_config: Pubkey,
    pub pda_authority: Pubkey,
    pub order: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub input_vault: Pubkey,
    pub taker_input_ata: Pubkey,
    pub taker_output_ata: Pubkey,
    pub intermediary_output_token_account: Pubkey,
    pub maker_output_ata: Pubkey,
    pub express_relay: Pubkey,
    pub express_relay_metadata: Pubkey,
    pub sysvar_instructions: Pubkey,
    pub permission: Pubkey,
    pub config_router: Pubkey,
    pub input_token_program: Pubkey,
    pub output_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FlashTakeOrderEndAccounts<'_, '_>> for FlashTakeOrderEndKeys {
    fn from(accounts: FlashTakeOrderEndAccounts) -> Self {
        Self {
            taker: *accounts.taker.key,
            maker: *accounts.maker.key,
            global_config: *accounts.global_config.key,
            pda_authority: *accounts.pda_authority.key,
            order: *accounts.order.key,
            input_mint: *accounts.input_mint.key,
            output_mint: *accounts.output_mint.key,
            input_vault: *accounts.input_vault.key,
            taker_input_ata: *accounts.taker_input_ata.key,
            taker_output_ata: *accounts.taker_output_ata.key,
            intermediary_output_token_account: *accounts
                .intermediary_output_token_account
                .key,
            maker_output_ata: *accounts.maker_output_ata.key,
            express_relay: *accounts.express_relay.key,
            express_relay_metadata: *accounts.express_relay_metadata.key,
            sysvar_instructions: *accounts.sysvar_instructions.key,
            permission: *accounts.permission.key,
            config_router: *accounts.config_router.key,
            input_token_program: *accounts.input_token_program.key,
            output_token_program: *accounts.output_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FlashTakeOrderEndKeys>
for [AccountMeta; FLASH_TAKE_ORDER_END_IX_ACCOUNTS_LEN] {
    fn from(keys: FlashTakeOrderEndKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.taker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker_input_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker_output_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.intermediary_output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker_output_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.express_relay,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.express_relay_metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sysvar_instructions,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.permission,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config_router,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_token_program,
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
impl From<[Pubkey; FLASH_TAKE_ORDER_END_IX_ACCOUNTS_LEN]> for FlashTakeOrderEndKeys {
    fn from(pubkeys: [Pubkey; FLASH_TAKE_ORDER_END_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            taker: pubkeys[0],
            maker: pubkeys[1],
            global_config: pubkeys[2],
            pda_authority: pubkeys[3],
            order: pubkeys[4],
            input_mint: pubkeys[5],
            output_mint: pubkeys[6],
            input_vault: pubkeys[7],
            taker_input_ata: pubkeys[8],
            taker_output_ata: pubkeys[9],
            intermediary_output_token_account: pubkeys[10],
            maker_output_ata: pubkeys[11],
            express_relay: pubkeys[12],
            express_relay_metadata: pubkeys[13],
            sysvar_instructions: pubkeys[14],
            permission: pubkeys[15],
            config_router: pubkeys[16],
            input_token_program: pubkeys[17],
            output_token_program: pubkeys[18],
            system_program: pubkeys[19],
            rent: pubkeys[20],
            event_authority: pubkeys[21],
            program: pubkeys[22],
        }
    }
}
impl<'info> From<FlashTakeOrderEndAccounts<'_, 'info>>
for [AccountInfo<'info>; FLASH_TAKE_ORDER_END_IX_ACCOUNTS_LEN] {
    fn from(accounts: FlashTakeOrderEndAccounts<'_, 'info>) -> Self {
        [
            accounts.taker.clone(),
            accounts.maker.clone(),
            accounts.global_config.clone(),
            accounts.pda_authority.clone(),
            accounts.order.clone(),
            accounts.input_mint.clone(),
            accounts.output_mint.clone(),
            accounts.input_vault.clone(),
            accounts.taker_input_ata.clone(),
            accounts.taker_output_ata.clone(),
            accounts.intermediary_output_token_account.clone(),
            accounts.maker_output_ata.clone(),
            accounts.express_relay.clone(),
            accounts.express_relay_metadata.clone(),
            accounts.sysvar_instructions.clone(),
            accounts.permission.clone(),
            accounts.config_router.clone(),
            accounts.input_token_program.clone(),
            accounts.output_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FLASH_TAKE_ORDER_END_IX_ACCOUNTS_LEN]>
for FlashTakeOrderEndAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; FLASH_TAKE_ORDER_END_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            taker: &arr[0],
            maker: &arr[1],
            global_config: &arr[2],
            pda_authority: &arr[3],
            order: &arr[4],
            input_mint: &arr[5],
            output_mint: &arr[6],
            input_vault: &arr[7],
            taker_input_ata: &arr[8],
            taker_output_ata: &arr[9],
            intermediary_output_token_account: &arr[10],
            maker_output_ata: &arr[11],
            express_relay: &arr[12],
            express_relay_metadata: &arr[13],
            sysvar_instructions: &arr[14],
            permission: &arr[15],
            config_router: &arr[16],
            input_token_program: &arr[17],
            output_token_program: &arr[18],
            system_program: &arr[19],
            rent: &arr[20],
            event_authority: &arr[21],
            program: &arr[22],
        }
    }
}
pub const FLASH_TAKE_ORDER_END_IX_DISCM: [u8; 8usize] = [
    206, 242, 215, 187, 134, 33, 224, 148,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlashTakeOrderEndIxArgs {
    pub input_amount: u64,
    pub min_output_amount: u64,
    pub tip_amount_permissionless_taking: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FlashTakeOrderEndIxData(pub FlashTakeOrderEndIxArgs);
impl From<FlashTakeOrderEndIxArgs> for FlashTakeOrderEndIxData {
    fn from(args: FlashTakeOrderEndIxArgs) -> Self {
        Self(args)
    }
}
impl FlashTakeOrderEndIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FLASH_TAKE_ORDER_END_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tip_amount_permissionless_taking: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(FlashTakeOrderEndIxArgs {
                input_amount,
                min_output_amount,
                tip_amount_permissionless_taking,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FLASH_TAKE_ORDER_END_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.tip_amount_permissionless_taking,
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
pub fn flash_take_order_end_ix_with_program_id(
    program_id: Pubkey,
    keys: FlashTakeOrderEndKeys,
    args: FlashTakeOrderEndIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FLASH_TAKE_ORDER_END_IX_ACCOUNTS_LEN] = keys.into();
    let data: FlashTakeOrderEndIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn flash_take_order_end_ix(
    keys: FlashTakeOrderEndKeys,
    args: FlashTakeOrderEndIxArgs,
) -> std::io::Result<Instruction> {
    flash_take_order_end_ix_with_program_id(LIMO_PROGRAM_ID, keys, args)
}
pub fn flash_take_order_end_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FlashTakeOrderEndAccounts<'_, '_>,
    args: FlashTakeOrderEndIxArgs,
) -> ProgramResult {
    let keys: FlashTakeOrderEndKeys = accounts.into();
    let ix = flash_take_order_end_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn flash_take_order_end_invoke(
    accounts: FlashTakeOrderEndAccounts<'_, '_>,
    args: FlashTakeOrderEndIxArgs,
) -> ProgramResult {
    flash_take_order_end_invoke_with_program_id(LIMO_PROGRAM_ID, accounts, args)
}
pub fn flash_take_order_end_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FlashTakeOrderEndAccounts<'_, '_>,
    args: FlashTakeOrderEndIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FlashTakeOrderEndKeys = accounts.into();
    let ix = flash_take_order_end_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn flash_take_order_end_invoke_signed(
    accounts: FlashTakeOrderEndAccounts<'_, '_>,
    args: FlashTakeOrderEndIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    flash_take_order_end_invoke_signed_with_program_id(
        LIMO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn flash_take_order_end_verify_account_keys(
    accounts: FlashTakeOrderEndAccounts<'_, '_>,
    keys: FlashTakeOrderEndKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.taker.key, keys.taker),
        (*accounts.maker.key, keys.maker),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pda_authority.key, keys.pda_authority),
        (*accounts.order.key, keys.order),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.taker_input_ata.key, keys.taker_input_ata),
        (*accounts.taker_output_ata.key, keys.taker_output_ata),
        (
            *accounts.intermediary_output_token_account.key,
            keys.intermediary_output_token_account,
        ),
        (*accounts.maker_output_ata.key, keys.maker_output_ata),
        (*accounts.express_relay.key, keys.express_relay),
        (*accounts.express_relay_metadata.key, keys.express_relay_metadata),
        (*accounts.sysvar_instructions.key, keys.sysvar_instructions),
        (*accounts.permission.key, keys.permission),
        (*accounts.config_router.key, keys.config_router),
        (*accounts.input_token_program.key, keys.input_token_program),
        (*accounts.output_token_program.key, keys.output_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn flash_take_order_end_verify_writable_privileges<'me, 'info>(
    accounts: FlashTakeOrderEndAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.taker,
        accounts.maker,
        accounts.global_config,
        accounts.pda_authority,
        accounts.order,
        accounts.input_vault,
        accounts.taker_input_ata,
        accounts.taker_output_ata,
        accounts.intermediary_output_token_account,
        accounts.maker_output_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn flash_take_order_end_verify_signer_privileges<'me, 'info>(
    accounts: FlashTakeOrderEndAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.taker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn flash_take_order_end_verify_account_privileges<'me, 'info>(
    accounts: FlashTakeOrderEndAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    flash_take_order_end_verify_writable_privileges(accounts)?;
    flash_take_order_end_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateGlobalConfigAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateGlobalConfigKeys {
    pub admin_authority: Pubkey,
    pub global_config: Pubkey,
}
impl From<UpdateGlobalConfigAccounts<'_, '_>> for UpdateGlobalConfigKeys {
    fn from(accounts: UpdateGlobalConfigAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<UpdateGlobalConfigKeys>
for [AccountMeta; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateGlobalConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]> for UpdateGlobalConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            global_config: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateGlobalConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateGlobalConfigAccounts<'_, 'info>) -> Self {
        [accounts.admin_authority.clone(), accounts.global_config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateGlobalConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            global_config: &arr[1],
        }
    }
}
pub const UPDATE_GLOBAL_CONFIG_IX_DISCM: [u8; 8usize] = [
    164, 84, 130, 189, 111, 58, 250, 200,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateGlobalConfigIxArgs {
    pub mode: u16,
    #[serde(with = "crate::big_array_serde")]
    pub value: [u8; 128],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateGlobalConfigIxData(pub UpdateGlobalConfigIxArgs);
impl From<UpdateGlobalConfigIxArgs> for UpdateGlobalConfigIxData {
    fn from(args: UpdateGlobalConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateGlobalConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_GLOBAL_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let mode: u16 = crate::borsh_de_or_default(&mut reader)?;
        let value = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        Ok(
            Self(UpdateGlobalConfigIxArgs {
                mode,
                value,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_GLOBAL_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.value, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_global_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateGlobalConfigKeys,
    args: UpdateGlobalConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateGlobalConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_global_config_ix(
    keys: UpdateGlobalConfigKeys,
    args: UpdateGlobalConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_global_config_ix_with_program_id(LIMO_PROGRAM_ID, keys, args)
}
pub fn update_global_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    args: UpdateGlobalConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateGlobalConfigKeys = accounts.into();
    let ix = update_global_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_global_config_invoke(
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    args: UpdateGlobalConfigIxArgs,
) -> ProgramResult {
    update_global_config_invoke_with_program_id(LIMO_PROGRAM_ID, accounts, args)
}
pub fn update_global_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    args: UpdateGlobalConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateGlobalConfigKeys = accounts.into();
    let ix = update_global_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_global_config_invoke_signed(
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    args: UpdateGlobalConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_global_config_invoke_signed_with_program_id(
        LIMO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_global_config_verify_account_keys(
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    keys: UpdateGlobalConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.global_config.key, keys.global_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_global_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin_authority, accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_global_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_global_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_global_config_verify_writable_privileges(accounts)?;
    update_global_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateGlobalConfigAdminAccounts<'me, 'info> {
    pub admin_authority_cached: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateGlobalConfigAdminKeys {
    pub admin_authority_cached: Pubkey,
    pub global_config: Pubkey,
}
impl From<UpdateGlobalConfigAdminAccounts<'_, '_>> for UpdateGlobalConfigAdminKeys {
    fn from(accounts: UpdateGlobalConfigAdminAccounts) -> Self {
        Self {
            admin_authority_cached: *accounts.admin_authority_cached.key,
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<UpdateGlobalConfigAdminKeys>
for [AccountMeta; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateGlobalConfigAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority_cached,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN]>
for UpdateGlobalConfigAdminKeys {
    fn from(pubkeys: [Pubkey; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority_cached: pubkeys[0],
            global_config: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateGlobalConfigAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateGlobalConfigAdminAccounts<'_, 'info>) -> Self {
        [accounts.admin_authority_cached.clone(), accounts.global_config.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN]>
for UpdateGlobalConfigAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority_cached: &arr[0],
            global_config: &arr[1],
        }
    }
}
pub const UPDATE_GLOBAL_CONFIG_ADMIN_IX_DISCM: [u8; 8usize] = [
    184, 87, 23, 193, 156, 238, 175, 119,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateGlobalConfigAdminIxData;
impl UpdateGlobalConfigAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_GLOBAL_CONFIG_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_GLOBAL_CONFIG_ADMIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_global_config_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateGlobalConfigAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateGlobalConfigAdminIxData.try_to_vec()?,
    })
}
pub fn update_global_config_admin_ix(
    keys: UpdateGlobalConfigAdminKeys,
) -> std::io::Result<Instruction> {
    update_global_config_admin_ix_with_program_id(LIMO_PROGRAM_ID, keys)
}
pub fn update_global_config_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalConfigAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateGlobalConfigAdminKeys = accounts.into();
    let ix = update_global_config_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_global_config_admin_invoke(
    accounts: UpdateGlobalConfigAdminAccounts<'_, '_>,
) -> ProgramResult {
    update_global_config_admin_invoke_with_program_id(LIMO_PROGRAM_ID, accounts)
}
pub fn update_global_config_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalConfigAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateGlobalConfigAdminKeys = accounts.into();
    let ix = update_global_config_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_global_config_admin_invoke_signed(
    accounts: UpdateGlobalConfigAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_global_config_admin_invoke_signed_with_program_id(
        LIMO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_global_config_admin_verify_account_keys(
    accounts: UpdateGlobalConfigAdminAccounts<'_, '_>,
    keys: UpdateGlobalConfigAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority_cached.key, keys.admin_authority_cached),
        (*accounts.global_config.key, keys.global_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_global_config_admin_verify_writable_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_global_config_admin_verify_signer_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority_cached] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_global_config_admin_verify_account_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_global_config_admin_verify_writable_privileges(accounts)?;
    update_global_config_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_HOST_TIP_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawHostTipAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pda_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawHostTipKeys {
    pub admin_authority: Pubkey,
    pub global_config: Pubkey,
    pub pda_authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<WithdrawHostTipAccounts<'_, '_>> for WithdrawHostTipKeys {
    fn from(accounts: WithdrawHostTipAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            global_config: *accounts.global_config.key,
            pda_authority: *accounts.pda_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<WithdrawHostTipKeys> for [AccountMeta; WITHDRAW_HOST_TIP_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawHostTipKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_authority,
                is_signer: false,
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
impl From<[Pubkey; WITHDRAW_HOST_TIP_IX_ACCOUNTS_LEN]> for WithdrawHostTipKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_HOST_TIP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            global_config: pubkeys[1],
            pda_authority: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<WithdrawHostTipAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_HOST_TIP_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawHostTipAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.global_config.clone(),
            accounts.pda_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_HOST_TIP_IX_ACCOUNTS_LEN]>
for WithdrawHostTipAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_HOST_TIP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: &arr[0],
            global_config: &arr[1],
            pda_authority: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const WITHDRAW_HOST_TIP_IX_DISCM: [u8; 8usize] = [
    140, 246, 105, 165, 80, 85, 143, 18,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawHostTipIxData;
impl WithdrawHostTipIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_HOST_TIP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_HOST_TIP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_host_tip_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawHostTipKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_HOST_TIP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawHostTipIxData.try_to_vec()?,
    })
}
pub fn withdraw_host_tip_ix(keys: WithdrawHostTipKeys) -> std::io::Result<Instruction> {
    withdraw_host_tip_ix_with_program_id(LIMO_PROGRAM_ID, keys)
}
pub fn withdraw_host_tip_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawHostTipAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawHostTipKeys = accounts.into();
    let ix = withdraw_host_tip_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_host_tip_invoke(
    accounts: WithdrawHostTipAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_host_tip_invoke_with_program_id(LIMO_PROGRAM_ID, accounts)
}
pub fn withdraw_host_tip_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawHostTipAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawHostTipKeys = accounts.into();
    let ix = withdraw_host_tip_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_host_tip_invoke_signed(
    accounts: WithdrawHostTipAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_host_tip_invoke_signed_with_program_id(LIMO_PROGRAM_ID, accounts, seeds)
}
pub fn withdraw_host_tip_verify_account_keys(
    accounts: WithdrawHostTipAccounts<'_, '_>,
    keys: WithdrawHostTipKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pda_authority.key, keys.pda_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_host_tip_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawHostTipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_authority,
        accounts.global_config,
        accounts.pda_authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_host_tip_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawHostTipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_host_tip_verify_account_privileges<'me, 'info>(
    accounts: WithdrawHostTipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_host_tip_verify_writable_privileges(accounts)?;
    withdraw_host_tip_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LOG_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct LogUserSwapBalancesStartAccounts<'me, 'info> {
    pub base_accounts_maker: &'me AccountInfo<'info>,
    pub base_accounts_input_mint: &'me AccountInfo<'info>,
    pub base_accounts_output_mint: &'me AccountInfo<'info>,
    pub base_accounts_input_ta: &'me AccountInfo<'info>,
    pub base_accounts_output_ta: &'me AccountInfo<'info>,
    pub base_accounts_pda_referrer: &'me AccountInfo<'info>,
    pub base_accounts_swap_program_id: &'me AccountInfo<'info>,
    pub user_swap_balance_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub sysvar_instructions: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LogUserSwapBalancesStartKeys {
    pub base_accounts_maker: Pubkey,
    pub base_accounts_input_mint: Pubkey,
    pub base_accounts_output_mint: Pubkey,
    pub base_accounts_input_ta: Pubkey,
    pub base_accounts_output_ta: Pubkey,
    pub base_accounts_pda_referrer: Pubkey,
    pub base_accounts_swap_program_id: Pubkey,
    pub user_swap_balance_state: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub sysvar_instructions: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<LogUserSwapBalancesStartAccounts<'_, '_>> for LogUserSwapBalancesStartKeys {
    fn from(accounts: LogUserSwapBalancesStartAccounts) -> Self {
        Self {
            base_accounts_maker: *accounts.base_accounts_maker.key,
            base_accounts_input_mint: *accounts.base_accounts_input_mint.key,
            base_accounts_output_mint: *accounts.base_accounts_output_mint.key,
            base_accounts_input_ta: *accounts.base_accounts_input_ta.key,
            base_accounts_output_ta: *accounts.base_accounts_output_ta.key,
            base_accounts_pda_referrer: *accounts.base_accounts_pda_referrer.key,
            base_accounts_swap_program_id: *accounts.base_accounts_swap_program_id.key,
            user_swap_balance_state: *accounts.user_swap_balance_state.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            sysvar_instructions: *accounts.sysvar_instructions.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<LogUserSwapBalancesStartKeys>
for [AccountMeta; LOG_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN] {
    fn from(keys: LogUserSwapBalancesStartKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.base_accounts_maker,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_input_ta,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_output_ta,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_pda_referrer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_swap_program_id,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_swap_balance_state,
                is_signer: false,
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
                pubkey: keys.sysvar_instructions,
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
impl From<[Pubkey; LOG_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN]>
for LogUserSwapBalancesStartKeys {
    fn from(pubkeys: [Pubkey; LOG_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base_accounts_maker: pubkeys[0],
            base_accounts_input_mint: pubkeys[1],
            base_accounts_output_mint: pubkeys[2],
            base_accounts_input_ta: pubkeys[3],
            base_accounts_output_ta: pubkeys[4],
            base_accounts_pda_referrer: pubkeys[5],
            base_accounts_swap_program_id: pubkeys[6],
            user_swap_balance_state: pubkeys[7],
            system_program: pubkeys[8],
            rent: pubkeys[9],
            sysvar_instructions: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<LogUserSwapBalancesStartAccounts<'_, 'info>>
for [AccountInfo<'info>; LOG_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN] {
    fn from(accounts: LogUserSwapBalancesStartAccounts<'_, 'info>) -> Self {
        [
            accounts.base_accounts_maker.clone(),
            accounts.base_accounts_input_mint.clone(),
            accounts.base_accounts_output_mint.clone(),
            accounts.base_accounts_input_ta.clone(),
            accounts.base_accounts_output_ta.clone(),
            accounts.base_accounts_pda_referrer.clone(),
            accounts.base_accounts_swap_program_id.clone(),
            accounts.user_swap_balance_state.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.sysvar_instructions.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LOG_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN]>
for LogUserSwapBalancesStartAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LOG_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            base_accounts_maker: &arr[0],
            base_accounts_input_mint: &arr[1],
            base_accounts_output_mint: &arr[2],
            base_accounts_input_ta: &arr[3],
            base_accounts_output_ta: &arr[4],
            base_accounts_pda_referrer: &arr[5],
            base_accounts_swap_program_id: &arr[6],
            user_swap_balance_state: &arr[7],
            system_program: &arr[8],
            rent: &arr[9],
            sysvar_instructions: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const LOG_USER_SWAP_BALANCES_START_IX_DISCM: [u8; 8usize] = [
    133, 108, 23, 15, 226, 215, 176, 95,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LogUserSwapBalancesStartIxData;
impl LogUserSwapBalancesStartIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_USER_SWAP_BALANCES_START_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_USER_SWAP_BALANCES_START_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn log_user_swap_balances_start_ix_with_program_id(
    program_id: Pubkey,
    keys: LogUserSwapBalancesStartKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LOG_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LogUserSwapBalancesStartIxData.try_to_vec()?,
    })
}
pub fn log_user_swap_balances_start_ix(
    keys: LogUserSwapBalancesStartKeys,
) -> std::io::Result<Instruction> {
    log_user_swap_balances_start_ix_with_program_id(LIMO_PROGRAM_ID, keys)
}
pub fn log_user_swap_balances_start_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LogUserSwapBalancesStartAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LogUserSwapBalancesStartKeys = accounts.into();
    let ix = log_user_swap_balances_start_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn log_user_swap_balances_start_invoke(
    accounts: LogUserSwapBalancesStartAccounts<'_, '_>,
) -> ProgramResult {
    log_user_swap_balances_start_invoke_with_program_id(LIMO_PROGRAM_ID, accounts)
}
pub fn log_user_swap_balances_start_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LogUserSwapBalancesStartAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LogUserSwapBalancesStartKeys = accounts.into();
    let ix = log_user_swap_balances_start_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn log_user_swap_balances_start_invoke_signed(
    accounts: LogUserSwapBalancesStartAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    log_user_swap_balances_start_invoke_signed_with_program_id(
        LIMO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn log_user_swap_balances_start_verify_account_keys(
    accounts: LogUserSwapBalancesStartAccounts<'_, '_>,
    keys: LogUserSwapBalancesStartKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.base_accounts_maker.key, keys.base_accounts_maker),
        (*accounts.base_accounts_input_mint.key, keys.base_accounts_input_mint),
        (*accounts.base_accounts_output_mint.key, keys.base_accounts_output_mint),
        (*accounts.base_accounts_input_ta.key, keys.base_accounts_input_ta),
        (*accounts.base_accounts_output_ta.key, keys.base_accounts_output_ta),
        (*accounts.base_accounts_pda_referrer.key, keys.base_accounts_pda_referrer),
        (
            *accounts.base_accounts_swap_program_id.key,
            keys.base_accounts_swap_program_id,
        ),
        (*accounts.user_swap_balance_state.key, keys.user_swap_balance_state),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.sysvar_instructions.key, keys.sysvar_instructions),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn log_user_swap_balances_start_verify_writable_privileges<'me, 'info>(
    accounts: LogUserSwapBalancesStartAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user_swap_balance_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn log_user_swap_balances_start_verify_signer_privileges<'me, 'info>(
    accounts: LogUserSwapBalancesStartAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base_accounts_maker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn log_user_swap_balances_start_verify_account_privileges<'me, 'info>(
    accounts: LogUserSwapBalancesStartAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    log_user_swap_balances_start_verify_writable_privileges(accounts)?;
    log_user_swap_balances_start_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LOG_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct LogUserSwapBalancesEndAccounts<'me, 'info> {
    pub base_accounts_maker: &'me AccountInfo<'info>,
    pub base_accounts_input_mint: &'me AccountInfo<'info>,
    pub base_accounts_output_mint: &'me AccountInfo<'info>,
    pub base_accounts_input_ta: &'me AccountInfo<'info>,
    pub base_accounts_output_ta: &'me AccountInfo<'info>,
    pub base_accounts_pda_referrer: &'me AccountInfo<'info>,
    pub base_accounts_swap_program_id: &'me AccountInfo<'info>,
    pub user_swap_balance_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub sysvar_instructions: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LogUserSwapBalancesEndKeys {
    pub base_accounts_maker: Pubkey,
    pub base_accounts_input_mint: Pubkey,
    pub base_accounts_output_mint: Pubkey,
    pub base_accounts_input_ta: Pubkey,
    pub base_accounts_output_ta: Pubkey,
    pub base_accounts_pda_referrer: Pubkey,
    pub base_accounts_swap_program_id: Pubkey,
    pub user_swap_balance_state: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub sysvar_instructions: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<LogUserSwapBalancesEndAccounts<'_, '_>> for LogUserSwapBalancesEndKeys {
    fn from(accounts: LogUserSwapBalancesEndAccounts) -> Self {
        Self {
            base_accounts_maker: *accounts.base_accounts_maker.key,
            base_accounts_input_mint: *accounts.base_accounts_input_mint.key,
            base_accounts_output_mint: *accounts.base_accounts_output_mint.key,
            base_accounts_input_ta: *accounts.base_accounts_input_ta.key,
            base_accounts_output_ta: *accounts.base_accounts_output_ta.key,
            base_accounts_pda_referrer: *accounts.base_accounts_pda_referrer.key,
            base_accounts_swap_program_id: *accounts.base_accounts_swap_program_id.key,
            user_swap_balance_state: *accounts.user_swap_balance_state.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            sysvar_instructions: *accounts.sysvar_instructions.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<LogUserSwapBalancesEndKeys>
for [AccountMeta; LOG_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN] {
    fn from(keys: LogUserSwapBalancesEndKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.base_accounts_maker,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_input_ta,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_output_ta,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_pda_referrer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_swap_program_id,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_swap_balance_state,
                is_signer: false,
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
                pubkey: keys.sysvar_instructions,
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
impl From<[Pubkey; LOG_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN]>
for LogUserSwapBalancesEndKeys {
    fn from(pubkeys: [Pubkey; LOG_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base_accounts_maker: pubkeys[0],
            base_accounts_input_mint: pubkeys[1],
            base_accounts_output_mint: pubkeys[2],
            base_accounts_input_ta: pubkeys[3],
            base_accounts_output_ta: pubkeys[4],
            base_accounts_pda_referrer: pubkeys[5],
            base_accounts_swap_program_id: pubkeys[6],
            user_swap_balance_state: pubkeys[7],
            system_program: pubkeys[8],
            rent: pubkeys[9],
            sysvar_instructions: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<LogUserSwapBalancesEndAccounts<'_, 'info>>
for [AccountInfo<'info>; LOG_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN] {
    fn from(accounts: LogUserSwapBalancesEndAccounts<'_, 'info>) -> Self {
        [
            accounts.base_accounts_maker.clone(),
            accounts.base_accounts_input_mint.clone(),
            accounts.base_accounts_output_mint.clone(),
            accounts.base_accounts_input_ta.clone(),
            accounts.base_accounts_output_ta.clone(),
            accounts.base_accounts_pda_referrer.clone(),
            accounts.base_accounts_swap_program_id.clone(),
            accounts.user_swap_balance_state.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.sysvar_instructions.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LOG_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN]>
for LogUserSwapBalancesEndAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LOG_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            base_accounts_maker: &arr[0],
            base_accounts_input_mint: &arr[1],
            base_accounts_output_mint: &arr[2],
            base_accounts_input_ta: &arr[3],
            base_accounts_output_ta: &arr[4],
            base_accounts_pda_referrer: &arr[5],
            base_accounts_swap_program_id: &arr[6],
            user_swap_balance_state: &arr[7],
            system_program: &arr[8],
            rent: &arr[9],
            sysvar_instructions: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const LOG_USER_SWAP_BALANCES_END_IX_DISCM: [u8; 8usize] = [
    140, 42, 198, 82, 147, 144, 44, 113,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUserSwapBalancesEndIxArgs {
    pub simulated_swap_amount_out: u64,
    pub simulated_ts: u64,
    pub minimum_amount_out: u64,
    pub swap_amount_in: u64,
    pub simulated_amount_out_next_best: u64,
    pub aggregator: u8,
    pub next_best_aggregator: u8,
    pub padding: [u8; 2],
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUserSwapBalancesEndIxData(pub LogUserSwapBalancesEndIxArgs);
impl From<LogUserSwapBalancesEndIxArgs> for LogUserSwapBalancesEndIxData {
    fn from(args: LogUserSwapBalancesEndIxArgs) -> Self {
        Self(args)
    }
}
impl LogUserSwapBalancesEndIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_USER_SWAP_BALANCES_END_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let simulated_swap_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let simulated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let simulated_amount_out_next_best: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let aggregator: u8 = crate::borsh_de_or_default(&mut reader)?;
        let next_best_aggregator: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LogUserSwapBalancesEndIxArgs {
                simulated_swap_amount_out,
                simulated_ts,
                minimum_amount_out,
                swap_amount_in,
                simulated_amount_out_next_best,
                aggregator,
                next_best_aggregator,
                padding,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_USER_SWAP_BALANCES_END_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.simulated_swap_amount_out,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.simulated_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.swap_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.simulated_amount_out_next_best,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.aggregator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.next_best_aggregator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.padding, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn log_user_swap_balances_end_ix_with_program_id(
    program_id: Pubkey,
    keys: LogUserSwapBalancesEndKeys,
    args: LogUserSwapBalancesEndIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LOG_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN] = keys.into();
    let data: LogUserSwapBalancesEndIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn log_user_swap_balances_end_ix(
    keys: LogUserSwapBalancesEndKeys,
    args: LogUserSwapBalancesEndIxArgs,
) -> std::io::Result<Instruction> {
    log_user_swap_balances_end_ix_with_program_id(LIMO_PROGRAM_ID, keys, args)
}
pub fn log_user_swap_balances_end_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LogUserSwapBalancesEndAccounts<'_, '_>,
    args: LogUserSwapBalancesEndIxArgs,
) -> ProgramResult {
    let keys: LogUserSwapBalancesEndKeys = accounts.into();
    let ix = log_user_swap_balances_end_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn log_user_swap_balances_end_invoke(
    accounts: LogUserSwapBalancesEndAccounts<'_, '_>,
    args: LogUserSwapBalancesEndIxArgs,
) -> ProgramResult {
    log_user_swap_balances_end_invoke_with_program_id(LIMO_PROGRAM_ID, accounts, args)
}
pub fn log_user_swap_balances_end_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LogUserSwapBalancesEndAccounts<'_, '_>,
    args: LogUserSwapBalancesEndIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LogUserSwapBalancesEndKeys = accounts.into();
    let ix = log_user_swap_balances_end_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn log_user_swap_balances_end_invoke_signed(
    accounts: LogUserSwapBalancesEndAccounts<'_, '_>,
    args: LogUserSwapBalancesEndIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    log_user_swap_balances_end_invoke_signed_with_program_id(
        LIMO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn log_user_swap_balances_end_verify_account_keys(
    accounts: LogUserSwapBalancesEndAccounts<'_, '_>,
    keys: LogUserSwapBalancesEndKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.base_accounts_maker.key, keys.base_accounts_maker),
        (*accounts.base_accounts_input_mint.key, keys.base_accounts_input_mint),
        (*accounts.base_accounts_output_mint.key, keys.base_accounts_output_mint),
        (*accounts.base_accounts_input_ta.key, keys.base_accounts_input_ta),
        (*accounts.base_accounts_output_ta.key, keys.base_accounts_output_ta),
        (*accounts.base_accounts_pda_referrer.key, keys.base_accounts_pda_referrer),
        (
            *accounts.base_accounts_swap_program_id.key,
            keys.base_accounts_swap_program_id,
        ),
        (*accounts.user_swap_balance_state.key, keys.user_swap_balance_state),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.sysvar_instructions.key, keys.sysvar_instructions),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn log_user_swap_balances_end_verify_writable_privileges<'me, 'info>(
    accounts: LogUserSwapBalancesEndAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user_swap_balance_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn log_user_swap_balances_end_verify_signer_privileges<'me, 'info>(
    accounts: LogUserSwapBalancesEndAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base_accounts_maker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn log_user_swap_balances_end_verify_account_privileges<'me, 'info>(
    accounts: LogUserSwapBalancesEndAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    log_user_swap_balances_end_verify_writable_privileges(accounts)?;
    log_user_swap_balances_end_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ASSERT_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct AssertUserSwapBalancesStartAccounts<'me, 'info> {
    pub maker: &'me AccountInfo<'info>,
    pub input_ta: &'me AccountInfo<'info>,
    pub output_ta: &'me AccountInfo<'info>,
    pub user_swap_balance_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub sysvar_instructions: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AssertUserSwapBalancesStartKeys {
    pub maker: Pubkey,
    pub input_ta: Pubkey,
    pub output_ta: Pubkey,
    pub user_swap_balance_state: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub sysvar_instructions: Pubkey,
}
impl From<AssertUserSwapBalancesStartAccounts<'_, '_>>
for AssertUserSwapBalancesStartKeys {
    fn from(accounts: AssertUserSwapBalancesStartAccounts) -> Self {
        Self {
            maker: *accounts.maker.key,
            input_ta: *accounts.input_ta.key,
            output_ta: *accounts.output_ta.key,
            user_swap_balance_state: *accounts.user_swap_balance_state.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            sysvar_instructions: *accounts.sysvar_instructions.key,
        }
    }
}
impl From<AssertUserSwapBalancesStartKeys>
for [AccountMeta; ASSERT_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN] {
    fn from(keys: AssertUserSwapBalancesStartKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.maker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_ta,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_ta,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_swap_balance_state,
                is_signer: false,
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
                pubkey: keys.sysvar_instructions,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ASSERT_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN]>
for AssertUserSwapBalancesStartKeys {
    fn from(pubkeys: [Pubkey; ASSERT_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            maker: pubkeys[0],
            input_ta: pubkeys[1],
            output_ta: pubkeys[2],
            user_swap_balance_state: pubkeys[3],
            system_program: pubkeys[4],
            rent: pubkeys[5],
            sysvar_instructions: pubkeys[6],
        }
    }
}
impl<'info> From<AssertUserSwapBalancesStartAccounts<'_, 'info>>
for [AccountInfo<'info>; ASSERT_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN] {
    fn from(accounts: AssertUserSwapBalancesStartAccounts<'_, 'info>) -> Self {
        [
            accounts.maker.clone(),
            accounts.input_ta.clone(),
            accounts.output_ta.clone(),
            accounts.user_swap_balance_state.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.sysvar_instructions.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ASSERT_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN]>
for AssertUserSwapBalancesStartAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ASSERT_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            maker: &arr[0],
            input_ta: &arr[1],
            output_ta: &arr[2],
            user_swap_balance_state: &arr[3],
            system_program: &arr[4],
            rent: &arr[5],
            sysvar_instructions: &arr[6],
        }
    }
}
pub const ASSERT_USER_SWAP_BALANCES_START_IX_DISCM: [u8; 8usize] = [
    95, 241, 226, 193, 214, 175, 142, 139,
];
#[derive(Clone, Debug, PartialEq)]
pub struct AssertUserSwapBalancesStartIxData;
impl AssertUserSwapBalancesStartIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ASSERT_USER_SWAP_BALANCES_START_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ASSERT_USER_SWAP_BALANCES_START_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn assert_user_swap_balances_start_ix_with_program_id(
    program_id: Pubkey,
    keys: AssertUserSwapBalancesStartKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ASSERT_USER_SWAP_BALANCES_START_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AssertUserSwapBalancesStartIxData.try_to_vec()?,
    })
}
pub fn assert_user_swap_balances_start_ix(
    keys: AssertUserSwapBalancesStartKeys,
) -> std::io::Result<Instruction> {
    assert_user_swap_balances_start_ix_with_program_id(LIMO_PROGRAM_ID, keys)
}
pub fn assert_user_swap_balances_start_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AssertUserSwapBalancesStartAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AssertUserSwapBalancesStartKeys = accounts.into();
    let ix = assert_user_swap_balances_start_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn assert_user_swap_balances_start_invoke(
    accounts: AssertUserSwapBalancesStartAccounts<'_, '_>,
) -> ProgramResult {
    assert_user_swap_balances_start_invoke_with_program_id(LIMO_PROGRAM_ID, accounts)
}
pub fn assert_user_swap_balances_start_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AssertUserSwapBalancesStartAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AssertUserSwapBalancesStartKeys = accounts.into();
    let ix = assert_user_swap_balances_start_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn assert_user_swap_balances_start_invoke_signed(
    accounts: AssertUserSwapBalancesStartAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    assert_user_swap_balances_start_invoke_signed_with_program_id(
        LIMO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn assert_user_swap_balances_start_verify_account_keys(
    accounts: AssertUserSwapBalancesStartAccounts<'_, '_>,
    keys: AssertUserSwapBalancesStartKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.maker.key, keys.maker),
        (*accounts.input_ta.key, keys.input_ta),
        (*accounts.output_ta.key, keys.output_ta),
        (*accounts.user_swap_balance_state.key, keys.user_swap_balance_state),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.sysvar_instructions.key, keys.sysvar_instructions),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn assert_user_swap_balances_start_verify_writable_privileges<'me, 'info>(
    accounts: AssertUserSwapBalancesStartAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.maker, accounts.user_swap_balance_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn assert_user_swap_balances_start_verify_signer_privileges<'me, 'info>(
    accounts: AssertUserSwapBalancesStartAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.maker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn assert_user_swap_balances_start_verify_account_privileges<'me, 'info>(
    accounts: AssertUserSwapBalancesStartAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    assert_user_swap_balances_start_verify_writable_privileges(accounts)?;
    assert_user_swap_balances_start_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ASSERT_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct AssertUserSwapBalancesEndAccounts<'me, 'info> {
    pub maker: &'me AccountInfo<'info>,
    pub input_ta: &'me AccountInfo<'info>,
    pub output_ta: &'me AccountInfo<'info>,
    pub user_swap_balance_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub sysvar_instructions: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AssertUserSwapBalancesEndKeys {
    pub maker: Pubkey,
    pub input_ta: Pubkey,
    pub output_ta: Pubkey,
    pub user_swap_balance_state: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub sysvar_instructions: Pubkey,
}
impl From<AssertUserSwapBalancesEndAccounts<'_, '_>> for AssertUserSwapBalancesEndKeys {
    fn from(accounts: AssertUserSwapBalancesEndAccounts) -> Self {
        Self {
            maker: *accounts.maker.key,
            input_ta: *accounts.input_ta.key,
            output_ta: *accounts.output_ta.key,
            user_swap_balance_state: *accounts.user_swap_balance_state.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            sysvar_instructions: *accounts.sysvar_instructions.key,
        }
    }
}
impl From<AssertUserSwapBalancesEndKeys>
for [AccountMeta; ASSERT_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN] {
    fn from(keys: AssertUserSwapBalancesEndKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.maker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_ta,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_ta,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_swap_balance_state,
                is_signer: false,
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
                pubkey: keys.sysvar_instructions,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ASSERT_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN]>
for AssertUserSwapBalancesEndKeys {
    fn from(pubkeys: [Pubkey; ASSERT_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            maker: pubkeys[0],
            input_ta: pubkeys[1],
            output_ta: pubkeys[2],
            user_swap_balance_state: pubkeys[3],
            system_program: pubkeys[4],
            rent: pubkeys[5],
            sysvar_instructions: pubkeys[6],
        }
    }
}
impl<'info> From<AssertUserSwapBalancesEndAccounts<'_, 'info>>
for [AccountInfo<'info>; ASSERT_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN] {
    fn from(accounts: AssertUserSwapBalancesEndAccounts<'_, 'info>) -> Self {
        [
            accounts.maker.clone(),
            accounts.input_ta.clone(),
            accounts.output_ta.clone(),
            accounts.user_swap_balance_state.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.sysvar_instructions.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ASSERT_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN]>
for AssertUserSwapBalancesEndAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ASSERT_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            maker: &arr[0],
            input_ta: &arr[1],
            output_ta: &arr[2],
            user_swap_balance_state: &arr[3],
            system_program: &arr[4],
            rent: &arr[5],
            sysvar_instructions: &arr[6],
        }
    }
}
pub const ASSERT_USER_SWAP_BALANCES_END_IX_DISCM: [u8; 8usize] = [
    163, 157, 174, 93, 28, 127, 250, 136,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AssertUserSwapBalancesEndIxArgs {
    pub max_input_amount_change: u64,
    pub min_output_amount_change: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AssertUserSwapBalancesEndIxData(pub AssertUserSwapBalancesEndIxArgs);
impl From<AssertUserSwapBalancesEndIxArgs> for AssertUserSwapBalancesEndIxData {
    fn from(args: AssertUserSwapBalancesEndIxArgs) -> Self {
        Self(args)
    }
}
impl AssertUserSwapBalancesEndIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ASSERT_USER_SWAP_BALANCES_END_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_input_amount_change: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_output_amount_change: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AssertUserSwapBalancesEndIxArgs {
                max_input_amount_change,
                min_output_amount_change,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ASSERT_USER_SWAP_BALANCES_END_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_input_amount_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_output_amount_change, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn assert_user_swap_balances_end_ix_with_program_id(
    program_id: Pubkey,
    keys: AssertUserSwapBalancesEndKeys,
    args: AssertUserSwapBalancesEndIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ASSERT_USER_SWAP_BALANCES_END_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: AssertUserSwapBalancesEndIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn assert_user_swap_balances_end_ix(
    keys: AssertUserSwapBalancesEndKeys,
    args: AssertUserSwapBalancesEndIxArgs,
) -> std::io::Result<Instruction> {
    assert_user_swap_balances_end_ix_with_program_id(LIMO_PROGRAM_ID, keys, args)
}
pub fn assert_user_swap_balances_end_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AssertUserSwapBalancesEndAccounts<'_, '_>,
    args: AssertUserSwapBalancesEndIxArgs,
) -> ProgramResult {
    let keys: AssertUserSwapBalancesEndKeys = accounts.into();
    let ix = assert_user_swap_balances_end_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn assert_user_swap_balances_end_invoke(
    accounts: AssertUserSwapBalancesEndAccounts<'_, '_>,
    args: AssertUserSwapBalancesEndIxArgs,
) -> ProgramResult {
    assert_user_swap_balances_end_invoke_with_program_id(LIMO_PROGRAM_ID, accounts, args)
}
pub fn assert_user_swap_balances_end_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AssertUserSwapBalancesEndAccounts<'_, '_>,
    args: AssertUserSwapBalancesEndIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AssertUserSwapBalancesEndKeys = accounts.into();
    let ix = assert_user_swap_balances_end_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn assert_user_swap_balances_end_invoke_signed(
    accounts: AssertUserSwapBalancesEndAccounts<'_, '_>,
    args: AssertUserSwapBalancesEndIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    assert_user_swap_balances_end_invoke_signed_with_program_id(
        LIMO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn assert_user_swap_balances_end_verify_account_keys(
    accounts: AssertUserSwapBalancesEndAccounts<'_, '_>,
    keys: AssertUserSwapBalancesEndKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.maker.key, keys.maker),
        (*accounts.input_ta.key, keys.input_ta),
        (*accounts.output_ta.key, keys.output_ta),
        (*accounts.user_swap_balance_state.key, keys.user_swap_balance_state),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.sysvar_instructions.key, keys.sysvar_instructions),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn assert_user_swap_balances_end_verify_writable_privileges<'me, 'info>(
    accounts: AssertUserSwapBalancesEndAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.maker, accounts.user_swap_balance_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn assert_user_swap_balances_end_verify_signer_privileges<'me, 'info>(
    accounts: AssertUserSwapBalancesEndAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.maker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn assert_user_swap_balances_end_verify_account_privileges<'me, 'info>(
    accounts: AssertUserSwapBalancesEndAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    assert_user_swap_balances_end_verify_writable_privileges(accounts)?;
    assert_user_swap_balances_end_verify_signer_privileges(accounts)?;
    Ok(())
}
