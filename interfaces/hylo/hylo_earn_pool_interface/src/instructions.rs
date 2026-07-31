use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum HyloEarnPoolProgramIx {
    AbsorbLoss(AbsorbLossIxArgs),
    DeprecateLevercoinPool,
    InitializeEarnPool,
    InitializeLpTokenMint(InitializeLpTokenMintIxArgs),
    PauseEarnPool,
    UnpauseEarnPool,
    UpdateDepositLimit(UpdateDepositLimitIxArgs),
    UpdateWithdrawalFee(UpdateWithdrawalFeeIxArgs),
    UpdateWithdrawalLimit(UpdateWithdrawalLimitIxArgs),
    UserDeposit(UserDepositIxArgs),
    UserWithdraw(UserWithdrawIxArgs),
}
impl HyloEarnPoolProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ABSORB_LOSS_IX_DISCM) {
            let mut reader = &buf[ABSORB_LOSS_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::AbsorbLoss(AbsorbLossIxArgs { amount }));
        }
        if buf.starts_with(&DEPRECATE_LEVERCOIN_POOL_IX_DISCM) {
            return Ok(Self::DeprecateLevercoinPool);
        }
        if buf.starts_with(&INITIALIZE_EARN_POOL_IX_DISCM) {
            return Ok(Self::InitializeEarnPool);
        }
        if buf.starts_with(&INITIALIZE_LP_TOKEN_MINT_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_LP_TOKEN_MINT_IX_DISCM.len()..];
            let lp_token_metadata = if reader.is_empty() {
                Default::default()
            } else {
                <TokenMetadata>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeLpTokenMint(InitializeLpTokenMintIxArgs {
                    lp_token_metadata,
                }),
            );
        }
        if buf.starts_with(&PAUSE_EARN_POOL_IX_DISCM) {
            return Ok(Self::PauseEarnPool);
        }
        if buf.starts_with(&UNPAUSE_EARN_POOL_IX_DISCM) {
            return Ok(Self::UnpauseEarnPool);
        }
        if buf.starts_with(&UPDATE_DEPOSIT_LIMIT_IX_DISCM) {
            let mut reader = &buf[UPDATE_DEPOSIT_LIMIT_IX_DISCM.len()..];
            let new_deposit_limit = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateDepositLimit(UpdateDepositLimitIxArgs {
                    new_deposit_limit,
                }),
            );
        }
        if buf.starts_with(&UPDATE_WITHDRAWAL_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_WITHDRAWAL_FEE_IX_DISCM.len()..];
            let new_withdrawal_fee = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateWithdrawalFee(UpdateWithdrawalFeeIxArgs {
                    new_withdrawal_fee,
                }),
            );
        }
        if buf.starts_with(&UPDATE_WITHDRAWAL_LIMIT_IX_DISCM) {
            let mut reader = &buf[UPDATE_WITHDRAWAL_LIMIT_IX_DISCM.len()..];
            let new_withdrawal_limit = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateWithdrawalLimit(UpdateWithdrawalLimitIxArgs {
                    new_withdrawal_limit,
                }),
            );
        }
        if buf.starts_with(&USER_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[USER_DEPOSIT_IX_DISCM.len()..];
            let amount_stablecoin: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UserDeposit(UserDepositIxArgs {
                    amount_stablecoin,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&USER_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[USER_WITHDRAW_IX_DISCM.len()..];
            let amount_lp_token: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UserWithdraw(UserWithdrawIxArgs {
                    amount_lp_token,
                    slippage_config,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AbsorbLoss(args) => {
                writer.write_all(&ABSORB_LOSS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::DeprecateLevercoinPool => {
                writer.write_all(&DEPRECATE_LEVERCOIN_POOL_IX_DISCM)
            }
            Self::InitializeEarnPool => writer.write_all(&INITIALIZE_EARN_POOL_IX_DISCM),
            Self::InitializeLpTokenMint(args) => {
                writer.write_all(&INITIALIZE_LP_TOKEN_MINT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lp_token_metadata, &mut writer)?;
                Ok(())
            }
            Self::PauseEarnPool => writer.write_all(&PAUSE_EARN_POOL_IX_DISCM),
            Self::UnpauseEarnPool => writer.write_all(&UNPAUSE_EARN_POOL_IX_DISCM),
            Self::UpdateDepositLimit(args) => {
                writer.write_all(&UPDATE_DEPOSIT_LIMIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_deposit_limit, &mut writer)?;
                Ok(())
            }
            Self::UpdateWithdrawalFee(args) => {
                writer.write_all(&UPDATE_WITHDRAWAL_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_withdrawal_fee, &mut writer)?;
                Ok(())
            }
            Self::UpdateWithdrawalLimit(args) => {
                writer.write_all(&UPDATE_WITHDRAWAL_LIMIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_withdrawal_limit,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UserDeposit(args) => {
                writer.write_all(&USER_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_stablecoin, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::UserWithdraw(args) => {
                writer.write_all(&USER_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_lp_token, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
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
pub const ABSORB_LOSS_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct AbsorbLossAccounts<'me, 'info> {
    pub settlement_auth: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AbsorbLossKeys {
    pub settlement_auth: Pubkey,
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub pool_auth: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub token_program: Pubkey,
}
impl From<AbsorbLossAccounts<'_, '_>> for AbsorbLossKeys {
    fn from(accounts: AbsorbLossAccounts) -> Self {
        Self {
            settlement_auth: *accounts.settlement_auth.key,
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            pool_auth: *accounts.pool_auth.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<AbsorbLossKeys> for [AccountMeta; ABSORB_LOSS_IX_ACCOUNTS_LEN] {
    fn from(keys: AbsorbLossKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.settlement_auth,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ABSORB_LOSS_IX_ACCOUNTS_LEN]> for AbsorbLossKeys {
    fn from(pubkeys: [Pubkey; ABSORB_LOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            settlement_auth: pubkeys[0],
            hylo: pubkeys[1],
            pool_config: pubkeys[2],
            pool_auth: pubkeys[3],
            stablecoin_pool: pubkeys[4],
            stablecoin_mint: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<AbsorbLossAccounts<'_, 'info>>
for [AccountInfo<'info>; ABSORB_LOSS_IX_ACCOUNTS_LEN] {
    fn from(accounts: AbsorbLossAccounts<'_, 'info>) -> Self {
        [
            accounts.settlement_auth.clone(),
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.pool_auth.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ABSORB_LOSS_IX_ACCOUNTS_LEN]>
for AbsorbLossAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ABSORB_LOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            settlement_auth: &arr[0],
            hylo: &arr[1],
            pool_config: &arr[2],
            pool_auth: &arr[3],
            stablecoin_pool: &arr[4],
            stablecoin_mint: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const ABSORB_LOSS_IX_DISCM: [u8; 8usize] = [42, 46, 116, 115, 100, 254, 176, 95];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AbsorbLossIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AbsorbLossIxData(pub AbsorbLossIxArgs);
impl From<AbsorbLossIxArgs> for AbsorbLossIxData {
    fn from(args: AbsorbLossIxArgs) -> Self {
        Self(args)
    }
}
impl AbsorbLossIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ABSORB_LOSS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(AbsorbLossIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ABSORB_LOSS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn absorb_loss_ix_with_program_id(
    program_id: Pubkey,
    keys: AbsorbLossKeys,
    args: AbsorbLossIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ABSORB_LOSS_IX_ACCOUNTS_LEN] = keys.into();
    let data: AbsorbLossIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn absorb_loss_ix(
    keys: AbsorbLossKeys,
    args: AbsorbLossIxArgs,
) -> std::io::Result<Instruction> {
    absorb_loss_ix_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, keys, args)
}
pub fn absorb_loss_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AbsorbLossAccounts<'_, '_>,
    args: AbsorbLossIxArgs,
) -> ProgramResult {
    let keys: AbsorbLossKeys = accounts.into();
    let ix = absorb_loss_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn absorb_loss_invoke(
    accounts: AbsorbLossAccounts<'_, '_>,
    args: AbsorbLossIxArgs,
) -> ProgramResult {
    absorb_loss_invoke_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, accounts, args)
}
pub fn absorb_loss_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AbsorbLossAccounts<'_, '_>,
    args: AbsorbLossIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AbsorbLossKeys = accounts.into();
    let ix = absorb_loss_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn absorb_loss_invoke_signed(
    accounts: AbsorbLossAccounts<'_, '_>,
    args: AbsorbLossIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    absorb_loss_invoke_signed_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn absorb_loss_verify_account_keys(
    accounts: AbsorbLossAccounts<'_, '_>,
    keys: AbsorbLossKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.settlement_auth.key, keys.settlement_auth),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn absorb_loss_verify_writable_privileges<'me, 'info>(
    accounts: AbsorbLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stablecoin_pool, accounts.stablecoin_mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn absorb_loss_verify_signer_privileges<'me, 'info>(
    accounts: AbsorbLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.settlement_auth] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn absorb_loss_verify_account_privileges<'me, 'info>(
    accounts: AbsorbLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    absorb_loss_verify_writable_privileges(accounts)?;
    absorb_loss_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPRECATE_LEVERCOIN_POOL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct DeprecateLevercoinPoolAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub levercoin_pool: &'me AccountInfo<'info>,
    pub admin_levercoin_ta: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeprecateLevercoinPoolKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub pool_auth: Pubkey,
    pub levercoin_pool: Pubkey,
    pub admin_levercoin_ta: Pubkey,
    pub levercoin_mint: Pubkey,
    pub token_program: Pubkey,
}
impl From<DeprecateLevercoinPoolAccounts<'_, '_>> for DeprecateLevercoinPoolKeys {
    fn from(accounts: DeprecateLevercoinPoolAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            pool_auth: *accounts.pool_auth.key,
            levercoin_pool: *accounts.levercoin_pool.key,
            admin_levercoin_ta: *accounts.admin_levercoin_ta.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DeprecateLevercoinPoolKeys>
for [AccountMeta; DEPRECATE_LEVERCOIN_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: DeprecateLevercoinPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.levercoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_levercoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
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
impl From<[Pubkey; DEPRECATE_LEVERCOIN_POOL_IX_ACCOUNTS_LEN]>
for DeprecateLevercoinPoolKeys {
    fn from(pubkeys: [Pubkey; DEPRECATE_LEVERCOIN_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            pool_config: pubkeys[2],
            pool_auth: pubkeys[3],
            levercoin_pool: pubkeys[4],
            admin_levercoin_ta: pubkeys[5],
            levercoin_mint: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<DeprecateLevercoinPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPRECATE_LEVERCOIN_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeprecateLevercoinPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.pool_auth.clone(),
            accounts.levercoin_pool.clone(),
            accounts.admin_levercoin_ta.clone(),
            accounts.levercoin_mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DEPRECATE_LEVERCOIN_POOL_IX_ACCOUNTS_LEN]>
for DeprecateLevercoinPoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPRECATE_LEVERCOIN_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            pool_config: &arr[2],
            pool_auth: &arr[3],
            levercoin_pool: &arr[4],
            admin_levercoin_ta: &arr[5],
            levercoin_mint: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const DEPRECATE_LEVERCOIN_POOL_IX_DISCM: [u8; 8usize] = [
    136, 100, 65, 143, 151, 37, 243, 56,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DeprecateLevercoinPoolIxData;
impl DeprecateLevercoinPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPRECATE_LEVERCOIN_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPRECATE_LEVERCOIN_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deprecate_levercoin_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: DeprecateLevercoinPoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPRECATE_LEVERCOIN_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DeprecateLevercoinPoolIxData.try_to_vec()?,
    })
}
pub fn deprecate_levercoin_pool_ix(
    keys: DeprecateLevercoinPoolKeys,
) -> std::io::Result<Instruction> {
    deprecate_levercoin_pool_ix_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, keys)
}
pub fn deprecate_levercoin_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeprecateLevercoinPoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DeprecateLevercoinPoolKeys = accounts.into();
    let ix = deprecate_levercoin_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn deprecate_levercoin_pool_invoke(
    accounts: DeprecateLevercoinPoolAccounts<'_, '_>,
) -> ProgramResult {
    deprecate_levercoin_pool_invoke_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, accounts)
}
pub fn deprecate_levercoin_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeprecateLevercoinPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeprecateLevercoinPoolKeys = accounts.into();
    let ix = deprecate_levercoin_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deprecate_levercoin_pool_invoke_signed(
    accounts: DeprecateLevercoinPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deprecate_levercoin_pool_invoke_signed_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn deprecate_levercoin_pool_verify_account_keys(
    accounts: DeprecateLevercoinPoolAccounts<'_, '_>,
    keys: DeprecateLevercoinPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.levercoin_pool.key, keys.levercoin_pool),
        (*accounts.admin_levercoin_ta.key, keys.admin_levercoin_ta),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deprecate_levercoin_pool_verify_writable_privileges<'me, 'info>(
    accounts: DeprecateLevercoinPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.levercoin_pool,
        accounts.admin_levercoin_ta,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deprecate_levercoin_pool_verify_signer_privileges<'me, 'info>(
    accounts: DeprecateLevercoinPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deprecate_levercoin_pool_verify_account_privileges<'me, 'info>(
    accounts: DeprecateLevercoinPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deprecate_levercoin_pool_verify_writable_privileges(accounts)?;
    deprecate_levercoin_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_EARN_POOL_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct InitializeEarnPoolAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub upgrade_authority: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program_data: &'me AccountInfo<'info>,
    pub hylo_earn_pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeEarnPoolKeys {
    pub admin: Pubkey,
    pub upgrade_authority: Pubkey,
    pub pool_config: Pubkey,
    pub hylo: Pubkey,
    pub pool_auth: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub program_data: Pubkey,
    pub hylo_earn_pool: Pubkey,
}
impl From<InitializeEarnPoolAccounts<'_, '_>> for InitializeEarnPoolKeys {
    fn from(accounts: InitializeEarnPoolAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            upgrade_authority: *accounts.upgrade_authority.key,
            pool_config: *accounts.pool_config.key,
            hylo: *accounts.hylo.key,
            pool_auth: *accounts.pool_auth.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            program_data: *accounts.program_data.key,
            hylo_earn_pool: *accounts.hylo_earn_pool.key,
        }
    }
}
impl From<InitializeEarnPoolKeys>
for [AccountMeta; INITIALIZE_EARN_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeEarnPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.upgrade_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
                pubkey: keys.program_data,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo_earn_pool,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_EARN_POOL_IX_ACCOUNTS_LEN]> for InitializeEarnPoolKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_EARN_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            upgrade_authority: pubkeys[1],
            pool_config: pubkeys[2],
            hylo: pubkeys[3],
            pool_auth: pubkeys[4],
            stablecoin_pool: pubkeys[5],
            stablecoin_mint: pubkeys[6],
            associated_token_program: pubkeys[7],
            token_program: pubkeys[8],
            system_program: pubkeys[9],
            program_data: pubkeys[10],
            hylo_earn_pool: pubkeys[11],
        }
    }
}
impl<'info> From<InitializeEarnPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_EARN_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeEarnPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.upgrade_authority.clone(),
            accounts.pool_config.clone(),
            accounts.hylo.clone(),
            accounts.pool_auth.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.program_data.clone(),
            accounts.hylo_earn_pool.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_EARN_POOL_IX_ACCOUNTS_LEN]>
for InitializeEarnPoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_EARN_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            upgrade_authority: &arr[1],
            pool_config: &arr[2],
            hylo: &arr[3],
            pool_auth: &arr[4],
            stablecoin_pool: &arr[5],
            stablecoin_mint: &arr[6],
            associated_token_program: &arr[7],
            token_program: &arr[8],
            system_program: &arr[9],
            program_data: &arr[10],
            hylo_earn_pool: &arr[11],
        }
    }
}
pub const INITIALIZE_EARN_POOL_IX_DISCM: [u8; 8usize] = [
    238, 212, 54, 107, 73, 253, 213, 167,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeEarnPoolIxData;
impl InitializeEarnPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_EARN_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_EARN_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_earn_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeEarnPoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_EARN_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeEarnPoolIxData.try_to_vec()?,
    })
}
pub fn initialize_earn_pool_ix(
    keys: InitializeEarnPoolKeys,
) -> std::io::Result<Instruction> {
    initialize_earn_pool_ix_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, keys)
}
pub fn initialize_earn_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeEarnPoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeEarnPoolKeys = accounts.into();
    let ix = initialize_earn_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_earn_pool_invoke(
    accounts: InitializeEarnPoolAccounts<'_, '_>,
) -> ProgramResult {
    initialize_earn_pool_invoke_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, accounts)
}
pub fn initialize_earn_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeEarnPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeEarnPoolKeys = accounts.into();
    let ix = initialize_earn_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_earn_pool_invoke_signed(
    accounts: InitializeEarnPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_earn_pool_invoke_signed_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_earn_pool_verify_account_keys(
    accounts: InitializeEarnPoolAccounts<'_, '_>,
    keys: InitializeEarnPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.upgrade_authority.key, keys.upgrade_authority),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program_data.key, keys.program_data),
        (*accounts.hylo_earn_pool.key, keys.hylo_earn_pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_earn_pool_verify_writable_privileges<'me, 'info>(
    accounts: InitializeEarnPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.pool_config,
        accounts.stablecoin_pool,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_earn_pool_verify_signer_privileges<'me, 'info>(
    accounts: InitializeEarnPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.upgrade_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_earn_pool_verify_account_privileges<'me, 'info>(
    accounts: InitializeEarnPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_earn_pool_verify_writable_privileges(accounts)?;
    initialize_earn_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_LP_TOKEN_MINT_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializeLpTokenMintAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub lp_token_auth: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub lp_token_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeLpTokenMintKeys {
    pub admin: Pubkey,
    pub pool_config: Pubkey,
    pub hylo: Pubkey,
    pub lp_token_auth: Pubkey,
    pub lp_token_mint: Pubkey,
    pub lp_token_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeLpTokenMintAccounts<'_, '_>> for InitializeLpTokenMintKeys {
    fn from(accounts: InitializeLpTokenMintAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            pool_config: *accounts.pool_config.key,
            hylo: *accounts.hylo.key,
            lp_token_auth: *accounts.lp_token_auth.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            lp_token_metadata: *accounts.lp_token_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeLpTokenMintKeys>
for [AccountMeta; INITIALIZE_LP_TOKEN_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeLpTokenMintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
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
        ]
    }
}
impl From<[Pubkey; INITIALIZE_LP_TOKEN_MINT_IX_ACCOUNTS_LEN]>
for InitializeLpTokenMintKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_LP_TOKEN_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            pool_config: pubkeys[1],
            hylo: pubkeys[2],
            lp_token_auth: pubkeys[3],
            lp_token_mint: pubkeys[4],
            lp_token_metadata: pubkeys[5],
            metadata_program: pubkeys[6],
            token_program: pubkeys[7],
            rent: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<InitializeLpTokenMintAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_LP_TOKEN_MINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeLpTokenMintAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.pool_config.clone(),
            accounts.hylo.clone(),
            accounts.lp_token_auth.clone(),
            accounts.lp_token_mint.clone(),
            accounts.lp_token_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_LP_TOKEN_MINT_IX_ACCOUNTS_LEN]>
for InitializeLpTokenMintAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_LP_TOKEN_MINT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            pool_config: &arr[1],
            hylo: &arr[2],
            lp_token_auth: &arr[3],
            lp_token_mint: &arr[4],
            lp_token_metadata: &arr[5],
            metadata_program: &arr[6],
            token_program: &arr[7],
            rent: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const INITIALIZE_LP_TOKEN_MINT_IX_DISCM: [u8; 8usize] = [
    188, 32, 168, 40, 166, 152, 159, 156,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeLpTokenMintIxArgs {
    pub lp_token_metadata: TokenMetadata,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeLpTokenMintIxData(pub InitializeLpTokenMintIxArgs);
impl From<InitializeLpTokenMintIxArgs> for InitializeLpTokenMintIxData {
    fn from(args: InitializeLpTokenMintIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeLpTokenMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_LP_TOKEN_MINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let lp_token_metadata = if reader.is_empty() {
            Default::default()
        } else {
            <TokenMetadata>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeLpTokenMintIxArgs {
                lp_token_metadata,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_LP_TOKEN_MINT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lp_token_metadata, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_lp_token_mint_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeLpTokenMintKeys,
    args: InitializeLpTokenMintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_LP_TOKEN_MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeLpTokenMintIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_lp_token_mint_ix(
    keys: InitializeLpTokenMintKeys,
    args: InitializeLpTokenMintIxArgs,
) -> std::io::Result<Instruction> {
    initialize_lp_token_mint_ix_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, keys, args)
}
pub fn initialize_lp_token_mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLpTokenMintAccounts<'_, '_>,
    args: InitializeLpTokenMintIxArgs,
) -> ProgramResult {
    let keys: InitializeLpTokenMintKeys = accounts.into();
    let ix = initialize_lp_token_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_lp_token_mint_invoke(
    accounts: InitializeLpTokenMintAccounts<'_, '_>,
    args: InitializeLpTokenMintIxArgs,
) -> ProgramResult {
    initialize_lp_token_mint_invoke_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_lp_token_mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLpTokenMintAccounts<'_, '_>,
    args: InitializeLpTokenMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeLpTokenMintKeys = accounts.into();
    let ix = initialize_lp_token_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_lp_token_mint_invoke_signed(
    accounts: InitializeLpTokenMintAccounts<'_, '_>,
    args: InitializeLpTokenMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_lp_token_mint_invoke_signed_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_lp_token_mint_verify_account_keys(
    accounts: InitializeLpTokenMintAccounts<'_, '_>,
    keys: InitializeLpTokenMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.lp_token_auth.key, keys.lp_token_auth),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.lp_token_metadata.key, keys.lp_token_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_lp_token_mint_verify_writable_privileges<'me, 'info>(
    accounts: InitializeLpTokenMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.pool_config,
        accounts.lp_token_mint,
        accounts.lp_token_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_lp_token_mint_verify_signer_privileges<'me, 'info>(
    accounts: InitializeLpTokenMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_lp_token_mint_verify_account_privileges<'me, 'info>(
    accounts: InitializeLpTokenMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_lp_token_mint_verify_writable_privileges(accounts)?;
    initialize_lp_token_mint_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_EARN_POOL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct PauseEarnPoolAccounts<'me, 'info> {
    pub pause_authority: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseEarnPoolKeys {
    pub pause_authority: Pubkey,
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PauseEarnPoolAccounts<'_, '_>> for PauseEarnPoolKeys {
    fn from(accounts: PauseEarnPoolAccounts) -> Self {
        Self {
            pause_authority: *accounts.pause_authority.key,
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PauseEarnPoolKeys> for [AccountMeta; PAUSE_EARN_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseEarnPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pause_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; PAUSE_EARN_POOL_IX_ACCOUNTS_LEN]> for PauseEarnPoolKeys {
    fn from(pubkeys: [Pubkey; PAUSE_EARN_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pause_authority: pubkeys[0],
            hylo: pubkeys[1],
            pool_config: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<PauseEarnPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_EARN_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseEarnPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.pause_authority.clone(),
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_EARN_POOL_IX_ACCOUNTS_LEN]>
for PauseEarnPoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAUSE_EARN_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pause_authority: &arr[0],
            hylo: &arr[1],
            pool_config: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const PAUSE_EARN_POOL_IX_DISCM: [u8; 8usize] = [161, 14, 172, 155, 237, 13, 37, 10];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseEarnPoolIxData;
impl PauseEarnPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_EARN_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_EARN_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_earn_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseEarnPoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_EARN_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseEarnPoolIxData.try_to_vec()?,
    })
}
pub fn pause_earn_pool_ix(keys: PauseEarnPoolKeys) -> std::io::Result<Instruction> {
    pause_earn_pool_ix_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, keys)
}
pub fn pause_earn_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseEarnPoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseEarnPoolKeys = accounts.into();
    let ix = pause_earn_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_earn_pool_invoke(accounts: PauseEarnPoolAccounts<'_, '_>) -> ProgramResult {
    pause_earn_pool_invoke_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, accounts)
}
pub fn pause_earn_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseEarnPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseEarnPoolKeys = accounts.into();
    let ix = pause_earn_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_earn_pool_invoke_signed(
    accounts: PauseEarnPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_earn_pool_invoke_signed_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_earn_pool_verify_account_keys(
    accounts: PauseEarnPoolAccounts<'_, '_>,
    keys: PauseEarnPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pause_authority.key, keys.pause_authority),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_earn_pool_verify_writable_privileges<'me, 'info>(
    accounts: PauseEarnPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_earn_pool_verify_signer_privileges<'me, 'info>(
    accounts: PauseEarnPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pause_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_earn_pool_verify_account_privileges<'me, 'info>(
    accounts: PauseEarnPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_earn_pool_verify_writable_privileges(accounts)?;
    pause_earn_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNPAUSE_EARN_POOL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UnpauseEarnPoolAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnpauseEarnPoolKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UnpauseEarnPoolAccounts<'_, '_>> for UnpauseEarnPoolKeys {
    fn from(accounts: UnpauseEarnPoolAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UnpauseEarnPoolKeys> for [AccountMeta; UNPAUSE_EARN_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: UnpauseEarnPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; UNPAUSE_EARN_POOL_IX_ACCOUNTS_LEN]> for UnpauseEarnPoolKeys {
    fn from(pubkeys: [Pubkey; UNPAUSE_EARN_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            pool_config: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UnpauseEarnPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; UNPAUSE_EARN_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnpauseEarnPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNPAUSE_EARN_POOL_IX_ACCOUNTS_LEN]>
for UnpauseEarnPoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNPAUSE_EARN_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            pool_config: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UNPAUSE_EARN_POOL_IX_DISCM: [u8; 8usize] = [
    114, 112, 75, 90, 222, 231, 255, 199,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UnpauseEarnPoolIxData;
impl UnpauseEarnPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNPAUSE_EARN_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNPAUSE_EARN_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unpause_earn_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: UnpauseEarnPoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNPAUSE_EARN_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnpauseEarnPoolIxData.try_to_vec()?,
    })
}
pub fn unpause_earn_pool_ix(keys: UnpauseEarnPoolKeys) -> std::io::Result<Instruction> {
    unpause_earn_pool_ix_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, keys)
}
pub fn unpause_earn_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseEarnPoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnpauseEarnPoolKeys = accounts.into();
    let ix = unpause_earn_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unpause_earn_pool_invoke(
    accounts: UnpauseEarnPoolAccounts<'_, '_>,
) -> ProgramResult {
    unpause_earn_pool_invoke_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, accounts)
}
pub fn unpause_earn_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseEarnPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnpauseEarnPoolKeys = accounts.into();
    let ix = unpause_earn_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unpause_earn_pool_invoke_signed(
    accounts: UnpauseEarnPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unpause_earn_pool_invoke_signed_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn unpause_earn_pool_verify_account_keys(
    accounts: UnpauseEarnPoolAccounts<'_, '_>,
    keys: UnpauseEarnPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unpause_earn_pool_verify_writable_privileges<'me, 'info>(
    accounts: UnpauseEarnPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unpause_earn_pool_verify_signer_privileges<'me, 'info>(
    accounts: UnpauseEarnPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unpause_earn_pool_verify_account_privileges<'me, 'info>(
    accounts: UnpauseEarnPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unpause_earn_pool_verify_writable_privileges(accounts)?;
    unpause_earn_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_DEPOSIT_LIMIT_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct UpdateDepositLimitAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateDepositLimitKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub pool_auth: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateDepositLimitAccounts<'_, '_>> for UpdateDepositLimitKeys {
    fn from(accounts: UpdateDepositLimitAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            pool_auth: *accounts.pool_auth.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateDepositLimitKeys>
for [AccountMeta; UPDATE_DEPOSIT_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateDepositLimitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
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
impl From<[Pubkey; UPDATE_DEPOSIT_LIMIT_IX_ACCOUNTS_LEN]> for UpdateDepositLimitKeys {
    fn from(pubkeys: [Pubkey; UPDATE_DEPOSIT_LIMIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            pool_config: pubkeys[2],
            pool_auth: pubkeys[3],
            stablecoin_pool: pubkeys[4],
            stablecoin_mint: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<UpdateDepositLimitAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_DEPOSIT_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateDepositLimitAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.pool_auth.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_DEPOSIT_LIMIT_IX_ACCOUNTS_LEN]>
for UpdateDepositLimitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_DEPOSIT_LIMIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            pool_config: &arr[2],
            pool_auth: &arr[3],
            stablecoin_pool: &arr[4],
            stablecoin_mint: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const UPDATE_DEPOSIT_LIMIT_IX_DISCM: [u8; 8usize] = [
    181, 115, 65, 169, 4, 1, 96, 109,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateDepositLimitIxArgs {
    pub new_deposit_limit: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDepositLimitIxData(pub UpdateDepositLimitIxArgs);
impl From<UpdateDepositLimitIxArgs> for UpdateDepositLimitIxData {
    fn from(args: UpdateDepositLimitIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateDepositLimitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_DEPOSIT_LIMIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_deposit_limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateDepositLimitIxArgs {
                new_deposit_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DEPOSIT_LIMIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_deposit_limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_deposit_limit_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateDepositLimitKeys,
    args: UpdateDepositLimitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_DEPOSIT_LIMIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateDepositLimitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_deposit_limit_ix(
    keys: UpdateDepositLimitKeys,
    args: UpdateDepositLimitIxArgs,
) -> std::io::Result<Instruction> {
    update_deposit_limit_ix_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, keys, args)
}
pub fn update_deposit_limit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDepositLimitAccounts<'_, '_>,
    args: UpdateDepositLimitIxArgs,
) -> ProgramResult {
    let keys: UpdateDepositLimitKeys = accounts.into();
    let ix = update_deposit_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_deposit_limit_invoke(
    accounts: UpdateDepositLimitAccounts<'_, '_>,
    args: UpdateDepositLimitIxArgs,
) -> ProgramResult {
    update_deposit_limit_invoke_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_deposit_limit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDepositLimitAccounts<'_, '_>,
    args: UpdateDepositLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateDepositLimitKeys = accounts.into();
    let ix = update_deposit_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_deposit_limit_invoke_signed(
    accounts: UpdateDepositLimitAccounts<'_, '_>,
    args: UpdateDepositLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_deposit_limit_invoke_signed_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_deposit_limit_verify_account_keys(
    accounts: UpdateDepositLimitAccounts<'_, '_>,
    keys: UpdateDepositLimitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_deposit_limit_verify_writable_privileges<'me, 'info>(
    accounts: UpdateDepositLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_deposit_limit_verify_signer_privileges<'me, 'info>(
    accounts: UpdateDepositLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_deposit_limit_verify_account_privileges<'me, 'info>(
    accounts: UpdateDepositLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_deposit_limit_verify_writable_privileges(accounts)?;
    update_deposit_limit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateWithdrawalFeeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateWithdrawalFeeKeys {
    pub admin: Pubkey,
    pub pool_config: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateWithdrawalFeeAccounts<'_, '_>> for UpdateWithdrawalFeeKeys {
    fn from(accounts: UpdateWithdrawalFeeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            pool_config: *accounts.pool_config.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateWithdrawalFeeKeys>
for [AccountMeta; UPDATE_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateWithdrawalFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
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
impl From<[Pubkey; UPDATE_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]> for UpdateWithdrawalFeeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            pool_config: pubkeys[1],
            hylo: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateWithdrawalFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateWithdrawalFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.pool_config.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]>
for UpdateWithdrawalFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            pool_config: &arr[1],
            hylo: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_WITHDRAWAL_FEE_IX_DISCM: [u8; 8usize] = [
    165, 235, 231, 25, 127, 99, 244, 133,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateWithdrawalFeeIxArgs {
    pub new_withdrawal_fee: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateWithdrawalFeeIxData(pub UpdateWithdrawalFeeIxArgs);
impl From<UpdateWithdrawalFeeIxArgs> for UpdateWithdrawalFeeIxData {
    fn from(args: UpdateWithdrawalFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateWithdrawalFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_WITHDRAWAL_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_withdrawal_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateWithdrawalFeeIxArgs {
                new_withdrawal_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_WITHDRAWAL_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_withdrawal_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_withdrawal_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateWithdrawalFeeKeys,
    args: UpdateWithdrawalFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateWithdrawalFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_withdrawal_fee_ix(
    keys: UpdateWithdrawalFeeKeys,
    args: UpdateWithdrawalFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_withdrawal_fee_ix_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, keys, args)
}
pub fn update_withdrawal_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateWithdrawalFeeAccounts<'_, '_>,
    args: UpdateWithdrawalFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateWithdrawalFeeKeys = accounts.into();
    let ix = update_withdrawal_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_withdrawal_fee_invoke(
    accounts: UpdateWithdrawalFeeAccounts<'_, '_>,
    args: UpdateWithdrawalFeeIxArgs,
) -> ProgramResult {
    update_withdrawal_fee_invoke_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_withdrawal_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateWithdrawalFeeAccounts<'_, '_>,
    args: UpdateWithdrawalFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateWithdrawalFeeKeys = accounts.into();
    let ix = update_withdrawal_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_withdrawal_fee_invoke_signed(
    accounts: UpdateWithdrawalFeeAccounts<'_, '_>,
    args: UpdateWithdrawalFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_withdrawal_fee_invoke_signed_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_withdrawal_fee_verify_account_keys(
    accounts: UpdateWithdrawalFeeAccounts<'_, '_>,
    keys: UpdateWithdrawalFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_withdrawal_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateWithdrawalFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_withdrawal_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateWithdrawalFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_withdrawal_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateWithdrawalFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_withdrawal_fee_verify_writable_privileges(accounts)?;
    update_withdrawal_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct UpdateWithdrawalLimitAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateWithdrawalLimitKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub pool_auth: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateWithdrawalLimitAccounts<'_, '_>> for UpdateWithdrawalLimitKeys {
    fn from(accounts: UpdateWithdrawalLimitAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            pool_auth: *accounts.pool_auth.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateWithdrawalLimitKeys>
for [AccountMeta; UPDATE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateWithdrawalLimitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
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
impl From<[Pubkey; UPDATE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN]>
for UpdateWithdrawalLimitKeys {
    fn from(pubkeys: [Pubkey; UPDATE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            pool_config: pubkeys[2],
            pool_auth: pubkeys[3],
            stablecoin_pool: pubkeys[4],
            stablecoin_mint: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<UpdateWithdrawalLimitAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateWithdrawalLimitAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.pool_auth.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN]>
for UpdateWithdrawalLimitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            pool_config: &arr[2],
            pool_auth: &arr[3],
            stablecoin_pool: &arr[4],
            stablecoin_mint: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const UPDATE_WITHDRAWAL_LIMIT_IX_DISCM: [u8; 8usize] = [
    146, 153, 162, 20, 19, 87, 243, 11,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateWithdrawalLimitIxArgs {
    pub new_withdrawal_limit: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateWithdrawalLimitIxData(pub UpdateWithdrawalLimitIxArgs);
impl From<UpdateWithdrawalLimitIxArgs> for UpdateWithdrawalLimitIxData {
    fn from(args: UpdateWithdrawalLimitIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateWithdrawalLimitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_WITHDRAWAL_LIMIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_withdrawal_limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateWithdrawalLimitIxArgs {
                new_withdrawal_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_WITHDRAWAL_LIMIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_withdrawal_limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_withdrawal_limit_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateWithdrawalLimitKeys,
    args: UpdateWithdrawalLimitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateWithdrawalLimitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_withdrawal_limit_ix(
    keys: UpdateWithdrawalLimitKeys,
    args: UpdateWithdrawalLimitIxArgs,
) -> std::io::Result<Instruction> {
    update_withdrawal_limit_ix_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, keys, args)
}
pub fn update_withdrawal_limit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateWithdrawalLimitAccounts<'_, '_>,
    args: UpdateWithdrawalLimitIxArgs,
) -> ProgramResult {
    let keys: UpdateWithdrawalLimitKeys = accounts.into();
    let ix = update_withdrawal_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_withdrawal_limit_invoke(
    accounts: UpdateWithdrawalLimitAccounts<'_, '_>,
    args: UpdateWithdrawalLimitIxArgs,
) -> ProgramResult {
    update_withdrawal_limit_invoke_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_withdrawal_limit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateWithdrawalLimitAccounts<'_, '_>,
    args: UpdateWithdrawalLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateWithdrawalLimitKeys = accounts.into();
    let ix = update_withdrawal_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_withdrawal_limit_invoke_signed(
    accounts: UpdateWithdrawalLimitAccounts<'_, '_>,
    args: UpdateWithdrawalLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_withdrawal_limit_invoke_signed_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_withdrawal_limit_verify_account_keys(
    accounts: UpdateWithdrawalLimitAccounts<'_, '_>,
    keys: UpdateWithdrawalLimitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_withdrawal_limit_verify_writable_privileges<'me, 'info>(
    accounts: UpdateWithdrawalLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_withdrawal_limit_verify_signer_privileges<'me, 'info>(
    accounts: UpdateWithdrawalLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_withdrawal_limit_verify_account_privileges<'me, 'info>(
    accounts: UpdateWithdrawalLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_withdrawal_limit_verify_writable_privileges(accounts)?;
    update_withdrawal_limit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const USER_DEPOSIT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct UserDepositAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub user_stablecoin_ta: &'me AccountInfo<'info>,
    pub user_lp_token_ta: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub lp_token_auth: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UserDepositKeys {
    pub user: Pubkey,
    pub pool_config: Pubkey,
    pub hylo: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub user_stablecoin_ta: Pubkey,
    pub user_lp_token_ta: Pubkey,
    pub pool_auth: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub lp_token_auth: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UserDepositAccounts<'_, '_>> for UserDepositKeys {
    fn from(accounts: UserDepositAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            pool_config: *accounts.pool_config.key,
            hylo: *accounts.hylo.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            user_stablecoin_ta: *accounts.user_stablecoin_ta.key,
            user_lp_token_ta: *accounts.user_lp_token_ta.key,
            pool_auth: *accounts.pool_auth.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            lp_token_auth: *accounts.lp_token_auth.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UserDepositKeys> for [AccountMeta; USER_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: UserDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_token_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; USER_DEPOSIT_IX_ACCOUNTS_LEN]> for UserDepositKeys {
    fn from(pubkeys: [Pubkey; USER_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            pool_config: pubkeys[1],
            hylo: pubkeys[2],
            stablecoin_mint: pubkeys[3],
            user_stablecoin_ta: pubkeys[4],
            user_lp_token_ta: pubkeys[5],
            pool_auth: pubkeys[6],
            stablecoin_pool: pubkeys[7],
            lp_token_auth: pubkeys[8],
            lp_token_mint: pubkeys[9],
            token_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<UserDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; USER_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UserDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.pool_config.clone(),
            accounts.hylo.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.user_stablecoin_ta.clone(),
            accounts.user_lp_token_ta.clone(),
            accounts.pool_auth.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.lp_token_auth.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; USER_DEPOSIT_IX_ACCOUNTS_LEN]>
for UserDepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; USER_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            pool_config: &arr[1],
            hylo: &arr[2],
            stablecoin_mint: &arr[3],
            user_stablecoin_ta: &arr[4],
            user_lp_token_ta: &arr[5],
            pool_auth: &arr[6],
            stablecoin_pool: &arr[7],
            lp_token_auth: &arr[8],
            lp_token_mint: &arr[9],
            token_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const USER_DEPOSIT_IX_DISCM: [u8; 8usize] = [186, 198, 140, 233, 129, 39, 98, 153];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserDepositIxArgs {
    pub amount_stablecoin: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserDepositIxData(pub UserDepositIxArgs);
impl From<UserDepositIxArgs> for UserDepositIxData {
    fn from(args: UserDepositIxArgs) -> Self {
        Self(args)
    }
}
impl UserDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_stablecoin: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UserDepositIxArgs {
                amount_stablecoin,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_stablecoin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn user_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: UserDepositKeys,
    args: UserDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; USER_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UserDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn user_deposit_ix(
    keys: UserDepositKeys,
    args: UserDepositIxArgs,
) -> std::io::Result<Instruction> {
    user_deposit_ix_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, keys, args)
}
pub fn user_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UserDepositAccounts<'_, '_>,
    args: UserDepositIxArgs,
) -> ProgramResult {
    let keys: UserDepositKeys = accounts.into();
    let ix = user_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn user_deposit_invoke(
    accounts: UserDepositAccounts<'_, '_>,
    args: UserDepositIxArgs,
) -> ProgramResult {
    user_deposit_invoke_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, accounts, args)
}
pub fn user_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UserDepositAccounts<'_, '_>,
    args: UserDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UserDepositKeys = accounts.into();
    let ix = user_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn user_deposit_invoke_signed(
    accounts: UserDepositAccounts<'_, '_>,
    args: UserDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    user_deposit_invoke_signed_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn user_deposit_verify_account_keys(
    accounts: UserDepositAccounts<'_, '_>,
    keys: UserDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.user_stablecoin_ta.key, keys.user_stablecoin_ta),
        (*accounts.user_lp_token_ta.key, keys.user_lp_token_ta),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.lp_token_auth.key, keys.lp_token_auth),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
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
pub fn user_deposit_verify_writable_privileges<'me, 'info>(
    accounts: UserDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.user_stablecoin_ta,
        accounts.user_lp_token_ta,
        accounts.stablecoin_pool,
        accounts.lp_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn user_deposit_verify_signer_privileges<'me, 'info>(
    accounts: UserDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn user_deposit_verify_account_privileges<'me, 'info>(
    accounts: UserDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    user_deposit_verify_writable_privileges(accounts)?;
    user_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const USER_WITHDRAW_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct UserWithdrawAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub user_stablecoin_ta: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub user_lp_token_ta: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UserWithdrawKeys {
    pub user: Pubkey,
    pub pool_config: Pubkey,
    pub hylo: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub user_stablecoin_ta: Pubkey,
    pub fee_auth: Pubkey,
    pub fee_vault: Pubkey,
    pub user_lp_token_ta: Pubkey,
    pub pool_auth: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UserWithdrawAccounts<'_, '_>> for UserWithdrawKeys {
    fn from(accounts: UserWithdrawAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            pool_config: *accounts.pool_config.key,
            hylo: *accounts.hylo.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            user_stablecoin_ta: *accounts.user_stablecoin_ta.key,
            fee_auth: *accounts.fee_auth.key,
            fee_vault: *accounts.fee_vault.key,
            user_lp_token_ta: *accounts.user_lp_token_ta.key,
            pool_auth: *accounts.pool_auth.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UserWithdrawKeys> for [AccountMeta; USER_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: UserWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_token_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; USER_WITHDRAW_IX_ACCOUNTS_LEN]> for UserWithdrawKeys {
    fn from(pubkeys: [Pubkey; USER_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            pool_config: pubkeys[1],
            hylo: pubkeys[2],
            stablecoin_mint: pubkeys[3],
            user_stablecoin_ta: pubkeys[4],
            fee_auth: pubkeys[5],
            fee_vault: pubkeys[6],
            user_lp_token_ta: pubkeys[7],
            pool_auth: pubkeys[8],
            stablecoin_pool: pubkeys[9],
            lp_token_mint: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<UserWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; USER_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: UserWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.pool_config.clone(),
            accounts.hylo.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.user_stablecoin_ta.clone(),
            accounts.fee_auth.clone(),
            accounts.fee_vault.clone(),
            accounts.user_lp_token_ta.clone(),
            accounts.pool_auth.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; USER_WITHDRAW_IX_ACCOUNTS_LEN]>
for UserWithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; USER_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            pool_config: &arr[1],
            hylo: &arr[2],
            stablecoin_mint: &arr[3],
            user_stablecoin_ta: &arr[4],
            fee_auth: &arr[5],
            fee_vault: &arr[6],
            user_lp_token_ta: &arr[7],
            pool_auth: &arr[8],
            stablecoin_pool: &arr[9],
            lp_token_mint: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const USER_WITHDRAW_IX_DISCM: [u8; 8usize] = [53, 254, 26, 242, 119, 237, 73, 33];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserWithdrawIxArgs {
    pub amount_lp_token: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserWithdrawIxData(pub UserWithdrawIxArgs);
impl From<UserWithdrawIxArgs> for UserWithdrawIxData {
    fn from(args: UserWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl UserWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_lp_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UserWithdrawIxArgs {
                amount_lp_token,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_lp_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn user_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: UserWithdrawKeys,
    args: UserWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; USER_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: UserWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn user_withdraw_ix(
    keys: UserWithdrawKeys,
    args: UserWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    user_withdraw_ix_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, keys, args)
}
pub fn user_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UserWithdrawAccounts<'_, '_>,
    args: UserWithdrawIxArgs,
) -> ProgramResult {
    let keys: UserWithdrawKeys = accounts.into();
    let ix = user_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn user_withdraw_invoke(
    accounts: UserWithdrawAccounts<'_, '_>,
    args: UserWithdrawIxArgs,
) -> ProgramResult {
    user_withdraw_invoke_with_program_id(HYLO_EARN_POOL_PROGRAM_ID, accounts, args)
}
pub fn user_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UserWithdrawAccounts<'_, '_>,
    args: UserWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UserWithdrawKeys = accounts.into();
    let ix = user_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn user_withdraw_invoke_signed(
    accounts: UserWithdrawAccounts<'_, '_>,
    args: UserWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    user_withdraw_invoke_signed_with_program_id(
        HYLO_EARN_POOL_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn user_withdraw_verify_account_keys(
    accounts: UserWithdrawAccounts<'_, '_>,
    keys: UserWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.user_stablecoin_ta.key, keys.user_stablecoin_ta),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.user_lp_token_ta.key, keys.user_lp_token_ta),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
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
pub fn user_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: UserWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.pool_config,
        accounts.user_stablecoin_ta,
        accounts.fee_vault,
        accounts.user_lp_token_ta,
        accounts.stablecoin_pool,
        accounts.lp_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn user_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: UserWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn user_withdraw_verify_account_privileges<'me, 'info>(
    accounts: UserWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    user_withdraw_verify_writable_privileges(accounts)?;
    user_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
