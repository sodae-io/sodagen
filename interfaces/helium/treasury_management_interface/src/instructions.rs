use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum TreasuryManagementProgramIx {
    InitializeTreasuryManagementV0(InitializeTreasuryManagementV0IxArgs),
    RedeemV0(RedeemV0IxArgs),
    UpdateTreasuryManagementV0(UpdateTreasuryManagementV0IxArgs),
}
impl TreasuryManagementProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_TREASURY_MANAGEMENT_V0_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_TREASURY_MANAGEMENT_V0_IX_DISCM.len()..];
            let args = <InitializeTreasuryManagementArgsV0>::deserialize(&mut reader)?;
            return Ok(
                Self::InitializeTreasuryManagementV0(InitializeTreasuryManagementV0IxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&REDEEM_V0_IX_DISCM) {
            let mut reader = &buf[REDEEM_V0_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <RedeemArgsV0>::deserialize(&mut reader)?
            };
            return Ok(Self::RedeemV0(RedeemV0IxArgs { args }));
        }
        if buf.starts_with(&UPDATE_TREASURY_MANAGEMENT_V0_IX_DISCM) {
            let mut reader = &buf[UPDATE_TREASURY_MANAGEMENT_V0_IX_DISCM.len()..];
            let args = <UpdateTreasuryManagementArgsV0>::deserialize(&mut reader)?;
            return Ok(
                Self::UpdateTreasuryManagementV0(UpdateTreasuryManagementV0IxArgs {
                    args,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeTreasuryManagementV0(args) => {
                writer.write_all(&INITIALIZE_TREASURY_MANAGEMENT_V0_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::RedeemV0(args) => {
                writer.write_all(&REDEEM_V0_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::UpdateTreasuryManagementV0(args) => {
                writer.write_all(&UPDATE_TREASURY_MANAGEMENT_V0_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
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
pub const INITIALIZE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitializeTreasuryManagementV0Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub treasury_management: &'me AccountInfo<'info>,
    pub treasury_mint: &'me AccountInfo<'info>,
    pub supply_mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub circuit_breaker: &'me AccountInfo<'info>,
    pub treasury: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub circuit_breaker_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeTreasuryManagementV0Keys {
    pub payer: Pubkey,
    pub treasury_management: Pubkey,
    pub treasury_mint: Pubkey,
    pub supply_mint: Pubkey,
    pub mint_authority: Pubkey,
    pub circuit_breaker: Pubkey,
    pub treasury: Pubkey,
    pub system_program: Pubkey,
    pub circuit_breaker_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<InitializeTreasuryManagementV0Accounts<'_, '_>>
for InitializeTreasuryManagementV0Keys {
    fn from(accounts: InitializeTreasuryManagementV0Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            treasury_management: *accounts.treasury_management.key,
            treasury_mint: *accounts.treasury_mint.key,
            supply_mint: *accounts.supply_mint.key,
            mint_authority: *accounts.mint_authority.key,
            circuit_breaker: *accounts.circuit_breaker.key,
            treasury: *accounts.treasury.key,
            system_program: *accounts.system_program.key,
            circuit_breaker_program: *accounts.circuit_breaker_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InitializeTreasuryManagementV0Keys>
for [AccountMeta; INITIALIZE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeTreasuryManagementV0Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_management,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.supply_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.circuit_breaker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.circuit_breaker_program,
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
        ]
    }
}
impl From<[Pubkey; INITIALIZE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN]>
for InitializeTreasuryManagementV0Keys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: pubkeys[0],
            treasury_management: pubkeys[1],
            treasury_mint: pubkeys[2],
            supply_mint: pubkeys[3],
            mint_authority: pubkeys[4],
            circuit_breaker: pubkeys[5],
            treasury: pubkeys[6],
            system_program: pubkeys[7],
            circuit_breaker_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<InitializeTreasuryManagementV0Accounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeTreasuryManagementV0Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.treasury_management.clone(),
            accounts.treasury_mint.clone(),
            accounts.supply_mint.clone(),
            accounts.mint_authority.clone(),
            accounts.circuit_breaker.clone(),
            accounts.treasury.clone(),
            accounts.system_program.clone(),
            accounts.circuit_breaker_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN]>
for InitializeTreasuryManagementV0Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            treasury_management: &arr[1],
            treasury_mint: &arr[2],
            supply_mint: &arr[3],
            mint_authority: &arr[4],
            circuit_breaker: &arr[5],
            treasury: &arr[6],
            system_program: &arr[7],
            circuit_breaker_program: &arr[8],
            associated_token_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const INITIALIZE_TREASURY_MANAGEMENT_V0_IX_DISCM: [u8; 8usize] = [
    149, 3, 201, 108, 130, 56, 56, 210,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeTreasuryManagementV0IxArgs {
    pub args: InitializeTreasuryManagementArgsV0,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTreasuryManagementV0IxData(
    pub InitializeTreasuryManagementV0IxArgs,
);
impl From<InitializeTreasuryManagementV0IxArgs>
for InitializeTreasuryManagementV0IxData {
    fn from(args: InitializeTreasuryManagementV0IxArgs) -> Self {
        Self(args)
    }
}
impl InitializeTreasuryManagementV0IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_TREASURY_MANAGEMENT_V0_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = <InitializeTreasuryManagementArgsV0>::deserialize(&mut reader)?;
        Ok(
            Self(InitializeTreasuryManagementV0IxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_TREASURY_MANAGEMENT_V0_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_treasury_management_v0_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeTreasuryManagementV0Keys,
    args: InitializeTreasuryManagementV0IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializeTreasuryManagementV0IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_treasury_management_v0_ix(
    keys: InitializeTreasuryManagementV0Keys,
    args: InitializeTreasuryManagementV0IxArgs,
) -> std::io::Result<Instruction> {
    initialize_treasury_management_v0_ix_with_program_id(
        TREASURY_MANAGEMENT_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn initialize_treasury_management_v0_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTreasuryManagementV0Accounts<'_, '_>,
    args: InitializeTreasuryManagementV0IxArgs,
) -> ProgramResult {
    let keys: InitializeTreasuryManagementV0Keys = accounts.into();
    let ix = initialize_treasury_management_v0_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_treasury_management_v0_invoke(
    accounts: InitializeTreasuryManagementV0Accounts<'_, '_>,
    args: InitializeTreasuryManagementV0IxArgs,
) -> ProgramResult {
    initialize_treasury_management_v0_invoke_with_program_id(
        TREASURY_MANAGEMENT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_treasury_management_v0_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTreasuryManagementV0Accounts<'_, '_>,
    args: InitializeTreasuryManagementV0IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeTreasuryManagementV0Keys = accounts.into();
    let ix = initialize_treasury_management_v0_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_treasury_management_v0_invoke_signed(
    accounts: InitializeTreasuryManagementV0Accounts<'_, '_>,
    args: InitializeTreasuryManagementV0IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_treasury_management_v0_invoke_signed_with_program_id(
        TREASURY_MANAGEMENT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_treasury_management_v0_verify_account_keys(
    accounts: InitializeTreasuryManagementV0Accounts<'_, '_>,
    keys: InitializeTreasuryManagementV0Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.treasury_management.key, keys.treasury_management),
        (*accounts.treasury_mint.key, keys.treasury_mint),
        (*accounts.supply_mint.key, keys.supply_mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.circuit_breaker.key, keys.circuit_breaker),
        (*accounts.treasury.key, keys.treasury),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.circuit_breaker_program.key, keys.circuit_breaker_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_treasury_management_v0_verify_writable_privileges<'me, 'info>(
    accounts: InitializeTreasuryManagementV0Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.treasury_management,
        accounts.circuit_breaker,
        accounts.treasury,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_treasury_management_v0_verify_signer_privileges<'me, 'info>(
    accounts: InitializeTreasuryManagementV0Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.mint_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_treasury_management_v0_verify_account_privileges<'me, 'info>(
    accounts: InitializeTreasuryManagementV0Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_treasury_management_v0_verify_writable_privileges(accounts)?;
    initialize_treasury_management_v0_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_V0_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct RedeemV0Accounts<'me, 'info> {
    pub treasury_management: &'me AccountInfo<'info>,
    pub treasury_mint: &'me AccountInfo<'info>,
    pub supply_mint: &'me AccountInfo<'info>,
    pub treasury: &'me AccountInfo<'info>,
    pub circuit_breaker: &'me AccountInfo<'info>,
    pub from: &'me AccountInfo<'info>,
    pub to: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub circuit_breaker_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemV0Keys {
    pub treasury_management: Pubkey,
    pub treasury_mint: Pubkey,
    pub supply_mint: Pubkey,
    pub treasury: Pubkey,
    pub circuit_breaker: Pubkey,
    pub from: Pubkey,
    pub to: Pubkey,
    pub owner: Pubkey,
    pub circuit_breaker_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<RedeemV0Accounts<'_, '_>> for RedeemV0Keys {
    fn from(accounts: RedeemV0Accounts) -> Self {
        Self {
            treasury_management: *accounts.treasury_management.key,
            treasury_mint: *accounts.treasury_mint.key,
            supply_mint: *accounts.supply_mint.key,
            treasury: *accounts.treasury.key,
            circuit_breaker: *accounts.circuit_breaker.key,
            from: *accounts.from.key,
            to: *accounts.to.key,
            owner: *accounts.owner.key,
            circuit_breaker_program: *accounts.circuit_breaker_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RedeemV0Keys> for [AccountMeta; REDEEM_V0_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemV0Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.treasury_management,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.treasury_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.supply_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.circuit_breaker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.circuit_breaker_program,
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
impl From<[Pubkey; REDEEM_V0_IX_ACCOUNTS_LEN]> for RedeemV0Keys {
    fn from(pubkeys: [Pubkey; REDEEM_V0_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            treasury_management: pubkeys[0],
            treasury_mint: pubkeys[1],
            supply_mint: pubkeys[2],
            treasury: pubkeys[3],
            circuit_breaker: pubkeys[4],
            from: pubkeys[5],
            to: pubkeys[6],
            owner: pubkeys[7],
            circuit_breaker_program: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<RedeemV0Accounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_V0_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemV0Accounts<'_, 'info>) -> Self {
        [
            accounts.treasury_management.clone(),
            accounts.treasury_mint.clone(),
            accounts.supply_mint.clone(),
            accounts.treasury.clone(),
            accounts.circuit_breaker.clone(),
            accounts.from.clone(),
            accounts.to.clone(),
            accounts.owner.clone(),
            accounts.circuit_breaker_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_V0_IX_ACCOUNTS_LEN]>
for RedeemV0Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REDEEM_V0_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            treasury_management: &arr[0],
            treasury_mint: &arr[1],
            supply_mint: &arr[2],
            treasury: &arr[3],
            circuit_breaker: &arr[4],
            from: &arr[5],
            to: &arr[6],
            owner: &arr[7],
            circuit_breaker_program: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const REDEEM_V0_IX_DISCM: [u8; 8usize] = [235, 127, 171, 139, 119, 77, 235, 118];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemV0IxArgs {
    pub args: RedeemArgsV0,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemV0IxData(pub RedeemV0IxArgs);
impl From<RedeemV0IxArgs> for RedeemV0IxData {
    fn from(args: RedeemV0IxArgs) -> Self {
        Self(args)
    }
}
impl RedeemV0IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_V0_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <RedeemArgsV0>::deserialize(&mut reader)?
        };
        Ok(Self(RedeemV0IxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_V0_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_v0_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemV0Keys,
    args: RedeemV0IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_V0_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemV0IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_v0_ix(
    keys: RedeemV0Keys,
    args: RedeemV0IxArgs,
) -> std::io::Result<Instruction> {
    redeem_v0_ix_with_program_id(TREASURY_MANAGEMENT_PROGRAM_ID, keys, args)
}
pub fn redeem_v0_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemV0Accounts<'_, '_>,
    args: RedeemV0IxArgs,
) -> ProgramResult {
    let keys: RedeemV0Keys = accounts.into();
    let ix = redeem_v0_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_v0_invoke(
    accounts: RedeemV0Accounts<'_, '_>,
    args: RedeemV0IxArgs,
) -> ProgramResult {
    redeem_v0_invoke_with_program_id(TREASURY_MANAGEMENT_PROGRAM_ID, accounts, args)
}
pub fn redeem_v0_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemV0Accounts<'_, '_>,
    args: RedeemV0IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemV0Keys = accounts.into();
    let ix = redeem_v0_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_v0_invoke_signed(
    accounts: RedeemV0Accounts<'_, '_>,
    args: RedeemV0IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_v0_invoke_signed_with_program_id(
        TREASURY_MANAGEMENT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn redeem_v0_verify_account_keys(
    accounts: RedeemV0Accounts<'_, '_>,
    keys: RedeemV0Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.treasury_management.key, keys.treasury_management),
        (*accounts.treasury_mint.key, keys.treasury_mint),
        (*accounts.supply_mint.key, keys.supply_mint),
        (*accounts.treasury.key, keys.treasury),
        (*accounts.circuit_breaker.key, keys.circuit_breaker),
        (*accounts.from.key, keys.from),
        (*accounts.to.key, keys.to),
        (*accounts.owner.key, keys.owner),
        (*accounts.circuit_breaker_program.key, keys.circuit_breaker_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_v0_verify_writable_privileges<'me, 'info>(
    accounts: RedeemV0Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.supply_mint,
        accounts.treasury,
        accounts.circuit_breaker,
        accounts.from,
        accounts.to,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_v0_verify_signer_privileges<'me, 'info>(
    accounts: RedeemV0Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_v0_verify_account_privileges<'me, 'info>(
    accounts: RedeemV0Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_v0_verify_writable_privileges(accounts)?;
    redeem_v0_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateTreasuryManagementV0Accounts<'me, 'info> {
    pub treasury_management: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateTreasuryManagementV0Keys {
    pub treasury_management: Pubkey,
    pub authority: Pubkey,
}
impl From<UpdateTreasuryManagementV0Accounts<'_, '_>>
for UpdateTreasuryManagementV0Keys {
    fn from(accounts: UpdateTreasuryManagementV0Accounts) -> Self {
        Self {
            treasury_management: *accounts.treasury_management.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<UpdateTreasuryManagementV0Keys>
for [AccountMeta; UPDATE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateTreasuryManagementV0Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.treasury_management,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN]>
for UpdateTreasuryManagementV0Keys {
    fn from(pubkeys: [Pubkey; UPDATE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            treasury_management: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateTreasuryManagementV0Accounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateTreasuryManagementV0Accounts<'_, 'info>) -> Self {
        [accounts.treasury_management.clone(), accounts.authority.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN]>
for UpdateTreasuryManagementV0Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            treasury_management: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const UPDATE_TREASURY_MANAGEMENT_V0_IX_DISCM: [u8; 8usize] = [
    209, 139, 90, 226, 249, 149, 89, 217,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateTreasuryManagementV0IxArgs {
    pub args: UpdateTreasuryManagementArgsV0,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTreasuryManagementV0IxData(pub UpdateTreasuryManagementV0IxArgs);
impl From<UpdateTreasuryManagementV0IxArgs> for UpdateTreasuryManagementV0IxData {
    fn from(args: UpdateTreasuryManagementV0IxArgs) -> Self {
        Self(args)
    }
}
impl UpdateTreasuryManagementV0IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_TREASURY_MANAGEMENT_V0_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = <UpdateTreasuryManagementArgsV0>::deserialize(&mut reader)?;
        Ok(
            Self(UpdateTreasuryManagementV0IxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_TREASURY_MANAGEMENT_V0_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_treasury_management_v0_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateTreasuryManagementV0Keys,
    args: UpdateTreasuryManagementV0IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_TREASURY_MANAGEMENT_V0_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateTreasuryManagementV0IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_treasury_management_v0_ix(
    keys: UpdateTreasuryManagementV0Keys,
    args: UpdateTreasuryManagementV0IxArgs,
) -> std::io::Result<Instruction> {
    update_treasury_management_v0_ix_with_program_id(
        TREASURY_MANAGEMENT_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_treasury_management_v0_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTreasuryManagementV0Accounts<'_, '_>,
    args: UpdateTreasuryManagementV0IxArgs,
) -> ProgramResult {
    let keys: UpdateTreasuryManagementV0Keys = accounts.into();
    let ix = update_treasury_management_v0_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_treasury_management_v0_invoke(
    accounts: UpdateTreasuryManagementV0Accounts<'_, '_>,
    args: UpdateTreasuryManagementV0IxArgs,
) -> ProgramResult {
    update_treasury_management_v0_invoke_with_program_id(
        TREASURY_MANAGEMENT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_treasury_management_v0_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTreasuryManagementV0Accounts<'_, '_>,
    args: UpdateTreasuryManagementV0IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateTreasuryManagementV0Keys = accounts.into();
    let ix = update_treasury_management_v0_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_treasury_management_v0_invoke_signed(
    accounts: UpdateTreasuryManagementV0Accounts<'_, '_>,
    args: UpdateTreasuryManagementV0IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_treasury_management_v0_invoke_signed_with_program_id(
        TREASURY_MANAGEMENT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_treasury_management_v0_verify_account_keys(
    accounts: UpdateTreasuryManagementV0Accounts<'_, '_>,
    keys: UpdateTreasuryManagementV0Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.treasury_management.key, keys.treasury_management),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_treasury_management_v0_verify_writable_privileges<'me, 'info>(
    accounts: UpdateTreasuryManagementV0Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.treasury_management] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_treasury_management_v0_verify_signer_privileges<'me, 'info>(
    accounts: UpdateTreasuryManagementV0Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_treasury_management_v0_verify_account_privileges<'me, 'info>(
    accounts: UpdateTreasuryManagementV0Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_treasury_management_v0_verify_writable_privileges(accounts)?;
    update_treasury_management_v0_verify_signer_privileges(accounts)?;
    Ok(())
}
