use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum AmmProgramIx {
    Buy(BuyIxArgs),
    Claim,
    Create(CreateIxArgs),
    QuoteBuy(QuoteBuyIxArgs),
    QuoteSell(QuoteSellIxArgs),
    Sell(SellIxArgs),
}
impl AmmProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&BUY_IX_DISCM) {
            let mut reader = &buf[BUY_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SwapParams>::deserialize(&mut reader)?
            };
            return Ok(Self::Buy(BuyIxArgs { params }));
        }
        if buf.starts_with(&CLAIM_IX_DISCM) {
            return Ok(Self::Claim);
        }
        if buf.starts_with(&CREATE_IX_DISCM) {
            let mut reader = &buf[CREATE_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <CreateParams>::deserialize(&mut reader)?
            };
            return Ok(Self::Create(CreateIxArgs { params }));
        }
        if buf.starts_with(&QUOTE_BUY_IX_DISCM) {
            let mut reader = &buf[QUOTE_BUY_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SwapParams>::deserialize(&mut reader)?
            };
            return Ok(Self::QuoteBuy(QuoteBuyIxArgs { params }));
        }
        if buf.starts_with(&QUOTE_SELL_IX_DISCM) {
            let mut reader = &buf[QUOTE_SELL_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SwapParams>::deserialize(&mut reader)?
            };
            return Ok(Self::QuoteSell(QuoteSellIxArgs { params }));
        }
        if buf.starts_with(&SELL_IX_DISCM) {
            let mut reader = &buf[SELL_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SwapParams>::deserialize(&mut reader)?
            };
            return Ok(Self::Sell(SellIxArgs { params }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Buy(args) => {
                writer.write_all(&BUY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::Claim => writer.write_all(&CLAIM_IX_DISCM),
            Self::Create(args) => {
                writer.write_all(&CREATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::QuoteBuy(args) => {
                writer.write_all(&QUOTE_BUY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::QuoteSell(args) => {
                writer.write_all(&QUOTE_SELL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::Sell(args) => {
                writer.write_all(&SELL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
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
pub const BUY_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct BuyAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub user_ta_a: &'me AccountInfo<'info>,
    pub user_ta_b: &'me AccountInfo<'info>,
    pub vault_a: &'me AccountInfo<'info>,
    pub vault_b: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyKeys {
    pub pool: Pubkey,
    pub user: Pubkey,
    pub owner: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub user_ta_a: Pubkey,
    pub user_ta_b: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub system_program: Pubkey,
    pub program: Pubkey,
}
impl From<BuyAccounts<'_, '_>> for BuyKeys {
    fn from(accounts: BuyAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            user: *accounts.user.key,
            owner: *accounts.owner.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            user_ta_a: *accounts.user_ta_a.key,
            user_ta_b: *accounts.user_ta_b.key,
            vault_a: *accounts.vault_a.key,
            vault_b: *accounts.vault_b.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            system_program: *accounts.system_program.key,
            program: *accounts.program.key,
        }
    }
}
impl From<BuyKeys> for [AccountMeta; BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_ta_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_ta_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
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
impl From<[Pubkey; BUY_IX_ACCOUNTS_LEN]> for BuyKeys {
    fn from(pubkeys: [Pubkey; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            user: pubkeys[1],
            owner: pubkeys[2],
            mint_a: pubkeys[3],
            mint_b: pubkeys[4],
            user_ta_a: pubkeys[5],
            user_ta_b: pubkeys[6],
            vault_a: pubkeys[7],
            vault_b: pubkeys[8],
            token_program_a: pubkeys[9],
            token_program_b: pubkeys[10],
            system_program: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<BuyAccounts<'_, 'info>> for [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.user.clone(),
            accounts.owner.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.user_ta_a.clone(),
            accounts.user_ta_b.clone(),
            accounts.vault_a.clone(),
            accounts.vault_b.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.system_program.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]>
for BuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            user: &arr[1],
            owner: &arr[2],
            mint_a: &arr[3],
            mint_b: &arr[4],
            user_ta_a: &arr[5],
            user_ta_b: &arr[6],
            vault_a: &arr[7],
            vault_b: &arr[8],
            token_program_a: &arr[9],
            token_program_b: &arr[10],
            system_program: &arr[11],
            program: &arr[12],
        }
    }
}
pub const BUY_IX_DISCM: [u8; 8usize] = [102, 6, 61, 18, 1, 218, 235, 234];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyIxArgs {
    pub params: SwapParams,
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
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SwapParams>::deserialize(&mut reader)?
        };
        Ok(Self(BuyIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
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
    buy_ix_with_program_id(AMM_PROGRAM_ID, keys, args)
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
    buy_invoke_with_program_id(AMM_PROGRAM_ID, accounts, args)
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
    buy_invoke_signed_with_program_id(AMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn buy_verify_account_keys(
    accounts: BuyAccounts<'_, '_>,
    keys: BuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.user.key, keys.user),
        (*accounts.owner.key, keys.owner),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.user_ta_a.key, keys.user_ta_a),
        (*accounts.user_ta_b.key, keys.user_ta_b),
        (*accounts.vault_a.key, keys.vault_a),
        (*accounts.vault_b.key, keys.vault_b),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program.key, keys.program),
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
        accounts.pool,
        accounts.user_ta_a,
        accounts.user_ta_b,
        accounts.vault_a,
        accounts.vault_b,
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
pub const CLAIM_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct ClaimAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub claimer: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub vault_a: &'me AccountInfo<'info>,
    pub receiver_ta_a: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimKeys {
    pub pool: Pubkey,
    pub system_program: Pubkey,
    pub claimer: Pubkey,
    pub mint_a: Pubkey,
    pub vault_a: Pubkey,
    pub receiver_ta_a: Pubkey,
    pub token_program_a: Pubkey,
}
impl From<ClaimAccounts<'_, '_>> for ClaimKeys {
    fn from(accounts: ClaimAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            system_program: *accounts.system_program.key,
            claimer: *accounts.claimer.key,
            mint_a: *accounts.mint_a.key,
            vault_a: *accounts.vault_a.key,
            receiver_ta_a: *accounts.receiver_ta_a.key,
            token_program_a: *accounts.token_program_a.key,
        }
    }
}
impl From<ClaimKeys> for [AccountMeta; CLAIM_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.claimer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver_ta_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_IX_ACCOUNTS_LEN]> for ClaimKeys {
    fn from(pubkeys: [Pubkey; CLAIM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            system_program: pubkeys[1],
            claimer: pubkeys[2],
            mint_a: pubkeys[3],
            vault_a: pubkeys[4],
            receiver_ta_a: pubkeys[5],
            token_program_a: pubkeys[6],
        }
    }
}
impl<'info> From<ClaimAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.system_program.clone(),
            accounts.claimer.clone(),
            accounts.mint_a.clone(),
            accounts.vault_a.clone(),
            accounts.receiver_ta_a.clone(),
            accounts.token_program_a.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN]>
for ClaimAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            system_program: &arr[1],
            claimer: &arr[2],
            mint_a: &arr[3],
            vault_a: &arr[4],
            receiver_ta_a: &arr[5],
            token_program_a: &arr[6],
        }
    }
}
pub const CLAIM_IX_DISCM: [u8; 8usize] = [62, 198, 214, 193, 213, 159, 108, 210];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimIxData;
impl ClaimIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimIxData.try_to_vec()?,
    })
}
pub fn claim_ix(keys: ClaimKeys) -> std::io::Result<Instruction> {
    claim_ix_with_program_id(AMM_PROGRAM_ID, keys)
}
pub fn claim_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimKeys = accounts.into();
    let ix = claim_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_invoke(accounts: ClaimAccounts<'_, '_>) -> ProgramResult {
    claim_invoke_with_program_id(AMM_PROGRAM_ID, accounts)
}
pub fn claim_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimKeys = accounts.into();
    let ix = claim_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_invoke_signed(
    accounts: ClaimAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_invoke_signed_with_program_id(AMM_PROGRAM_ID, accounts, seeds)
}
pub fn claim_verify_account_keys(
    accounts: ClaimAccounts<'_, '_>,
    keys: ClaimKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.claimer.key, keys.claimer),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.vault_a.key, keys.vault_a),
        (*accounts.receiver_ta_a.key, keys.receiver_ta_a),
        (*accounts.token_program_a.key, keys.token_program_a),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_verify_writable_privileges<'me, 'info>(
    accounts: ClaimAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool, accounts.vault_a, accounts.receiver_ta_a] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_verify_signer_privileges<'me, 'info>(
    accounts: ClaimAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.claimer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_verify_account_privileges<'me, 'info>(
    accounts: ClaimAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_verify_writable_privileges(accounts)?;
    claim_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct CreateAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_wallet_authority: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub token_wallet_b: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub vault_a: &'me AccountInfo<'info>,
    pub vault_b: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateKeys {
    pub payer: Pubkey,
    pub owner: Pubkey,
    pub token_wallet_authority: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub token_wallet_b: Pubkey,
    pub pool: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateAccounts<'_, '_>> for CreateKeys {
    fn from(accounts: CreateAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            owner: *accounts.owner.key,
            token_wallet_authority: *accounts.token_wallet_authority.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            token_wallet_b: *accounts.token_wallet_b.key,
            pool: *accounts.pool.key,
            vault_a: *accounts.vault_a.key,
            vault_b: *accounts.vault_b.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateKeys> for [AccountMeta; CREATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_wallet_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_wallet_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
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
impl From<[Pubkey; CREATE_IX_ACCOUNTS_LEN]> for CreateKeys {
    fn from(pubkeys: [Pubkey; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            owner: pubkeys[1],
            token_wallet_authority: pubkeys[2],
            mint_a: pubkeys[3],
            mint_b: pubkeys[4],
            token_wallet_b: pubkeys[5],
            pool: pubkeys[6],
            vault_a: pubkeys[7],
            vault_b: pubkeys[8],
            token_program_a: pubkeys[9],
            token_program_b: pubkeys[10],
            system_program: pubkeys[11],
            rent: pubkeys[12],
        }
    }
}
impl<'info> From<CreateAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.owner.clone(),
            accounts.token_wallet_authority.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.token_wallet_b.clone(),
            accounts.pool.clone(),
            accounts.vault_a.clone(),
            accounts.vault_b.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]>
for CreateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            owner: &arr[1],
            token_wallet_authority: &arr[2],
            mint_a: &arr[3],
            mint_b: &arr[4],
            token_wallet_b: &arr[5],
            pool: &arr[6],
            vault_a: &arr[7],
            vault_b: &arr[8],
            token_program_a: &arr[9],
            token_program_b: &arr[10],
            system_program: &arr[11],
            rent: &arr[12],
        }
    }
}
pub const CREATE_IX_DISCM: [u8; 8usize] = [24, 30, 200, 40, 5, 28, 7, 119];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateIxArgs {
    pub params: CreateParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateIxData(pub CreateIxArgs);
impl From<CreateIxArgs> for CreateIxData {
    fn from(args: CreateIxArgs) -> Self {
        Self(args)
    }
}
impl CreateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <CreateParams>::deserialize(&mut reader)?
        };
        Ok(Self(CreateIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateKeys,
    args: CreateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_ix(keys: CreateKeys, args: CreateIxArgs) -> std::io::Result<Instruction> {
    create_ix_with_program_id(AMM_PROGRAM_ID, keys, args)
}
pub fn create_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
) -> ProgramResult {
    let keys: CreateKeys = accounts.into();
    let ix = create_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_invoke(
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
) -> ProgramResult {
    create_invoke_with_program_id(AMM_PROGRAM_ID, accounts, args)
}
pub fn create_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateKeys = accounts.into();
    let ix = create_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_invoke_signed(
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_invoke_signed_with_program_id(AMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_verify_account_keys(
    accounts: CreateAccounts<'_, '_>,
    keys: CreateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_wallet_authority.key, keys.token_wallet_authority),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.token_wallet_b.key, keys.token_wallet_b),
        (*accounts.pool.key, keys.pool),
        (*accounts.vault_a.key, keys.vault_a),
        (*accounts.vault_b.key, keys.vault_b),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_verify_writable_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.token_wallet_b,
        accounts.pool,
        accounts.vault_a,
        accounts.vault_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_verify_signer_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.payer,
        accounts.owner,
        accounts.token_wallet_authority,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_verify_account_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_verify_writable_privileges(accounts)?;
    create_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const QUOTE_BUY_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct QuoteBuyAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QuoteBuyKeys {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub user: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub program: Pubkey,
}
impl From<QuoteBuyAccounts<'_, '_>> for QuoteBuyKeys {
    fn from(accounts: QuoteBuyAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            owner: *accounts.owner.key,
            user: *accounts.user.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            program: *accounts.program.key,
        }
    }
}
impl From<QuoteBuyKeys> for [AccountMeta; QUOTE_BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: QuoteBuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_b,
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
impl From<[Pubkey; QUOTE_BUY_IX_ACCOUNTS_LEN]> for QuoteBuyKeys {
    fn from(pubkeys: [Pubkey; QUOTE_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            owner: pubkeys[1],
            user: pubkeys[2],
            mint_a: pubkeys[3],
            mint_b: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<QuoteBuyAccounts<'_, 'info>>
for [AccountInfo<'info>; QUOTE_BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: QuoteBuyAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.owner.clone(),
            accounts.user.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; QUOTE_BUY_IX_ACCOUNTS_LEN]>
for QuoteBuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; QUOTE_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            owner: &arr[1],
            user: &arr[2],
            mint_a: &arr[3],
            mint_b: &arr[4],
            program: &arr[5],
        }
    }
}
pub const QUOTE_BUY_IX_DISCM: [u8; 8usize] = [83, 9, 231, 110, 146, 31, 40, 12];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuoteBuyIxArgs {
    pub params: SwapParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuoteBuyIxData(pub QuoteBuyIxArgs);
impl From<QuoteBuyIxArgs> for QuoteBuyIxData {
    fn from(args: QuoteBuyIxArgs) -> Self {
        Self(args)
    }
}
impl QuoteBuyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != QUOTE_BUY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SwapParams>::deserialize(&mut reader)?
        };
        Ok(Self(QuoteBuyIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&QUOTE_BUY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn quote_buy_ix_with_program_id(
    program_id: Pubkey,
    keys: QuoteBuyKeys,
    args: QuoteBuyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; QUOTE_BUY_IX_ACCOUNTS_LEN] = keys.into();
    let data: QuoteBuyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn quote_buy_ix(
    keys: QuoteBuyKeys,
    args: QuoteBuyIxArgs,
) -> std::io::Result<Instruction> {
    quote_buy_ix_with_program_id(AMM_PROGRAM_ID, keys, args)
}
pub fn quote_buy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: QuoteBuyAccounts<'_, '_>,
    args: QuoteBuyIxArgs,
) -> ProgramResult {
    let keys: QuoteBuyKeys = accounts.into();
    let ix = quote_buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn quote_buy_invoke(
    accounts: QuoteBuyAccounts<'_, '_>,
    args: QuoteBuyIxArgs,
) -> ProgramResult {
    quote_buy_invoke_with_program_id(AMM_PROGRAM_ID, accounts, args)
}
pub fn quote_buy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: QuoteBuyAccounts<'_, '_>,
    args: QuoteBuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: QuoteBuyKeys = accounts.into();
    let ix = quote_buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn quote_buy_invoke_signed(
    accounts: QuoteBuyAccounts<'_, '_>,
    args: QuoteBuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    quote_buy_invoke_signed_with_program_id(AMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn quote_buy_verify_account_keys(
    accounts: QuoteBuyAccounts<'_, '_>,
    keys: QuoteBuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.owner.key, keys.owner),
        (*accounts.user.key, keys.user),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const QUOTE_SELL_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct QuoteSellAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QuoteSellKeys {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub user: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub program: Pubkey,
}
impl From<QuoteSellAccounts<'_, '_>> for QuoteSellKeys {
    fn from(accounts: QuoteSellAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            owner: *accounts.owner.key,
            user: *accounts.user.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            program: *accounts.program.key,
        }
    }
}
impl From<QuoteSellKeys> for [AccountMeta; QUOTE_SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: QuoteSellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_b,
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
impl From<[Pubkey; QUOTE_SELL_IX_ACCOUNTS_LEN]> for QuoteSellKeys {
    fn from(pubkeys: [Pubkey; QUOTE_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            owner: pubkeys[1],
            user: pubkeys[2],
            mint_a: pubkeys[3],
            mint_b: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<QuoteSellAccounts<'_, 'info>>
for [AccountInfo<'info>; QUOTE_SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: QuoteSellAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.owner.clone(),
            accounts.user.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; QUOTE_SELL_IX_ACCOUNTS_LEN]>
for QuoteSellAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; QUOTE_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            owner: &arr[1],
            user: &arr[2],
            mint_a: &arr[3],
            mint_b: &arr[4],
            program: &arr[5],
        }
    }
}
pub const QUOTE_SELL_IX_DISCM: [u8; 8usize] = [5, 178, 49, 206, 140, 231, 131, 145];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuoteSellIxArgs {
    pub params: SwapParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuoteSellIxData(pub QuoteSellIxArgs);
impl From<QuoteSellIxArgs> for QuoteSellIxData {
    fn from(args: QuoteSellIxArgs) -> Self {
        Self(args)
    }
}
impl QuoteSellIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != QUOTE_SELL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SwapParams>::deserialize(&mut reader)?
        };
        Ok(Self(QuoteSellIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&QUOTE_SELL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn quote_sell_ix_with_program_id(
    program_id: Pubkey,
    keys: QuoteSellKeys,
    args: QuoteSellIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; QUOTE_SELL_IX_ACCOUNTS_LEN] = keys.into();
    let data: QuoteSellIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn quote_sell_ix(
    keys: QuoteSellKeys,
    args: QuoteSellIxArgs,
) -> std::io::Result<Instruction> {
    quote_sell_ix_with_program_id(AMM_PROGRAM_ID, keys, args)
}
pub fn quote_sell_invoke_with_program_id(
    program_id: Pubkey,
    accounts: QuoteSellAccounts<'_, '_>,
    args: QuoteSellIxArgs,
) -> ProgramResult {
    let keys: QuoteSellKeys = accounts.into();
    let ix = quote_sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn quote_sell_invoke(
    accounts: QuoteSellAccounts<'_, '_>,
    args: QuoteSellIxArgs,
) -> ProgramResult {
    quote_sell_invoke_with_program_id(AMM_PROGRAM_ID, accounts, args)
}
pub fn quote_sell_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: QuoteSellAccounts<'_, '_>,
    args: QuoteSellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: QuoteSellKeys = accounts.into();
    let ix = quote_sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn quote_sell_invoke_signed(
    accounts: QuoteSellAccounts<'_, '_>,
    args: QuoteSellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    quote_sell_invoke_signed_with_program_id(AMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn quote_sell_verify_account_keys(
    accounts: QuoteSellAccounts<'_, '_>,
    keys: QuoteSellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.owner.key, keys.owner),
        (*accounts.user.key, keys.user),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const SELL_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SellAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub user_ta_a: &'me AccountInfo<'info>,
    pub user_ta_b: &'me AccountInfo<'info>,
    pub vault_a: &'me AccountInfo<'info>,
    pub vault_b: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellKeys {
    pub pool: Pubkey,
    pub user: Pubkey,
    pub owner: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub user_ta_a: Pubkey,
    pub user_ta_b: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub system_program: Pubkey,
    pub program: Pubkey,
}
impl From<SellAccounts<'_, '_>> for SellKeys {
    fn from(accounts: SellAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            user: *accounts.user.key,
            owner: *accounts.owner.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            user_ta_a: *accounts.user_ta_a.key,
            user_ta_b: *accounts.user_ta_b.key,
            vault_a: *accounts.vault_a.key,
            vault_b: *accounts.vault_b.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            system_program: *accounts.system_program.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SellKeys> for [AccountMeta; SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: SellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_ta_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_ta_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
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
impl From<[Pubkey; SELL_IX_ACCOUNTS_LEN]> for SellKeys {
    fn from(pubkeys: [Pubkey; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            user: pubkeys[1],
            owner: pubkeys[2],
            mint_a: pubkeys[3],
            mint_b: pubkeys[4],
            user_ta_a: pubkeys[5],
            user_ta_b: pubkeys[6],
            vault_a: pubkeys[7],
            vault_b: pubkeys[8],
            token_program_a: pubkeys[9],
            token_program_b: pubkeys[10],
            system_program: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<SellAccounts<'_, 'info>>
for [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.user.clone(),
            accounts.owner.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.user_ta_a.clone(),
            accounts.user_ta_b.clone(),
            accounts.vault_a.clone(),
            accounts.vault_b.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.system_program.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]>
for SellAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            user: &arr[1],
            owner: &arr[2],
            mint_a: &arr[3],
            mint_b: &arr[4],
            user_ta_a: &arr[5],
            user_ta_b: &arr[6],
            vault_a: &arr[7],
            vault_b: &arr[8],
            token_program_a: &arr[9],
            token_program_b: &arr[10],
            system_program: &arr[11],
            program: &arr[12],
        }
    }
}
pub const SELL_IX_DISCM: [u8; 8usize] = [51, 230, 133, 164, 1, 127, 131, 173];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellIxArgs {
    pub params: SwapParams,
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
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SwapParams>::deserialize(&mut reader)?
        };
        Ok(Self(SellIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
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
    sell_ix_with_program_id(AMM_PROGRAM_ID, keys, args)
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
    sell_invoke_with_program_id(AMM_PROGRAM_ID, accounts, args)
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
    sell_invoke_signed_with_program_id(AMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn sell_verify_account_keys(
    accounts: SellAccounts<'_, '_>,
    keys: SellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.user.key, keys.user),
        (*accounts.owner.key, keys.owner),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.user_ta_a.key, keys.user_ta_a),
        (*accounts.user_ta_b.key, keys.user_ta_b),
        (*accounts.vault_a.key, keys.vault_a),
        (*accounts.vault_b.key, keys.vault_b),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program.key, keys.program),
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
        accounts.pool,
        accounts.user_ta_a,
        accounts.user_ta_b,
        accounts.vault_a,
        accounts.vault_b,
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
