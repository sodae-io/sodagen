use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum MarcoPoloProgramIx {
    Swap(SwapIxArgs),
}
impl MarcoPoloProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let price_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
            let x_to_y: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    amount,
                    price_limit,
                    x_to_y,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.x_to_y, &mut writer)?;
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
pub const SWAP_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub vault_x: &'me AccountInfo<'info>,
    pub vault_y: &'me AccountInfo<'info>,
    pub swapper_x_account: &'me AccountInfo<'info>,
    pub swapper_y_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub referrer_x_account: &'me AccountInfo<'info>,
    pub referrer_y_account: &'me AccountInfo<'info>,
    pub referrer_x_authority: &'me AccountInfo<'info>,
    pub referrer_y_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub vault_x: Pubkey,
    pub vault_y: Pubkey,
    pub swapper_x_account: Pubkey,
    pub swapper_y_account: Pubkey,
    pub owner: Pubkey,
    pub referrer_x_account: Pubkey,
    pub referrer_y_account: Pubkey,
    pub referrer_x_authority: Pubkey,
    pub referrer_y_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            vault_x: *accounts.vault_x.key,
            vault_y: *accounts.vault_y.key,
            swapper_x_account: *accounts.swapper_x_account.key,
            swapper_y_account: *accounts.swapper_y_account.key,
            owner: *accounts.owner.key,
            referrer_x_account: *accounts.referrer_x_account.key,
            referrer_y_account: *accounts.referrer_y_account.key,
            referrer_x_authority: *accounts.referrer_x_authority.key,
            referrer_y_authority: *accounts.referrer_y_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swapper_x_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swapper_y_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referrer_x_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referrer_y_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referrer_x_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referrer_y_authority,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            token_x_mint: pubkeys[2],
            token_y_mint: pubkeys[3],
            vault_x: pubkeys[4],
            vault_y: pubkeys[5],
            swapper_x_account: pubkeys[6],
            swapper_y_account: pubkeys[7],
            owner: pubkeys[8],
            referrer_x_account: pubkeys[9],
            referrer_y_account: pubkeys[10],
            referrer_x_authority: pubkeys[11],
            referrer_y_authority: pubkeys[12],
            system_program: pubkeys[13],
            token_program: pubkeys[14],
            associated_token_program: pubkeys[15],
            rent: pubkeys[16],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.vault_x.clone(),
            accounts.vault_y.clone(),
            accounts.swapper_x_account.clone(),
            accounts.swapper_y_account.clone(),
            accounts.owner.clone(),
            accounts.referrer_x_account.clone(),
            accounts.referrer_y_account.clone(),
            accounts.referrer_x_authority.clone(),
            accounts.referrer_y_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            token_x_mint: &arr[2],
            token_y_mint: &arr[3],
            vault_x: &arr[4],
            vault_y: &arr[5],
            swapper_x_account: &arr[6],
            swapper_y_account: &arr[7],
            owner: &arr[8],
            referrer_x_account: &arr[9],
            referrer_y_account: &arr[10],
            referrer_x_authority: &arr[11],
            referrer_y_authority: &arr[12],
            system_program: &arr[13],
            token_program: &arr[14],
            associated_token_program: &arr[15],
            rent: &arr[16],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub amount: u64,
    pub price_limit: u128,
    pub x_to_y: bool,
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
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
        let x_to_y: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                amount,
                price_limit,
                x_to_y,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.price_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.x_to_y, &mut writer)?;
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
    swap_ix_with_program_id(MARCO_POLO_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(MARCO_POLO_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(MARCO_POLO_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.vault_x.key, keys.vault_x),
        (*accounts.vault_y.key, keys.vault_y),
        (*accounts.swapper_x_account.key, keys.swapper_x_account),
        (*accounts.swapper_y_account.key, keys.swapper_y_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.referrer_x_account.key, keys.referrer_x_account),
        (*accounts.referrer_y_account.key, keys.referrer_y_account),
        (*accounts.referrer_x_authority.key, keys.referrer_x_authority),
        (*accounts.referrer_y_authority.key, keys.referrer_y_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
