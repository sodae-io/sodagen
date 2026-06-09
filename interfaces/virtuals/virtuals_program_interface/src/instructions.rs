use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum VirtualsProgramProgramIx {
    Buy(BuyIxArgs),
    ClaimFees,
    CreateMeteoraPool,
    Initialize,
    InitializeMeteoraAccounts,
    Launch(LaunchIxArgs),
    Sell(SellIxArgs),
    UpdatePoolCreator,
}
impl VirtualsProgramProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&BUY_IX_DISCM) {
            let mut reader = &buf[BUY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Buy(BuyIxArgs {
                    amount,
                    max_amount_out,
                }),
            );
        }
        if buf.starts_with(&CLAIM_FEES_IX_DISCM) {
            return Ok(Self::ClaimFees);
        }
        if buf.starts_with(&CREATE_METEORA_POOL_IX_DISCM) {
            return Ok(Self::CreateMeteoraPool);
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            return Ok(Self::Initialize);
        }
        if buf.starts_with(&INITIALIZE_METEORA_ACCOUNTS_IX_DISCM) {
            return Ok(Self::InitializeMeteoraAccounts);
        }
        if buf.starts_with(&LAUNCH_IX_DISCM) {
            let mut reader = &buf[LAUNCH_IX_DISCM.len()..];
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Launch(LaunchIxArgs { symbol, name, uri }));
        }
        if buf.starts_with(&SELL_IX_DISCM) {
            let mut reader = &buf[SELL_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Sell(SellIxArgs {
                    amount,
                    min_amount_out,
                }),
            );
        }
        if buf.starts_with(&UPDATE_POOL_CREATOR_IX_DISCM) {
            return Ok(Self::UpdatePoolCreator);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Buy(args) => {
                writer.write_all(&BUY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_amount_out, &mut writer)?;
                Ok(())
            }
            Self::ClaimFees => writer.write_all(&CLAIM_FEES_IX_DISCM),
            Self::CreateMeteoraPool => writer.write_all(&CREATE_METEORA_POOL_IX_DISCM),
            Self::Initialize => writer.write_all(&INITIALIZE_IX_DISCM),
            Self::InitializeMeteoraAccounts => {
                writer.write_all(&INITIALIZE_METEORA_ACCOUNTS_IX_DISCM)
            }
            Self::Launch(args) => {
                writer.write_all(&LAUNCH_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                Ok(())
            }
            Self::Sell(args) => {
                writer.write_all(&SELL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                Ok(())
            }
            Self::UpdatePoolCreator => writer.write_all(&UPDATE_POOL_CREATOR_IX_DISCM),
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
pub const BUY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct BuyAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vpool: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub user_virtuals_ata: &'me AccountInfo<'info>,
    pub user_token_ata: &'me AccountInfo<'info>,
    pub vpool_token_ata: &'me AccountInfo<'info>,
    pub platform_prototype: &'me AccountInfo<'info>,
    pub platform_prototype_virtuals_ata: &'me AccountInfo<'info>,
    pub vpool_virtuals_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyKeys {
    pub user: Pubkey,
    pub vpool: Pubkey,
    pub token_mint: Pubkey,
    pub user_virtuals_ata: Pubkey,
    pub user_token_ata: Pubkey,
    pub vpool_token_ata: Pubkey,
    pub platform_prototype: Pubkey,
    pub platform_prototype_virtuals_ata: Pubkey,
    pub vpool_virtuals_ata: Pubkey,
    pub token_program: Pubkey,
}
impl From<BuyAccounts<'_, '_>> for BuyKeys {
    fn from(accounts: BuyAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vpool: *accounts.vpool.key,
            token_mint: *accounts.token_mint.key,
            user_virtuals_ata: *accounts.user_virtuals_ata.key,
            user_token_ata: *accounts.user_token_ata.key,
            vpool_token_ata: *accounts.vpool_token_ata.key,
            platform_prototype: *accounts.platform_prototype.key,
            platform_prototype_virtuals_ata: *accounts
                .platform_prototype_virtuals_ata
                .key,
            vpool_virtuals_ata: *accounts.vpool_virtuals_ata.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<BuyKeys> for [AccountMeta; BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_prototype,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_prototype_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool_virtuals_ata,
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
impl From<[Pubkey; BUY_IX_ACCOUNTS_LEN]> for BuyKeys {
    fn from(pubkeys: [Pubkey; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            vpool: pubkeys[1],
            token_mint: pubkeys[2],
            user_virtuals_ata: pubkeys[3],
            user_token_ata: pubkeys[4],
            vpool_token_ata: pubkeys[5],
            platform_prototype: pubkeys[6],
            platform_prototype_virtuals_ata: pubkeys[7],
            vpool_virtuals_ata: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<BuyAccounts<'_, 'info>> for [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vpool.clone(),
            accounts.token_mint.clone(),
            accounts.user_virtuals_ata.clone(),
            accounts.user_token_ata.clone(),
            accounts.vpool_token_ata.clone(),
            accounts.platform_prototype.clone(),
            accounts.platform_prototype_virtuals_ata.clone(),
            accounts.vpool_virtuals_ata.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]>
for BuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            vpool: &arr[1],
            token_mint: &arr[2],
            user_virtuals_ata: &arr[3],
            user_token_ata: &arr[4],
            vpool_token_ata: &arr[5],
            platform_prototype: &arr[6],
            platform_prototype_virtuals_ata: &arr[7],
            vpool_virtuals_ata: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const BUY_IX_DISCM: [u8; 8usize] = [102, 6, 61, 18, 1, 218, 235, 234];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyIxArgs {
    pub amount: u64,
    pub max_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyIxData(pub BuyIxArgs);
impl From<BuyIxArgs> for BuyIxData {
    fn from(args: BuyIxArgs) -> Self {
        Self(args)
    }
}
impl BuyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyIxArgs {
                amount,
                max_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyKeys,
    args: BuyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_ix(keys: BuyKeys, args: BuyIxArgs) -> std::io::Result<Instruction> {
    buy_ix_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, keys, args)
}
pub fn buy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
) -> ProgramResult {
    let keys: BuyKeys = accounts.into();
    let ix = buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_invoke(accounts: BuyAccounts<'_, '_>, args: BuyIxArgs) -> ProgramResult {
    buy_invoke_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, accounts, args)
}
pub fn buy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyKeys = accounts.into();
    let ix = buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_invoke_signed(
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_invoke_signed_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, accounts, args, seeds)
}
pub fn buy_verify_account_keys(
    accounts: BuyAccounts<'_, '_>,
    keys: BuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vpool.key, keys.vpool),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.user_virtuals_ata.key, keys.user_virtuals_ata),
        (*accounts.user_token_ata.key, keys.user_token_ata),
        (*accounts.vpool_token_ata.key, keys.vpool_token_ata),
        (*accounts.platform_prototype.key, keys.platform_prototype),
        (
            *accounts.platform_prototype_virtuals_ata.key,
            keys.platform_prototype_virtuals_ata,
        ),
        (*accounts.vpool_virtuals_ata.key, keys.vpool_virtuals_ata),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_verify_writable_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vpool,
        accounts.user_virtuals_ata,
        accounts.user_token_ata,
        accounts.vpool_token_ata,
        accounts.platform_prototype,
        accounts.platform_prototype_virtuals_ata,
        accounts.vpool_virtuals_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_verify_signer_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_verify_account_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_verify_writable_privileges(accounts)?;
    buy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_FEES_IX_ACCOUNTS_LEN: usize = 28;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFeesAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub vpool: &'me AccountInfo<'info>,
    pub virtuals_mint: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub vpool_virtuals_ata: &'me AccountInfo<'info>,
    pub vpool_token_ata: &'me AccountInfo<'info>,
    pub platform: &'me AccountInfo<'info>,
    pub platform_virtuals_ata: &'me AccountInfo<'info>,
    pub platform_token_ata: &'me AccountInfo<'info>,
    pub creator_virtuals_ata: &'me AccountInfo<'info>,
    pub creator_token_ata: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub escrow_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub virtuals_vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub virtuals_token_vault: &'me AccountInfo<'info>,
    pub token_token_vault: &'me AccountInfo<'info>,
    pub virtuals_vault_lp_mint: &'me AccountInfo<'info>,
    pub token_vault_lp_mint: &'me AccountInfo<'info>,
    pub virtuals_vault_lp: &'me AccountInfo<'info>,
    pub token_vault_lp: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub dynamic_amm_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFeesKeys {
    pub payer: Pubkey,
    pub vpool: Pubkey,
    pub virtuals_mint: Pubkey,
    pub token_mint: Pubkey,
    pub vpool_virtuals_ata: Pubkey,
    pub vpool_token_ata: Pubkey,
    pub platform: Pubkey,
    pub platform_virtuals_ata: Pubkey,
    pub platform_token_ata: Pubkey,
    pub creator_virtuals_ata: Pubkey,
    pub creator_token_ata: Pubkey,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub lock_escrow: Pubkey,
    pub escrow_vault: Pubkey,
    pub token_program: Pubkey,
    pub virtuals_vault: Pubkey,
    pub token_vault: Pubkey,
    pub virtuals_token_vault: Pubkey,
    pub token_token_vault: Pubkey,
    pub virtuals_vault_lp_mint: Pubkey,
    pub token_vault_lp_mint: Pubkey,
    pub virtuals_vault_lp: Pubkey,
    pub token_vault_lp: Pubkey,
    pub vault_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub dynamic_amm_program: Pubkey,
}
impl From<ClaimFeesAccounts<'_, '_>> for ClaimFeesKeys {
    fn from(accounts: ClaimFeesAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            vpool: *accounts.vpool.key,
            virtuals_mint: *accounts.virtuals_mint.key,
            token_mint: *accounts.token_mint.key,
            vpool_virtuals_ata: *accounts.vpool_virtuals_ata.key,
            vpool_token_ata: *accounts.vpool_token_ata.key,
            platform: *accounts.platform.key,
            platform_virtuals_ata: *accounts.platform_virtuals_ata.key,
            platform_token_ata: *accounts.platform_token_ata.key,
            creator_virtuals_ata: *accounts.creator_virtuals_ata.key,
            creator_token_ata: *accounts.creator_token_ata.key,
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            lock_escrow: *accounts.lock_escrow.key,
            escrow_vault: *accounts.escrow_vault.key,
            token_program: *accounts.token_program.key,
            virtuals_vault: *accounts.virtuals_vault.key,
            token_vault: *accounts.token_vault.key,
            virtuals_token_vault: *accounts.virtuals_token_vault.key,
            token_token_vault: *accounts.token_token_vault.key,
            virtuals_vault_lp_mint: *accounts.virtuals_vault_lp_mint.key,
            token_vault_lp_mint: *accounts.token_vault_lp_mint.key,
            virtuals_vault_lp: *accounts.virtuals_vault_lp.key,
            token_vault_lp: *accounts.token_vault_lp.key,
            vault_program: *accounts.vault_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            dynamic_amm_program: *accounts.dynamic_amm_program.key,
        }
    }
}
impl From<ClaimFeesKeys> for [AccountMeta; CLAIM_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vpool_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_token_ata,
                is_signer: false,
                is_writable: true,
            },
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
                pubkey: keys.lock_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.virtuals_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_program,
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
            AccountMeta {
                pubkey: keys.dynamic_amm_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_FEES_IX_ACCOUNTS_LEN]> for ClaimFeesKeys {
    fn from(pubkeys: [Pubkey; CLAIM_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            vpool: pubkeys[1],
            virtuals_mint: pubkeys[2],
            token_mint: pubkeys[3],
            vpool_virtuals_ata: pubkeys[4],
            vpool_token_ata: pubkeys[5],
            platform: pubkeys[6],
            platform_virtuals_ata: pubkeys[7],
            platform_token_ata: pubkeys[8],
            creator_virtuals_ata: pubkeys[9],
            creator_token_ata: pubkeys[10],
            pool: pubkeys[11],
            lp_mint: pubkeys[12],
            lock_escrow: pubkeys[13],
            escrow_vault: pubkeys[14],
            token_program: pubkeys[15],
            virtuals_vault: pubkeys[16],
            token_vault: pubkeys[17],
            virtuals_token_vault: pubkeys[18],
            token_token_vault: pubkeys[19],
            virtuals_vault_lp_mint: pubkeys[20],
            token_vault_lp_mint: pubkeys[21],
            virtuals_vault_lp: pubkeys[22],
            token_vault_lp: pubkeys[23],
            vault_program: pubkeys[24],
            associated_token_program: pubkeys[25],
            system_program: pubkeys[26],
            dynamic_amm_program: pubkeys[27],
        }
    }
}
impl<'info> From<ClaimFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.vpool.clone(),
            accounts.virtuals_mint.clone(),
            accounts.token_mint.clone(),
            accounts.vpool_virtuals_ata.clone(),
            accounts.vpool_token_ata.clone(),
            accounts.platform.clone(),
            accounts.platform_virtuals_ata.clone(),
            accounts.platform_token_ata.clone(),
            accounts.creator_virtuals_ata.clone(),
            accounts.creator_token_ata.clone(),
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.lock_escrow.clone(),
            accounts.escrow_vault.clone(),
            accounts.token_program.clone(),
            accounts.virtuals_vault.clone(),
            accounts.token_vault.clone(),
            accounts.virtuals_token_vault.clone(),
            accounts.token_token_vault.clone(),
            accounts.virtuals_vault_lp_mint.clone(),
            accounts.token_vault_lp_mint.clone(),
            accounts.virtuals_vault_lp.clone(),
            accounts.token_vault_lp.clone(),
            accounts.vault_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.dynamic_amm_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FEES_IX_ACCOUNTS_LEN]>
for ClaimFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            vpool: &arr[1],
            virtuals_mint: &arr[2],
            token_mint: &arr[3],
            vpool_virtuals_ata: &arr[4],
            vpool_token_ata: &arr[5],
            platform: &arr[6],
            platform_virtuals_ata: &arr[7],
            platform_token_ata: &arr[8],
            creator_virtuals_ata: &arr[9],
            creator_token_ata: &arr[10],
            pool: &arr[11],
            lp_mint: &arr[12],
            lock_escrow: &arr[13],
            escrow_vault: &arr[14],
            token_program: &arr[15],
            virtuals_vault: &arr[16],
            token_vault: &arr[17],
            virtuals_token_vault: &arr[18],
            token_token_vault: &arr[19],
            virtuals_vault_lp_mint: &arr[20],
            token_vault_lp_mint: &arr[21],
            virtuals_vault_lp: &arr[22],
            token_vault_lp: &arr[23],
            vault_program: &arr[24],
            associated_token_program: &arr[25],
            system_program: &arr[26],
            dynamic_amm_program: &arr[27],
        }
    }
}
pub const CLAIM_FEES_IX_DISCM: [u8; 8usize] = [82, 251, 233, 156, 12, 52, 184, 202];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeesIxData;
impl ClaimFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimFeesIxData.try_to_vec()?,
    })
}
pub fn claim_fees_ix(keys: ClaimFeesKeys) -> std::io::Result<Instruction> {
    claim_fees_ix_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, keys)
}
pub fn claim_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimFeesKeys = accounts.into();
    let ix = claim_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_fees_invoke(accounts: ClaimFeesAccounts<'_, '_>) -> ProgramResult {
    claim_fees_invoke_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, accounts)
}
pub fn claim_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimFeesKeys = accounts.into();
    let ix = claim_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_fees_invoke_signed(
    accounts: ClaimFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_fees_invoke_signed_with_program_id(
        VIRTUALS_PROGRAM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_fees_verify_account_keys(
    accounts: ClaimFeesAccounts<'_, '_>,
    keys: ClaimFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.vpool.key, keys.vpool),
        (*accounts.virtuals_mint.key, keys.virtuals_mint),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.vpool_virtuals_ata.key, keys.vpool_virtuals_ata),
        (*accounts.vpool_token_ata.key, keys.vpool_token_ata),
        (*accounts.platform.key, keys.platform),
        (*accounts.platform_virtuals_ata.key, keys.platform_virtuals_ata),
        (*accounts.platform_token_ata.key, keys.platform_token_ata),
        (*accounts.creator_virtuals_ata.key, keys.creator_virtuals_ata),
        (*accounts.creator_token_ata.key, keys.creator_token_ata),
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.escrow_vault.key, keys.escrow_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.virtuals_vault.key, keys.virtuals_vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.virtuals_token_vault.key, keys.virtuals_token_vault),
        (*accounts.token_token_vault.key, keys.token_token_vault),
        (*accounts.virtuals_vault_lp_mint.key, keys.virtuals_vault_lp_mint),
        (*accounts.token_vault_lp_mint.key, keys.token_vault_lp_mint),
        (*accounts.virtuals_vault_lp.key, keys.virtuals_vault_lp),
        (*accounts.token_vault_lp.key, keys.token_vault_lp),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.dynamic_amm_program.key, keys.dynamic_amm_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_fees_verify_writable_privileges<'me, 'info>(
    accounts: ClaimFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.vpool,
        accounts.vpool_virtuals_ata,
        accounts.vpool_token_ata,
        accounts.platform,
        accounts.platform_virtuals_ata,
        accounts.platform_token_ata,
        accounts.creator_virtuals_ata,
        accounts.creator_token_ata,
        accounts.pool,
        accounts.lp_mint,
        accounts.lock_escrow,
        accounts.escrow_vault,
        accounts.virtuals_vault,
        accounts.token_vault,
        accounts.virtuals_token_vault,
        accounts.token_token_vault,
        accounts.virtuals_vault_lp_mint,
        accounts.token_vault_lp_mint,
        accounts.virtuals_vault_lp,
        accounts.token_vault_lp,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_fees_verify_signer_privileges<'me, 'info>(
    accounts: ClaimFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_fees_verify_account_privileges<'me, 'info>(
    accounts: ClaimFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_fees_verify_writable_privileges(accounts)?;
    claim_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_METEORA_POOL_IX_ACCOUNTS_LEN: usize = 36;
#[derive(Copy, Clone, Debug)]
pub struct CreateMeteoraPoolAccounts<'me, 'info> {
    pub vpool: &'me AccountInfo<'info>,
    pub meteora_deployer: &'me AccountInfo<'info>,
    pub meteora_deployer_virtuals_ata: &'me AccountInfo<'info>,
    pub meteora_deployer_token_ata: &'me AccountInfo<'info>,
    pub vpool_virtuals_ata: &'me AccountInfo<'info>,
    pub vpool_token_ata: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub escrow_vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub virtuals_mint: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub virtuals_vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub virtuals_token_vault: &'me AccountInfo<'info>,
    pub token_token_vault: &'me AccountInfo<'info>,
    pub virtuals_vault_lp_mint: &'me AccountInfo<'info>,
    pub token_vault_lp_mint: &'me AccountInfo<'info>,
    pub virtuals_vault_lp: &'me AccountInfo<'info>,
    pub token_vault_lp: &'me AccountInfo<'info>,
    pub pool_virtuals_ata: &'me AccountInfo<'info>,
    pub pool_token_ata: &'me AccountInfo<'info>,
    pub meteora_deployer_pool_lp: &'me AccountInfo<'info>,
    pub protocol_virtuals_fee: &'me AccountInfo<'info>,
    pub protocol_token_fee: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_metadata: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub mint_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub dynamic_amm_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMeteoraPoolKeys {
    pub vpool: Pubkey,
    pub meteora_deployer: Pubkey,
    pub meteora_deployer_virtuals_ata: Pubkey,
    pub meteora_deployer_token_ata: Pubkey,
    pub vpool_virtuals_ata: Pubkey,
    pub vpool_token_ata: Pubkey,
    pub lock_escrow: Pubkey,
    pub escrow_vault: Pubkey,
    pub pool: Pubkey,
    pub config: Pubkey,
    pub lp_mint: Pubkey,
    pub virtuals_mint: Pubkey,
    pub token_mint: Pubkey,
    pub virtuals_vault: Pubkey,
    pub token_vault: Pubkey,
    pub virtuals_token_vault: Pubkey,
    pub token_token_vault: Pubkey,
    pub virtuals_vault_lp_mint: Pubkey,
    pub token_vault_lp_mint: Pubkey,
    pub virtuals_vault_lp: Pubkey,
    pub token_vault_lp: Pubkey,
    pub pool_virtuals_ata: Pubkey,
    pub pool_token_ata: Pubkey,
    pub meteora_deployer_pool_lp: Pubkey,
    pub protocol_virtuals_fee: Pubkey,
    pub protocol_token_fee: Pubkey,
    pub payer: Pubkey,
    pub token_metadata: Pubkey,
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub dynamic_amm_program: Pubkey,
}
impl From<CreateMeteoraPoolAccounts<'_, '_>> for CreateMeteoraPoolKeys {
    fn from(accounts: CreateMeteoraPoolAccounts) -> Self {
        Self {
            vpool: *accounts.vpool.key,
            meteora_deployer: *accounts.meteora_deployer.key,
            meteora_deployer_virtuals_ata: *accounts.meteora_deployer_virtuals_ata.key,
            meteora_deployer_token_ata: *accounts.meteora_deployer_token_ata.key,
            vpool_virtuals_ata: *accounts.vpool_virtuals_ata.key,
            vpool_token_ata: *accounts.vpool_token_ata.key,
            lock_escrow: *accounts.lock_escrow.key,
            escrow_vault: *accounts.escrow_vault.key,
            pool: *accounts.pool.key,
            config: *accounts.config.key,
            lp_mint: *accounts.lp_mint.key,
            virtuals_mint: *accounts.virtuals_mint.key,
            token_mint: *accounts.token_mint.key,
            virtuals_vault: *accounts.virtuals_vault.key,
            token_vault: *accounts.token_vault.key,
            virtuals_token_vault: *accounts.virtuals_token_vault.key,
            token_token_vault: *accounts.token_token_vault.key,
            virtuals_vault_lp_mint: *accounts.virtuals_vault_lp_mint.key,
            token_vault_lp_mint: *accounts.token_vault_lp_mint.key,
            virtuals_vault_lp: *accounts.virtuals_vault_lp.key,
            token_vault_lp: *accounts.token_vault_lp.key,
            pool_virtuals_ata: *accounts.pool_virtuals_ata.key,
            pool_token_ata: *accounts.pool_token_ata.key,
            meteora_deployer_pool_lp: *accounts.meteora_deployer_pool_lp.key,
            protocol_virtuals_fee: *accounts.protocol_virtuals_fee.key,
            protocol_token_fee: *accounts.protocol_token_fee.key,
            payer: *accounts.payer.key,
            token_metadata: *accounts.token_metadata.key,
            rent: *accounts.rent.key,
            mint_metadata: *accounts.mint_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            dynamic_amm_program: *accounts.dynamic_amm_program.key,
        }
    }
}
impl From<CreateMeteoraPoolKeys> for [AccountMeta; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMeteoraPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_deployer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_deployer_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_deployer_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lock_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.virtuals_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_deployer_pool_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_virtuals_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_token_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
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
            AccountMeta {
                pubkey: keys.dynamic_amm_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN]> for CreateMeteoraPoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vpool: pubkeys[0],
            meteora_deployer: pubkeys[1],
            meteora_deployer_virtuals_ata: pubkeys[2],
            meteora_deployer_token_ata: pubkeys[3],
            vpool_virtuals_ata: pubkeys[4],
            vpool_token_ata: pubkeys[5],
            lock_escrow: pubkeys[6],
            escrow_vault: pubkeys[7],
            pool: pubkeys[8],
            config: pubkeys[9],
            lp_mint: pubkeys[10],
            virtuals_mint: pubkeys[11],
            token_mint: pubkeys[12],
            virtuals_vault: pubkeys[13],
            token_vault: pubkeys[14],
            virtuals_token_vault: pubkeys[15],
            token_token_vault: pubkeys[16],
            virtuals_vault_lp_mint: pubkeys[17],
            token_vault_lp_mint: pubkeys[18],
            virtuals_vault_lp: pubkeys[19],
            token_vault_lp: pubkeys[20],
            pool_virtuals_ata: pubkeys[21],
            pool_token_ata: pubkeys[22],
            meteora_deployer_pool_lp: pubkeys[23],
            protocol_virtuals_fee: pubkeys[24],
            protocol_token_fee: pubkeys[25],
            payer: pubkeys[26],
            token_metadata: pubkeys[27],
            rent: pubkeys[28],
            mint_metadata: pubkeys[29],
            metadata_program: pubkeys[30],
            vault_program: pubkeys[31],
            token_program: pubkeys[32],
            associated_token_program: pubkeys[33],
            system_program: pubkeys[34],
            dynamic_amm_program: pubkeys[35],
        }
    }
}
impl<'info> From<CreateMeteoraPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMeteoraPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.vpool.clone(),
            accounts.meteora_deployer.clone(),
            accounts.meteora_deployer_virtuals_ata.clone(),
            accounts.meteora_deployer_token_ata.clone(),
            accounts.vpool_virtuals_ata.clone(),
            accounts.vpool_token_ata.clone(),
            accounts.lock_escrow.clone(),
            accounts.escrow_vault.clone(),
            accounts.pool.clone(),
            accounts.config.clone(),
            accounts.lp_mint.clone(),
            accounts.virtuals_mint.clone(),
            accounts.token_mint.clone(),
            accounts.virtuals_vault.clone(),
            accounts.token_vault.clone(),
            accounts.virtuals_token_vault.clone(),
            accounts.token_token_vault.clone(),
            accounts.virtuals_vault_lp_mint.clone(),
            accounts.token_vault_lp_mint.clone(),
            accounts.virtuals_vault_lp.clone(),
            accounts.token_vault_lp.clone(),
            accounts.pool_virtuals_ata.clone(),
            accounts.pool_token_ata.clone(),
            accounts.meteora_deployer_pool_lp.clone(),
            accounts.protocol_virtuals_fee.clone(),
            accounts.protocol_token_fee.clone(),
            accounts.payer.clone(),
            accounts.token_metadata.clone(),
            accounts.rent.clone(),
            accounts.mint_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.dynamic_amm_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN]>
for CreateMeteoraPoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vpool: &arr[0],
            meteora_deployer: &arr[1],
            meteora_deployer_virtuals_ata: &arr[2],
            meteora_deployer_token_ata: &arr[3],
            vpool_virtuals_ata: &arr[4],
            vpool_token_ata: &arr[5],
            lock_escrow: &arr[6],
            escrow_vault: &arr[7],
            pool: &arr[8],
            config: &arr[9],
            lp_mint: &arr[10],
            virtuals_mint: &arr[11],
            token_mint: &arr[12],
            virtuals_vault: &arr[13],
            token_vault: &arr[14],
            virtuals_token_vault: &arr[15],
            token_token_vault: &arr[16],
            virtuals_vault_lp_mint: &arr[17],
            token_vault_lp_mint: &arr[18],
            virtuals_vault_lp: &arr[19],
            token_vault_lp: &arr[20],
            pool_virtuals_ata: &arr[21],
            pool_token_ata: &arr[22],
            meteora_deployer_pool_lp: &arr[23],
            protocol_virtuals_fee: &arr[24],
            protocol_token_fee: &arr[25],
            payer: &arr[26],
            token_metadata: &arr[27],
            rent: &arr[28],
            mint_metadata: &arr[29],
            metadata_program: &arr[30],
            vault_program: &arr[31],
            token_program: &arr[32],
            associated_token_program: &arr[33],
            system_program: &arr[34],
            dynamic_amm_program: &arr[35],
        }
    }
}
pub const CREATE_METEORA_POOL_IX_DISCM: [u8; 8usize] = [
    246, 254, 33, 37, 225, 176, 41, 232,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMeteoraPoolIxData;
impl CreateMeteoraPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_METEORA_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_METEORA_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_meteora_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateMeteoraPoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateMeteoraPoolIxData.try_to_vec()?,
    })
}
pub fn create_meteora_pool_ix(
    keys: CreateMeteoraPoolKeys,
) -> std::io::Result<Instruction> {
    create_meteora_pool_ix_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, keys)
}
pub fn create_meteora_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateMeteoraPoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateMeteoraPoolKeys = accounts.into();
    let ix = create_meteora_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_meteora_pool_invoke(
    accounts: CreateMeteoraPoolAccounts<'_, '_>,
) -> ProgramResult {
    create_meteora_pool_invoke_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, accounts)
}
pub fn create_meteora_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateMeteoraPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateMeteoraPoolKeys = accounts.into();
    let ix = create_meteora_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_meteora_pool_invoke_signed(
    accounts: CreateMeteoraPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_meteora_pool_invoke_signed_with_program_id(
        VIRTUALS_PROGRAM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_meteora_pool_verify_account_keys(
    accounts: CreateMeteoraPoolAccounts<'_, '_>,
    keys: CreateMeteoraPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vpool.key, keys.vpool),
        (*accounts.meteora_deployer.key, keys.meteora_deployer),
        (
            *accounts.meteora_deployer_virtuals_ata.key,
            keys.meteora_deployer_virtuals_ata,
        ),
        (*accounts.meteora_deployer_token_ata.key, keys.meteora_deployer_token_ata),
        (*accounts.vpool_virtuals_ata.key, keys.vpool_virtuals_ata),
        (*accounts.vpool_token_ata.key, keys.vpool_token_ata),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.escrow_vault.key, keys.escrow_vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.config.key, keys.config),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.virtuals_mint.key, keys.virtuals_mint),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.virtuals_vault.key, keys.virtuals_vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.virtuals_token_vault.key, keys.virtuals_token_vault),
        (*accounts.token_token_vault.key, keys.token_token_vault),
        (*accounts.virtuals_vault_lp_mint.key, keys.virtuals_vault_lp_mint),
        (*accounts.token_vault_lp_mint.key, keys.token_vault_lp_mint),
        (*accounts.virtuals_vault_lp.key, keys.virtuals_vault_lp),
        (*accounts.token_vault_lp.key, keys.token_vault_lp),
        (*accounts.pool_virtuals_ata.key, keys.pool_virtuals_ata),
        (*accounts.pool_token_ata.key, keys.pool_token_ata),
        (*accounts.meteora_deployer_pool_lp.key, keys.meteora_deployer_pool_lp),
        (*accounts.protocol_virtuals_fee.key, keys.protocol_virtuals_fee),
        (*accounts.protocol_token_fee.key, keys.protocol_token_fee),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_metadata.key, keys.token_metadata),
        (*accounts.rent.key, keys.rent),
        (*accounts.mint_metadata.key, keys.mint_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.dynamic_amm_program.key, keys.dynamic_amm_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_meteora_pool_verify_writable_privileges<'me, 'info>(
    accounts: CreateMeteoraPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vpool,
        accounts.meteora_deployer,
        accounts.meteora_deployer_virtuals_ata,
        accounts.meteora_deployer_token_ata,
        accounts.vpool_virtuals_ata,
        accounts.vpool_token_ata,
        accounts.lock_escrow,
        accounts.escrow_vault,
        accounts.pool,
        accounts.lp_mint,
        accounts.virtuals_vault,
        accounts.token_vault,
        accounts.virtuals_token_vault,
        accounts.token_token_vault,
        accounts.virtuals_vault_lp_mint,
        accounts.token_vault_lp_mint,
        accounts.virtuals_vault_lp,
        accounts.token_vault_lp,
        accounts.pool_virtuals_ata,
        accounts.pool_token_ata,
        accounts.meteora_deployer_pool_lp,
        accounts.protocol_virtuals_fee,
        accounts.protocol_token_fee,
        accounts.payer,
        accounts.token_metadata,
        accounts.mint_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_meteora_pool_verify_signer_privileges<'me, 'info>(
    accounts: CreateMeteoraPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_meteora_pool_verify_account_privileges<'me, 'info>(
    accounts: CreateMeteoraPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_meteora_pool_verify_writable_privileges(accounts)?;
    create_meteora_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub virtuals_mint: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub vpool_virtuals_ata: &'me AccountInfo<'info>,
    pub vpool_token_ata: &'me AccountInfo<'info>,
    pub vpool: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub payer: Pubkey,
    pub virtuals_mint: Pubkey,
    pub token_mint: Pubkey,
    pub vpool_virtuals_ata: Pubkey,
    pub vpool_token_ata: Pubkey,
    pub vpool: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            virtuals_mint: *accounts.virtuals_mint.key,
            token_mint: *accounts.token_mint.key,
            vpool_virtuals_ata: *accounts.vpool_virtuals_ata.key,
            vpool_token_ata: *accounts.vpool_token_ata.key,
            vpool: *accounts.vpool.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool,
                is_signer: false,
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
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            virtuals_mint: pubkeys[1],
            token_mint: pubkeys[2],
            vpool_virtuals_ata: pubkeys[3],
            vpool_token_ata: pubkeys[4],
            vpool: pubkeys[5],
            token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.virtuals_mint.clone(),
            accounts.token_mint.clone(),
            accounts.vpool_virtuals_ata.clone(),
            accounts.vpool_token_ata.clone(),
            accounts.vpool.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            virtuals_mint: &arr[1],
            token_mint: &arr[2],
            vpool_virtuals_ata: &arr[3],
            vpool_token_ata: &arr[4],
            vpool: &arr[5],
            token_program: &arr[6],
            associated_token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8usize] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData;
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeIxData.try_to_vec()?,
    })
}
pub fn initialize_ix(keys: InitializeKeys) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, keys)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(accounts: InitializeAccounts<'_, '_>) -> ProgramResult {
    initialize_invoke_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, accounts)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(
        VIRTUALS_PROGRAM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.virtuals_mint.key, keys.virtuals_mint),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.vpool_virtuals_ata.key, keys.vpool_virtuals_ata),
        (*accounts.vpool_token_ata.key, keys.vpool_token_ata),
        (*accounts.vpool.key, keys.vpool),
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
pub fn initialize_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.token_mint,
        accounts.vpool_virtuals_ata,
        accounts.vpool_token_ata,
        accounts.vpool,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_verify_writable_privileges(accounts)?;
    initialize_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_METEORA_ACCOUNTS_IX_ACCOUNTS_LEN: usize = 36;
#[derive(Copy, Clone, Debug)]
pub struct InitializeMeteoraAccountsAccounts<'me, 'info> {
    pub vpool: &'me AccountInfo<'info>,
    pub meteora_deployer: &'me AccountInfo<'info>,
    pub meteora_deployer_virtuals_ata: &'me AccountInfo<'info>,
    pub meteora_deployer_token_ata: &'me AccountInfo<'info>,
    pub vpool_virtuals_ata: &'me AccountInfo<'info>,
    pub vpool_token_ata: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub escrow_vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub virtuals_mint: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub virtuals_vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub virtuals_token_vault: &'me AccountInfo<'info>,
    pub token_token_vault: &'me AccountInfo<'info>,
    pub virtuals_vault_lp_mint: &'me AccountInfo<'info>,
    pub token_vault_lp_mint: &'me AccountInfo<'info>,
    pub virtuals_vault_lp: &'me AccountInfo<'info>,
    pub token_vault_lp: &'me AccountInfo<'info>,
    pub pool_virtuals_ata: &'me AccountInfo<'info>,
    pub pool_token_ata: &'me AccountInfo<'info>,
    pub meteora_deployer_pool_lp: &'me AccountInfo<'info>,
    pub protocol_virtuals_fee: &'me AccountInfo<'info>,
    pub protocol_token_fee: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_metadata: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub mint_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub dynamic_amm_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeMeteoraAccountsKeys {
    pub vpool: Pubkey,
    pub meteora_deployer: Pubkey,
    pub meteora_deployer_virtuals_ata: Pubkey,
    pub meteora_deployer_token_ata: Pubkey,
    pub vpool_virtuals_ata: Pubkey,
    pub vpool_token_ata: Pubkey,
    pub lock_escrow: Pubkey,
    pub escrow_vault: Pubkey,
    pub pool: Pubkey,
    pub config: Pubkey,
    pub lp_mint: Pubkey,
    pub virtuals_mint: Pubkey,
    pub token_mint: Pubkey,
    pub virtuals_vault: Pubkey,
    pub token_vault: Pubkey,
    pub virtuals_token_vault: Pubkey,
    pub token_token_vault: Pubkey,
    pub virtuals_vault_lp_mint: Pubkey,
    pub token_vault_lp_mint: Pubkey,
    pub virtuals_vault_lp: Pubkey,
    pub token_vault_lp: Pubkey,
    pub pool_virtuals_ata: Pubkey,
    pub pool_token_ata: Pubkey,
    pub meteora_deployer_pool_lp: Pubkey,
    pub protocol_virtuals_fee: Pubkey,
    pub protocol_token_fee: Pubkey,
    pub payer: Pubkey,
    pub token_metadata: Pubkey,
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub dynamic_amm_program: Pubkey,
}
impl From<InitializeMeteoraAccountsAccounts<'_, '_>> for InitializeMeteoraAccountsKeys {
    fn from(accounts: InitializeMeteoraAccountsAccounts) -> Self {
        Self {
            vpool: *accounts.vpool.key,
            meteora_deployer: *accounts.meteora_deployer.key,
            meteora_deployer_virtuals_ata: *accounts.meteora_deployer_virtuals_ata.key,
            meteora_deployer_token_ata: *accounts.meteora_deployer_token_ata.key,
            vpool_virtuals_ata: *accounts.vpool_virtuals_ata.key,
            vpool_token_ata: *accounts.vpool_token_ata.key,
            lock_escrow: *accounts.lock_escrow.key,
            escrow_vault: *accounts.escrow_vault.key,
            pool: *accounts.pool.key,
            config: *accounts.config.key,
            lp_mint: *accounts.lp_mint.key,
            virtuals_mint: *accounts.virtuals_mint.key,
            token_mint: *accounts.token_mint.key,
            virtuals_vault: *accounts.virtuals_vault.key,
            token_vault: *accounts.token_vault.key,
            virtuals_token_vault: *accounts.virtuals_token_vault.key,
            token_token_vault: *accounts.token_token_vault.key,
            virtuals_vault_lp_mint: *accounts.virtuals_vault_lp_mint.key,
            token_vault_lp_mint: *accounts.token_vault_lp_mint.key,
            virtuals_vault_lp: *accounts.virtuals_vault_lp.key,
            token_vault_lp: *accounts.token_vault_lp.key,
            pool_virtuals_ata: *accounts.pool_virtuals_ata.key,
            pool_token_ata: *accounts.pool_token_ata.key,
            meteora_deployer_pool_lp: *accounts.meteora_deployer_pool_lp.key,
            protocol_virtuals_fee: *accounts.protocol_virtuals_fee.key,
            protocol_token_fee: *accounts.protocol_token_fee.key,
            payer: *accounts.payer.key,
            token_metadata: *accounts.token_metadata.key,
            rent: *accounts.rent.key,
            mint_metadata: *accounts.mint_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            dynamic_amm_program: *accounts.dynamic_amm_program.key,
        }
    }
}
impl From<InitializeMeteoraAccountsKeys>
for [AccountMeta; INITIALIZE_METEORA_ACCOUNTS_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeMeteoraAccountsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_deployer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_deployer_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_deployer_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lock_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.virtuals_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtuals_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_deployer_pool_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_virtuals_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_token_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
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
            AccountMeta {
                pubkey: keys.dynamic_amm_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_METEORA_ACCOUNTS_IX_ACCOUNTS_LEN]>
for InitializeMeteoraAccountsKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_METEORA_ACCOUNTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vpool: pubkeys[0],
            meteora_deployer: pubkeys[1],
            meteora_deployer_virtuals_ata: pubkeys[2],
            meteora_deployer_token_ata: pubkeys[3],
            vpool_virtuals_ata: pubkeys[4],
            vpool_token_ata: pubkeys[5],
            lock_escrow: pubkeys[6],
            escrow_vault: pubkeys[7],
            pool: pubkeys[8],
            config: pubkeys[9],
            lp_mint: pubkeys[10],
            virtuals_mint: pubkeys[11],
            token_mint: pubkeys[12],
            virtuals_vault: pubkeys[13],
            token_vault: pubkeys[14],
            virtuals_token_vault: pubkeys[15],
            token_token_vault: pubkeys[16],
            virtuals_vault_lp_mint: pubkeys[17],
            token_vault_lp_mint: pubkeys[18],
            virtuals_vault_lp: pubkeys[19],
            token_vault_lp: pubkeys[20],
            pool_virtuals_ata: pubkeys[21],
            pool_token_ata: pubkeys[22],
            meteora_deployer_pool_lp: pubkeys[23],
            protocol_virtuals_fee: pubkeys[24],
            protocol_token_fee: pubkeys[25],
            payer: pubkeys[26],
            token_metadata: pubkeys[27],
            rent: pubkeys[28],
            mint_metadata: pubkeys[29],
            metadata_program: pubkeys[30],
            vault_program: pubkeys[31],
            token_program: pubkeys[32],
            associated_token_program: pubkeys[33],
            system_program: pubkeys[34],
            dynamic_amm_program: pubkeys[35],
        }
    }
}
impl<'info> From<InitializeMeteoraAccountsAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_METEORA_ACCOUNTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeMeteoraAccountsAccounts<'_, 'info>) -> Self {
        [
            accounts.vpool.clone(),
            accounts.meteora_deployer.clone(),
            accounts.meteora_deployer_virtuals_ata.clone(),
            accounts.meteora_deployer_token_ata.clone(),
            accounts.vpool_virtuals_ata.clone(),
            accounts.vpool_token_ata.clone(),
            accounts.lock_escrow.clone(),
            accounts.escrow_vault.clone(),
            accounts.pool.clone(),
            accounts.config.clone(),
            accounts.lp_mint.clone(),
            accounts.virtuals_mint.clone(),
            accounts.token_mint.clone(),
            accounts.virtuals_vault.clone(),
            accounts.token_vault.clone(),
            accounts.virtuals_token_vault.clone(),
            accounts.token_token_vault.clone(),
            accounts.virtuals_vault_lp_mint.clone(),
            accounts.token_vault_lp_mint.clone(),
            accounts.virtuals_vault_lp.clone(),
            accounts.token_vault_lp.clone(),
            accounts.pool_virtuals_ata.clone(),
            accounts.pool_token_ata.clone(),
            accounts.meteora_deployer_pool_lp.clone(),
            accounts.protocol_virtuals_fee.clone(),
            accounts.protocol_token_fee.clone(),
            accounts.payer.clone(),
            accounts.token_metadata.clone(),
            accounts.rent.clone(),
            accounts.mint_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.dynamic_amm_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_METEORA_ACCOUNTS_IX_ACCOUNTS_LEN]>
for InitializeMeteoraAccountsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_METEORA_ACCOUNTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vpool: &arr[0],
            meteora_deployer: &arr[1],
            meteora_deployer_virtuals_ata: &arr[2],
            meteora_deployer_token_ata: &arr[3],
            vpool_virtuals_ata: &arr[4],
            vpool_token_ata: &arr[5],
            lock_escrow: &arr[6],
            escrow_vault: &arr[7],
            pool: &arr[8],
            config: &arr[9],
            lp_mint: &arr[10],
            virtuals_mint: &arr[11],
            token_mint: &arr[12],
            virtuals_vault: &arr[13],
            token_vault: &arr[14],
            virtuals_token_vault: &arr[15],
            token_token_vault: &arr[16],
            virtuals_vault_lp_mint: &arr[17],
            token_vault_lp_mint: &arr[18],
            virtuals_vault_lp: &arr[19],
            token_vault_lp: &arr[20],
            pool_virtuals_ata: &arr[21],
            pool_token_ata: &arr[22],
            meteora_deployer_pool_lp: &arr[23],
            protocol_virtuals_fee: &arr[24],
            protocol_token_fee: &arr[25],
            payer: &arr[26],
            token_metadata: &arr[27],
            rent: &arr[28],
            mint_metadata: &arr[29],
            metadata_program: &arr[30],
            vault_program: &arr[31],
            token_program: &arr[32],
            associated_token_program: &arr[33],
            system_program: &arr[34],
            dynamic_amm_program: &arr[35],
        }
    }
}
pub const INITIALIZE_METEORA_ACCOUNTS_IX_DISCM: [u8; 8usize] = [
    53, 12, 118, 158, 253, 239, 185, 214,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMeteoraAccountsIxData;
impl InitializeMeteoraAccountsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_METEORA_ACCOUNTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_METEORA_ACCOUNTS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_meteora_accounts_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeMeteoraAccountsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_METEORA_ACCOUNTS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeMeteoraAccountsIxData.try_to_vec()?,
    })
}
pub fn initialize_meteora_accounts_ix(
    keys: InitializeMeteoraAccountsKeys,
) -> std::io::Result<Instruction> {
    initialize_meteora_accounts_ix_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, keys)
}
pub fn initialize_meteora_accounts_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeMeteoraAccountsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeMeteoraAccountsKeys = accounts.into();
    let ix = initialize_meteora_accounts_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_meteora_accounts_invoke(
    accounts: InitializeMeteoraAccountsAccounts<'_, '_>,
) -> ProgramResult {
    initialize_meteora_accounts_invoke_with_program_id(
        VIRTUALS_PROGRAM_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_meteora_accounts_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeMeteoraAccountsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeMeteoraAccountsKeys = accounts.into();
    let ix = initialize_meteora_accounts_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_meteora_accounts_invoke_signed(
    accounts: InitializeMeteoraAccountsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_meteora_accounts_invoke_signed_with_program_id(
        VIRTUALS_PROGRAM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_meteora_accounts_verify_account_keys(
    accounts: InitializeMeteoraAccountsAccounts<'_, '_>,
    keys: InitializeMeteoraAccountsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vpool.key, keys.vpool),
        (*accounts.meteora_deployer.key, keys.meteora_deployer),
        (
            *accounts.meteora_deployer_virtuals_ata.key,
            keys.meteora_deployer_virtuals_ata,
        ),
        (*accounts.meteora_deployer_token_ata.key, keys.meteora_deployer_token_ata),
        (*accounts.vpool_virtuals_ata.key, keys.vpool_virtuals_ata),
        (*accounts.vpool_token_ata.key, keys.vpool_token_ata),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.escrow_vault.key, keys.escrow_vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.config.key, keys.config),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.virtuals_mint.key, keys.virtuals_mint),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.virtuals_vault.key, keys.virtuals_vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.virtuals_token_vault.key, keys.virtuals_token_vault),
        (*accounts.token_token_vault.key, keys.token_token_vault),
        (*accounts.virtuals_vault_lp_mint.key, keys.virtuals_vault_lp_mint),
        (*accounts.token_vault_lp_mint.key, keys.token_vault_lp_mint),
        (*accounts.virtuals_vault_lp.key, keys.virtuals_vault_lp),
        (*accounts.token_vault_lp.key, keys.token_vault_lp),
        (*accounts.pool_virtuals_ata.key, keys.pool_virtuals_ata),
        (*accounts.pool_token_ata.key, keys.pool_token_ata),
        (*accounts.meteora_deployer_pool_lp.key, keys.meteora_deployer_pool_lp),
        (*accounts.protocol_virtuals_fee.key, keys.protocol_virtuals_fee),
        (*accounts.protocol_token_fee.key, keys.protocol_token_fee),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_metadata.key, keys.token_metadata),
        (*accounts.rent.key, keys.rent),
        (*accounts.mint_metadata.key, keys.mint_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.dynamic_amm_program.key, keys.dynamic_amm_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_meteora_accounts_verify_writable_privileges<'me, 'info>(
    accounts: InitializeMeteoraAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vpool,
        accounts.meteora_deployer,
        accounts.meteora_deployer_virtuals_ata,
        accounts.meteora_deployer_token_ata,
        accounts.vpool_virtuals_ata,
        accounts.vpool_token_ata,
        accounts.lock_escrow,
        accounts.escrow_vault,
        accounts.pool,
        accounts.lp_mint,
        accounts.virtuals_vault,
        accounts.token_vault,
        accounts.virtuals_token_vault,
        accounts.token_token_vault,
        accounts.virtuals_vault_lp_mint,
        accounts.token_vault_lp_mint,
        accounts.virtuals_vault_lp,
        accounts.token_vault_lp,
        accounts.pool_virtuals_ata,
        accounts.pool_token_ata,
        accounts.meteora_deployer_pool_lp,
        accounts.protocol_virtuals_fee,
        accounts.protocol_token_fee,
        accounts.payer,
        accounts.token_metadata,
        accounts.mint_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_meteora_accounts_verify_signer_privileges<'me, 'info>(
    accounts: InitializeMeteoraAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_meteora_accounts_verify_account_privileges<'me, 'info>(
    accounts: InitializeMeteoraAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_meteora_accounts_verify_writable_privileges(accounts)?;
    initialize_meteora_accounts_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LAUNCH_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct LaunchAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub creator_virtuals_ata: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub platform_prototype: &'me AccountInfo<'info>,
    pub platform_prototype_virtuals_ata: &'me AccountInfo<'info>,
    pub vpool: &'me AccountInfo<'info>,
    pub token_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LaunchKeys {
    pub creator: Pubkey,
    pub creator_virtuals_ata: Pubkey,
    pub token_mint: Pubkey,
    pub platform_prototype: Pubkey,
    pub platform_prototype_virtuals_ata: Pubkey,
    pub vpool: Pubkey,
    pub token_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<LaunchAccounts<'_, '_>> for LaunchKeys {
    fn from(accounts: LaunchAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            creator_virtuals_ata: *accounts.creator_virtuals_ata.key,
            token_mint: *accounts.token_mint.key,
            platform_prototype: *accounts.platform_prototype.key,
            platform_prototype_virtuals_ata: *accounts
                .platform_prototype_virtuals_ata
                .key,
            vpool: *accounts.vpool.key,
            token_metadata: *accounts.token_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<LaunchKeys> for [AccountMeta; LAUNCH_IX_ACCOUNTS_LEN] {
    fn from(keys: LaunchKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_prototype,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_prototype_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_metadata,
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
                pubkey: keys.associated_token_program,
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
        ]
    }
}
impl From<[Pubkey; LAUNCH_IX_ACCOUNTS_LEN]> for LaunchKeys {
    fn from(pubkeys: [Pubkey; LAUNCH_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            creator_virtuals_ata: pubkeys[1],
            token_mint: pubkeys[2],
            platform_prototype: pubkeys[3],
            platform_prototype_virtuals_ata: pubkeys[4],
            vpool: pubkeys[5],
            token_metadata: pubkeys[6],
            metadata_program: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            system_program: pubkeys[10],
            rent: pubkeys[11],
        }
    }
}
impl<'info> From<LaunchAccounts<'_, 'info>>
for [AccountInfo<'info>; LAUNCH_IX_ACCOUNTS_LEN] {
    fn from(accounts: LaunchAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.creator_virtuals_ata.clone(),
            accounts.token_mint.clone(),
            accounts.platform_prototype.clone(),
            accounts.platform_prototype_virtuals_ata.clone(),
            accounts.vpool.clone(),
            accounts.token_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LAUNCH_IX_ACCOUNTS_LEN]>
for LaunchAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; LAUNCH_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: &arr[0],
            creator_virtuals_ata: &arr[1],
            token_mint: &arr[2],
            platform_prototype: &arr[3],
            platform_prototype_virtuals_ata: &arr[4],
            vpool: &arr[5],
            token_metadata: &arr[6],
            metadata_program: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
            system_program: &arr[10],
            rent: &arr[11],
        }
    }
}
pub const LAUNCH_IX_DISCM: [u8; 8usize] = [153, 241, 93, 225, 22, 69, 74, 61];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LaunchIxArgs {
    pub symbol: String,
    pub name: String,
    pub uri: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LaunchIxData(pub LaunchIxArgs);
impl From<LaunchIxArgs> for LaunchIxData {
    fn from(args: LaunchIxArgs) -> Self {
        Self(args)
    }
}
impl LaunchIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LAUNCH_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(LaunchIxArgs { symbol, name, uri }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LAUNCH_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn launch_ix_with_program_id(
    program_id: Pubkey,
    keys: LaunchKeys,
    args: LaunchIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LAUNCH_IX_ACCOUNTS_LEN] = keys.into();
    let data: LaunchIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn launch_ix(keys: LaunchKeys, args: LaunchIxArgs) -> std::io::Result<Instruction> {
    launch_ix_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, keys, args)
}
pub fn launch_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LaunchAccounts<'_, '_>,
    args: LaunchIxArgs,
) -> ProgramResult {
    let keys: LaunchKeys = accounts.into();
    let ix = launch_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn launch_invoke(
    accounts: LaunchAccounts<'_, '_>,
    args: LaunchIxArgs,
) -> ProgramResult {
    launch_invoke_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, accounts, args)
}
pub fn launch_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LaunchAccounts<'_, '_>,
    args: LaunchIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LaunchKeys = accounts.into();
    let ix = launch_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn launch_invoke_signed(
    accounts: LaunchAccounts<'_, '_>,
    args: LaunchIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    launch_invoke_signed_with_program_id(
        VIRTUALS_PROGRAM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn launch_verify_account_keys(
    accounts: LaunchAccounts<'_, '_>,
    keys: LaunchKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.creator_virtuals_ata.key, keys.creator_virtuals_ata),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.platform_prototype.key, keys.platform_prototype),
        (
            *accounts.platform_prototype_virtuals_ata.key,
            keys.platform_prototype_virtuals_ata,
        ),
        (*accounts.vpool.key, keys.vpool),
        (*accounts.token_metadata.key, keys.token_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn launch_verify_writable_privileges<'me, 'info>(
    accounts: LaunchAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator,
        accounts.creator_virtuals_ata,
        accounts.token_mint,
        accounts.platform_prototype,
        accounts.platform_prototype_virtuals_ata,
        accounts.vpool,
        accounts.token_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn launch_verify_signer_privileges<'me, 'info>(
    accounts: LaunchAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn launch_verify_account_privileges<'me, 'info>(
    accounts: LaunchAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    launch_verify_writable_privileges(accounts)?;
    launch_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct SellAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vpool: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub user_virtuals_ata: &'me AccountInfo<'info>,
    pub user_token_ata: &'me AccountInfo<'info>,
    pub vpool_token_ata: &'me AccountInfo<'info>,
    pub platform_prototype: &'me AccountInfo<'info>,
    pub platform_prototype_virtuals_ata: &'me AccountInfo<'info>,
    pub vpool_virtuals_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellKeys {
    pub user: Pubkey,
    pub vpool: Pubkey,
    pub token_mint: Pubkey,
    pub user_virtuals_ata: Pubkey,
    pub user_token_ata: Pubkey,
    pub vpool_token_ata: Pubkey,
    pub platform_prototype: Pubkey,
    pub platform_prototype_virtuals_ata: Pubkey,
    pub vpool_virtuals_ata: Pubkey,
    pub token_program: Pubkey,
}
impl From<SellAccounts<'_, '_>> for SellKeys {
    fn from(accounts: SellAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vpool: *accounts.vpool.key,
            token_mint: *accounts.token_mint.key,
            user_virtuals_ata: *accounts.user_virtuals_ata.key,
            user_token_ata: *accounts.user_token_ata.key,
            vpool_token_ata: *accounts.vpool_token_ata.key,
            platform_prototype: *accounts.platform_prototype.key,
            platform_prototype_virtuals_ata: *accounts
                .platform_prototype_virtuals_ata
                .key,
            vpool_virtuals_ata: *accounts.vpool_virtuals_ata.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SellKeys> for [AccountMeta; SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: SellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_prototype,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_prototype_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool_virtuals_ata,
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
impl From<[Pubkey; SELL_IX_ACCOUNTS_LEN]> for SellKeys {
    fn from(pubkeys: [Pubkey; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            vpool: pubkeys[1],
            token_mint: pubkeys[2],
            user_virtuals_ata: pubkeys[3],
            user_token_ata: pubkeys[4],
            vpool_token_ata: pubkeys[5],
            platform_prototype: pubkeys[6],
            platform_prototype_virtuals_ata: pubkeys[7],
            vpool_virtuals_ata: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<SellAccounts<'_, 'info>>
for [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vpool.clone(),
            accounts.token_mint.clone(),
            accounts.user_virtuals_ata.clone(),
            accounts.user_token_ata.clone(),
            accounts.vpool_token_ata.clone(),
            accounts.platform_prototype.clone(),
            accounts.platform_prototype_virtuals_ata.clone(),
            accounts.vpool_virtuals_ata.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]>
for SellAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            vpool: &arr[1],
            token_mint: &arr[2],
            user_virtuals_ata: &arr[3],
            user_token_ata: &arr[4],
            vpool_token_ata: &arr[5],
            platform_prototype: &arr[6],
            platform_prototype_virtuals_ata: &arr[7],
            vpool_virtuals_ata: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const SELL_IX_DISCM: [u8; 8usize] = [51, 230, 133, 164, 1, 127, 131, 173];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellIxArgs {
    pub amount: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellIxData(pub SellIxArgs);
impl From<SellIxArgs> for SellIxData {
    fn from(args: SellIxArgs) -> Self {
        Self(args)
    }
}
impl SellIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SellIxArgs {
                amount,
                min_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sell_ix_with_program_id(
    program_id: Pubkey,
    keys: SellKeys,
    args: SellIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sell_ix(keys: SellKeys, args: SellIxArgs) -> std::io::Result<Instruction> {
    sell_ix_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, keys, args)
}
pub fn sell_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
) -> ProgramResult {
    let keys: SellKeys = accounts.into();
    let ix = sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_invoke(accounts: SellAccounts<'_, '_>, args: SellIxArgs) -> ProgramResult {
    sell_invoke_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, accounts, args)
}
pub fn sell_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellKeys = accounts.into();
    let ix = sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sell_invoke_signed(
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_invoke_signed_with_program_id(
        VIRTUALS_PROGRAM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn sell_verify_account_keys(
    accounts: SellAccounts<'_, '_>,
    keys: SellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vpool.key, keys.vpool),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.user_virtuals_ata.key, keys.user_virtuals_ata),
        (*accounts.user_token_ata.key, keys.user_token_ata),
        (*accounts.vpool_token_ata.key, keys.vpool_token_ata),
        (*accounts.platform_prototype.key, keys.platform_prototype),
        (
            *accounts.platform_prototype_virtuals_ata.key,
            keys.platform_prototype_virtuals_ata,
        ),
        (*accounts.vpool_virtuals_ata.key, keys.vpool_virtuals_ata),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sell_verify_writable_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vpool,
        accounts.user_virtuals_ata,
        accounts.user_token_ata,
        accounts.vpool_token_ata,
        accounts.platform_prototype,
        accounts.platform_prototype_virtuals_ata,
        accounts.vpool_virtuals_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_verify_signer_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_verify_account_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_verify_writable_privileges(accounts)?;
    sell_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_POOL_CREATOR_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePoolCreatorAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub new_creator: &'me AccountInfo<'info>,
    pub virtuals_mint: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub new_creator_virtuals_ata: &'me AccountInfo<'info>,
    pub new_creator_token_ata: &'me AccountInfo<'info>,
    pub vpool: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePoolCreatorKeys {
    pub creator: Pubkey,
    pub new_creator: Pubkey,
    pub virtuals_mint: Pubkey,
    pub token_mint: Pubkey,
    pub new_creator_virtuals_ata: Pubkey,
    pub new_creator_token_ata: Pubkey,
    pub vpool: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdatePoolCreatorAccounts<'_, '_>> for UpdatePoolCreatorKeys {
    fn from(accounts: UpdatePoolCreatorAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            new_creator: *accounts.new_creator.key,
            virtuals_mint: *accounts.virtuals_mint.key,
            token_mint: *accounts.token_mint.key,
            new_creator_virtuals_ata: *accounts.new_creator_virtuals_ata.key,
            new_creator_token_ata: *accounts.new_creator_token_ata.key,
            vpool: *accounts.vpool.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdatePoolCreatorKeys> for [AccountMeta; UPDATE_POOL_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePoolCreatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.virtuals_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_creator_virtuals_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_creator_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vpool,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; UPDATE_POOL_CREATOR_IX_ACCOUNTS_LEN]> for UpdatePoolCreatorKeys {
    fn from(pubkeys: [Pubkey; UPDATE_POOL_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            new_creator: pubkeys[1],
            virtuals_mint: pubkeys[2],
            token_mint: pubkeys[3],
            new_creator_virtuals_ata: pubkeys[4],
            new_creator_token_ata: pubkeys[5],
            vpool: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<UpdatePoolCreatorAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_POOL_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePoolCreatorAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.new_creator.clone(),
            accounts.virtuals_mint.clone(),
            accounts.token_mint.clone(),
            accounts.new_creator_virtuals_ata.clone(),
            accounts.new_creator_token_ata.clone(),
            accounts.vpool.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_POOL_CREATOR_IX_ACCOUNTS_LEN]>
for UpdatePoolCreatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_POOL_CREATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            creator: &arr[0],
            new_creator: &arr[1],
            virtuals_mint: &arr[2],
            token_mint: &arr[3],
            new_creator_virtuals_ata: &arr[4],
            new_creator_token_ata: &arr[5],
            vpool: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const UPDATE_POOL_CREATOR_IX_DISCM: [u8; 8usize] = [
    113, 225, 166, 185, 94, 231, 96, 28,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePoolCreatorIxData;
impl UpdatePoolCreatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_POOL_CREATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_POOL_CREATOR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_pool_creator_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePoolCreatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_POOL_CREATOR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdatePoolCreatorIxData.try_to_vec()?,
    })
}
pub fn update_pool_creator_ix(
    keys: UpdatePoolCreatorKeys,
) -> std::io::Result<Instruction> {
    update_pool_creator_ix_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, keys)
}
pub fn update_pool_creator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolCreatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdatePoolCreatorKeys = accounts.into();
    let ix = update_pool_creator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_pool_creator_invoke(
    accounts: UpdatePoolCreatorAccounts<'_, '_>,
) -> ProgramResult {
    update_pool_creator_invoke_with_program_id(VIRTUALS_PROGRAM_PROGRAM_ID, accounts)
}
pub fn update_pool_creator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolCreatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePoolCreatorKeys = accounts.into();
    let ix = update_pool_creator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_pool_creator_invoke_signed(
    accounts: UpdatePoolCreatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_pool_creator_invoke_signed_with_program_id(
        VIRTUALS_PROGRAM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_pool_creator_verify_account_keys(
    accounts: UpdatePoolCreatorAccounts<'_, '_>,
    keys: UpdatePoolCreatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.new_creator.key, keys.new_creator),
        (*accounts.virtuals_mint.key, keys.virtuals_mint),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.new_creator_virtuals_ata.key, keys.new_creator_virtuals_ata),
        (*accounts.new_creator_token_ata.key, keys.new_creator_token_ata),
        (*accounts.vpool.key, keys.vpool),
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
pub fn update_pool_creator_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePoolCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator,
        accounts.new_creator_virtuals_ata,
        accounts.new_creator_token_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_pool_creator_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePoolCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_pool_creator_verify_account_privileges<'me, 'info>(
    accounts: UpdatePoolCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_pool_creator_verify_writable_privileges(accounts)?;
    update_pool_creator_verify_signer_privileges(accounts)?;
    Ok(())
}
