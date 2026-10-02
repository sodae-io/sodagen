use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum RunnerRodeoProgramIx {
    BuyConsolidatedCoinFromCurvePool(BuyConsolidatedCoinFromCurvePoolIxArgs),
    BuyConsolidatedCoinFromDamm(BuyConsolidatedCoinFromDammIxArgs),
    ClaimConsolidation,
    ClaimCreatorRewards,
    ConsolidateCoinFromCurvePool,
    ConsolidateCoinFromDamm(ConsolidateCoinFromDammIxArgs),
    FundDistributor(FundDistributorIxArgs),
    HarvestTransferFees(HarvestTransferFeesIxArgs),
    InitCoin,
    InitConfig(InitConfigIxArgs),
    MigrateCurvePool,
    Swap(SwapIxArgs),
    SweepHolderRewards,
}
impl RunnerRodeoProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_DISCM) {
            let mut reader = &buf[BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_DISCM
                .len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyConsolidatedCoinFromCurvePool(BuyConsolidatedCoinFromCurvePoolIxArgs {
                    field_0,
                    field_1,
                }),
            );
        }
        if buf.starts_with(&BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_DISCM) {
            let mut reader = &buf[BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyConsolidatedCoinFromDamm(BuyConsolidatedCoinFromDammIxArgs {
                    field_0,
                    field_1,
                }),
            );
        }
        if buf.starts_with(&CLAIM_CONSOLIDATION_IX_DISCM) {
            return Ok(Self::ClaimConsolidation);
        }
        if buf.starts_with(&CLAIM_CREATOR_REWARDS_IX_DISCM) {
            return Ok(Self::ClaimCreatorRewards);
        }
        if buf.starts_with(&CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_DISCM) {
            return Ok(Self::ConsolidateCoinFromCurvePool);
        }
        if buf.starts_with(&CONSOLIDATE_COIN_FROM_DAMM_IX_DISCM) {
            let mut reader = &buf[CONSOLIDATE_COIN_FROM_DAMM_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ConsolidateCoinFromDamm(ConsolidateCoinFromDammIxArgs {
                    field_0,
                    field_1,
                }),
            );
        }
        if buf.starts_with(&FUND_DISTRIBUTOR_IX_DISCM) {
            let mut reader = &buf[FUND_DISTRIBUTOR_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::FundDistributor(FundDistributorIxArgs {
                    field_0,
                    field_1,
                }),
            );
        }
        if buf.starts_with(&HARVEST_TRANSFER_FEES_IX_DISCM) {
            let mut reader = &buf[HARVEST_TRANSFER_FEES_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::HarvestTransferFees(HarvestTransferFeesIxArgs {
                    field_0,
                }),
            );
        }
        if buf.starts_with(&INIT_COIN_IX_DISCM) {
            return Ok(Self::InitCoin);
        }
        if buf.starts_with(&INIT_CONFIG_IX_DISCM) {
            let mut reader = &buf[INIT_CONFIG_IX_DISCM.len()..];
            let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let field_2: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let field_3: u16 = crate::borsh_de_or_default(&mut reader)?;
            let field_4: u16 = crate::borsh_de_or_default(&mut reader)?;
            let field_5: u16 = crate::borsh_de_or_default(&mut reader)?;
            let field_6: u16 = crate::borsh_de_or_default(&mut reader)?;
            let field_7: u16 = crate::borsh_de_or_default(&mut reader)?;
            let field_8: u16 = crate::borsh_de_or_default(&mut reader)?;
            let field_9: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_10: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_11: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitConfig(InitConfigIxArgs {
                    field_0,
                    field_1,
                    field_2,
                    field_3,
                    field_4,
                    field_5,
                    field_6,
                    field_7,
                    field_8,
                    field_9,
                    field_10,
                    field_11,
                }),
            );
        }
        if buf.starts_with(&MIGRATE_CURVE_POOL_IX_DISCM) {
            return Ok(Self::MigrateCurvePool);
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Swap(SwapIxArgs { field_0, field_1 }));
        }
        if buf.starts_with(&SWEEP_HOLDER_REWARDS_IX_DISCM) {
            return Ok(Self::SweepHolderRewards);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::BuyConsolidatedCoinFromCurvePool(args) => {
                writer.write_all(&BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                Ok(())
            }
            Self::BuyConsolidatedCoinFromDamm(args) => {
                writer.write_all(&BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                Ok(())
            }
            Self::ClaimConsolidation => writer.write_all(&CLAIM_CONSOLIDATION_IX_DISCM),
            Self::ClaimCreatorRewards => {
                writer.write_all(&CLAIM_CREATOR_REWARDS_IX_DISCM)
            }
            Self::ConsolidateCoinFromCurvePool => {
                writer.write_all(&CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_DISCM)
            }
            Self::ConsolidateCoinFromDamm(args) => {
                writer.write_all(&CONSOLIDATE_COIN_FROM_DAMM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                Ok(())
            }
            Self::FundDistributor(args) => {
                writer.write_all(&FUND_DISTRIBUTOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                Ok(())
            }
            Self::HarvestTransferFees(args) => {
                writer.write_all(&HARVEST_TRANSFER_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                Ok(())
            }
            Self::InitCoin => writer.write_all(&INIT_COIN_IX_DISCM),
            Self::InitConfig(args) => {
                writer.write_all(&INIT_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_3, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_4, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_5, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_6, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_7, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_8, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_9, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_10, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_11, &mut writer)?;
                Ok(())
            }
            Self::MigrateCurvePool => writer.write_all(&MIGRATE_CURVE_POOL_IX_DISCM),
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                Ok(())
            }
            Self::SweepHolderRewards => writer.write_all(&SWEEP_HOLDER_REWARDS_IX_DISCM),
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
pub const BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct BuyConsolidatedCoinFromCurvePoolAccounts<'me, 'info> {
    pub quote_vault: &'me AccountInfo<'info>,
    pub consolidation: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub consolidation_vault: &'me AccountInfo<'info>,
    pub damm_pool: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub damm_pool_authority: &'me AccountInfo<'info>,
    pub damm_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub consolidation_distribution_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub damm_event_authority: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub new_coin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyConsolidatedCoinFromCurvePoolKeys {
    pub quote_vault: Pubkey,
    pub consolidation: Pubkey,
    pub config: Pubkey,
    pub consolidation_vault: Pubkey,
    pub damm_pool: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub damm_pool_authority: Pubkey,
    pub damm_program: Pubkey,
    pub event_authority: Pubkey,
    pub consolidation_distribution_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub damm_event_authority: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub system_program: Pubkey,
    pub program: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub new_coin: Pubkey,
}
impl From<BuyConsolidatedCoinFromCurvePoolAccounts<'_, '_>>
for BuyConsolidatedCoinFromCurvePoolKeys {
    fn from(accounts: BuyConsolidatedCoinFromCurvePoolAccounts) -> Self {
        Self {
            quote_vault: *accounts.quote_vault.key,
            consolidation: *accounts.consolidation.key,
            config: *accounts.config.key,
            consolidation_vault: *accounts.consolidation_vault.key,
            damm_pool: *accounts.damm_pool.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            damm_pool_authority: *accounts.damm_pool_authority.key,
            damm_program: *accounts.damm_program.key,
            event_authority: *accounts.event_authority.key,
            consolidation_distribution_vault: *accounts
                .consolidation_distribution_vault
                .key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            damm_event_authority: *accounts.damm_event_authority.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            system_program: *accounts.system_program.key,
            program: *accounts.program.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            new_coin: *accounts.new_coin.key,
        }
    }
}
impl From<BuyConsolidatedCoinFromCurvePoolKeys>
for [AccountMeta; BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyConsolidatedCoinFromCurvePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.consolidation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.consolidation_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.consolidation_distribution_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_coin,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN]>
for BuyConsolidatedCoinFromCurvePoolKeys {
    fn from(
        pubkeys: [Pubkey; BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            quote_vault: pubkeys[0],
            consolidation: pubkeys[1],
            config: pubkeys[2],
            consolidation_vault: pubkeys[3],
            damm_pool: pubkeys[4],
            token_a_vault: pubkeys[5],
            token_b_vault: pubkeys[6],
            damm_pool_authority: pubkeys[7],
            damm_program: pubkeys[8],
            event_authority: pubkeys[9],
            consolidation_distribution_vault: pubkeys[10],
            base_mint: pubkeys[11],
            quote_mint: pubkeys[12],
            damm_event_authority: pubkeys[13],
            base_token_program: pubkeys[14],
            quote_token_program: pubkeys[15],
            system_program: pubkeys[16],
            program: pubkeys[17],
            authority: pubkeys[18],
            payer: pubkeys[19],
            new_coin: pubkeys[20],
        }
    }
}
impl<'info> From<BuyConsolidatedCoinFromCurvePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyConsolidatedCoinFromCurvePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.quote_vault.clone(),
            accounts.consolidation.clone(),
            accounts.config.clone(),
            accounts.consolidation_vault.clone(),
            accounts.damm_pool.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.damm_pool_authority.clone(),
            accounts.damm_program.clone(),
            accounts.event_authority.clone(),
            accounts.consolidation_distribution_vault.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.damm_event_authority.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.system_program.clone(),
            accounts.program.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.new_coin.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN]>
for BuyConsolidatedCoinFromCurvePoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            quote_vault: &arr[0],
            consolidation: &arr[1],
            config: &arr[2],
            consolidation_vault: &arr[3],
            damm_pool: &arr[4],
            token_a_vault: &arr[5],
            token_b_vault: &arr[6],
            damm_pool_authority: &arr[7],
            damm_program: &arr[8],
            event_authority: &arr[9],
            consolidation_distribution_vault: &arr[10],
            base_mint: &arr[11],
            quote_mint: &arr[12],
            damm_event_authority: &arr[13],
            base_token_program: &arr[14],
            quote_token_program: &arr[15],
            system_program: &arr[16],
            program: &arr[17],
            authority: &arr[18],
            payer: &arr[19],
            new_coin: &arr[20],
        }
    }
}
pub const BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_DISCM: [u8; 8usize] = [
    181, 76, 35, 205, 116, 176, 106, 141,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyConsolidatedCoinFromCurvePoolIxArgs {
    pub field_0: u64,
    pub field_1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyConsolidatedCoinFromCurvePoolIxData(
    pub BuyConsolidatedCoinFromCurvePoolIxArgs,
);
impl From<BuyConsolidatedCoinFromCurvePoolIxArgs>
for BuyConsolidatedCoinFromCurvePoolIxData {
    fn from(args: BuyConsolidatedCoinFromCurvePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BuyConsolidatedCoinFromCurvePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyConsolidatedCoinFromCurvePoolIxArgs {
                field_0,
                field_1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_DISCM)?;
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
pub fn buy_consolidated_coin_from_curve_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyConsolidatedCoinFromCurvePoolKeys,
    args: BuyConsolidatedCoinFromCurvePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_CONSOLIDATED_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: BuyConsolidatedCoinFromCurvePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_consolidated_coin_from_curve_pool_ix(
    keys: BuyConsolidatedCoinFromCurvePoolKeys,
    args: BuyConsolidatedCoinFromCurvePoolIxArgs,
) -> std::io::Result<Instruction> {
    buy_consolidated_coin_from_curve_pool_ix_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn buy_consolidated_coin_from_curve_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyConsolidatedCoinFromCurvePoolAccounts<'_, '_>,
    args: BuyConsolidatedCoinFromCurvePoolIxArgs,
) -> ProgramResult {
    let keys: BuyConsolidatedCoinFromCurvePoolKeys = accounts.into();
    let ix = buy_consolidated_coin_from_curve_pool_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_consolidated_coin_from_curve_pool_invoke(
    accounts: BuyConsolidatedCoinFromCurvePoolAccounts<'_, '_>,
    args: BuyConsolidatedCoinFromCurvePoolIxArgs,
) -> ProgramResult {
    buy_consolidated_coin_from_curve_pool_invoke_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn buy_consolidated_coin_from_curve_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyConsolidatedCoinFromCurvePoolAccounts<'_, '_>,
    args: BuyConsolidatedCoinFromCurvePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyConsolidatedCoinFromCurvePoolKeys = accounts.into();
    let ix = buy_consolidated_coin_from_curve_pool_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_consolidated_coin_from_curve_pool_invoke_signed(
    accounts: BuyConsolidatedCoinFromCurvePoolAccounts<'_, '_>,
    args: BuyConsolidatedCoinFromCurvePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_consolidated_coin_from_curve_pool_invoke_signed_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn buy_consolidated_coin_from_curve_pool_verify_account_keys(
    accounts: BuyConsolidatedCoinFromCurvePoolAccounts<'_, '_>,
    keys: BuyConsolidatedCoinFromCurvePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.consolidation.key, keys.consolidation),
        (*accounts.config.key, keys.config),
        (*accounts.consolidation_vault.key, keys.consolidation_vault),
        (*accounts.damm_pool.key, keys.damm_pool),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.damm_pool_authority.key, keys.damm_pool_authority),
        (*accounts.damm_program.key, keys.damm_program),
        (*accounts.event_authority.key, keys.event_authority),
        (
            *accounts.consolidation_distribution_vault.key,
            keys.consolidation_distribution_vault,
        ),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.damm_event_authority.key, keys.damm_event_authority),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program.key, keys.program),
        (*accounts.authority.key, keys.authority),
        (*accounts.payer.key, keys.payer),
        (*accounts.new_coin.key, keys.new_coin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_consolidated_coin_from_curve_pool_verify_writable_privileges<'me, 'info>(
    accounts: BuyConsolidatedCoinFromCurvePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.consolidation,
        accounts.consolidation_vault,
        accounts.damm_pool,
        accounts.consolidation_distribution_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_consolidated_coin_from_curve_pool_verify_account_privileges<'me, 'info>(
    accounts: BuyConsolidatedCoinFromCurvePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_consolidated_coin_from_curve_pool_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct BuyConsolidatedCoinFromDammAccounts<'me, 'info> {
    pub distributor_vault: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub distributor_pubkey: &'me AccountInfo<'info>,
    pub distributor_program_id: &'me AccountInfo<'info>,
    pub wsol_token_account: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub coin: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub creator_rewards_vault: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub holder_rewards_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyConsolidatedCoinFromDammKeys {
    pub distributor_vault: Pubkey,
    pub wsol_mint: Pubkey,
    pub authority: Pubkey,
    pub token_program: Pubkey,
    pub config: Pubkey,
    pub payer: Pubkey,
    pub distributor_pubkey: Pubkey,
    pub distributor_program_id: Pubkey,
    pub wsol_token_account: Pubkey,
    pub quote_mint: Pubkey,
    pub coin: Pubkey,
    pub quote_vault: Pubkey,
    pub creator_rewards_vault: Pubkey,
    pub base_vault: Pubkey,
    pub holder_rewards_vault: Pubkey,
    pub system_program: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
}
impl From<BuyConsolidatedCoinFromDammAccounts<'_, '_>>
for BuyConsolidatedCoinFromDammKeys {
    fn from(accounts: BuyConsolidatedCoinFromDammAccounts) -> Self {
        Self {
            distributor_vault: *accounts.distributor_vault.key,
            wsol_mint: *accounts.wsol_mint.key,
            authority: *accounts.authority.key,
            token_program: *accounts.token_program.key,
            config: *accounts.config.key,
            payer: *accounts.payer.key,
            distributor_pubkey: *accounts.distributor_pubkey.key,
            distributor_program_id: *accounts.distributor_program_id.key,
            wsol_token_account: *accounts.wsol_token_account.key,
            quote_mint: *accounts.quote_mint.key,
            coin: *accounts.coin.key,
            quote_vault: *accounts.quote_vault.key,
            creator_rewards_vault: *accounts.creator_rewards_vault.key,
            base_vault: *accounts.base_vault.key,
            holder_rewards_vault: *accounts.holder_rewards_vault.key,
            system_program: *accounts.system_program.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            creator: *accounts.creator.key,
            base_mint: *accounts.base_mint.key,
        }
    }
}
impl From<BuyConsolidatedCoinFromDammKeys>
for [AccountMeta; BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyConsolidatedCoinFromDammKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.distributor_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wsol_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.distributor_pubkey,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.distributor_program_id,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wsol_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_rewards_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.holder_rewards_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_ACCOUNTS_LEN]>
for BuyConsolidatedCoinFromDammKeys {
    fn from(pubkeys: [Pubkey; BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            distributor_vault: pubkeys[0],
            wsol_mint: pubkeys[1],
            authority: pubkeys[2],
            token_program: pubkeys[3],
            config: pubkeys[4],
            payer: pubkeys[5],
            distributor_pubkey: pubkeys[6],
            distributor_program_id: pubkeys[7],
            wsol_token_account: pubkeys[8],
            quote_mint: pubkeys[9],
            coin: pubkeys[10],
            quote_vault: pubkeys[11],
            creator_rewards_vault: pubkeys[12],
            base_vault: pubkeys[13],
            holder_rewards_vault: pubkeys[14],
            system_program: pubkeys[15],
            base_token_program: pubkeys[16],
            quote_token_program: pubkeys[17],
            event_authority: pubkeys[18],
            program: pubkeys[19],
            creator: pubkeys[20],
            base_mint: pubkeys[21],
        }
    }
}
impl<'info> From<BuyConsolidatedCoinFromDammAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyConsolidatedCoinFromDammAccounts<'_, 'info>) -> Self {
        [
            accounts.distributor_vault.clone(),
            accounts.wsol_mint.clone(),
            accounts.authority.clone(),
            accounts.token_program.clone(),
            accounts.config.clone(),
            accounts.payer.clone(),
            accounts.distributor_pubkey.clone(),
            accounts.distributor_program_id.clone(),
            accounts.wsol_token_account.clone(),
            accounts.quote_mint.clone(),
            accounts.coin.clone(),
            accounts.quote_vault.clone(),
            accounts.creator_rewards_vault.clone(),
            accounts.base_vault.clone(),
            accounts.holder_rewards_vault.clone(),
            accounts.system_program.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.creator.clone(),
            accounts.base_mint.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_ACCOUNTS_LEN]>
for BuyConsolidatedCoinFromDammAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            distributor_vault: &arr[0],
            wsol_mint: &arr[1],
            authority: &arr[2],
            token_program: &arr[3],
            config: &arr[4],
            payer: &arr[5],
            distributor_pubkey: &arr[6],
            distributor_program_id: &arr[7],
            wsol_token_account: &arr[8],
            quote_mint: &arr[9],
            coin: &arr[10],
            quote_vault: &arr[11],
            creator_rewards_vault: &arr[12],
            base_vault: &arr[13],
            holder_rewards_vault: &arr[14],
            system_program: &arr[15],
            base_token_program: &arr[16],
            quote_token_program: &arr[17],
            event_authority: &arr[18],
            program: &arr[19],
            creator: &arr[20],
            base_mint: &arr[21],
        }
    }
}
pub const BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_DISCM: [u8; 8usize] = [
    153, 7, 133, 43, 72, 20, 13, 143,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyConsolidatedCoinFromDammIxArgs {
    pub field_0: u64,
    pub field_1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyConsolidatedCoinFromDammIxData(pub BuyConsolidatedCoinFromDammIxArgs);
impl From<BuyConsolidatedCoinFromDammIxArgs> for BuyConsolidatedCoinFromDammIxData {
    fn from(args: BuyConsolidatedCoinFromDammIxArgs) -> Self {
        Self(args)
    }
}
impl BuyConsolidatedCoinFromDammIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyConsolidatedCoinFromDammIxArgs {
                field_0,
                field_1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_DISCM)?;
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
pub fn buy_consolidated_coin_from_damm_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyConsolidatedCoinFromDammKeys,
    args: BuyConsolidatedCoinFromDammIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_CONSOLIDATED_COIN_FROM_DAMM_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: BuyConsolidatedCoinFromDammIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_consolidated_coin_from_damm_ix(
    keys: BuyConsolidatedCoinFromDammKeys,
    args: BuyConsolidatedCoinFromDammIxArgs,
) -> std::io::Result<Instruction> {
    buy_consolidated_coin_from_damm_ix_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn buy_consolidated_coin_from_damm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyConsolidatedCoinFromDammAccounts<'_, '_>,
    args: BuyConsolidatedCoinFromDammIxArgs,
) -> ProgramResult {
    let keys: BuyConsolidatedCoinFromDammKeys = accounts.into();
    let ix = buy_consolidated_coin_from_damm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_consolidated_coin_from_damm_invoke(
    accounts: BuyConsolidatedCoinFromDammAccounts<'_, '_>,
    args: BuyConsolidatedCoinFromDammIxArgs,
) -> ProgramResult {
    buy_consolidated_coin_from_damm_invoke_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn buy_consolidated_coin_from_damm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyConsolidatedCoinFromDammAccounts<'_, '_>,
    args: BuyConsolidatedCoinFromDammIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyConsolidatedCoinFromDammKeys = accounts.into();
    let ix = buy_consolidated_coin_from_damm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_consolidated_coin_from_damm_invoke_signed(
    accounts: BuyConsolidatedCoinFromDammAccounts<'_, '_>,
    args: BuyConsolidatedCoinFromDammIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_consolidated_coin_from_damm_invoke_signed_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn buy_consolidated_coin_from_damm_verify_account_keys(
    accounts: BuyConsolidatedCoinFromDammAccounts<'_, '_>,
    keys: BuyConsolidatedCoinFromDammKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.distributor_vault.key, keys.distributor_vault),
        (*accounts.wsol_mint.key, keys.wsol_mint),
        (*accounts.authority.key, keys.authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.config.key, keys.config),
        (*accounts.payer.key, keys.payer),
        (*accounts.distributor_pubkey.key, keys.distributor_pubkey),
        (*accounts.distributor_program_id.key, keys.distributor_program_id),
        (*accounts.wsol_token_account.key, keys.wsol_token_account),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.coin.key, keys.coin),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.creator_rewards_vault.key, keys.creator_rewards_vault),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.holder_rewards_vault.key, keys.holder_rewards_vault),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.creator.key, keys.creator),
        (*accounts.base_mint.key, keys.base_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_consolidated_coin_from_damm_verify_writable_privileges<'me, 'info>(
    accounts: BuyConsolidatedCoinFromDammAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.distributor_vault,
        accounts.payer,
        accounts.wsol_token_account,
        accounts.coin,
        accounts.quote_vault,
        accounts.creator_rewards_vault,
        accounts.base_vault,
        accounts.holder_rewards_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_consolidated_coin_from_damm_verify_account_privileges<'me, 'info>(
    accounts: BuyConsolidatedCoinFromDammAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_consolidated_coin_from_damm_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_CONSOLIDATION_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ClaimConsolidationAccounts<'me, 'info> {
    pub new_coin: &'me AccountInfo<'info>,
    pub old_coin: &'me AccountInfo<'info>,
    pub consolidation_distribution_vault: &'me AccountInfo<'info>,
    pub old_mint: &'me AccountInfo<'info>,
    pub new_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub payer_input_account: &'me AccountInfo<'info>,
    pub payer_output_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimConsolidationKeys {
    pub new_coin: Pubkey,
    pub old_coin: Pubkey,
    pub consolidation_distribution_vault: Pubkey,
    pub old_mint: Pubkey,
    pub new_mint: Pubkey,
    pub event_authority: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub program: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub payer_input_account: Pubkey,
    pub payer_output_account: Pubkey,
}
impl From<ClaimConsolidationAccounts<'_, '_>> for ClaimConsolidationKeys {
    fn from(accounts: ClaimConsolidationAccounts) -> Self {
        Self {
            new_coin: *accounts.new_coin.key,
            old_coin: *accounts.old_coin.key,
            consolidation_distribution_vault: *accounts
                .consolidation_distribution_vault
                .key,
            old_mint: *accounts.old_mint.key,
            new_mint: *accounts.new_mint.key,
            event_authority: *accounts.event_authority.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            program: *accounts.program.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            payer_input_account: *accounts.payer_input_account.key,
            payer_output_account: *accounts.payer_output_account.key,
        }
    }
}
impl From<ClaimConsolidationKeys>
for [AccountMeta; CLAIM_CONSOLIDATION_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimConsolidationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.new_coin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_coin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.consolidation_distribution_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.old_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
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
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer_input_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer_output_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_CONSOLIDATION_IX_ACCOUNTS_LEN]> for ClaimConsolidationKeys {
    fn from(pubkeys: [Pubkey; CLAIM_CONSOLIDATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            new_coin: pubkeys[0],
            old_coin: pubkeys[1],
            consolidation_distribution_vault: pubkeys[2],
            old_mint: pubkeys[3],
            new_mint: pubkeys[4],
            event_authority: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            program: pubkeys[8],
            authority: pubkeys[9],
            payer: pubkeys[10],
            payer_input_account: pubkeys[11],
            payer_output_account: pubkeys[12],
        }
    }
}
impl<'info> From<ClaimConsolidationAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_CONSOLIDATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimConsolidationAccounts<'_, 'info>) -> Self {
        [
            accounts.new_coin.clone(),
            accounts.old_coin.clone(),
            accounts.consolidation_distribution_vault.clone(),
            accounts.old_mint.clone(),
            accounts.new_mint.clone(),
            accounts.event_authority.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.program.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.payer_input_account.clone(),
            accounts.payer_output_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_CONSOLIDATION_IX_ACCOUNTS_LEN]>
for ClaimConsolidationAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_CONSOLIDATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            new_coin: &arr[0],
            old_coin: &arr[1],
            consolidation_distribution_vault: &arr[2],
            old_mint: &arr[3],
            new_mint: &arr[4],
            event_authority: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            program: &arr[8],
            authority: &arr[9],
            payer: &arr[10],
            payer_input_account: &arr[11],
            payer_output_account: &arr[12],
        }
    }
}
pub const CLAIM_CONSOLIDATION_IX_DISCM: [u8; 8usize] = [
    195, 206, 192, 81, 165, 56, 251, 244,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimConsolidationIxData;
impl ClaimConsolidationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_CONSOLIDATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_CONSOLIDATION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_consolidation_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimConsolidationKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_CONSOLIDATION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimConsolidationIxData.try_to_vec()?,
    })
}
pub fn claim_consolidation_ix(
    keys: ClaimConsolidationKeys,
) -> std::io::Result<Instruction> {
    claim_consolidation_ix_with_program_id(RUNNER_RODEO_PROGRAM_ID, keys)
}
pub fn claim_consolidation_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimConsolidationAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimConsolidationKeys = accounts.into();
    let ix = claim_consolidation_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_consolidation_invoke(
    accounts: ClaimConsolidationAccounts<'_, '_>,
) -> ProgramResult {
    claim_consolidation_invoke_with_program_id(RUNNER_RODEO_PROGRAM_ID, accounts)
}
pub fn claim_consolidation_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimConsolidationAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimConsolidationKeys = accounts.into();
    let ix = claim_consolidation_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_consolidation_invoke_signed(
    accounts: ClaimConsolidationAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_consolidation_invoke_signed_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_consolidation_verify_account_keys(
    accounts: ClaimConsolidationAccounts<'_, '_>,
    keys: ClaimConsolidationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.new_coin.key, keys.new_coin),
        (*accounts.old_coin.key, keys.old_coin),
        (
            *accounts.consolidation_distribution_vault.key,
            keys.consolidation_distribution_vault,
        ),
        (*accounts.old_mint.key, keys.old_mint),
        (*accounts.new_mint.key, keys.new_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program.key, keys.program),
        (*accounts.authority.key, keys.authority),
        (*accounts.payer.key, keys.payer),
        (*accounts.payer_input_account.key, keys.payer_input_account),
        (*accounts.payer_output_account.key, keys.payer_output_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_consolidation_verify_writable_privileges<'me, 'info>(
    accounts: ClaimConsolidationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.consolidation_distribution_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_consolidation_verify_account_privileges<'me, 'info>(
    accounts: ClaimConsolidationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_consolidation_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct ClaimCreatorRewardsAccounts<'me, 'info> {
    pub quote_mint: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub coin: &'me AccountInfo<'info>,
    pub creator_rewards_vault: &'me AccountInfo<'info>,
    pub new_coin: &'me AccountInfo<'info>,
    pub old_coin: &'me AccountInfo<'info>,
    pub consolidation_distribution_vault: &'me AccountInfo<'info>,
    pub creator_token_account: &'me AccountInfo<'info>,
    pub old_mint: &'me AccountInfo<'info>,
    pub new_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub payer_input_account: &'me AccountInfo<'info>,
    pub payer_output_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimCreatorRewardsKeys {
    pub quote_mint: Pubkey,
    pub quote_token_program: Pubkey,
    pub system_program: Pubkey,
    pub authority: Pubkey,
    pub creator: Pubkey,
    pub coin: Pubkey,
    pub creator_rewards_vault: Pubkey,
    pub new_coin: Pubkey,
    pub old_coin: Pubkey,
    pub consolidation_distribution_vault: Pubkey,
    pub creator_token_account: Pubkey,
    pub old_mint: Pubkey,
    pub new_mint: Pubkey,
    pub event_authority: Pubkey,
    pub token_program: Pubkey,
    pub program: Pubkey,
    pub payer: Pubkey,
    pub payer_input_account: Pubkey,
    pub payer_output_account: Pubkey,
}
impl From<ClaimCreatorRewardsAccounts<'_, '_>> for ClaimCreatorRewardsKeys {
    fn from(accounts: ClaimCreatorRewardsAccounts) -> Self {
        Self {
            quote_mint: *accounts.quote_mint.key,
            quote_token_program: *accounts.quote_token_program.key,
            system_program: *accounts.system_program.key,
            authority: *accounts.authority.key,
            creator: *accounts.creator.key,
            coin: *accounts.coin.key,
            creator_rewards_vault: *accounts.creator_rewards_vault.key,
            new_coin: *accounts.new_coin.key,
            old_coin: *accounts.old_coin.key,
            consolidation_distribution_vault: *accounts
                .consolidation_distribution_vault
                .key,
            creator_token_account: *accounts.creator_token_account.key,
            old_mint: *accounts.old_mint.key,
            new_mint: *accounts.new_mint.key,
            event_authority: *accounts.event_authority.key,
            token_program: *accounts.token_program.key,
            program: *accounts.program.key,
            payer: *accounts.payer.key,
            payer_input_account: *accounts.payer_input_account.key,
            payer_output_account: *accounts.payer_output_account.key,
        }
    }
}
impl From<ClaimCreatorRewardsKeys>
for [AccountMeta; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimCreatorRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_rewards_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_coin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_coin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.consolidation_distribution_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer_input_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer_output_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN]> for ClaimCreatorRewardsKeys {
    fn from(pubkeys: [Pubkey; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            quote_mint: pubkeys[0],
            quote_token_program: pubkeys[1],
            system_program: pubkeys[2],
            authority: pubkeys[3],
            creator: pubkeys[4],
            coin: pubkeys[5],
            creator_rewards_vault: pubkeys[6],
            new_coin: pubkeys[7],
            old_coin: pubkeys[8],
            consolidation_distribution_vault: pubkeys[9],
            creator_token_account: pubkeys[10],
            old_mint: pubkeys[11],
            new_mint: pubkeys[12],
            event_authority: pubkeys[13],
            token_program: pubkeys[14],
            program: pubkeys[15],
            payer: pubkeys[16],
            payer_input_account: pubkeys[17],
            payer_output_account: pubkeys[18],
        }
    }
}
impl<'info> From<ClaimCreatorRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimCreatorRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.quote_mint.clone(),
            accounts.quote_token_program.clone(),
            accounts.system_program.clone(),
            accounts.authority.clone(),
            accounts.creator.clone(),
            accounts.coin.clone(),
            accounts.creator_rewards_vault.clone(),
            accounts.new_coin.clone(),
            accounts.old_coin.clone(),
            accounts.consolidation_distribution_vault.clone(),
            accounts.creator_token_account.clone(),
            accounts.old_mint.clone(),
            accounts.new_mint.clone(),
            accounts.event_authority.clone(),
            accounts.token_program.clone(),
            accounts.program.clone(),
            accounts.payer.clone(),
            accounts.payer_input_account.clone(),
            accounts.payer_output_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN]>
for ClaimCreatorRewardsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            quote_mint: &arr[0],
            quote_token_program: &arr[1],
            system_program: &arr[2],
            authority: &arr[3],
            creator: &arr[4],
            coin: &arr[5],
            creator_rewards_vault: &arr[6],
            new_coin: &arr[7],
            old_coin: &arr[8],
            consolidation_distribution_vault: &arr[9],
            creator_token_account: &arr[10],
            old_mint: &arr[11],
            new_mint: &arr[12],
            event_authority: &arr[13],
            token_program: &arr[14],
            program: &arr[15],
            payer: &arr[16],
            payer_input_account: &arr[17],
            payer_output_account: &arr[18],
        }
    }
}
pub const CLAIM_CREATOR_REWARDS_IX_DISCM: [u8; 8usize] = [
    14, 215, 177, 181, 221, 193, 125, 85,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimCreatorRewardsIxData;
impl ClaimCreatorRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_CREATOR_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_CREATOR_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_creator_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimCreatorRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimCreatorRewardsIxData.try_to_vec()?,
    })
}
pub fn claim_creator_rewards_ix(
    keys: ClaimCreatorRewardsKeys,
) -> std::io::Result<Instruction> {
    claim_creator_rewards_ix_with_program_id(RUNNER_RODEO_PROGRAM_ID, keys)
}
pub fn claim_creator_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimCreatorRewardsKeys = accounts.into();
    let ix = claim_creator_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_creator_rewards_invoke(
    accounts: ClaimCreatorRewardsAccounts<'_, '_>,
) -> ProgramResult {
    claim_creator_rewards_invoke_with_program_id(RUNNER_RODEO_PROGRAM_ID, accounts)
}
pub fn claim_creator_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimCreatorRewardsKeys = accounts.into();
    let ix = claim_creator_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_creator_rewards_invoke_signed(
    accounts: ClaimCreatorRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_creator_rewards_invoke_signed_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_creator_rewards_verify_account_keys(
    accounts: ClaimCreatorRewardsAccounts<'_, '_>,
    keys: ClaimCreatorRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.authority.key, keys.authority),
        (*accounts.creator.key, keys.creator),
        (*accounts.coin.key, keys.coin),
        (*accounts.creator_rewards_vault.key, keys.creator_rewards_vault),
        (*accounts.new_coin.key, keys.new_coin),
        (*accounts.old_coin.key, keys.old_coin),
        (
            *accounts.consolidation_distribution_vault.key,
            keys.consolidation_distribution_vault,
        ),
        (*accounts.creator_token_account.key, keys.creator_token_account),
        (*accounts.old_mint.key, keys.old_mint),
        (*accounts.new_mint.key, keys.new_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.program.key, keys.program),
        (*accounts.payer.key, keys.payer),
        (*accounts.payer_input_account.key, keys.payer_input_account),
        (*accounts.payer_output_account.key, keys.payer_output_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_creator_rewards_verify_writable_privileges<'me, 'info>(
    accounts: ClaimCreatorRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator_rewards_vault,
        accounts.consolidation_distribution_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_creator_rewards_verify_account_privileges<'me, 'info>(
    accounts: ClaimCreatorRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_creator_rewards_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct ConsolidateCoinFromCurvePoolAccounts<'me, 'info> {
    pub payer_burn_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub consolidation: &'me AccountInfo<'info>,
    pub consolidation_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub damm_pool: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub damm_pool_authority: &'me AccountInfo<'info>,
    pub damm_event_authority: &'me AccountInfo<'info>,
    pub damm_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub old_coin: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConsolidateCoinFromCurvePoolKeys {
    pub payer_burn_account: Pubkey,
    pub authority: Pubkey,
    pub consolidation: Pubkey,
    pub consolidation_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub system_program: Pubkey,
    pub damm_pool: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub position_nft_account: Pubkey,
    pub position: Pubkey,
    pub damm_pool_authority: Pubkey,
    pub damm_event_authority: Pubkey,
    pub damm_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub payer: Pubkey,
    pub old_coin: Pubkey,
    pub config: Pubkey,
}
impl From<ConsolidateCoinFromCurvePoolAccounts<'_, '_>>
for ConsolidateCoinFromCurvePoolKeys {
    fn from(accounts: ConsolidateCoinFromCurvePoolAccounts) -> Self {
        Self {
            payer_burn_account: *accounts.payer_burn_account.key,
            authority: *accounts.authority.key,
            consolidation: *accounts.consolidation.key,
            consolidation_vault: *accounts.consolidation_vault.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            system_program: *accounts.system_program.key,
            damm_pool: *accounts.damm_pool.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            position_nft_account: *accounts.position_nft_account.key,
            position: *accounts.position.key,
            damm_pool_authority: *accounts.damm_pool_authority.key,
            damm_event_authority: *accounts.damm_event_authority.key,
            damm_program: *accounts.damm_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            payer: *accounts.payer.key,
            old_coin: *accounts.old_coin.key,
            config: *accounts.config.key,
        }
    }
}
impl From<ConsolidateCoinFromCurvePoolKeys>
for [AccountMeta; CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: ConsolidateCoinFromCurvePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer_burn_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.consolidation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.consolidation_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_program,
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
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_coin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN]>
for ConsolidateCoinFromCurvePoolKeys {
    fn from(
        pubkeys: [Pubkey; CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer_burn_account: pubkeys[0],
            authority: pubkeys[1],
            consolidation: pubkeys[2],
            consolidation_vault: pubkeys[3],
            base_mint: pubkeys[4],
            quote_mint: pubkeys[5],
            base_token_program: pubkeys[6],
            quote_token_program: pubkeys[7],
            system_program: pubkeys[8],
            damm_pool: pubkeys[9],
            token_a_vault: pubkeys[10],
            token_b_vault: pubkeys[11],
            position_nft_account: pubkeys[12],
            position: pubkeys[13],
            damm_pool_authority: pubkeys[14],
            damm_event_authority: pubkeys[15],
            damm_program: pubkeys[16],
            event_authority: pubkeys[17],
            program: pubkeys[18],
            payer: pubkeys[19],
            old_coin: pubkeys[20],
            config: pubkeys[21],
        }
    }
}
impl<'info> From<ConsolidateCoinFromCurvePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConsolidateCoinFromCurvePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.payer_burn_account.clone(),
            accounts.authority.clone(),
            accounts.consolidation.clone(),
            accounts.consolidation_vault.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.system_program.clone(),
            accounts.damm_pool.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.position_nft_account.clone(),
            accounts.position.clone(),
            accounts.damm_pool_authority.clone(),
            accounts.damm_event_authority.clone(),
            accounts.damm_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.payer.clone(),
            accounts.old_coin.clone(),
            accounts.config.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN]>
for ConsolidateCoinFromCurvePoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer_burn_account: &arr[0],
            authority: &arr[1],
            consolidation: &arr[2],
            consolidation_vault: &arr[3],
            base_mint: &arr[4],
            quote_mint: &arr[5],
            base_token_program: &arr[6],
            quote_token_program: &arr[7],
            system_program: &arr[8],
            damm_pool: &arr[9],
            token_a_vault: &arr[10],
            token_b_vault: &arr[11],
            position_nft_account: &arr[12],
            position: &arr[13],
            damm_pool_authority: &arr[14],
            damm_event_authority: &arr[15],
            damm_program: &arr[16],
            event_authority: &arr[17],
            program: &arr[18],
            payer: &arr[19],
            old_coin: &arr[20],
            config: &arr[21],
        }
    }
}
pub const CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_DISCM: [u8; 8usize] = [
    55, 206, 167, 245, 20, 226, 231, 88,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ConsolidateCoinFromCurvePoolIxData;
impl ConsolidateCoinFromCurvePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn consolidate_coin_from_curve_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: ConsolidateCoinFromCurvePoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONSOLIDATE_COIN_FROM_CURVE_POOL_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ConsolidateCoinFromCurvePoolIxData.try_to_vec()?,
    })
}
pub fn consolidate_coin_from_curve_pool_ix(
    keys: ConsolidateCoinFromCurvePoolKeys,
) -> std::io::Result<Instruction> {
    consolidate_coin_from_curve_pool_ix_with_program_id(RUNNER_RODEO_PROGRAM_ID, keys)
}
pub fn consolidate_coin_from_curve_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConsolidateCoinFromCurvePoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ConsolidateCoinFromCurvePoolKeys = accounts.into();
    let ix = consolidate_coin_from_curve_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn consolidate_coin_from_curve_pool_invoke(
    accounts: ConsolidateCoinFromCurvePoolAccounts<'_, '_>,
) -> ProgramResult {
    consolidate_coin_from_curve_pool_invoke_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
    )
}
pub fn consolidate_coin_from_curve_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConsolidateCoinFromCurvePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConsolidateCoinFromCurvePoolKeys = accounts.into();
    let ix = consolidate_coin_from_curve_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn consolidate_coin_from_curve_pool_invoke_signed(
    accounts: ConsolidateCoinFromCurvePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    consolidate_coin_from_curve_pool_invoke_signed_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn consolidate_coin_from_curve_pool_verify_account_keys(
    accounts: ConsolidateCoinFromCurvePoolAccounts<'_, '_>,
    keys: ConsolidateCoinFromCurvePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer_burn_account.key, keys.payer_burn_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.consolidation.key, keys.consolidation),
        (*accounts.consolidation_vault.key, keys.consolidation_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.damm_pool.key, keys.damm_pool),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.position.key, keys.position),
        (*accounts.damm_pool_authority.key, keys.damm_pool_authority),
        (*accounts.damm_event_authority.key, keys.damm_event_authority),
        (*accounts.damm_program.key, keys.damm_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.payer.key, keys.payer),
        (*accounts.old_coin.key, keys.old_coin),
        (*accounts.config.key, keys.config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn consolidate_coin_from_curve_pool_verify_writable_privileges<'me, 'info>(
    accounts: ConsolidateCoinFromCurvePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer_burn_account,
        accounts.consolidation,
        accounts.consolidation_vault,
        accounts.base_mint,
        accounts.damm_pool,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.position_nft_account,
        accounts.position,
        accounts.damm_pool_authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn consolidate_coin_from_curve_pool_verify_account_privileges<'me, 'info>(
    accounts: ConsolidateCoinFromCurvePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    consolidate_coin_from_curve_pool_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CONSOLIDATE_COIN_FROM_DAMM_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct ConsolidateCoinFromDammAccounts<'me, 'info> {
    pub payer_burn_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub consolidation: &'me AccountInfo<'info>,
    pub consolidation_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub damm_pool: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub damm_pool_authority: &'me AccountInfo<'info>,
    pub damm_event_authority: &'me AccountInfo<'info>,
    pub damm_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub old_coin: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConsolidateCoinFromDammKeys {
    pub payer_burn_account: Pubkey,
    pub authority: Pubkey,
    pub consolidation: Pubkey,
    pub consolidation_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub system_program: Pubkey,
    pub damm_pool: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub position_nft_account: Pubkey,
    pub position: Pubkey,
    pub damm_pool_authority: Pubkey,
    pub damm_event_authority: Pubkey,
    pub damm_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub payer: Pubkey,
    pub old_coin: Pubkey,
    pub config: Pubkey,
}
impl From<ConsolidateCoinFromDammAccounts<'_, '_>> for ConsolidateCoinFromDammKeys {
    fn from(accounts: ConsolidateCoinFromDammAccounts) -> Self {
        Self {
            payer_burn_account: *accounts.payer_burn_account.key,
            authority: *accounts.authority.key,
            consolidation: *accounts.consolidation.key,
            consolidation_vault: *accounts.consolidation_vault.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            system_program: *accounts.system_program.key,
            damm_pool: *accounts.damm_pool.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            position_nft_account: *accounts.position_nft_account.key,
            position: *accounts.position.key,
            damm_pool_authority: *accounts.damm_pool_authority.key,
            damm_event_authority: *accounts.damm_event_authority.key,
            damm_program: *accounts.damm_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            payer: *accounts.payer.key,
            old_coin: *accounts.old_coin.key,
            config: *accounts.config.key,
        }
    }
}
impl From<ConsolidateCoinFromDammKeys>
for [AccountMeta; CONSOLIDATE_COIN_FROM_DAMM_IX_ACCOUNTS_LEN] {
    fn from(keys: ConsolidateCoinFromDammKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer_burn_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.consolidation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.consolidation_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_program,
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
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_coin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CONSOLIDATE_COIN_FROM_DAMM_IX_ACCOUNTS_LEN]>
for ConsolidateCoinFromDammKeys {
    fn from(pubkeys: [Pubkey; CONSOLIDATE_COIN_FROM_DAMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer_burn_account: pubkeys[0],
            authority: pubkeys[1],
            consolidation: pubkeys[2],
            consolidation_vault: pubkeys[3],
            base_mint: pubkeys[4],
            quote_mint: pubkeys[5],
            base_token_program: pubkeys[6],
            quote_token_program: pubkeys[7],
            system_program: pubkeys[8],
            damm_pool: pubkeys[9],
            token_a_vault: pubkeys[10],
            token_b_vault: pubkeys[11],
            position_nft_account: pubkeys[12],
            position: pubkeys[13],
            damm_pool_authority: pubkeys[14],
            damm_event_authority: pubkeys[15],
            damm_program: pubkeys[16],
            event_authority: pubkeys[17],
            program: pubkeys[18],
            payer: pubkeys[19],
            old_coin: pubkeys[20],
            config: pubkeys[21],
        }
    }
}
impl<'info> From<ConsolidateCoinFromDammAccounts<'_, 'info>>
for [AccountInfo<'info>; CONSOLIDATE_COIN_FROM_DAMM_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConsolidateCoinFromDammAccounts<'_, 'info>) -> Self {
        [
            accounts.payer_burn_account.clone(),
            accounts.authority.clone(),
            accounts.consolidation.clone(),
            accounts.consolidation_vault.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.system_program.clone(),
            accounts.damm_pool.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.position_nft_account.clone(),
            accounts.position.clone(),
            accounts.damm_pool_authority.clone(),
            accounts.damm_event_authority.clone(),
            accounts.damm_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.payer.clone(),
            accounts.old_coin.clone(),
            accounts.config.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CONSOLIDATE_COIN_FROM_DAMM_IX_ACCOUNTS_LEN]>
for ConsolidateCoinFromDammAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CONSOLIDATE_COIN_FROM_DAMM_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer_burn_account: &arr[0],
            authority: &arr[1],
            consolidation: &arr[2],
            consolidation_vault: &arr[3],
            base_mint: &arr[4],
            quote_mint: &arr[5],
            base_token_program: &arr[6],
            quote_token_program: &arr[7],
            system_program: &arr[8],
            damm_pool: &arr[9],
            token_a_vault: &arr[10],
            token_b_vault: &arr[11],
            position_nft_account: &arr[12],
            position: &arr[13],
            damm_pool_authority: &arr[14],
            damm_event_authority: &arr[15],
            damm_program: &arr[16],
            event_authority: &arr[17],
            program: &arr[18],
            payer: &arr[19],
            old_coin: &arr[20],
            config: &arr[21],
        }
    }
}
pub const CONSOLIDATE_COIN_FROM_DAMM_IX_DISCM: [u8; 8usize] = [
    206, 217, 224, 54, 92, 119, 137, 233,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConsolidateCoinFromDammIxArgs {
    pub field_0: u64,
    pub field_1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConsolidateCoinFromDammIxData(pub ConsolidateCoinFromDammIxArgs);
impl From<ConsolidateCoinFromDammIxArgs> for ConsolidateCoinFromDammIxData {
    fn from(args: ConsolidateCoinFromDammIxArgs) -> Self {
        Self(args)
    }
}
impl ConsolidateCoinFromDammIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONSOLIDATE_COIN_FROM_DAMM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ConsolidateCoinFromDammIxArgs {
                field_0,
                field_1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONSOLIDATE_COIN_FROM_DAMM_IX_DISCM)?;
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
pub fn consolidate_coin_from_damm_ix_with_program_id(
    program_id: Pubkey,
    keys: ConsolidateCoinFromDammKeys,
    args: ConsolidateCoinFromDammIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONSOLIDATE_COIN_FROM_DAMM_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConsolidateCoinFromDammIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn consolidate_coin_from_damm_ix(
    keys: ConsolidateCoinFromDammKeys,
    args: ConsolidateCoinFromDammIxArgs,
) -> std::io::Result<Instruction> {
    consolidate_coin_from_damm_ix_with_program_id(RUNNER_RODEO_PROGRAM_ID, keys, args)
}
pub fn consolidate_coin_from_damm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConsolidateCoinFromDammAccounts<'_, '_>,
    args: ConsolidateCoinFromDammIxArgs,
) -> ProgramResult {
    let keys: ConsolidateCoinFromDammKeys = accounts.into();
    let ix = consolidate_coin_from_damm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn consolidate_coin_from_damm_invoke(
    accounts: ConsolidateCoinFromDammAccounts<'_, '_>,
    args: ConsolidateCoinFromDammIxArgs,
) -> ProgramResult {
    consolidate_coin_from_damm_invoke_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn consolidate_coin_from_damm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConsolidateCoinFromDammAccounts<'_, '_>,
    args: ConsolidateCoinFromDammIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConsolidateCoinFromDammKeys = accounts.into();
    let ix = consolidate_coin_from_damm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn consolidate_coin_from_damm_invoke_signed(
    accounts: ConsolidateCoinFromDammAccounts<'_, '_>,
    args: ConsolidateCoinFromDammIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    consolidate_coin_from_damm_invoke_signed_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn consolidate_coin_from_damm_verify_account_keys(
    accounts: ConsolidateCoinFromDammAccounts<'_, '_>,
    keys: ConsolidateCoinFromDammKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer_burn_account.key, keys.payer_burn_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.consolidation.key, keys.consolidation),
        (*accounts.consolidation_vault.key, keys.consolidation_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.damm_pool.key, keys.damm_pool),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.position.key, keys.position),
        (*accounts.damm_pool_authority.key, keys.damm_pool_authority),
        (*accounts.damm_event_authority.key, keys.damm_event_authority),
        (*accounts.damm_program.key, keys.damm_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.payer.key, keys.payer),
        (*accounts.old_coin.key, keys.old_coin),
        (*accounts.config.key, keys.config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn consolidate_coin_from_damm_verify_writable_privileges<'me, 'info>(
    accounts: ConsolidateCoinFromDammAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer_burn_account,
        accounts.consolidation,
        accounts.consolidation_vault,
        accounts.base_mint,
        accounts.damm_pool,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.position_nft_account,
        accounts.position,
        accounts.damm_pool_authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn consolidate_coin_from_damm_verify_account_privileges<'me, 'info>(
    accounts: ConsolidateCoinFromDammAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    consolidate_coin_from_damm_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const FUND_DISTRIBUTOR_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct FundDistributorAccounts<'me, 'info> {
    pub distributor_vault: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub distributor_pubkey: &'me AccountInfo<'info>,
    pub distributor_program_id: &'me AccountInfo<'info>,
    pub wsol_token_account: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub coin: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub creator_rewards_vault: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub holder_rewards_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FundDistributorKeys {
    pub distributor_vault: Pubkey,
    pub wsol_mint: Pubkey,
    pub authority: Pubkey,
    pub token_program: Pubkey,
    pub config: Pubkey,
    pub payer: Pubkey,
    pub distributor_pubkey: Pubkey,
    pub distributor_program_id: Pubkey,
    pub wsol_token_account: Pubkey,
    pub quote_mint: Pubkey,
    pub coin: Pubkey,
    pub quote_vault: Pubkey,
    pub creator_rewards_vault: Pubkey,
    pub base_vault: Pubkey,
    pub holder_rewards_vault: Pubkey,
    pub system_program: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
}
impl From<FundDistributorAccounts<'_, '_>> for FundDistributorKeys {
    fn from(accounts: FundDistributorAccounts) -> Self {
        Self {
            distributor_vault: *accounts.distributor_vault.key,
            wsol_mint: *accounts.wsol_mint.key,
            authority: *accounts.authority.key,
            token_program: *accounts.token_program.key,
            config: *accounts.config.key,
            payer: *accounts.payer.key,
            distributor_pubkey: *accounts.distributor_pubkey.key,
            distributor_program_id: *accounts.distributor_program_id.key,
            wsol_token_account: *accounts.wsol_token_account.key,
            quote_mint: *accounts.quote_mint.key,
            coin: *accounts.coin.key,
            quote_vault: *accounts.quote_vault.key,
            creator_rewards_vault: *accounts.creator_rewards_vault.key,
            base_vault: *accounts.base_vault.key,
            holder_rewards_vault: *accounts.holder_rewards_vault.key,
            system_program: *accounts.system_program.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            creator: *accounts.creator.key,
            base_mint: *accounts.base_mint.key,
        }
    }
}
impl From<FundDistributorKeys> for [AccountMeta; FUND_DISTRIBUTOR_IX_ACCOUNTS_LEN] {
    fn from(keys: FundDistributorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.distributor_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wsol_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.distributor_pubkey,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.distributor_program_id,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wsol_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_rewards_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.holder_rewards_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FUND_DISTRIBUTOR_IX_ACCOUNTS_LEN]> for FundDistributorKeys {
    fn from(pubkeys: [Pubkey; FUND_DISTRIBUTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            distributor_vault: pubkeys[0],
            wsol_mint: pubkeys[1],
            authority: pubkeys[2],
            token_program: pubkeys[3],
            config: pubkeys[4],
            payer: pubkeys[5],
            distributor_pubkey: pubkeys[6],
            distributor_program_id: pubkeys[7],
            wsol_token_account: pubkeys[8],
            quote_mint: pubkeys[9],
            coin: pubkeys[10],
            quote_vault: pubkeys[11],
            creator_rewards_vault: pubkeys[12],
            base_vault: pubkeys[13],
            holder_rewards_vault: pubkeys[14],
            system_program: pubkeys[15],
            base_token_program: pubkeys[16],
            quote_token_program: pubkeys[17],
            event_authority: pubkeys[18],
            program: pubkeys[19],
            creator: pubkeys[20],
            base_mint: pubkeys[21],
        }
    }
}
impl<'info> From<FundDistributorAccounts<'_, 'info>>
for [AccountInfo<'info>; FUND_DISTRIBUTOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: FundDistributorAccounts<'_, 'info>) -> Self {
        [
            accounts.distributor_vault.clone(),
            accounts.wsol_mint.clone(),
            accounts.authority.clone(),
            accounts.token_program.clone(),
            accounts.config.clone(),
            accounts.payer.clone(),
            accounts.distributor_pubkey.clone(),
            accounts.distributor_program_id.clone(),
            accounts.wsol_token_account.clone(),
            accounts.quote_mint.clone(),
            accounts.coin.clone(),
            accounts.quote_vault.clone(),
            accounts.creator_rewards_vault.clone(),
            accounts.base_vault.clone(),
            accounts.holder_rewards_vault.clone(),
            accounts.system_program.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.creator.clone(),
            accounts.base_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FUND_DISTRIBUTOR_IX_ACCOUNTS_LEN]>
for FundDistributorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FUND_DISTRIBUTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            distributor_vault: &arr[0],
            wsol_mint: &arr[1],
            authority: &arr[2],
            token_program: &arr[3],
            config: &arr[4],
            payer: &arr[5],
            distributor_pubkey: &arr[6],
            distributor_program_id: &arr[7],
            wsol_token_account: &arr[8],
            quote_mint: &arr[9],
            coin: &arr[10],
            quote_vault: &arr[11],
            creator_rewards_vault: &arr[12],
            base_vault: &arr[13],
            holder_rewards_vault: &arr[14],
            system_program: &arr[15],
            base_token_program: &arr[16],
            quote_token_program: &arr[17],
            event_authority: &arr[18],
            program: &arr[19],
            creator: &arr[20],
            base_mint: &arr[21],
        }
    }
}
pub const FUND_DISTRIBUTOR_IX_DISCM: [u8; 8usize] = [
    223, 255, 163, 89, 36, 250, 65, 156,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundDistributorIxArgs {
    pub field_0: u64,
    pub field_1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundDistributorIxData(pub FundDistributorIxArgs);
impl From<FundDistributorIxArgs> for FundDistributorIxData {
    fn from(args: FundDistributorIxArgs) -> Self {
        Self(args)
    }
}
impl FundDistributorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUND_DISTRIBUTOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(FundDistributorIxArgs {
                field_0,
                field_1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUND_DISTRIBUTOR_IX_DISCM)?;
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
pub fn fund_distributor_ix_with_program_id(
    program_id: Pubkey,
    keys: FundDistributorKeys,
    args: FundDistributorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FUND_DISTRIBUTOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: FundDistributorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fund_distributor_ix(
    keys: FundDistributorKeys,
    args: FundDistributorIxArgs,
) -> std::io::Result<Instruction> {
    fund_distributor_ix_with_program_id(RUNNER_RODEO_PROGRAM_ID, keys, args)
}
pub fn fund_distributor_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FundDistributorAccounts<'_, '_>,
    args: FundDistributorIxArgs,
) -> ProgramResult {
    let keys: FundDistributorKeys = accounts.into();
    let ix = fund_distributor_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fund_distributor_invoke(
    accounts: FundDistributorAccounts<'_, '_>,
    args: FundDistributorIxArgs,
) -> ProgramResult {
    fund_distributor_invoke_with_program_id(RUNNER_RODEO_PROGRAM_ID, accounts, args)
}
pub fn fund_distributor_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FundDistributorAccounts<'_, '_>,
    args: FundDistributorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FundDistributorKeys = accounts.into();
    let ix = fund_distributor_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fund_distributor_invoke_signed(
    accounts: FundDistributorAccounts<'_, '_>,
    args: FundDistributorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fund_distributor_invoke_signed_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn fund_distributor_verify_account_keys(
    accounts: FundDistributorAccounts<'_, '_>,
    keys: FundDistributorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.distributor_vault.key, keys.distributor_vault),
        (*accounts.wsol_mint.key, keys.wsol_mint),
        (*accounts.authority.key, keys.authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.config.key, keys.config),
        (*accounts.payer.key, keys.payer),
        (*accounts.distributor_pubkey.key, keys.distributor_pubkey),
        (*accounts.distributor_program_id.key, keys.distributor_program_id),
        (*accounts.wsol_token_account.key, keys.wsol_token_account),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.coin.key, keys.coin),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.creator_rewards_vault.key, keys.creator_rewards_vault),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.holder_rewards_vault.key, keys.holder_rewards_vault),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.creator.key, keys.creator),
        (*accounts.base_mint.key, keys.base_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fund_distributor_verify_writable_privileges<'me, 'info>(
    accounts: FundDistributorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.distributor_vault,
        accounts.payer,
        accounts.wsol_token_account,
        accounts.coin,
        accounts.quote_vault,
        accounts.creator_rewards_vault,
        accounts.base_vault,
        accounts.holder_rewards_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fund_distributor_verify_account_privileges<'me, 'info>(
    accounts: FundDistributorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fund_distributor_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const HARVEST_TRANSFER_FEES_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct HarvestTransferFeesAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub damm_pool_authority: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub damm_pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub damm_event_authority: &'me AccountInfo<'info>,
    pub damm_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub coin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HarvestTransferFeesKeys {
    pub pool: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub damm_pool_authority: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub damm_pool: Pubkey,
    pub position: Pubkey,
    pub damm_event_authority: Pubkey,
    pub damm_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub config: Pubkey,
    pub coin: Pubkey,
}
impl From<HarvestTransferFeesAccounts<'_, '_>> for HarvestTransferFeesKeys {
    fn from(accounts: HarvestTransferFeesAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            damm_pool_authority: *accounts.damm_pool_authority.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            damm_pool: *accounts.damm_pool.key,
            position: *accounts.position.key,
            damm_event_authority: *accounts.damm_event_authority.key,
            damm_program: *accounts.damm_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            config: *accounts.config.key,
            coin: *accounts.coin.key,
        }
    }
}
impl From<HarvestTransferFeesKeys>
for [AccountMeta; HARVEST_TRANSFER_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: HarvestTransferFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; HARVEST_TRANSFER_FEES_IX_ACCOUNTS_LEN]> for HarvestTransferFeesKeys {
    fn from(pubkeys: [Pubkey; HARVEST_TRANSFER_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            base_vault: pubkeys[1],
            quote_vault: pubkeys[2],
            base_mint: pubkeys[3],
            quote_mint: pubkeys[4],
            base_token_program: pubkeys[5],
            quote_token_program: pubkeys[6],
            token_a_vault: pubkeys[7],
            token_b_vault: pubkeys[8],
            damm_pool_authority: pubkeys[9],
            position_nft_mint: pubkeys[10],
            position_nft_account: pubkeys[11],
            damm_pool: pubkeys[12],
            position: pubkeys[13],
            damm_event_authority: pubkeys[14],
            damm_program: pubkeys[15],
            token_2022_program: pubkeys[16],
            token_program: pubkeys[17],
            system_program: pubkeys[18],
            event_authority: pubkeys[19],
            program: pubkeys[20],
            authority: pubkeys[21],
            payer: pubkeys[22],
            config: pubkeys[23],
            coin: pubkeys[24],
        }
    }
}
impl<'info> From<HarvestTransferFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; HARVEST_TRANSFER_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: HarvestTransferFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.damm_pool_authority.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.damm_pool.clone(),
            accounts.position.clone(),
            accounts.damm_event_authority.clone(),
            accounts.damm_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.config.clone(),
            accounts.coin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; HARVEST_TRANSFER_FEES_IX_ACCOUNTS_LEN]>
for HarvestTransferFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; HARVEST_TRANSFER_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            base_vault: &arr[1],
            quote_vault: &arr[2],
            base_mint: &arr[3],
            quote_mint: &arr[4],
            base_token_program: &arr[5],
            quote_token_program: &arr[6],
            token_a_vault: &arr[7],
            token_b_vault: &arr[8],
            damm_pool_authority: &arr[9],
            position_nft_mint: &arr[10],
            position_nft_account: &arr[11],
            damm_pool: &arr[12],
            position: &arr[13],
            damm_event_authority: &arr[14],
            damm_program: &arr[15],
            token_2022_program: &arr[16],
            token_program: &arr[17],
            system_program: &arr[18],
            event_authority: &arr[19],
            program: &arr[20],
            authority: &arr[21],
            payer: &arr[22],
            config: &arr[23],
            coin: &arr[24],
        }
    }
}
pub const HARVEST_TRANSFER_FEES_IX_DISCM: [u8; 8usize] = [
    246, 104, 231, 171, 131, 105, 50, 55,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HarvestTransferFeesIxArgs {
    pub field_0: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct HarvestTransferFeesIxData(pub HarvestTransferFeesIxArgs);
impl From<HarvestTransferFeesIxArgs> for HarvestTransferFeesIxData {
    fn from(args: HarvestTransferFeesIxArgs) -> Self {
        Self(args)
    }
}
impl HarvestTransferFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HARVEST_TRANSFER_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(HarvestTransferFeesIxArgs {
                field_0,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HARVEST_TRANSFER_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn harvest_transfer_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: HarvestTransferFeesKeys,
    args: HarvestTransferFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; HARVEST_TRANSFER_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: HarvestTransferFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn harvest_transfer_fees_ix(
    keys: HarvestTransferFeesKeys,
    args: HarvestTransferFeesIxArgs,
) -> std::io::Result<Instruction> {
    harvest_transfer_fees_ix_with_program_id(RUNNER_RODEO_PROGRAM_ID, keys, args)
}
pub fn harvest_transfer_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: HarvestTransferFeesAccounts<'_, '_>,
    args: HarvestTransferFeesIxArgs,
) -> ProgramResult {
    let keys: HarvestTransferFeesKeys = accounts.into();
    let ix = harvest_transfer_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn harvest_transfer_fees_invoke(
    accounts: HarvestTransferFeesAccounts<'_, '_>,
    args: HarvestTransferFeesIxArgs,
) -> ProgramResult {
    harvest_transfer_fees_invoke_with_program_id(RUNNER_RODEO_PROGRAM_ID, accounts, args)
}
pub fn harvest_transfer_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: HarvestTransferFeesAccounts<'_, '_>,
    args: HarvestTransferFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: HarvestTransferFeesKeys = accounts.into();
    let ix = harvest_transfer_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn harvest_transfer_fees_invoke_signed(
    accounts: HarvestTransferFeesAccounts<'_, '_>,
    args: HarvestTransferFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    harvest_transfer_fees_invoke_signed_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn harvest_transfer_fees_verify_account_keys(
    accounts: HarvestTransferFeesAccounts<'_, '_>,
    keys: HarvestTransferFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.damm_pool_authority.key, keys.damm_pool_authority),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.damm_pool.key, keys.damm_pool),
        (*accounts.position.key, keys.position),
        (*accounts.damm_event_authority.key, keys.damm_event_authority),
        (*accounts.damm_program.key, keys.damm_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.authority.key, keys.authority),
        (*accounts.payer.key, keys.payer),
        (*accounts.config.key, keys.config),
        (*accounts.coin.key, keys.coin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn harvest_transfer_fees_verify_writable_privileges<'me, 'info>(
    accounts: HarvestTransferFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.damm_pool_authority,
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.damm_pool,
        accounts.position,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn harvest_transfer_fees_verify_signer_privileges<'me, 'info>(
    accounts: HarvestTransferFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_nft_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn harvest_transfer_fees_verify_account_privileges<'me, 'info>(
    accounts: HarvestTransferFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    harvest_transfer_fees_verify_writable_privileges(accounts)?;
    harvest_transfer_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_COIN_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct InitCoinAccounts<'me, 'info> {
    pub quote_vault: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub creator_rewards_vault: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub holder_rewards_vault: &'me AccountInfo<'info>,
    pub protocol_fee_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitCoinKeys {
    pub quote_vault: Pubkey,
    pub output_token_account: Pubkey,
    pub creator_rewards_vault: Pubkey,
    pub base_vault: Pubkey,
    pub holder_rewards_vault: Pubkey,
    pub protocol_fee_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub quote_token_program: Pubkey,
    pub system_program: Pubkey,
    pub program: Pubkey,
    pub authority: Pubkey,
    pub config: Pubkey,
    pub pool: Pubkey,
    pub input_token_account: Pubkey,
}
impl From<InitCoinAccounts<'_, '_>> for InitCoinKeys {
    fn from(accounts: InitCoinAccounts) -> Self {
        Self {
            quote_vault: *accounts.quote_vault.key,
            output_token_account: *accounts.output_token_account.key,
            creator_rewards_vault: *accounts.creator_rewards_vault.key,
            base_vault: *accounts.base_vault.key,
            holder_rewards_vault: *accounts.holder_rewards_vault.key,
            protocol_fee_vault: *accounts.protocol_fee_vault.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_token_program: *accounts.base_token_program.key,
            event_authority: *accounts.event_authority.key,
            quote_token_program: *accounts.quote_token_program.key,
            system_program: *accounts.system_program.key,
            program: *accounts.program.key,
            authority: *accounts.authority.key,
            config: *accounts.config.key,
            pool: *accounts.pool.key,
            input_token_account: *accounts.input_token_account.key,
        }
    }
}
impl From<InitCoinKeys> for [AccountMeta; INIT_COIN_IX_ACCOUNTS_LEN] {
    fn from(keys: InitCoinKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_rewards_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.holder_rewards_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_COIN_IX_ACCOUNTS_LEN]> for InitCoinKeys {
    fn from(pubkeys: [Pubkey; INIT_COIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            quote_vault: pubkeys[0],
            output_token_account: pubkeys[1],
            creator_rewards_vault: pubkeys[2],
            base_vault: pubkeys[3],
            holder_rewards_vault: pubkeys[4],
            protocol_fee_vault: pubkeys[5],
            base_mint: pubkeys[6],
            quote_mint: pubkeys[7],
            base_token_program: pubkeys[8],
            event_authority: pubkeys[9],
            quote_token_program: pubkeys[10],
            system_program: pubkeys[11],
            program: pubkeys[12],
            authority: pubkeys[13],
            config: pubkeys[14],
            pool: pubkeys[15],
            input_token_account: pubkeys[16],
        }
    }
}
impl<'info> From<InitCoinAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_COIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitCoinAccounts<'_, 'info>) -> Self {
        [
            accounts.quote_vault.clone(),
            accounts.output_token_account.clone(),
            accounts.creator_rewards_vault.clone(),
            accounts.base_vault.clone(),
            accounts.holder_rewards_vault.clone(),
            accounts.protocol_fee_vault.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.quote_token_program.clone(),
            accounts.system_program.clone(),
            accounts.program.clone(),
            accounts.authority.clone(),
            accounts.config.clone(),
            accounts.pool.clone(),
            accounts.input_token_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_COIN_IX_ACCOUNTS_LEN]>
for InitCoinAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_COIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            quote_vault: &arr[0],
            output_token_account: &arr[1],
            creator_rewards_vault: &arr[2],
            base_vault: &arr[3],
            holder_rewards_vault: &arr[4],
            protocol_fee_vault: &arr[5],
            base_mint: &arr[6],
            quote_mint: &arr[7],
            base_token_program: &arr[8],
            event_authority: &arr[9],
            quote_token_program: &arr[10],
            system_program: &arr[11],
            program: &arr[12],
            authority: &arr[13],
            config: &arr[14],
            pool: &arr[15],
            input_token_account: &arr[16],
        }
    }
}
pub const INIT_COIN_IX_DISCM: [u8; 8usize] = [122, 52, 164, 62, 152, 32, 150, 168];
#[derive(Clone, Debug, PartialEq)]
pub struct InitCoinIxData;
impl InitCoinIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_COIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_COIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_coin_ix_with_program_id(
    program_id: Pubkey,
    keys: InitCoinKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_COIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitCoinIxData.try_to_vec()?,
    })
}
pub fn init_coin_ix(keys: InitCoinKeys) -> std::io::Result<Instruction> {
    init_coin_ix_with_program_id(RUNNER_RODEO_PROGRAM_ID, keys)
}
pub fn init_coin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitCoinAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitCoinKeys = accounts.into();
    let ix = init_coin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_coin_invoke(accounts: InitCoinAccounts<'_, '_>) -> ProgramResult {
    init_coin_invoke_with_program_id(RUNNER_RODEO_PROGRAM_ID, accounts)
}
pub fn init_coin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitCoinAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitCoinKeys = accounts.into();
    let ix = init_coin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_coin_invoke_signed(
    accounts: InitCoinAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_coin_invoke_signed_with_program_id(RUNNER_RODEO_PROGRAM_ID, accounts, seeds)
}
pub fn init_coin_verify_account_keys(
    accounts: InitCoinAccounts<'_, '_>,
    keys: InitCoinKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.creator_rewards_vault.key, keys.creator_rewards_vault),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.holder_rewards_vault.key, keys.holder_rewards_vault),
        (*accounts.protocol_fee_vault.key, keys.protocol_fee_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program.key, keys.program),
        (*accounts.authority.key, keys.authority),
        (*accounts.config.key, keys.config),
        (*accounts.pool.key, keys.pool),
        (*accounts.input_token_account.key, keys.input_token_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_coin_verify_writable_privileges<'me, 'info>(
    accounts: InitCoinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.output_token_account,
        accounts.creator_rewards_vault,
        accounts.base_vault,
        accounts.holder_rewards_vault,
        accounts.protocol_fee_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_coin_verify_account_privileges<'me, 'info>(
    accounts: InitCoinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_coin_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INIT_CONFIG_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct InitConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub damm_program: &'me AccountInfo<'info>,
    pub damm_event_authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub coin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitConfigKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub output_token_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_b_mint: Pubkey,
    pub payer: Pubkey,
    pub token_b_program: Pubkey,
    pub damm_program: Pubkey,
    pub damm_event_authority: Pubkey,
    pub event_authority: Pubkey,
    pub system_program: Pubkey,
    pub program: Pubkey,
    pub mint: Pubkey,
    pub coin: Pubkey,
}
impl From<InitConfigAccounts<'_, '_>> for InitConfigKeys {
    fn from(accounts: InitConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            fee_recipient: *accounts.fee_recipient.key,
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            output_token_account: *accounts.output_token_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_b_mint: *accounts.token_b_mint.key,
            payer: *accounts.payer.key,
            token_b_program: *accounts.token_b_program.key,
            damm_program: *accounts.damm_program.key,
            damm_event_authority: *accounts.damm_event_authority.key,
            event_authority: *accounts.event_authority.key,
            system_program: *accounts.system_program.key,
            program: *accounts.program.key,
            mint: *accounts.mint.key,
            coin: *accounts.coin.key,
        }
    }
}
impl From<InitConfigKeys> for [AccountMeta; INIT_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
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
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_CONFIG_IX_ACCOUNTS_LEN]> for InitConfigKeys {
    fn from(pubkeys: [Pubkey; INIT_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            fee_recipient: pubkeys[2],
            pool_authority: pubkeys[3],
            pool: pubkeys[4],
            output_token_account: pubkeys[5],
            token_a_vault: pubkeys[6],
            token_b_vault: pubkeys[7],
            token_b_mint: pubkeys[8],
            payer: pubkeys[9],
            token_b_program: pubkeys[10],
            damm_program: pubkeys[11],
            damm_event_authority: pubkeys[12],
            event_authority: pubkeys[13],
            system_program: pubkeys[14],
            program: pubkeys[15],
            mint: pubkeys[16],
            coin: pubkeys[17],
        }
    }
}
impl<'info> From<InitConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.fee_recipient.clone(),
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.output_token_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_b_mint.clone(),
            accounts.payer.clone(),
            accounts.token_b_program.clone(),
            accounts.damm_program.clone(),
            accounts.damm_event_authority.clone(),
            accounts.event_authority.clone(),
            accounts.system_program.clone(),
            accounts.program.clone(),
            accounts.mint.clone(),
            accounts.coin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_CONFIG_IX_ACCOUNTS_LEN]>
for InitConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            fee_recipient: &arr[2],
            pool_authority: &arr[3],
            pool: &arr[4],
            output_token_account: &arr[5],
            token_a_vault: &arr[6],
            token_b_vault: &arr[7],
            token_b_mint: &arr[8],
            payer: &arr[9],
            token_b_program: &arr[10],
            damm_program: &arr[11],
            damm_event_authority: &arr[12],
            event_authority: &arr[13],
            system_program: &arr[14],
            program: &arr[15],
            mint: &arr[16],
            coin: &arr[17],
        }
    }
}
pub const INIT_CONFIG_IX_DISCM: [u8; 8usize] = [23, 235, 115, 232, 168, 96, 1, 231];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitConfigIxArgs {
    pub field_0: u8,
    pub field_1: Pubkey,
    pub field_2: Pubkey,
    pub field_3: u16,
    pub field_4: u16,
    pub field_5: u16,
    pub field_6: u16,
    pub field_7: u16,
    pub field_8: u16,
    pub field_9: u64,
    pub field_10: u64,
    pub field_11: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitConfigIxData(pub InitConfigIxArgs);
impl From<InitConfigIxArgs> for InitConfigIxData {
    fn from(args: InitConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field_2: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field_3: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_4: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_5: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_6: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_7: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_8: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_9: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_10: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_11: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitConfigIxArgs {
                field_0,
                field_1,
                field_2,
                field_3,
                field_4,
                field_5,
                field_6,
                field_7,
                field_8,
                field_9,
                field_10,
                field_11,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_6, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_7, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_8, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_9, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_10, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_11, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitConfigKeys,
    args: InitConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_config_ix(
    keys: InitConfigKeys,
    args: InitConfigIxArgs,
) -> std::io::Result<Instruction> {
    init_config_ix_with_program_id(RUNNER_RODEO_PROGRAM_ID, keys, args)
}
pub fn init_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitConfigAccounts<'_, '_>,
    args: InitConfigIxArgs,
) -> ProgramResult {
    let keys: InitConfigKeys = accounts.into();
    let ix = init_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_config_invoke(
    accounts: InitConfigAccounts<'_, '_>,
    args: InitConfigIxArgs,
) -> ProgramResult {
    init_config_invoke_with_program_id(RUNNER_RODEO_PROGRAM_ID, accounts, args)
}
pub fn init_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitConfigAccounts<'_, '_>,
    args: InitConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitConfigKeys = accounts.into();
    let ix = init_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_config_invoke_signed(
    accounts: InitConfigAccounts<'_, '_>,
    args: InitConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_config_invoke_signed_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_config_verify_account_keys(
    accounts: InitConfigAccounts<'_, '_>,
    keys: InitConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_b_program.key, keys.token_b_program),
        (*accounts.damm_program.key, keys.damm_program),
        (*accounts.damm_event_authority.key, keys.damm_event_authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program.key, keys.program),
        (*accounts.mint.key, keys.mint),
        (*accounts.coin.key, keys.coin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_config_verify_writable_privileges<'me, 'info>(
    accounts: InitConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.output_token_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_config_verify_signer_privileges<'me, 'info>(
    accounts: InitConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_recipient, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_config_verify_account_privileges<'me, 'info>(
    accounts: InitConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_config_verify_writable_privileges(accounts)?;
    init_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_CURVE_POOL_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct MigrateCurvePoolAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub damm_pool_authority: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub damm_pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub damm_event_authority: &'me AccountInfo<'info>,
    pub damm_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub coin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateCurvePoolKeys {
    pub pool: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub damm_pool_authority: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub damm_pool: Pubkey,
    pub position: Pubkey,
    pub damm_event_authority: Pubkey,
    pub damm_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub config: Pubkey,
    pub coin: Pubkey,
}
impl From<MigrateCurvePoolAccounts<'_, '_>> for MigrateCurvePoolKeys {
    fn from(accounts: MigrateCurvePoolAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            damm_pool_authority: *accounts.damm_pool_authority.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            damm_pool: *accounts.damm_pool.key,
            position: *accounts.position.key,
            damm_event_authority: *accounts.damm_event_authority.key,
            damm_program: *accounts.damm_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            config: *accounts.config.key,
            coin: *accounts.coin.key,
        }
    }
}
impl From<MigrateCurvePoolKeys> for [AccountMeta; MIGRATE_CURVE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateCurvePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MIGRATE_CURVE_POOL_IX_ACCOUNTS_LEN]> for MigrateCurvePoolKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_CURVE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            base_vault: pubkeys[1],
            quote_vault: pubkeys[2],
            base_mint: pubkeys[3],
            quote_mint: pubkeys[4],
            base_token_program: pubkeys[5],
            quote_token_program: pubkeys[6],
            token_a_vault: pubkeys[7],
            token_b_vault: pubkeys[8],
            damm_pool_authority: pubkeys[9],
            position_nft_mint: pubkeys[10],
            position_nft_account: pubkeys[11],
            damm_pool: pubkeys[12],
            position: pubkeys[13],
            damm_event_authority: pubkeys[14],
            damm_program: pubkeys[15],
            token_2022_program: pubkeys[16],
            token_program: pubkeys[17],
            system_program: pubkeys[18],
            event_authority: pubkeys[19],
            program: pubkeys[20],
            authority: pubkeys[21],
            payer: pubkeys[22],
            config: pubkeys[23],
            coin: pubkeys[24],
        }
    }
}
impl<'info> From<MigrateCurvePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_CURVE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateCurvePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.damm_pool_authority.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.damm_pool.clone(),
            accounts.position.clone(),
            accounts.damm_event_authority.clone(),
            accounts.damm_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.config.clone(),
            accounts.coin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_CURVE_POOL_IX_ACCOUNTS_LEN]>
for MigrateCurvePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_CURVE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            base_vault: &arr[1],
            quote_vault: &arr[2],
            base_mint: &arr[3],
            quote_mint: &arr[4],
            base_token_program: &arr[5],
            quote_token_program: &arr[6],
            token_a_vault: &arr[7],
            token_b_vault: &arr[8],
            damm_pool_authority: &arr[9],
            position_nft_mint: &arr[10],
            position_nft_account: &arr[11],
            damm_pool: &arr[12],
            position: &arr[13],
            damm_event_authority: &arr[14],
            damm_program: &arr[15],
            token_2022_program: &arr[16],
            token_program: &arr[17],
            system_program: &arr[18],
            event_authority: &arr[19],
            program: &arr[20],
            authority: &arr[21],
            payer: &arr[22],
            config: &arr[23],
            coin: &arr[24],
        }
    }
}
pub const MIGRATE_CURVE_POOL_IX_DISCM: [u8; 8usize] = [10, 12, 1, 132, 163, 90, 63, 132];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateCurvePoolIxData;
impl MigrateCurvePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_CURVE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_CURVE_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_curve_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateCurvePoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_CURVE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateCurvePoolIxData.try_to_vec()?,
    })
}
pub fn migrate_curve_pool_ix(
    keys: MigrateCurvePoolKeys,
) -> std::io::Result<Instruction> {
    migrate_curve_pool_ix_with_program_id(RUNNER_RODEO_PROGRAM_ID, keys)
}
pub fn migrate_curve_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateCurvePoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateCurvePoolKeys = accounts.into();
    let ix = migrate_curve_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_curve_pool_invoke(
    accounts: MigrateCurvePoolAccounts<'_, '_>,
) -> ProgramResult {
    migrate_curve_pool_invoke_with_program_id(RUNNER_RODEO_PROGRAM_ID, accounts)
}
pub fn migrate_curve_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateCurvePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateCurvePoolKeys = accounts.into();
    let ix = migrate_curve_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_curve_pool_invoke_signed(
    accounts: MigrateCurvePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_curve_pool_invoke_signed_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn migrate_curve_pool_verify_account_keys(
    accounts: MigrateCurvePoolAccounts<'_, '_>,
    keys: MigrateCurvePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.damm_pool_authority.key, keys.damm_pool_authority),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.damm_pool.key, keys.damm_pool),
        (*accounts.position.key, keys.position),
        (*accounts.damm_event_authority.key, keys.damm_event_authority),
        (*accounts.damm_program.key, keys.damm_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.authority.key, keys.authority),
        (*accounts.payer.key, keys.payer),
        (*accounts.config.key, keys.config),
        (*accounts.coin.key, keys.coin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_curve_pool_verify_writable_privileges<'me, 'info>(
    accounts: MigrateCurvePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.damm_pool_authority,
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.damm_pool,
        accounts.position,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_curve_pool_verify_signer_privileges<'me, 'info>(
    accounts: MigrateCurvePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_nft_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_curve_pool_verify_account_privileges<'me, 'info>(
    accounts: MigrateCurvePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_curve_pool_verify_writable_privileges(accounts)?;
    migrate_curve_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub holder_rewards_vault: &'me AccountInfo<'info>,
    pub protocol_fee_vault: &'me AccountInfo<'info>,
    pub creator_rewards_vault: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub coin: &'me AccountInfo<'info>,
    pub new_coin: &'me AccountInfo<'info>,
    pub old_coin: &'me AccountInfo<'info>,
    pub consolidation_distribution_vault: &'me AccountInfo<'info>,
    pub creator_token_account: &'me AccountInfo<'info>,
    pub old_mint: &'me AccountInfo<'info>,
    pub new_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub payer_input_account: &'me AccountInfo<'info>,
    pub payer_output_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub holder_rewards_vault: Pubkey,
    pub protocol_fee_vault: Pubkey,
    pub creator_rewards_vault: Pubkey,
    pub quote_mint: Pubkey,
    pub quote_token_program: Pubkey,
    pub system_program: Pubkey,
    pub authority: Pubkey,
    pub creator: Pubkey,
    pub coin: Pubkey,
    pub new_coin: Pubkey,
    pub old_coin: Pubkey,
    pub consolidation_distribution_vault: Pubkey,
    pub creator_token_account: Pubkey,
    pub old_mint: Pubkey,
    pub new_mint: Pubkey,
    pub event_authority: Pubkey,
    pub token_program: Pubkey,
    pub program: Pubkey,
    pub payer: Pubkey,
    pub payer_input_account: Pubkey,
    pub payer_output_account: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            holder_rewards_vault: *accounts.holder_rewards_vault.key,
            protocol_fee_vault: *accounts.protocol_fee_vault.key,
            creator_rewards_vault: *accounts.creator_rewards_vault.key,
            quote_mint: *accounts.quote_mint.key,
            quote_token_program: *accounts.quote_token_program.key,
            system_program: *accounts.system_program.key,
            authority: *accounts.authority.key,
            creator: *accounts.creator.key,
            coin: *accounts.coin.key,
            new_coin: *accounts.new_coin.key,
            old_coin: *accounts.old_coin.key,
            consolidation_distribution_vault: *accounts
                .consolidation_distribution_vault
                .key,
            creator_token_account: *accounts.creator_token_account.key,
            old_mint: *accounts.old_mint.key,
            new_mint: *accounts.new_mint.key,
            event_authority: *accounts.event_authority.key,
            token_program: *accounts.token_program.key,
            program: *accounts.program.key,
            payer: *accounts.payer.key,
            payer_input_account: *accounts.payer_input_account.key,
            payer_output_account: *accounts.payer_output_account.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.holder_rewards_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_rewards_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_coin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_coin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.consolidation_distribution_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer_input_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer_output_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            holder_rewards_vault: pubkeys[0],
            protocol_fee_vault: pubkeys[1],
            creator_rewards_vault: pubkeys[2],
            quote_mint: pubkeys[3],
            quote_token_program: pubkeys[4],
            system_program: pubkeys[5],
            authority: pubkeys[6],
            creator: pubkeys[7],
            coin: pubkeys[8],
            new_coin: pubkeys[9],
            old_coin: pubkeys[10],
            consolidation_distribution_vault: pubkeys[11],
            creator_token_account: pubkeys[12],
            old_mint: pubkeys[13],
            new_mint: pubkeys[14],
            event_authority: pubkeys[15],
            token_program: pubkeys[16],
            program: pubkeys[17],
            payer: pubkeys[18],
            payer_input_account: pubkeys[19],
            payer_output_account: pubkeys[20],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.holder_rewards_vault.clone(),
            accounts.protocol_fee_vault.clone(),
            accounts.creator_rewards_vault.clone(),
            accounts.quote_mint.clone(),
            accounts.quote_token_program.clone(),
            accounts.system_program.clone(),
            accounts.authority.clone(),
            accounts.creator.clone(),
            accounts.coin.clone(),
            accounts.new_coin.clone(),
            accounts.old_coin.clone(),
            accounts.consolidation_distribution_vault.clone(),
            accounts.creator_token_account.clone(),
            accounts.old_mint.clone(),
            accounts.new_mint.clone(),
            accounts.event_authority.clone(),
            accounts.token_program.clone(),
            accounts.program.clone(),
            accounts.payer.clone(),
            accounts.payer_input_account.clone(),
            accounts.payer_output_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            holder_rewards_vault: &arr[0],
            protocol_fee_vault: &arr[1],
            creator_rewards_vault: &arr[2],
            quote_mint: &arr[3],
            quote_token_program: &arr[4],
            system_program: &arr[5],
            authority: &arr[6],
            creator: &arr[7],
            coin: &arr[8],
            new_coin: &arr[9],
            old_coin: &arr[10],
            consolidation_distribution_vault: &arr[11],
            creator_token_account: &arr[12],
            old_mint: &arr[13],
            new_mint: &arr[14],
            event_authority: &arr[15],
            token_program: &arr[16],
            program: &arr[17],
            payer: &arr[18],
            payer_input_account: &arr[19],
            payer_output_account: &arr[20],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub field_0: u64,
    pub field_1: u64,
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
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SwapIxArgs { field_0, field_1 }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
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
    swap_ix_with_program_id(RUNNER_RODEO_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(RUNNER_RODEO_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(RUNNER_RODEO_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.holder_rewards_vault.key, keys.holder_rewards_vault),
        (*accounts.protocol_fee_vault.key, keys.protocol_fee_vault),
        (*accounts.creator_rewards_vault.key, keys.creator_rewards_vault),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.authority.key, keys.authority),
        (*accounts.creator.key, keys.creator),
        (*accounts.coin.key, keys.coin),
        (*accounts.new_coin.key, keys.new_coin),
        (*accounts.old_coin.key, keys.old_coin),
        (
            *accounts.consolidation_distribution_vault.key,
            keys.consolidation_distribution_vault,
        ),
        (*accounts.creator_token_account.key, keys.creator_token_account),
        (*accounts.old_mint.key, keys.old_mint),
        (*accounts.new_mint.key, keys.new_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.program.key, keys.program),
        (*accounts.payer.key, keys.payer),
        (*accounts.payer_input_account.key, keys.payer_input_account),
        (*accounts.payer_output_account.key, keys.payer_output_account),
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
        accounts.creator_rewards_vault,
        accounts.consolidation_distribution_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_verify_account_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SWEEP_HOLDER_REWARDS_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct SweepHolderRewardsAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub wsol_token_account: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub coin: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub creator_rewards_vault: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub holder_rewards_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SweepHolderRewardsKeys {
    pub token_program: Pubkey,
    pub authority: Pubkey,
    pub wsol_token_account: Pubkey,
    pub wsol_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub coin: Pubkey,
    pub quote_vault: Pubkey,
    pub creator_rewards_vault: Pubkey,
    pub base_vault: Pubkey,
    pub holder_rewards_vault: Pubkey,
    pub system_program: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub creator: Pubkey,
    pub payer: Pubkey,
    pub base_mint: Pubkey,
}
impl From<SweepHolderRewardsAccounts<'_, '_>> for SweepHolderRewardsKeys {
    fn from(accounts: SweepHolderRewardsAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            authority: *accounts.authority.key,
            wsol_token_account: *accounts.wsol_token_account.key,
            wsol_mint: *accounts.wsol_mint.key,
            quote_mint: *accounts.quote_mint.key,
            coin: *accounts.coin.key,
            quote_vault: *accounts.quote_vault.key,
            creator_rewards_vault: *accounts.creator_rewards_vault.key,
            base_vault: *accounts.base_vault.key,
            holder_rewards_vault: *accounts.holder_rewards_vault.key,
            system_program: *accounts.system_program.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            creator: *accounts.creator.key,
            payer: *accounts.payer.key,
            base_mint: *accounts.base_mint.key,
        }
    }
}
impl From<SweepHolderRewardsKeys>
for [AccountMeta; SWEEP_HOLDER_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: SweepHolderRewardsKeys) -> Self {
        [
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
                pubkey: keys.wsol_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wsol_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_rewards_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.holder_rewards_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWEEP_HOLDER_REWARDS_IX_ACCOUNTS_LEN]> for SweepHolderRewardsKeys {
    fn from(pubkeys: [Pubkey; SWEEP_HOLDER_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            authority: pubkeys[1],
            wsol_token_account: pubkeys[2],
            wsol_mint: pubkeys[3],
            quote_mint: pubkeys[4],
            coin: pubkeys[5],
            quote_vault: pubkeys[6],
            creator_rewards_vault: pubkeys[7],
            base_vault: pubkeys[8],
            holder_rewards_vault: pubkeys[9],
            system_program: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
            creator: pubkeys[15],
            payer: pubkeys[16],
            base_mint: pubkeys[17],
        }
    }
}
impl<'info> From<SweepHolderRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; SWEEP_HOLDER_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SweepHolderRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.authority.clone(),
            accounts.wsol_token_account.clone(),
            accounts.wsol_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.coin.clone(),
            accounts.quote_vault.clone(),
            accounts.creator_rewards_vault.clone(),
            accounts.base_vault.clone(),
            accounts.holder_rewards_vault.clone(),
            accounts.system_program.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.creator.clone(),
            accounts.payer.clone(),
            accounts.base_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWEEP_HOLDER_REWARDS_IX_ACCOUNTS_LEN]>
for SweepHolderRewardsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWEEP_HOLDER_REWARDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_program: &arr[0],
            authority: &arr[1],
            wsol_token_account: &arr[2],
            wsol_mint: &arr[3],
            quote_mint: &arr[4],
            coin: &arr[5],
            quote_vault: &arr[6],
            creator_rewards_vault: &arr[7],
            base_vault: &arr[8],
            holder_rewards_vault: &arr[9],
            system_program: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
            creator: &arr[15],
            payer: &arr[16],
            base_mint: &arr[17],
        }
    }
}
pub const SWEEP_HOLDER_REWARDS_IX_DISCM: [u8; 8usize] = [
    168, 228, 70, 64, 234, 29, 224, 19,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SweepHolderRewardsIxData;
impl SweepHolderRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWEEP_HOLDER_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWEEP_HOLDER_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sweep_holder_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: SweepHolderRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWEEP_HOLDER_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SweepHolderRewardsIxData.try_to_vec()?,
    })
}
pub fn sweep_holder_rewards_ix(
    keys: SweepHolderRewardsKeys,
) -> std::io::Result<Instruction> {
    sweep_holder_rewards_ix_with_program_id(RUNNER_RODEO_PROGRAM_ID, keys)
}
pub fn sweep_holder_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SweepHolderRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SweepHolderRewardsKeys = accounts.into();
    let ix = sweep_holder_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn sweep_holder_rewards_invoke(
    accounts: SweepHolderRewardsAccounts<'_, '_>,
) -> ProgramResult {
    sweep_holder_rewards_invoke_with_program_id(RUNNER_RODEO_PROGRAM_ID, accounts)
}
pub fn sweep_holder_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SweepHolderRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SweepHolderRewardsKeys = accounts.into();
    let ix = sweep_holder_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sweep_holder_rewards_invoke_signed(
    accounts: SweepHolderRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sweep_holder_rewards_invoke_signed_with_program_id(
        RUNNER_RODEO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn sweep_holder_rewards_verify_account_keys(
    accounts: SweepHolderRewardsAccounts<'_, '_>,
    keys: SweepHolderRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (*accounts.authority.key, keys.authority),
        (*accounts.wsol_token_account.key, keys.wsol_token_account),
        (*accounts.wsol_mint.key, keys.wsol_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.coin.key, keys.coin),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.creator_rewards_vault.key, keys.creator_rewards_vault),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.holder_rewards_vault.key, keys.holder_rewards_vault),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.creator.key, keys.creator),
        (*accounts.payer.key, keys.payer),
        (*accounts.base_mint.key, keys.base_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sweep_holder_rewards_verify_writable_privileges<'me, 'info>(
    accounts: SweepHolderRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.wsol_token_account,
        accounts.coin,
        accounts.quote_vault,
        accounts.creator_rewards_vault,
        accounts.base_vault,
        accounts.holder_rewards_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sweep_holder_rewards_verify_account_privileges<'me, 'info>(
    accounts: SweepHolderRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sweep_holder_rewards_verify_writable_privileges(accounts)?;
    Ok(())
}
