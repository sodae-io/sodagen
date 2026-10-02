use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum JupiterProgramIx {
    Claim(ClaimIxArgs),
    ClaimToken(ClaimTokenIxArgs),
    CloseToken(CloseTokenIxArgs),
    CreateTokenLedger,
    CreateTokenAccount(CreateTokenAccountIxArgs),
    CloseWsolTokenAccount,
    ExactOutRoute(ExactOutRouteIxArgs),
    Route(RouteIxArgs),
    RouteWithTokenLedger(RouteWithTokenLedgerIxArgs),
    SetTokenLedger,
    SharedAccountsExactOutRoute(SharedAccountsExactOutRouteIxArgs),
    SharedAccountsRoute(SharedAccountsRouteIxArgs),
    SharedAccountsRouteWithTokenLedger(SharedAccountsRouteWithTokenLedgerIxArgs),
    ExactOutRouteV2(ExactOutRouteV2IxArgs),
    RouteV2(RouteV2IxArgs),
    SharedAccountsExactOutRouteV2(SharedAccountsExactOutRouteV2IxArgs),
    SharedAccountsRouteV2(SharedAccountsRouteV2IxArgs),
}
impl JupiterProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CLAIM_IX_DISCM) {
            let mut reader = &buf[CLAIM_IX_DISCM.len()..];
            let id: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Claim(ClaimIxArgs { id }));
        }
        if buf.starts_with(&CLAIM_TOKEN_IX_DISCM) {
            let mut reader = &buf[CLAIM_TOKEN_IX_DISCM.len()..];
            let id: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ClaimToken(ClaimTokenIxArgs { id }));
        }
        if buf.starts_with(&CLOSE_TOKEN_IX_DISCM) {
            let mut reader = &buf[CLOSE_TOKEN_IX_DISCM.len()..];
            let id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let burn_all: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::CloseToken(CloseTokenIxArgs { id, burn_all }));
        }
        if buf.starts_with(&CREATE_TOKEN_LEDGER_IX_DISCM) {
            return Ok(Self::CreateTokenLedger);
        }
        if buf.starts_with(&CREATE_TOKEN_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[CREATE_TOKEN_ACCOUNT_IX_DISCM.len()..];
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::CreateTokenAccount(CreateTokenAccountIxArgs { bump }));
        }
        if buf.starts_with(&CLOSE_WSOL_TOKEN_ACCOUNT_IX_DISCM) {
            return Ok(Self::CloseWsolTokenAccount);
        }
        if buf.starts_with(&EXACT_OUT_ROUTE_IX_DISCM) {
            let mut reader = &buf[EXACT_OUT_ROUTE_IX_DISCM.len()..];
            let route_plan: Vec<RoutePlanStep> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quoted_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let platform_fee_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ExactOutRoute(ExactOutRouteIxArgs {
                    route_plan,
                    out_amount,
                    quoted_in_amount,
                    slippage_bps,
                    platform_fee_bps,
                }),
            );
        }
        if buf.starts_with(&ROUTE_IX_DISCM) {
            let mut reader = &buf[ROUTE_IX_DISCM.len()..];
            let route_plan: Vec<RoutePlanStep> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quoted_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let platform_fee_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Route(RouteIxArgs {
                    route_plan,
                    in_amount,
                    quoted_out_amount,
                    slippage_bps,
                    platform_fee_bps,
                }),
            );
        }
        if buf.starts_with(&ROUTE_WITH_TOKEN_LEDGER_IX_DISCM) {
            let mut reader = &buf[ROUTE_WITH_TOKEN_LEDGER_IX_DISCM.len()..];
            let route_plan: Vec<RoutePlanStep> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let quoted_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let platform_fee_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RouteWithTokenLedger(RouteWithTokenLedgerIxArgs {
                    route_plan,
                    quoted_out_amount,
                    slippage_bps,
                    platform_fee_bps,
                }),
            );
        }
        if buf.starts_with(&SET_TOKEN_LEDGER_IX_DISCM) {
            return Ok(Self::SetTokenLedger);
        }
        if buf.starts_with(&SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_DISCM) {
            let mut reader = &buf[SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_DISCM.len()..];
            let id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let route_plan: Vec<RoutePlanStep> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quoted_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let platform_fee_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SharedAccountsExactOutRoute(SharedAccountsExactOutRouteIxArgs {
                    id,
                    route_plan,
                    out_amount,
                    quoted_in_amount,
                    slippage_bps,
                    platform_fee_bps,
                }),
            );
        }
        if buf.starts_with(&SHARED_ACCOUNTS_ROUTE_IX_DISCM) {
            let mut reader = &buf[SHARED_ACCOUNTS_ROUTE_IX_DISCM.len()..];
            let id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let route_plan: Vec<RoutePlanStep> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quoted_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let platform_fee_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SharedAccountsRoute(SharedAccountsRouteIxArgs {
                    id,
                    route_plan,
                    in_amount,
                    quoted_out_amount,
                    slippage_bps,
                    platform_fee_bps,
                }),
            );
        }
        if buf.starts_with(&SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_DISCM) {
            let mut reader = &buf[SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_DISCM
                .len()..];
            let id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let route_plan: Vec<RoutePlanStep> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let quoted_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let platform_fee_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SharedAccountsRouteWithTokenLedger(SharedAccountsRouteWithTokenLedgerIxArgs {
                    id,
                    route_plan,
                    quoted_out_amount,
                    slippage_bps,
                    platform_fee_bps,
                }),
            );
        }
        if buf.starts_with(&EXACT_OUT_ROUTE_V2_IX_DISCM) {
            let mut reader = &buf[EXACT_OUT_ROUTE_V2_IX_DISCM.len()..];
            let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quoted_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let platform_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let positive_slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let route_plan: Vec<RoutePlanStepV2> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::ExactOutRouteV2(ExactOutRouteV2IxArgs {
                    out_amount,
                    quoted_in_amount,
                    slippage_bps,
                    platform_fee_bps,
                    positive_slippage_bps,
                    route_plan,
                }),
            );
        }
        if buf.starts_with(&ROUTE_V2_IX_DISCM) {
            let mut reader = &buf[ROUTE_V2_IX_DISCM.len()..];
            let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quoted_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let platform_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let positive_slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let route_plan: Vec<RoutePlanStepV2> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::RouteV2(RouteV2IxArgs {
                    in_amount,
                    quoted_out_amount,
                    slippage_bps,
                    platform_fee_bps,
                    positive_slippage_bps,
                    route_plan,
                }),
            );
        }
        if buf.starts_with(&SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_DISCM) {
            let mut reader = &buf[SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_DISCM.len()..];
            let id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quoted_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let platform_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let positive_slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let route_plan: Vec<RoutePlanStepV2> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SharedAccountsExactOutRouteV2(SharedAccountsExactOutRouteV2IxArgs {
                    id,
                    out_amount,
                    quoted_in_amount,
                    slippage_bps,
                    platform_fee_bps,
                    positive_slippage_bps,
                    route_plan,
                }),
            );
        }
        if buf.starts_with(&SHARED_ACCOUNTS_ROUTE_V2_IX_DISCM) {
            let mut reader = &buf[SHARED_ACCOUNTS_ROUTE_V2_IX_DISCM.len()..];
            let id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quoted_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let platform_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let positive_slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let route_plan: Vec<RoutePlanStepV2> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SharedAccountsRouteV2(SharedAccountsRouteV2IxArgs {
                    id,
                    in_amount,
                    quoted_out_amount,
                    slippage_bps,
                    platform_fee_bps,
                    positive_slippage_bps,
                    route_plan,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Claim(args) => {
                writer.write_all(&CLAIM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.id, &mut writer)?;
                Ok(())
            }
            Self::ClaimToken(args) => {
                writer.write_all(&CLAIM_TOKEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.id, &mut writer)?;
                Ok(())
            }
            Self::CloseToken(args) => {
                writer.write_all(&CLOSE_TOKEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.burn_all, &mut writer)?;
                Ok(())
            }
            Self::CreateTokenLedger => writer.write_all(&CREATE_TOKEN_LEDGER_IX_DISCM),
            Self::CreateTokenAccount(args) => {
                writer.write_all(&CREATE_TOKEN_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                Ok(())
            }
            Self::CloseWsolTokenAccount => {
                writer.write_all(&CLOSE_WSOL_TOKEN_ACCOUNT_IX_DISCM)
            }
            Self::ExactOutRoute(args) => {
                writer.write_all(&EXACT_OUT_ROUTE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.route_plan, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quoted_in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.platform_fee_bps, &mut writer)?;
                Ok(())
            }
            Self::Route(args) => {
                writer.write_all(&ROUTE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.route_plan, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quoted_out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.platform_fee_bps, &mut writer)?;
                Ok(())
            }
            Self::RouteWithTokenLedger(args) => {
                writer.write_all(&ROUTE_WITH_TOKEN_LEDGER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.route_plan, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quoted_out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.platform_fee_bps, &mut writer)?;
                Ok(())
            }
            Self::SetTokenLedger => writer.write_all(&SET_TOKEN_LEDGER_IX_DISCM),
            Self::SharedAccountsExactOutRoute(args) => {
                writer.write_all(&SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.route_plan, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quoted_in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.platform_fee_bps, &mut writer)?;
                Ok(())
            }
            Self::SharedAccountsRoute(args) => {
                writer.write_all(&SHARED_ACCOUNTS_ROUTE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.route_plan, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quoted_out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.platform_fee_bps, &mut writer)?;
                Ok(())
            }
            Self::SharedAccountsRouteWithTokenLedger(args) => {
                writer.write_all(&SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.route_plan, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quoted_out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.platform_fee_bps, &mut writer)?;
                Ok(())
            }
            Self::ExactOutRouteV2(args) => {
                writer.write_all(&EXACT_OUT_ROUTE_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quoted_in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.platform_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.positive_slippage_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.route_plan, &mut writer)?;
                Ok(())
            }
            Self::RouteV2(args) => {
                writer.write_all(&ROUTE_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quoted_out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.platform_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.positive_slippage_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.route_plan, &mut writer)?;
                Ok(())
            }
            Self::SharedAccountsExactOutRouteV2(args) => {
                writer.write_all(&SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quoted_in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.platform_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.positive_slippage_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.route_plan, &mut writer)?;
                Ok(())
            }
            Self::SharedAccountsRouteV2(args) => {
                writer.write_all(&SHARED_ACCOUNTS_ROUTE_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quoted_out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.platform_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.positive_slippage_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.route_plan, &mut writer)?;
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
pub const CLAIM_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ClaimAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimKeys {
    pub wallet: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<ClaimAccounts<'_, '_>> for ClaimKeys {
    fn from(accounts: ClaimAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ClaimKeys> for [AccountMeta; CLAIM_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
impl From<[Pubkey; CLAIM_IX_ACCOUNTS_LEN]> for ClaimKeys {
    fn from(pubkeys: [Pubkey; CLAIM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wallet: pubkeys[0],
            program_authority: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<ClaimAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN]>
for ClaimAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wallet: &arr[0],
            program_authority: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CLAIM_IX_DISCM: [u8; 8usize] = [62, 198, 214, 193, 213, 159, 108, 210];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimIxArgs {
    pub id: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimIxData(pub ClaimIxArgs);
impl From<ClaimIxArgs> for ClaimIxData {
    fn from(args: ClaimIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let id: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ClaimIxArgs { id }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.id, &mut writer)?;
        Ok(())
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
    args: ClaimIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_ix(keys: ClaimKeys, args: ClaimIxArgs) -> std::io::Result<Instruction> {
    claim_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn claim_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimAccounts<'_, '_>,
    args: ClaimIxArgs,
) -> ProgramResult {
    let keys: ClaimKeys = accounts.into();
    let ix = claim_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_invoke(
    accounts: ClaimAccounts<'_, '_>,
    args: ClaimIxArgs,
) -> ProgramResult {
    claim_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts, args)
}
pub fn claim_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimAccounts<'_, '_>,
    args: ClaimIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimKeys = accounts.into();
    let ix = claim_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_invoke_signed(
    accounts: ClaimAccounts<'_, '_>,
    args: ClaimIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_invoke_signed_with_program_id(JUPITER_PROGRAM_ID, accounts, args, seeds)
}
pub fn claim_verify_account_keys(
    accounts: ClaimAccounts<'_, '_>,
    keys: ClaimKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.system_program.key, keys.system_program),
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
    for should_be_writable in [accounts.wallet, accounts.program_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_verify_account_privileges<'me, 'info>(
    accounts: ClaimAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_TOKEN_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct ClaimTokenAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub wallet: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub program_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimTokenKeys {
    pub payer: Pubkey,
    pub wallet: Pubkey,
    pub program_authority: Pubkey,
    pub program_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<ClaimTokenAccounts<'_, '_>> for ClaimTokenKeys {
    fn from(accounts: ClaimTokenAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            wallet: *accounts.wallet.key,
            program_authority: *accounts.program_authority.key,
            program_token_account: *accounts.program_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ClaimTokenKeys> for [AccountMeta; CLAIM_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
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
impl From<[Pubkey; CLAIM_TOKEN_IX_ACCOUNTS_LEN]> for ClaimTokenKeys {
    fn from(pubkeys: [Pubkey; CLAIM_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            wallet: pubkeys[1],
            program_authority: pubkeys[2],
            program_token_account: pubkeys[3],
            destination_token_account: pubkeys[4],
            mint: pubkeys[5],
            token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<ClaimTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.wallet.clone(),
            accounts.program_authority.clone(),
            accounts.program_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_TOKEN_IX_ACCOUNTS_LEN]>
for ClaimTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            wallet: &arr[1],
            program_authority: &arr[2],
            program_token_account: &arr[3],
            destination_token_account: &arr[4],
            mint: &arr[5],
            token_program: &arr[6],
            associated_token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const CLAIM_TOKEN_IX_DISCM: [u8; 8usize] = [116, 206, 27, 191, 166, 19, 0, 73];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimTokenIxArgs {
    pub id: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTokenIxData(pub ClaimTokenIxArgs);
impl From<ClaimTokenIxArgs> for ClaimTokenIxData {
    fn from(args: ClaimTokenIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let id: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ClaimTokenIxArgs { id }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_TOKEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_token_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimTokenKeys,
    args: ClaimTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_token_ix(
    keys: ClaimTokenKeys,
    args: ClaimTokenIxArgs,
) -> std::io::Result<Instruction> {
    claim_token_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn claim_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenAccounts<'_, '_>,
    args: ClaimTokenIxArgs,
) -> ProgramResult {
    let keys: ClaimTokenKeys = accounts.into();
    let ix = claim_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_token_invoke(
    accounts: ClaimTokenAccounts<'_, '_>,
    args: ClaimTokenIxArgs,
) -> ProgramResult {
    claim_token_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts, args)
}
pub fn claim_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenAccounts<'_, '_>,
    args: ClaimTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimTokenKeys = accounts.into();
    let ix = claim_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_token_invoke_signed(
    accounts: ClaimTokenAccounts<'_, '_>,
    args: ClaimTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_token_invoke_signed_with_program_id(JUPITER_PROGRAM_ID, accounts, args, seeds)
}
pub fn claim_token_verify_account_keys(
    accounts: ClaimTokenAccounts<'_, '_>,
    keys: ClaimTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.wallet.key, keys.wallet),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.program_token_account.key, keys.program_token_account),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.mint.key, keys.mint),
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
pub fn claim_token_verify_writable_privileges<'me, 'info>(
    accounts: ClaimTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.program_token_account,
        accounts.destination_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_token_verify_signer_privileges<'me, 'info>(
    accounts: ClaimTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_token_verify_account_privileges<'me, 'info>(
    accounts: ClaimTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_token_verify_writable_privileges(accounts)?;
    claim_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_TOKEN_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CloseTokenAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub wallet: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub program_token_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseTokenKeys {
    pub operator: Pubkey,
    pub wallet: Pubkey,
    pub program_authority: Pubkey,
    pub program_token_account: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
}
impl From<CloseTokenAccounts<'_, '_>> for CloseTokenKeys {
    fn from(accounts: CloseTokenAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            wallet: *accounts.wallet.key,
            program_authority: *accounts.program_authority.key,
            program_token_account: *accounts.program_token_account.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CloseTokenKeys> for [AccountMeta; CLOSE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
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
impl From<[Pubkey; CLOSE_TOKEN_IX_ACCOUNTS_LEN]> for CloseTokenKeys {
    fn from(pubkeys: [Pubkey; CLOSE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            wallet: pubkeys[1],
            program_authority: pubkeys[2],
            program_token_account: pubkeys[3],
            mint: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<CloseTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.wallet.clone(),
            accounts.program_authority.clone(),
            accounts.program_token_account.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_TOKEN_IX_ACCOUNTS_LEN]>
for CloseTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: &arr[0],
            wallet: &arr[1],
            program_authority: &arr[2],
            program_token_account: &arr[3],
            mint: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const CLOSE_TOKEN_IX_DISCM: [u8; 8usize] = [26, 74, 236, 151, 104, 64, 183, 249];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CloseTokenIxArgs {
    pub id: u8,
    pub burn_all: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseTokenIxData(pub CloseTokenIxArgs);
impl From<CloseTokenIxArgs> for CloseTokenIxData {
    fn from(args: CloseTokenIxArgs) -> Self {
        Self(args)
    }
}
impl CloseTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let burn_all: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(CloseTokenIxArgs { id, burn_all }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_TOKEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.burn_all, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_token_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseTokenKeys,
    args: CloseTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: CloseTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn close_token_ix(
    keys: CloseTokenKeys,
    args: CloseTokenIxArgs,
) -> std::io::Result<Instruction> {
    close_token_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn close_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseTokenAccounts<'_, '_>,
    args: CloseTokenIxArgs,
) -> ProgramResult {
    let keys: CloseTokenKeys = accounts.into();
    let ix = close_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_token_invoke(
    accounts: CloseTokenAccounts<'_, '_>,
    args: CloseTokenIxArgs,
) -> ProgramResult {
    close_token_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts, args)
}
pub fn close_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseTokenAccounts<'_, '_>,
    args: CloseTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseTokenKeys = accounts.into();
    let ix = close_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_token_invoke_signed(
    accounts: CloseTokenAccounts<'_, '_>,
    args: CloseTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_token_invoke_signed_with_program_id(JUPITER_PROGRAM_ID, accounts, args, seeds)
}
pub fn close_token_verify_account_keys(
    accounts: CloseTokenAccounts<'_, '_>,
    keys: CloseTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.wallet.key, keys.wallet),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.program_token_account.key, keys.program_token_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_token_verify_writable_privileges<'me, 'info>(
    accounts: CloseTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.wallet,
        accounts.program_token_account,
        accounts.mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_token_verify_signer_privileges<'me, 'info>(
    accounts: CloseTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_token_verify_account_privileges<'me, 'info>(
    accounts: CloseTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_token_verify_writable_privileges(accounts)?;
    close_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateTokenLedgerAccounts<'me, 'info> {
    pub token_ledger: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTokenLedgerKeys {
    pub token_ledger: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateTokenLedgerAccounts<'_, '_>> for CreateTokenLedgerKeys {
    fn from(accounts: CreateTokenLedgerAccounts) -> Self {
        Self {
            token_ledger: *accounts.token_ledger.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateTokenLedgerKeys> for [AccountMeta; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTokenLedgerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_ledger,
                is_signer: true,
                is_writable: true,
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
impl From<[Pubkey; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN]> for CreateTokenLedgerKeys {
    fn from(pubkeys: [Pubkey; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_ledger: pubkeys[0],
            payer: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreateTokenLedgerAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTokenLedgerAccounts<'_, 'info>) -> Self {
        [
            accounts.token_ledger.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
for CreateTokenLedgerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_ledger: &arr[0],
            payer: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CREATE_TOKEN_LEDGER_IX_DISCM: [u8; 8usize] = [
    232, 242, 197, 253, 240, 143, 129, 52,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTokenLedgerIxData;
impl CreateTokenLedgerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TOKEN_LEDGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TOKEN_LEDGER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_token_ledger_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTokenLedgerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateTokenLedgerIxData.try_to_vec()?,
    })
}
pub fn create_token_ledger_ix(
    keys: CreateTokenLedgerKeys,
) -> std::io::Result<Instruction> {
    create_token_ledger_ix_with_program_id(JUPITER_PROGRAM_ID, keys)
}
pub fn create_token_ledger_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenLedgerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateTokenLedgerKeys = accounts.into();
    let ix = create_token_ledger_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_token_ledger_invoke(
    accounts: CreateTokenLedgerAccounts<'_, '_>,
) -> ProgramResult {
    create_token_ledger_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts)
}
pub fn create_token_ledger_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenLedgerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTokenLedgerKeys = accounts.into();
    let ix = create_token_ledger_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_token_ledger_invoke_signed(
    accounts: CreateTokenLedgerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_token_ledger_invoke_signed_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_token_ledger_verify_account_keys(
    accounts: CreateTokenLedgerAccounts<'_, '_>,
    keys: CreateTokenLedgerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_ledger.key, keys.token_ledger),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_token_ledger_verify_writable_privileges<'me, 'info>(
    accounts: CreateTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_ledger, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_token_ledger_verify_signer_privileges<'me, 'info>(
    accounts: CreateTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.token_ledger, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_token_ledger_verify_account_privileges<'me, 'info>(
    accounts: CreateTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_token_ledger_verify_writable_privileges(accounts)?;
    create_token_ledger_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateTokenAccountAccounts<'me, 'info> {
    pub token_account: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTokenAccountKeys {
    pub token_account: Pubkey,
    pub user: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateTokenAccountAccounts<'_, '_>> for CreateTokenAccountKeys {
    fn from(accounts: CreateTokenAccountAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            user: *accounts.user.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateTokenAccountKeys>
for [AccountMeta; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTokenAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
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
        ]
    }
}
impl From<[Pubkey; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]> for CreateTokenAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            user: pubkeys[1],
            mint: pubkeys[2],
            token_program: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateTokenAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTokenAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.token_account.clone(),
            accounts.user.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateTokenAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_account: &arr[0],
            user: &arr[1],
            mint: &arr[2],
            token_program: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_TOKEN_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    147, 241, 123, 100, 244, 132, 174, 118,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTokenAccountIxArgs {
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTokenAccountIxData(pub CreateTokenAccountIxArgs);
impl From<CreateTokenAccountIxArgs> for CreateTokenAccountIxData {
    fn from(args: CreateTokenAccountIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTokenAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TOKEN_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(CreateTokenAccountIxArgs { bump }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TOKEN_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_token_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTokenAccountKeys,
    args: CreateTokenAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateTokenAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_token_account_ix(
    keys: CreateTokenAccountKeys,
    args: CreateTokenAccountIxArgs,
) -> std::io::Result<Instruction> {
    create_token_account_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn create_token_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenAccountAccounts<'_, '_>,
    args: CreateTokenAccountIxArgs,
) -> ProgramResult {
    let keys: CreateTokenAccountKeys = accounts.into();
    let ix = create_token_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_token_account_invoke(
    accounts: CreateTokenAccountAccounts<'_, '_>,
    args: CreateTokenAccountIxArgs,
) -> ProgramResult {
    create_token_account_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts, args)
}
pub fn create_token_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenAccountAccounts<'_, '_>,
    args: CreateTokenAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTokenAccountKeys = accounts.into();
    let ix = create_token_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_token_account_invoke_signed(
    accounts: CreateTokenAccountAccounts<'_, '_>,
    args: CreateTokenAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_token_account_invoke_signed_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_token_account_verify_account_keys(
    accounts: CreateTokenAccountAccounts<'_, '_>,
    keys: CreateTokenAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_account.key, keys.token_account),
        (*accounts.user.key, keys.user),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_token_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_token_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_token_account_verify_account_privileges<'me, 'info>(
    accounts: CreateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_token_account_verify_writable_privileges(accounts)?;
    create_token_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_WSOL_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CloseWsolTokenAccountAccounts<'me, 'info> {
    pub token_account: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseWsolTokenAccountKeys {
    pub token_account: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseWsolTokenAccountAccounts<'_, '_>> for CloseWsolTokenAccountKeys {
    fn from(accounts: CloseWsolTokenAccountAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseWsolTokenAccountKeys>
for [AccountMeta; CLOSE_WSOL_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseWsolTokenAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
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
impl From<[Pubkey; CLOSE_WSOL_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseWsolTokenAccountKeys {
    fn from(pubkeys: [Pubkey; CLOSE_WSOL_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            user: pubkeys[1],
            token_program: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CloseWsolTokenAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_WSOL_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseWsolTokenAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.token_account.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_WSOL_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseWsolTokenAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_WSOL_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_account: &arr[0],
            user: &arr[1],
            token_program: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CLOSE_WSOL_TOKEN_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    203, 129, 103, 133, 197, 125, 107, 86,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseWsolTokenAccountIxData;
impl CloseWsolTokenAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_WSOL_TOKEN_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_WSOL_TOKEN_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_wsol_token_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseWsolTokenAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_WSOL_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseWsolTokenAccountIxData.try_to_vec()?,
    })
}
pub fn close_wsol_token_account_ix(
    keys: CloseWsolTokenAccountKeys,
) -> std::io::Result<Instruction> {
    close_wsol_token_account_ix_with_program_id(JUPITER_PROGRAM_ID, keys)
}
pub fn close_wsol_token_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseWsolTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseWsolTokenAccountKeys = accounts.into();
    let ix = close_wsol_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_wsol_token_account_invoke(
    accounts: CloseWsolTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    close_wsol_token_account_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts)
}
pub fn close_wsol_token_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseWsolTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseWsolTokenAccountKeys = accounts.into();
    let ix = close_wsol_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_wsol_token_account_invoke_signed(
    accounts: CloseWsolTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_wsol_token_account_invoke_signed_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_wsol_token_account_verify_account_keys(
    accounts: CloseWsolTokenAccountAccounts<'_, '_>,
    keys: CloseWsolTokenAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_account.key, keys.token_account),
        (*accounts.user.key, keys.user),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_wsol_token_account_verify_writable_privileges<'me, 'info>(
    accounts: CloseWsolTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_wsol_token_account_verify_signer_privileges<'me, 'info>(
    accounts: CloseWsolTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_wsol_token_account_verify_account_privileges<'me, 'info>(
    accounts: CloseWsolTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_wsol_token_account_verify_writable_privileges(accounts)?;
    close_wsol_token_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct ExactOutRouteAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub user_source_token_account: &'me AccountInfo<'info>,
    pub user_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub platform_fee_account: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExactOutRouteKeys {
    pub token_program: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Pubkey,
    pub token_2022_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ExactOutRouteAccounts<'_, '_>> for ExactOutRouteKeys {
    fn from(accounts: ExactOutRouteAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            user_source_token_account: *accounts.user_source_token_account.key,
            user_destination_token_account: *accounts.user_destination_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            platform_fee_account: *accounts.platform_fee_account.key,
            token_2022_program: *accounts.token_2022_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ExactOutRouteKeys> for [AccountMeta; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN] {
    fn from(keys: ExactOutRouteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
impl From<[Pubkey; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]> for ExactOutRouteKeys {
    fn from(pubkeys: [Pubkey; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            user_transfer_authority: pubkeys[1],
            user_source_token_account: pubkeys[2],
            user_destination_token_account: pubkeys[3],
            destination_token_account: pubkeys[4],
            source_mint: pubkeys[5],
            destination_mint: pubkeys[6],
            platform_fee_account: pubkeys[7],
            token_2022_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<ExactOutRouteAccounts<'_, 'info>>
for [AccountInfo<'info>; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExactOutRouteAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.user_source_token_account.clone(),
            accounts.user_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.platform_fee_account.clone(),
            accounts.token_2022_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]>
for ExactOutRouteAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            user_transfer_authority: &arr[1],
            user_source_token_account: &arr[2],
            user_destination_token_account: &arr[3],
            destination_token_account: &arr[4],
            source_mint: &arr[5],
            destination_mint: &arr[6],
            platform_fee_account: &arr[7],
            token_2022_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const EXACT_OUT_ROUTE_IX_DISCM: [u8; 8usize] = [208, 51, 239, 151, 123, 43, 237, 92];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExactOutRouteIxArgs {
    pub route_plan: Vec<RoutePlanStep>,
    pub out_amount: u64,
    pub quoted_in_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExactOutRouteIxData(pub ExactOutRouteIxArgs);
impl From<ExactOutRouteIxArgs> for ExactOutRouteIxData {
    fn from(args: ExactOutRouteIxArgs) -> Self {
        Self(args)
    }
}
impl ExactOutRouteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXACT_OUT_ROUTE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let route_plan: Vec<RoutePlanStep> = crate::borsh_de_or_default(&mut reader)?;
        let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quoted_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExactOutRouteIxArgs {
                route_plan,
                out_amount,
                quoted_in_amount,
                slippage_bps,
                platform_fee_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXACT_OUT_ROUTE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.route_plan, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quoted_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.platform_fee_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn exact_out_route_ix_with_program_id(
    program_id: Pubkey,
    keys: ExactOutRouteKeys,
    args: ExactOutRouteIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExactOutRouteIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn exact_out_route_ix(
    keys: ExactOutRouteKeys,
    args: ExactOutRouteIxArgs,
) -> std::io::Result<Instruction> {
    exact_out_route_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn exact_out_route_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExactOutRouteAccounts<'_, '_>,
    args: ExactOutRouteIxArgs,
) -> ProgramResult {
    let keys: ExactOutRouteKeys = accounts.into();
    let ix = exact_out_route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn exact_out_route_invoke(
    accounts: ExactOutRouteAccounts<'_, '_>,
    args: ExactOutRouteIxArgs,
) -> ProgramResult {
    exact_out_route_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts, args)
}
pub fn exact_out_route_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExactOutRouteAccounts<'_, '_>,
    args: ExactOutRouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExactOutRouteKeys = accounts.into();
    let ix = exact_out_route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn exact_out_route_invoke_signed(
    accounts: ExactOutRouteAccounts<'_, '_>,
    args: ExactOutRouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    exact_out_route_invoke_signed_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn exact_out_route_verify_account_keys(
    accounts: ExactOutRouteAccounts<'_, '_>,
    keys: ExactOutRouteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.user_source_token_account.key, keys.user_source_token_account),
        (
            *accounts.user_destination_token_account.key,
            keys.user_destination_token_account,
        ),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.platform_fee_account.key, keys.platform_fee_account),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn exact_out_route_verify_writable_privileges<'me, 'info>(
    accounts: ExactOutRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_source_token_account,
        accounts.user_destination_token_account,
        accounts.destination_token_account,
        accounts.platform_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn exact_out_route_verify_signer_privileges<'me, 'info>(
    accounts: ExactOutRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn exact_out_route_verify_account_privileges<'me, 'info>(
    accounts: ExactOutRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    exact_out_route_verify_writable_privileges(accounts)?;
    exact_out_route_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ROUTE_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct RouteAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub user_source_token_account: &'me AccountInfo<'info>,
    pub user_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub platform_fee_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RouteKeys {
    pub token_program: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RouteAccounts<'_, '_>> for RouteKeys {
    fn from(accounts: RouteAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            user_source_token_account: *accounts.user_source_token_account.key,
            user_destination_token_account: *accounts.user_destination_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
            destination_mint: *accounts.destination_mint.key,
            platform_fee_account: *accounts.platform_fee_account.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RouteKeys> for [AccountMeta; ROUTE_IX_ACCOUNTS_LEN] {
    fn from(keys: RouteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_account,
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
impl From<[Pubkey; ROUTE_IX_ACCOUNTS_LEN]> for RouteKeys {
    fn from(pubkeys: [Pubkey; ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            user_transfer_authority: pubkeys[1],
            user_source_token_account: pubkeys[2],
            user_destination_token_account: pubkeys[3],
            destination_token_account: pubkeys[4],
            destination_mint: pubkeys[5],
            platform_fee_account: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<RouteAccounts<'_, 'info>>
for [AccountInfo<'info>; ROUTE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RouteAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.user_source_token_account.clone(),
            accounts.user_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.destination_mint.clone(),
            accounts.platform_fee_account.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ROUTE_IX_ACCOUNTS_LEN]>
for RouteAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            user_transfer_authority: &arr[1],
            user_source_token_account: &arr[2],
            user_destination_token_account: &arr[3],
            destination_token_account: &arr[4],
            destination_mint: &arr[5],
            platform_fee_account: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const ROUTE_IX_DISCM: [u8; 8usize] = [229, 23, 203, 151, 122, 227, 173, 42];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RouteIxArgs {
    pub route_plan: Vec<RoutePlanStep>,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RouteIxData(pub RouteIxArgs);
impl From<RouteIxArgs> for RouteIxData {
    fn from(args: RouteIxArgs) -> Self {
        Self(args)
    }
}
impl RouteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ROUTE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let route_plan: Vec<RoutePlanStep> = crate::borsh_de_or_default(&mut reader)?;
        let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quoted_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RouteIxArgs {
                route_plan,
                in_amount,
                quoted_out_amount,
                slippage_bps,
                platform_fee_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ROUTE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.route_plan, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quoted_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.platform_fee_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn route_ix_with_program_id(
    program_id: Pubkey,
    keys: RouteKeys,
    args: RouteIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ROUTE_IX_ACCOUNTS_LEN] = keys.into();
    let data: RouteIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn route_ix(keys: RouteKeys, args: RouteIxArgs) -> std::io::Result<Instruction> {
    route_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn route_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RouteAccounts<'_, '_>,
    args: RouteIxArgs,
) -> ProgramResult {
    let keys: RouteKeys = accounts.into();
    let ix = route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn route_invoke(
    accounts: RouteAccounts<'_, '_>,
    args: RouteIxArgs,
) -> ProgramResult {
    route_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts, args)
}
pub fn route_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RouteAccounts<'_, '_>,
    args: RouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RouteKeys = accounts.into();
    let ix = route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn route_invoke_signed(
    accounts: RouteAccounts<'_, '_>,
    args: RouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    route_invoke_signed_with_program_id(JUPITER_PROGRAM_ID, accounts, args, seeds)
}
pub fn route_verify_account_keys(
    accounts: RouteAccounts<'_, '_>,
    keys: RouteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.user_source_token_account.key, keys.user_source_token_account),
        (
            *accounts.user_destination_token_account.key,
            keys.user_destination_token_account,
        ),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.platform_fee_account.key, keys.platform_fee_account),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn route_verify_writable_privileges<'me, 'info>(
    accounts: RouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_source_token_account,
        accounts.user_destination_token_account,
        accounts.destination_token_account,
        accounts.platform_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn route_verify_signer_privileges<'me, 'info>(
    accounts: RouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn route_verify_account_privileges<'me, 'info>(
    accounts: RouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    route_verify_writable_privileges(accounts)?;
    route_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct RouteWithTokenLedgerAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub user_source_token_account: &'me AccountInfo<'info>,
    pub user_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub platform_fee_account: &'me AccountInfo<'info>,
    pub token_ledger: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RouteWithTokenLedgerKeys {
    pub token_program: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Pubkey,
    pub token_ledger: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RouteWithTokenLedgerAccounts<'_, '_>> for RouteWithTokenLedgerKeys {
    fn from(accounts: RouteWithTokenLedgerAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            user_source_token_account: *accounts.user_source_token_account.key,
            user_destination_token_account: *accounts.user_destination_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
            destination_mint: *accounts.destination_mint.key,
            platform_fee_account: *accounts.platform_fee_account.key,
            token_ledger: *accounts.token_ledger.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RouteWithTokenLedgerKeys>
for [AccountMeta; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(keys: RouteWithTokenLedgerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_ledger,
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
impl From<[Pubkey; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
for RouteWithTokenLedgerKeys {
    fn from(pubkeys: [Pubkey; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            user_transfer_authority: pubkeys[1],
            user_source_token_account: pubkeys[2],
            user_destination_token_account: pubkeys[3],
            destination_token_account: pubkeys[4],
            destination_mint: pubkeys[5],
            platform_fee_account: pubkeys[6],
            token_ledger: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<RouteWithTokenLedgerAccounts<'_, 'info>>
for [AccountInfo<'info>; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: RouteWithTokenLedgerAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.user_source_token_account.clone(),
            accounts.user_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.destination_mint.clone(),
            accounts.platform_fee_account.clone(),
            accounts.token_ledger.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
for RouteWithTokenLedgerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_program: &arr[0],
            user_transfer_authority: &arr[1],
            user_source_token_account: &arr[2],
            user_destination_token_account: &arr[3],
            destination_token_account: &arr[4],
            destination_mint: &arr[5],
            platform_fee_account: &arr[6],
            token_ledger: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const ROUTE_WITH_TOKEN_LEDGER_IX_DISCM: [u8; 8usize] = [
    150, 86, 71, 116, 167, 93, 14, 104,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RouteWithTokenLedgerIxArgs {
    pub route_plan: Vec<RoutePlanStep>,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RouteWithTokenLedgerIxData(pub RouteWithTokenLedgerIxArgs);
impl From<RouteWithTokenLedgerIxArgs> for RouteWithTokenLedgerIxData {
    fn from(args: RouteWithTokenLedgerIxArgs) -> Self {
        Self(args)
    }
}
impl RouteWithTokenLedgerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ROUTE_WITH_TOKEN_LEDGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let route_plan: Vec<RoutePlanStep> = crate::borsh_de_or_default(&mut reader)?;
        let quoted_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RouteWithTokenLedgerIxArgs {
                route_plan,
                quoted_out_amount,
                slippage_bps,
                platform_fee_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ROUTE_WITH_TOKEN_LEDGER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.route_plan, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quoted_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.platform_fee_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn route_with_token_ledger_ix_with_program_id(
    program_id: Pubkey,
    keys: RouteWithTokenLedgerKeys,
    args: RouteWithTokenLedgerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN] = keys.into();
    let data: RouteWithTokenLedgerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn route_with_token_ledger_ix(
    keys: RouteWithTokenLedgerKeys,
    args: RouteWithTokenLedgerIxArgs,
) -> std::io::Result<Instruction> {
    route_with_token_ledger_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn route_with_token_ledger_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RouteWithTokenLedgerAccounts<'_, '_>,
    args: RouteWithTokenLedgerIxArgs,
) -> ProgramResult {
    let keys: RouteWithTokenLedgerKeys = accounts.into();
    let ix = route_with_token_ledger_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn route_with_token_ledger_invoke(
    accounts: RouteWithTokenLedgerAccounts<'_, '_>,
    args: RouteWithTokenLedgerIxArgs,
) -> ProgramResult {
    route_with_token_ledger_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts, args)
}
pub fn route_with_token_ledger_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RouteWithTokenLedgerAccounts<'_, '_>,
    args: RouteWithTokenLedgerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RouteWithTokenLedgerKeys = accounts.into();
    let ix = route_with_token_ledger_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn route_with_token_ledger_invoke_signed(
    accounts: RouteWithTokenLedgerAccounts<'_, '_>,
    args: RouteWithTokenLedgerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    route_with_token_ledger_invoke_signed_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn route_with_token_ledger_verify_account_keys(
    accounts: RouteWithTokenLedgerAccounts<'_, '_>,
    keys: RouteWithTokenLedgerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.user_source_token_account.key, keys.user_source_token_account),
        (
            *accounts.user_destination_token_account.key,
            keys.user_destination_token_account,
        ),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.platform_fee_account.key, keys.platform_fee_account),
        (*accounts.token_ledger.key, keys.token_ledger),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn route_with_token_ledger_verify_writable_privileges<'me, 'info>(
    accounts: RouteWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_source_token_account,
        accounts.user_destination_token_account,
        accounts.destination_token_account,
        accounts.platform_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn route_with_token_ledger_verify_signer_privileges<'me, 'info>(
    accounts: RouteWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn route_with_token_ledger_verify_account_privileges<'me, 'info>(
    accounts: RouteWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    route_with_token_ledger_verify_writable_privileges(accounts)?;
    route_with_token_ledger_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetTokenLedgerAccounts<'me, 'info> {
    pub token_ledger: &'me AccountInfo<'info>,
    pub token_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetTokenLedgerKeys {
    pub token_ledger: Pubkey,
    pub token_account: Pubkey,
}
impl From<SetTokenLedgerAccounts<'_, '_>> for SetTokenLedgerKeys {
    fn from(accounts: SetTokenLedgerAccounts) -> Self {
        Self {
            token_ledger: *accounts.token_ledger.key,
            token_account: *accounts.token_account.key,
        }
    }
}
impl From<SetTokenLedgerKeys> for [AccountMeta; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetTokenLedgerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_ledger,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]> for SetTokenLedgerKeys {
    fn from(pubkeys: [Pubkey; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_ledger: pubkeys[0],
            token_account: pubkeys[1],
        }
    }
}
impl<'info> From<SetTokenLedgerAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetTokenLedgerAccounts<'_, 'info>) -> Self {
        [accounts.token_ledger.clone(), accounts.token_account.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
for SetTokenLedgerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_ledger: &arr[0],
            token_account: &arr[1],
        }
    }
}
pub const SET_TOKEN_LEDGER_IX_DISCM: [u8; 8usize] = [228, 85, 185, 112, 78, 79, 77, 2];
#[derive(Clone, Debug, PartialEq)]
pub struct SetTokenLedgerIxData;
impl SetTokenLedgerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_TOKEN_LEDGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_TOKEN_LEDGER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_token_ledger_ix_with_program_id(
    program_id: Pubkey,
    keys: SetTokenLedgerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetTokenLedgerIxData.try_to_vec()?,
    })
}
pub fn set_token_ledger_ix(keys: SetTokenLedgerKeys) -> std::io::Result<Instruction> {
    set_token_ledger_ix_with_program_id(JUPITER_PROGRAM_ID, keys)
}
pub fn set_token_ledger_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetTokenLedgerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetTokenLedgerKeys = accounts.into();
    let ix = set_token_ledger_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_token_ledger_invoke(
    accounts: SetTokenLedgerAccounts<'_, '_>,
) -> ProgramResult {
    set_token_ledger_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts)
}
pub fn set_token_ledger_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetTokenLedgerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetTokenLedgerKeys = accounts.into();
    let ix = set_token_ledger_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_token_ledger_invoke_signed(
    accounts: SetTokenLedgerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_token_ledger_invoke_signed_with_program_id(JUPITER_PROGRAM_ID, accounts, seeds)
}
pub fn set_token_ledger_verify_account_keys(
    accounts: SetTokenLedgerAccounts<'_, '_>,
    keys: SetTokenLedgerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_ledger.key, keys.token_ledger),
        (*accounts.token_account.key, keys.token_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_token_ledger_verify_writable_privileges<'me, 'info>(
    accounts: SetTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_ledger] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_token_ledger_verify_account_privileges<'me, 'info>(
    accounts: SetTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_token_ledger_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SharedAccountsExactOutRouteAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source_token_account: &'me AccountInfo<'info>,
    pub program_source_token_account: &'me AccountInfo<'info>,
    pub program_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub platform_fee_account: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SharedAccountsExactOutRouteKeys {
    pub token_program: Pubkey,
    pub program_authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token_account: Pubkey,
    pub program_source_token_account: Pubkey,
    pub program_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Pubkey,
    pub token_2022_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SharedAccountsExactOutRouteAccounts<'_, '_>>
for SharedAccountsExactOutRouteKeys {
    fn from(accounts: SharedAccountsExactOutRouteAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            program_authority: *accounts.program_authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source_token_account: *accounts.source_token_account.key,
            program_source_token_account: *accounts.program_source_token_account.key,
            program_destination_token_account: *accounts
                .program_destination_token_account
                .key,
            destination_token_account: *accounts.destination_token_account.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            platform_fee_account: *accounts.platform_fee_account.key,
            token_2022_program: *accounts.token_2022_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SharedAccountsExactOutRouteKeys>
for [AccountMeta; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN] {
    fn from(keys: SharedAccountsExactOutRouteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
impl From<[Pubkey; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]>
for SharedAccountsExactOutRouteKeys {
    fn from(pubkeys: [Pubkey; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            program_authority: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            source_token_account: pubkeys[3],
            program_source_token_account: pubkeys[4],
            program_destination_token_account: pubkeys[5],
            destination_token_account: pubkeys[6],
            source_mint: pubkeys[7],
            destination_mint: pubkeys[8],
            platform_fee_account: pubkeys[9],
            token_2022_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<SharedAccountsExactOutRouteAccounts<'_, 'info>>
for [AccountInfo<'info>; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SharedAccountsExactOutRouteAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.program_authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_token_account.clone(),
            accounts.program_source_token_account.clone(),
            accounts.program_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.platform_fee_account.clone(),
            accounts.token_2022_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]>
for SharedAccountsExactOutRouteAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_program: &arr[0],
            program_authority: &arr[1],
            user_transfer_authority: &arr[2],
            source_token_account: &arr[3],
            program_source_token_account: &arr[4],
            program_destination_token_account: &arr[5],
            destination_token_account: &arr[6],
            source_mint: &arr[7],
            destination_mint: &arr[8],
            platform_fee_account: &arr[9],
            token_2022_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_DISCM: [u8; 8usize] = [
    176, 209, 105, 168, 154, 125, 69, 62,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharedAccountsExactOutRouteIxArgs {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub out_amount: u64,
    pub quoted_in_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SharedAccountsExactOutRouteIxData(pub SharedAccountsExactOutRouteIxArgs);
impl From<SharedAccountsExactOutRouteIxArgs> for SharedAccountsExactOutRouteIxData {
    fn from(args: SharedAccountsExactOutRouteIxArgs) -> Self {
        Self(args)
    }
}
impl SharedAccountsExactOutRouteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let route_plan: Vec<RoutePlanStep> = crate::borsh_de_or_default(&mut reader)?;
        let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quoted_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SharedAccountsExactOutRouteIxArgs {
                id,
                route_plan,
                out_amount,
                quoted_in_amount,
                slippage_bps,
                platform_fee_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.route_plan, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quoted_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.platform_fee_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn shared_accounts_exact_out_route_ix_with_program_id(
    program_id: Pubkey,
    keys: SharedAccountsExactOutRouteKeys,
    args: SharedAccountsExactOutRouteIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SharedAccountsExactOutRouteIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn shared_accounts_exact_out_route_ix(
    keys: SharedAccountsExactOutRouteKeys,
    args: SharedAccountsExactOutRouteIxArgs,
) -> std::io::Result<Instruction> {
    shared_accounts_exact_out_route_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn shared_accounts_exact_out_route_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsExactOutRouteAccounts<'_, '_>,
    args: SharedAccountsExactOutRouteIxArgs,
) -> ProgramResult {
    let keys: SharedAccountsExactOutRouteKeys = accounts.into();
    let ix = shared_accounts_exact_out_route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn shared_accounts_exact_out_route_invoke(
    accounts: SharedAccountsExactOutRouteAccounts<'_, '_>,
    args: SharedAccountsExactOutRouteIxArgs,
) -> ProgramResult {
    shared_accounts_exact_out_route_invoke_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn shared_accounts_exact_out_route_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsExactOutRouteAccounts<'_, '_>,
    args: SharedAccountsExactOutRouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SharedAccountsExactOutRouteKeys = accounts.into();
    let ix = shared_accounts_exact_out_route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn shared_accounts_exact_out_route_invoke_signed(
    accounts: SharedAccountsExactOutRouteAccounts<'_, '_>,
    args: SharedAccountsExactOutRouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    shared_accounts_exact_out_route_invoke_signed_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn shared_accounts_exact_out_route_verify_account_keys(
    accounts: SharedAccountsExactOutRouteAccounts<'_, '_>,
    keys: SharedAccountsExactOutRouteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.source_token_account.key, keys.source_token_account),
        (*accounts.program_source_token_account.key, keys.program_source_token_account),
        (
            *accounts.program_destination_token_account.key,
            keys.program_destination_token_account,
        ),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.platform_fee_account.key, keys.platform_fee_account),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn shared_accounts_exact_out_route_verify_writable_privileges<'me, 'info>(
    accounts: SharedAccountsExactOutRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source_token_account,
        accounts.program_source_token_account,
        accounts.program_destination_token_account,
        accounts.destination_token_account,
        accounts.platform_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn shared_accounts_exact_out_route_verify_signer_privileges<'me, 'info>(
    accounts: SharedAccountsExactOutRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn shared_accounts_exact_out_route_verify_account_privileges<'me, 'info>(
    accounts: SharedAccountsExactOutRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    shared_accounts_exact_out_route_verify_writable_privileges(accounts)?;
    shared_accounts_exact_out_route_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SharedAccountsRouteAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source_token_account: &'me AccountInfo<'info>,
    pub program_source_token_account: &'me AccountInfo<'info>,
    pub program_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub platform_fee_account: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SharedAccountsRouteKeys {
    pub token_program: Pubkey,
    pub program_authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token_account: Pubkey,
    pub program_source_token_account: Pubkey,
    pub program_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Pubkey,
    pub token_2022_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SharedAccountsRouteAccounts<'_, '_>> for SharedAccountsRouteKeys {
    fn from(accounts: SharedAccountsRouteAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            program_authority: *accounts.program_authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source_token_account: *accounts.source_token_account.key,
            program_source_token_account: *accounts.program_source_token_account.key,
            program_destination_token_account: *accounts
                .program_destination_token_account
                .key,
            destination_token_account: *accounts.destination_token_account.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            platform_fee_account: *accounts.platform_fee_account.key,
            token_2022_program: *accounts.token_2022_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SharedAccountsRouteKeys>
for [AccountMeta; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN] {
    fn from(keys: SharedAccountsRouteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
impl From<[Pubkey; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN]> for SharedAccountsRouteKeys {
    fn from(pubkeys: [Pubkey; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            program_authority: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            source_token_account: pubkeys[3],
            program_source_token_account: pubkeys[4],
            program_destination_token_account: pubkeys[5],
            destination_token_account: pubkeys[6],
            source_mint: pubkeys[7],
            destination_mint: pubkeys[8],
            platform_fee_account: pubkeys[9],
            token_2022_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<SharedAccountsRouteAccounts<'_, 'info>>
for [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SharedAccountsRouteAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.program_authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_token_account.clone(),
            accounts.program_source_token_account.clone(),
            accounts.program_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.platform_fee_account.clone(),
            accounts.token_2022_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN]>
for SharedAccountsRouteAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_program: &arr[0],
            program_authority: &arr[1],
            user_transfer_authority: &arr[2],
            source_token_account: &arr[3],
            program_source_token_account: &arr[4],
            program_destination_token_account: &arr[5],
            destination_token_account: &arr[6],
            source_mint: &arr[7],
            destination_mint: &arr[8],
            platform_fee_account: &arr[9],
            token_2022_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const SHARED_ACCOUNTS_ROUTE_IX_DISCM: [u8; 8usize] = [
    193, 32, 155, 51, 65, 214, 156, 129,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharedAccountsRouteIxArgs {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SharedAccountsRouteIxData(pub SharedAccountsRouteIxArgs);
impl From<SharedAccountsRouteIxArgs> for SharedAccountsRouteIxData {
    fn from(args: SharedAccountsRouteIxArgs) -> Self {
        Self(args)
    }
}
impl SharedAccountsRouteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHARED_ACCOUNTS_ROUTE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let route_plan: Vec<RoutePlanStep> = crate::borsh_de_or_default(&mut reader)?;
        let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quoted_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SharedAccountsRouteIxArgs {
                id,
                route_plan,
                in_amount,
                quoted_out_amount,
                slippage_bps,
                platform_fee_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHARED_ACCOUNTS_ROUTE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.route_plan, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quoted_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.platform_fee_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn shared_accounts_route_ix_with_program_id(
    program_id: Pubkey,
    keys: SharedAccountsRouteKeys,
    args: SharedAccountsRouteIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SharedAccountsRouteIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn shared_accounts_route_ix(
    keys: SharedAccountsRouteKeys,
    args: SharedAccountsRouteIxArgs,
) -> std::io::Result<Instruction> {
    shared_accounts_route_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn shared_accounts_route_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsRouteAccounts<'_, '_>,
    args: SharedAccountsRouteIxArgs,
) -> ProgramResult {
    let keys: SharedAccountsRouteKeys = accounts.into();
    let ix = shared_accounts_route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn shared_accounts_route_invoke(
    accounts: SharedAccountsRouteAccounts<'_, '_>,
    args: SharedAccountsRouteIxArgs,
) -> ProgramResult {
    shared_accounts_route_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts, args)
}
pub fn shared_accounts_route_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsRouteAccounts<'_, '_>,
    args: SharedAccountsRouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SharedAccountsRouteKeys = accounts.into();
    let ix = shared_accounts_route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn shared_accounts_route_invoke_signed(
    accounts: SharedAccountsRouteAccounts<'_, '_>,
    args: SharedAccountsRouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    shared_accounts_route_invoke_signed_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn shared_accounts_route_verify_account_keys(
    accounts: SharedAccountsRouteAccounts<'_, '_>,
    keys: SharedAccountsRouteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.source_token_account.key, keys.source_token_account),
        (*accounts.program_source_token_account.key, keys.program_source_token_account),
        (
            *accounts.program_destination_token_account.key,
            keys.program_destination_token_account,
        ),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.platform_fee_account.key, keys.platform_fee_account),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_verify_writable_privileges<'me, 'info>(
    accounts: SharedAccountsRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source_token_account,
        accounts.program_source_token_account,
        accounts.program_destination_token_account,
        accounts.destination_token_account,
        accounts.platform_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_verify_signer_privileges<'me, 'info>(
    accounts: SharedAccountsRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_verify_account_privileges<'me, 'info>(
    accounts: SharedAccountsRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    shared_accounts_route_verify_writable_privileges(accounts)?;
    shared_accounts_route_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SharedAccountsRouteWithTokenLedgerAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source_token_account: &'me AccountInfo<'info>,
    pub program_source_token_account: &'me AccountInfo<'info>,
    pub program_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub platform_fee_account: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub token_ledger: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SharedAccountsRouteWithTokenLedgerKeys {
    pub token_program: Pubkey,
    pub program_authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token_account: Pubkey,
    pub program_source_token_account: Pubkey,
    pub program_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Pubkey,
    pub token_2022_program: Pubkey,
    pub token_ledger: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SharedAccountsRouteWithTokenLedgerAccounts<'_, '_>>
for SharedAccountsRouteWithTokenLedgerKeys {
    fn from(accounts: SharedAccountsRouteWithTokenLedgerAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            program_authority: *accounts.program_authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source_token_account: *accounts.source_token_account.key,
            program_source_token_account: *accounts.program_source_token_account.key,
            program_destination_token_account: *accounts
                .program_destination_token_account
                .key,
            destination_token_account: *accounts.destination_token_account.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            platform_fee_account: *accounts.platform_fee_account.key,
            token_2022_program: *accounts.token_2022_program.key,
            token_ledger: *accounts.token_ledger.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SharedAccountsRouteWithTokenLedgerKeys>
for [AccountMeta; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(keys: SharedAccountsRouteWithTokenLedgerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_ledger,
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
impl From<[Pubkey; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
for SharedAccountsRouteWithTokenLedgerKeys {
    fn from(
        pubkeys: [Pubkey; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_program: pubkeys[0],
            program_authority: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            source_token_account: pubkeys[3],
            program_source_token_account: pubkeys[4],
            program_destination_token_account: pubkeys[5],
            destination_token_account: pubkeys[6],
            source_mint: pubkeys[7],
            destination_mint: pubkeys[8],
            platform_fee_account: pubkeys[9],
            token_2022_program: pubkeys[10],
            token_ledger: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<SharedAccountsRouteWithTokenLedgerAccounts<'_, 'info>>
for [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SharedAccountsRouteWithTokenLedgerAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.program_authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_token_account.clone(),
            accounts.program_source_token_account.clone(),
            accounts.program_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.platform_fee_account.clone(),
            accounts.token_2022_program.clone(),
            accounts.token_ledger.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN],
> for SharedAccountsRouteWithTokenLedgerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_program: &arr[0],
            program_authority: &arr[1],
            user_transfer_authority: &arr[2],
            source_token_account: &arr[3],
            program_source_token_account: &arr[4],
            program_destination_token_account: &arr[5],
            destination_token_account: &arr[6],
            source_mint: &arr[7],
            destination_mint: &arr[8],
            platform_fee_account: &arr[9],
            token_2022_program: &arr[10],
            token_ledger: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_DISCM: [u8; 8usize] = [
    230, 121, 143, 80, 119, 159, 106, 170,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharedAccountsRouteWithTokenLedgerIxArgs {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SharedAccountsRouteWithTokenLedgerIxData(
    pub SharedAccountsRouteWithTokenLedgerIxArgs,
);
impl From<SharedAccountsRouteWithTokenLedgerIxArgs>
for SharedAccountsRouteWithTokenLedgerIxData {
    fn from(args: SharedAccountsRouteWithTokenLedgerIxArgs) -> Self {
        Self(args)
    }
}
impl SharedAccountsRouteWithTokenLedgerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let route_plan: Vec<RoutePlanStep> = crate::borsh_de_or_default(&mut reader)?;
        let quoted_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SharedAccountsRouteWithTokenLedgerIxArgs {
                id,
                route_plan,
                quoted_out_amount,
                slippage_bps,
                platform_fee_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.route_plan, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quoted_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.platform_fee_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn shared_accounts_route_with_token_ledger_ix_with_program_id(
    program_id: Pubkey,
    keys: SharedAccountsRouteWithTokenLedgerKeys,
    args: SharedAccountsRouteWithTokenLedgerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SharedAccountsRouteWithTokenLedgerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn shared_accounts_route_with_token_ledger_ix(
    keys: SharedAccountsRouteWithTokenLedgerKeys,
    args: SharedAccountsRouteWithTokenLedgerIxArgs,
) -> std::io::Result<Instruction> {
    shared_accounts_route_with_token_ledger_ix_with_program_id(
        JUPITER_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn shared_accounts_route_with_token_ledger_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'_, '_>,
    args: SharedAccountsRouteWithTokenLedgerIxArgs,
) -> ProgramResult {
    let keys: SharedAccountsRouteWithTokenLedgerKeys = accounts.into();
    let ix = shared_accounts_route_with_token_ledger_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn shared_accounts_route_with_token_ledger_invoke(
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'_, '_>,
    args: SharedAccountsRouteWithTokenLedgerIxArgs,
) -> ProgramResult {
    shared_accounts_route_with_token_ledger_invoke_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn shared_accounts_route_with_token_ledger_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'_, '_>,
    args: SharedAccountsRouteWithTokenLedgerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SharedAccountsRouteWithTokenLedgerKeys = accounts.into();
    let ix = shared_accounts_route_with_token_ledger_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn shared_accounts_route_with_token_ledger_invoke_signed(
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'_, '_>,
    args: SharedAccountsRouteWithTokenLedgerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    shared_accounts_route_with_token_ledger_invoke_signed_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn shared_accounts_route_with_token_ledger_verify_account_keys(
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'_, '_>,
    keys: SharedAccountsRouteWithTokenLedgerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.source_token_account.key, keys.source_token_account),
        (*accounts.program_source_token_account.key, keys.program_source_token_account),
        (
            *accounts.program_destination_token_account.key,
            keys.program_destination_token_account,
        ),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.platform_fee_account.key, keys.platform_fee_account),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.token_ledger.key, keys.token_ledger),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_with_token_ledger_verify_writable_privileges<'me, 'info>(
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source_token_account,
        accounts.program_source_token_account,
        accounts.program_destination_token_account,
        accounts.destination_token_account,
        accounts.platform_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_with_token_ledger_verify_signer_privileges<'me, 'info>(
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_with_token_ledger_verify_account_privileges<'me, 'info>(
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    shared_accounts_route_with_token_ledger_verify_writable_privileges(accounts)?;
    shared_accounts_route_with_token_ledger_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ExactOutRouteV2Accounts<'me, 'info> {
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub user_source_token_account: &'me AccountInfo<'info>,
    pub user_destination_token_account: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub source_token_program: &'me AccountInfo<'info>,
    pub destination_token_program: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExactOutRouteV2Keys {
    pub user_transfer_authority: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub source_token_program: Pubkey,
    pub destination_token_program: Pubkey,
    pub destination_token_account: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ExactOutRouteV2Accounts<'_, '_>> for ExactOutRouteV2Keys {
    fn from(accounts: ExactOutRouteV2Accounts) -> Self {
        Self {
            user_transfer_authority: *accounts.user_transfer_authority.key,
            user_source_token_account: *accounts.user_source_token_account.key,
            user_destination_token_account: *accounts.user_destination_token_account.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            source_token_program: *accounts.source_token_program.key,
            destination_token_program: *accounts.destination_token_program.key,
            destination_token_account: *accounts.destination_token_account.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ExactOutRouteV2Keys> for [AccountMeta; EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: ExactOutRouteV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
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
impl From<[Pubkey; EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN]> for ExactOutRouteV2Keys {
    fn from(pubkeys: [Pubkey; EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: pubkeys[0],
            user_source_token_account: pubkeys[1],
            user_destination_token_account: pubkeys[2],
            source_mint: pubkeys[3],
            destination_mint: pubkeys[4],
            source_token_program: pubkeys[5],
            destination_token_program: pubkeys[6],
            destination_token_account: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<ExactOutRouteV2Accounts<'_, 'info>>
for [AccountInfo<'info>; EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExactOutRouteV2Accounts<'_, 'info>) -> Self {
        [
            accounts.user_transfer_authority.clone(),
            accounts.user_source_token_account.clone(),
            accounts.user_destination_token_account.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.source_token_program.clone(),
            accounts.destination_token_program.clone(),
            accounts.destination_token_account.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN]>
for ExactOutRouteV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: &arr[0],
            user_source_token_account: &arr[1],
            user_destination_token_account: &arr[2],
            source_mint: &arr[3],
            destination_mint: &arr[4],
            source_token_program: &arr[5],
            destination_token_program: &arr[6],
            destination_token_account: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const EXACT_OUT_ROUTE_V2_IX_DISCM: [u8; 8usize] = [
    157, 138, 184, 82, 21, 244, 243, 36,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExactOutRouteV2IxArgs {
    pub out_amount: u64,
    pub quoted_in_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u16,
    pub positive_slippage_bps: u16,
    pub route_plan: Vec<RoutePlanStepV2>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExactOutRouteV2IxData(pub ExactOutRouteV2IxArgs);
impl From<ExactOutRouteV2IxArgs> for ExactOutRouteV2IxData {
    fn from(args: ExactOutRouteV2IxArgs) -> Self {
        Self(args)
    }
}
impl ExactOutRouteV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXACT_OUT_ROUTE_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quoted_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let positive_slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let route_plan: Vec<RoutePlanStepV2> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExactOutRouteV2IxArgs {
                out_amount,
                quoted_in_amount,
                slippage_bps,
                platform_fee_bps,
                positive_slippage_bps,
                route_plan,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXACT_OUT_ROUTE_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quoted_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.platform_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.positive_slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.route_plan, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn exact_out_route_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: ExactOutRouteV2Keys,
    args: ExactOutRouteV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExactOutRouteV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn exact_out_route_v2_ix(
    keys: ExactOutRouteV2Keys,
    args: ExactOutRouteV2IxArgs,
) -> std::io::Result<Instruction> {
    exact_out_route_v2_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn exact_out_route_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExactOutRouteV2Accounts<'_, '_>,
    args: ExactOutRouteV2IxArgs,
) -> ProgramResult {
    let keys: ExactOutRouteV2Keys = accounts.into();
    let ix = exact_out_route_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn exact_out_route_v2_invoke(
    accounts: ExactOutRouteV2Accounts<'_, '_>,
    args: ExactOutRouteV2IxArgs,
) -> ProgramResult {
    exact_out_route_v2_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts, args)
}
pub fn exact_out_route_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExactOutRouteV2Accounts<'_, '_>,
    args: ExactOutRouteV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExactOutRouteV2Keys = accounts.into();
    let ix = exact_out_route_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn exact_out_route_v2_invoke_signed(
    accounts: ExactOutRouteV2Accounts<'_, '_>,
    args: ExactOutRouteV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    exact_out_route_v2_invoke_signed_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn exact_out_route_v2_verify_account_keys(
    accounts: ExactOutRouteV2Accounts<'_, '_>,
    keys: ExactOutRouteV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.user_source_token_account.key, keys.user_source_token_account),
        (
            *accounts.user_destination_token_account.key,
            keys.user_destination_token_account,
        ),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.source_token_program.key, keys.source_token_program),
        (*accounts.destination_token_program.key, keys.destination_token_program),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn exact_out_route_v2_verify_writable_privileges<'me, 'info>(
    accounts: ExactOutRouteV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_source_token_account,
        accounts.user_destination_token_account,
        accounts.destination_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn exact_out_route_v2_verify_signer_privileges<'me, 'info>(
    accounts: ExactOutRouteV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn exact_out_route_v2_verify_account_privileges<'me, 'info>(
    accounts: ExactOutRouteV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    exact_out_route_v2_verify_writable_privileges(accounts)?;
    exact_out_route_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ROUTE_V2_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct RouteV2Accounts<'me, 'info> {
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub user_source_token_account: &'me AccountInfo<'info>,
    pub user_destination_token_account: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub source_token_program: &'me AccountInfo<'info>,
    pub destination_token_program: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RouteV2Keys {
    pub user_transfer_authority: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub source_token_program: Pubkey,
    pub destination_token_program: Pubkey,
    pub destination_token_account: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RouteV2Accounts<'_, '_>> for RouteV2Keys {
    fn from(accounts: RouteV2Accounts) -> Self {
        Self {
            user_transfer_authority: *accounts.user_transfer_authority.key,
            user_source_token_account: *accounts.user_source_token_account.key,
            user_destination_token_account: *accounts.user_destination_token_account.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            source_token_program: *accounts.source_token_program.key,
            destination_token_program: *accounts.destination_token_program.key,
            destination_token_account: *accounts.destination_token_account.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RouteV2Keys> for [AccountMeta; ROUTE_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: RouteV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
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
impl From<[Pubkey; ROUTE_V2_IX_ACCOUNTS_LEN]> for RouteV2Keys {
    fn from(pubkeys: [Pubkey; ROUTE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: pubkeys[0],
            user_source_token_account: pubkeys[1],
            user_destination_token_account: pubkeys[2],
            source_mint: pubkeys[3],
            destination_mint: pubkeys[4],
            source_token_program: pubkeys[5],
            destination_token_program: pubkeys[6],
            destination_token_account: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<RouteV2Accounts<'_, 'info>>
for [AccountInfo<'info>; ROUTE_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: RouteV2Accounts<'_, 'info>) -> Self {
        [
            accounts.user_transfer_authority.clone(),
            accounts.user_source_token_account.clone(),
            accounts.user_destination_token_account.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.source_token_program.clone(),
            accounts.destination_token_program.clone(),
            accounts.destination_token_account.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ROUTE_V2_IX_ACCOUNTS_LEN]>
for RouteV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ROUTE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: &arr[0],
            user_source_token_account: &arr[1],
            user_destination_token_account: &arr[2],
            source_mint: &arr[3],
            destination_mint: &arr[4],
            source_token_program: &arr[5],
            destination_token_program: &arr[6],
            destination_token_account: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const ROUTE_V2_IX_DISCM: [u8; 8usize] = [187, 100, 250, 204, 49, 196, 175, 20];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RouteV2IxArgs {
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u16,
    pub positive_slippage_bps: u16,
    pub route_plan: Vec<RoutePlanStepV2>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RouteV2IxData(pub RouteV2IxArgs);
impl From<RouteV2IxArgs> for RouteV2IxData {
    fn from(args: RouteV2IxArgs) -> Self {
        Self(args)
    }
}
impl RouteV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ROUTE_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quoted_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let positive_slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let route_plan: Vec<RoutePlanStepV2> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RouteV2IxArgs {
                in_amount,
                quoted_out_amount,
                slippage_bps,
                platform_fee_bps,
                positive_slippage_bps,
                route_plan,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ROUTE_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quoted_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.platform_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.positive_slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.route_plan, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn route_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: RouteV2Keys,
    args: RouteV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ROUTE_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: RouteV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn route_v2_ix(
    keys: RouteV2Keys,
    args: RouteV2IxArgs,
) -> std::io::Result<Instruction> {
    route_v2_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn route_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RouteV2Accounts<'_, '_>,
    args: RouteV2IxArgs,
) -> ProgramResult {
    let keys: RouteV2Keys = accounts.into();
    let ix = route_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn route_v2_invoke(
    accounts: RouteV2Accounts<'_, '_>,
    args: RouteV2IxArgs,
) -> ProgramResult {
    route_v2_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts, args)
}
pub fn route_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RouteV2Accounts<'_, '_>,
    args: RouteV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RouteV2Keys = accounts.into();
    let ix = route_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn route_v2_invoke_signed(
    accounts: RouteV2Accounts<'_, '_>,
    args: RouteV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    route_v2_invoke_signed_with_program_id(JUPITER_PROGRAM_ID, accounts, args, seeds)
}
pub fn route_v2_verify_account_keys(
    accounts: RouteV2Accounts<'_, '_>,
    keys: RouteV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.user_source_token_account.key, keys.user_source_token_account),
        (
            *accounts.user_destination_token_account.key,
            keys.user_destination_token_account,
        ),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.source_token_program.key, keys.source_token_program),
        (*accounts.destination_token_program.key, keys.destination_token_program),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn route_v2_verify_writable_privileges<'me, 'info>(
    accounts: RouteV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_source_token_account,
        accounts.user_destination_token_account,
        accounts.destination_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn route_v2_verify_signer_privileges<'me, 'info>(
    accounts: RouteV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn route_v2_verify_account_privileges<'me, 'info>(
    accounts: RouteV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    route_v2_verify_writable_privileges(accounts)?;
    route_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SharedAccountsExactOutRouteV2Accounts<'me, 'info> {
    pub program_authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source_token_account: &'me AccountInfo<'info>,
    pub program_source_token_account: &'me AccountInfo<'info>,
    pub program_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub source_token_program: &'me AccountInfo<'info>,
    pub destination_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SharedAccountsExactOutRouteV2Keys {
    pub program_authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token_account: Pubkey,
    pub program_source_token_account: Pubkey,
    pub program_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub source_token_program: Pubkey,
    pub destination_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SharedAccountsExactOutRouteV2Accounts<'_, '_>>
for SharedAccountsExactOutRouteV2Keys {
    fn from(accounts: SharedAccountsExactOutRouteV2Accounts) -> Self {
        Self {
            program_authority: *accounts.program_authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source_token_account: *accounts.source_token_account.key,
            program_source_token_account: *accounts.program_source_token_account.key,
            program_destination_token_account: *accounts
                .program_destination_token_account
                .key,
            destination_token_account: *accounts.destination_token_account.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            source_token_program: *accounts.source_token_program.key,
            destination_token_program: *accounts.destination_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SharedAccountsExactOutRouteV2Keys>
for [AccountMeta; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: SharedAccountsExactOutRouteV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_token_program,
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
impl From<[Pubkey; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN]>
for SharedAccountsExactOutRouteV2Keys {
    fn from(
        pubkeys: [Pubkey; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            program_authority: pubkeys[0],
            user_transfer_authority: pubkeys[1],
            source_token_account: pubkeys[2],
            program_source_token_account: pubkeys[3],
            program_destination_token_account: pubkeys[4],
            destination_token_account: pubkeys[5],
            source_mint: pubkeys[6],
            destination_mint: pubkeys[7],
            source_token_program: pubkeys[8],
            destination_token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<SharedAccountsExactOutRouteV2Accounts<'_, 'info>>
for [AccountInfo<'info>; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: SharedAccountsExactOutRouteV2Accounts<'_, 'info>) -> Self {
        [
            accounts.program_authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_token_account.clone(),
            accounts.program_source_token_account.clone(),
            accounts.program_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.source_token_program.clone(),
            accounts.destination_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN]>
for SharedAccountsExactOutRouteV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            program_authority: &arr[0],
            user_transfer_authority: &arr[1],
            source_token_account: &arr[2],
            program_source_token_account: &arr[3],
            program_destination_token_account: &arr[4],
            destination_token_account: &arr[5],
            source_mint: &arr[6],
            destination_mint: &arr[7],
            source_token_program: &arr[8],
            destination_token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_DISCM: [u8; 8usize] = [
    53, 96, 229, 202, 216, 187, 250, 24,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharedAccountsExactOutRouteV2IxArgs {
    pub id: u8,
    pub out_amount: u64,
    pub quoted_in_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u16,
    pub positive_slippage_bps: u16,
    pub route_plan: Vec<RoutePlanStepV2>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SharedAccountsExactOutRouteV2IxData(pub SharedAccountsExactOutRouteV2IxArgs);
impl From<SharedAccountsExactOutRouteV2IxArgs> for SharedAccountsExactOutRouteV2IxData {
    fn from(args: SharedAccountsExactOutRouteV2IxArgs) -> Self {
        Self(args)
    }
}
impl SharedAccountsExactOutRouteV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quoted_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let positive_slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let route_plan: Vec<RoutePlanStepV2> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SharedAccountsExactOutRouteV2IxArgs {
                id,
                out_amount,
                quoted_in_amount,
                slippage_bps,
                platform_fee_bps,
                positive_slippage_bps,
                route_plan,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quoted_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.platform_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.positive_slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.route_plan, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn shared_accounts_exact_out_route_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: SharedAccountsExactOutRouteV2Keys,
    args: SharedAccountsExactOutRouteV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_V2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SharedAccountsExactOutRouteV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn shared_accounts_exact_out_route_v2_ix(
    keys: SharedAccountsExactOutRouteV2Keys,
    args: SharedAccountsExactOutRouteV2IxArgs,
) -> std::io::Result<Instruction> {
    shared_accounts_exact_out_route_v2_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn shared_accounts_exact_out_route_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsExactOutRouteV2Accounts<'_, '_>,
    args: SharedAccountsExactOutRouteV2IxArgs,
) -> ProgramResult {
    let keys: SharedAccountsExactOutRouteV2Keys = accounts.into();
    let ix = shared_accounts_exact_out_route_v2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn shared_accounts_exact_out_route_v2_invoke(
    accounts: SharedAccountsExactOutRouteV2Accounts<'_, '_>,
    args: SharedAccountsExactOutRouteV2IxArgs,
) -> ProgramResult {
    shared_accounts_exact_out_route_v2_invoke_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn shared_accounts_exact_out_route_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsExactOutRouteV2Accounts<'_, '_>,
    args: SharedAccountsExactOutRouteV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SharedAccountsExactOutRouteV2Keys = accounts.into();
    let ix = shared_accounts_exact_out_route_v2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn shared_accounts_exact_out_route_v2_invoke_signed(
    accounts: SharedAccountsExactOutRouteV2Accounts<'_, '_>,
    args: SharedAccountsExactOutRouteV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    shared_accounts_exact_out_route_v2_invoke_signed_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn shared_accounts_exact_out_route_v2_verify_account_keys(
    accounts: SharedAccountsExactOutRouteV2Accounts<'_, '_>,
    keys: SharedAccountsExactOutRouteV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.source_token_account.key, keys.source_token_account),
        (*accounts.program_source_token_account.key, keys.program_source_token_account),
        (
            *accounts.program_destination_token_account.key,
            keys.program_destination_token_account,
        ),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.source_token_program.key, keys.source_token_program),
        (*accounts.destination_token_program.key, keys.destination_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn shared_accounts_exact_out_route_v2_verify_writable_privileges<'me, 'info>(
    accounts: SharedAccountsExactOutRouteV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source_token_account,
        accounts.program_source_token_account,
        accounts.program_destination_token_account,
        accounts.destination_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn shared_accounts_exact_out_route_v2_verify_signer_privileges<'me, 'info>(
    accounts: SharedAccountsExactOutRouteV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn shared_accounts_exact_out_route_v2_verify_account_privileges<'me, 'info>(
    accounts: SharedAccountsExactOutRouteV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    shared_accounts_exact_out_route_v2_verify_writable_privileges(accounts)?;
    shared_accounts_exact_out_route_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SHARED_ACCOUNTS_ROUTE_V2_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SharedAccountsRouteV2Accounts<'me, 'info> {
    pub program_authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source_token_account: &'me AccountInfo<'info>,
    pub program_source_token_account: &'me AccountInfo<'info>,
    pub program_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub source_token_program: &'me AccountInfo<'info>,
    pub destination_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SharedAccountsRouteV2Keys {
    pub program_authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token_account: Pubkey,
    pub program_source_token_account: Pubkey,
    pub program_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub source_token_program: Pubkey,
    pub destination_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SharedAccountsRouteV2Accounts<'_, '_>> for SharedAccountsRouteV2Keys {
    fn from(accounts: SharedAccountsRouteV2Accounts) -> Self {
        Self {
            program_authority: *accounts.program_authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source_token_account: *accounts.source_token_account.key,
            program_source_token_account: *accounts.program_source_token_account.key,
            program_destination_token_account: *accounts
                .program_destination_token_account
                .key,
            destination_token_account: *accounts.destination_token_account.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            source_token_program: *accounts.source_token_program.key,
            destination_token_program: *accounts.destination_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SharedAccountsRouteV2Keys>
for [AccountMeta; SHARED_ACCOUNTS_ROUTE_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: SharedAccountsRouteV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_token_program,
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
impl From<[Pubkey; SHARED_ACCOUNTS_ROUTE_V2_IX_ACCOUNTS_LEN]>
for SharedAccountsRouteV2Keys {
    fn from(pubkeys: [Pubkey; SHARED_ACCOUNTS_ROUTE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            program_authority: pubkeys[0],
            user_transfer_authority: pubkeys[1],
            source_token_account: pubkeys[2],
            program_source_token_account: pubkeys[3],
            program_destination_token_account: pubkeys[4],
            destination_token_account: pubkeys[5],
            source_mint: pubkeys[6],
            destination_mint: pubkeys[7],
            source_token_program: pubkeys[8],
            destination_token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<SharedAccountsRouteV2Accounts<'_, 'info>>
for [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: SharedAccountsRouteV2Accounts<'_, 'info>) -> Self {
        [
            accounts.program_authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_token_account.clone(),
            accounts.program_source_token_account.clone(),
            accounts.program_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.source_token_program.clone(),
            accounts.destination_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_V2_IX_ACCOUNTS_LEN]>
for SharedAccountsRouteV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            program_authority: &arr[0],
            user_transfer_authority: &arr[1],
            source_token_account: &arr[2],
            program_source_token_account: &arr[3],
            program_destination_token_account: &arr[4],
            destination_token_account: &arr[5],
            source_mint: &arr[6],
            destination_mint: &arr[7],
            source_token_program: &arr[8],
            destination_token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const SHARED_ACCOUNTS_ROUTE_V2_IX_DISCM: [u8; 8usize] = [
    209, 152, 83, 147, 124, 254, 216, 233,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharedAccountsRouteV2IxArgs {
    pub id: u8,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u16,
    pub positive_slippage_bps: u16,
    pub route_plan: Vec<RoutePlanStepV2>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SharedAccountsRouteV2IxData(pub SharedAccountsRouteV2IxArgs);
impl From<SharedAccountsRouteV2IxArgs> for SharedAccountsRouteV2IxData {
    fn from(args: SharedAccountsRouteV2IxArgs) -> Self {
        Self(args)
    }
}
impl SharedAccountsRouteV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHARED_ACCOUNTS_ROUTE_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quoted_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let positive_slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let route_plan: Vec<RoutePlanStepV2> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SharedAccountsRouteV2IxArgs {
                id,
                in_amount,
                quoted_out_amount,
                slippage_bps,
                platform_fee_bps,
                positive_slippage_bps,
                route_plan,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHARED_ACCOUNTS_ROUTE_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quoted_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.platform_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.positive_slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.route_plan, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn shared_accounts_route_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: SharedAccountsRouteV2Keys,
    args: SharedAccountsRouteV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SHARED_ACCOUNTS_ROUTE_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: SharedAccountsRouteV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn shared_accounts_route_v2_ix(
    keys: SharedAccountsRouteV2Keys,
    args: SharedAccountsRouteV2IxArgs,
) -> std::io::Result<Instruction> {
    shared_accounts_route_v2_ix_with_program_id(JUPITER_PROGRAM_ID, keys, args)
}
pub fn shared_accounts_route_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsRouteV2Accounts<'_, '_>,
    args: SharedAccountsRouteV2IxArgs,
) -> ProgramResult {
    let keys: SharedAccountsRouteV2Keys = accounts.into();
    let ix = shared_accounts_route_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn shared_accounts_route_v2_invoke(
    accounts: SharedAccountsRouteV2Accounts<'_, '_>,
    args: SharedAccountsRouteV2IxArgs,
) -> ProgramResult {
    shared_accounts_route_v2_invoke_with_program_id(JUPITER_PROGRAM_ID, accounts, args)
}
pub fn shared_accounts_route_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsRouteV2Accounts<'_, '_>,
    args: SharedAccountsRouteV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SharedAccountsRouteV2Keys = accounts.into();
    let ix = shared_accounts_route_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn shared_accounts_route_v2_invoke_signed(
    accounts: SharedAccountsRouteV2Accounts<'_, '_>,
    args: SharedAccountsRouteV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    shared_accounts_route_v2_invoke_signed_with_program_id(
        JUPITER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn shared_accounts_route_v2_verify_account_keys(
    accounts: SharedAccountsRouteV2Accounts<'_, '_>,
    keys: SharedAccountsRouteV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.source_token_account.key, keys.source_token_account),
        (*accounts.program_source_token_account.key, keys.program_source_token_account),
        (
            *accounts.program_destination_token_account.key,
            keys.program_destination_token_account,
        ),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.source_token_program.key, keys.source_token_program),
        (*accounts.destination_token_program.key, keys.destination_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_v2_verify_writable_privileges<'me, 'info>(
    accounts: SharedAccountsRouteV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source_token_account,
        accounts.program_source_token_account,
        accounts.program_destination_token_account,
        accounts.destination_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_v2_verify_signer_privileges<'me, 'info>(
    accounts: SharedAccountsRouteV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_v2_verify_account_privileges<'me, 'info>(
    accounts: SharedAccountsRouteV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    shared_accounts_route_v2_verify_writable_privileges(accounts)?;
    shared_accounts_route_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
