use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum BonkswapProgramIx {
    CreatePool(CreatePoolIxArgs),
    CreateProvider(CreateProviderIxArgs),
    CreateState(CreateStateIxArgs),
    AddTokens(AddTokensIxArgs),
    WithdrawBuyback,
    Swap(SwapIxArgs),
    WithdrawShares(WithdrawSharesIxArgs),
    WithdrawLpFee,
    WithdrawProjectFee,
    CreateFarm(CreateFarmIxArgs),
    CreateDualFarm(CreateDualFarmIxArgs),
    CreateTripleFarm(CreateTripleFarmIxArgs),
    WithdrawRewards,
    ClosePool,
    WithdrawMercantiFee,
    AddSupply(AddSupplyIxArgs),
    UpdateFees(UpdateFeesIxArgs),
    ResetFarm,
    UpdateRewardTokens,
}
impl BonkswapProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CREATE_POOL_IX_DISCM) {
            let mut reader = &buf[CREATE_POOL_IX_DISCM.len()..];
            let lp_fee = if reader.is_empty() {
                Default::default()
            } else {
                <FixedPoint>::deserialize(&mut reader)?
            };
            let buyback_fee = if reader.is_empty() {
                Default::default()
            } else {
                <FixedPoint>::deserialize(&mut reader)?
            };
            let project_fee = if reader.is_empty() {
                Default::default()
            } else {
                <FixedPoint>::deserialize(&mut reader)?
            };
            let mercanti_fee = if reader.is_empty() {
                Default::default()
            } else {
                <FixedPoint>::deserialize(&mut reader)?
            };
            let initial_token_x = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let initial_token_y = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreatePool(CreatePoolIxArgs {
                    lp_fee,
                    buyback_fee,
                    project_fee,
                    mercanti_fee,
                    initial_token_x,
                    initial_token_y,
                    bump,
                }),
            );
        }
        if buf.starts_with(&CREATE_PROVIDER_IX_DISCM) {
            let mut reader = &buf[CREATE_PROVIDER_IX_DISCM.len()..];
            let token_x_amount = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let token_y_amount = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateProvider(CreateProviderIxArgs {
                    token_x_amount,
                    token_y_amount,
                    bump,
                }),
            );
        }
        if buf.starts_with(&CREATE_STATE_IX_DISCM) {
            let mut reader = &buf[CREATE_STATE_IX_DISCM.len()..];
            let nonce: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::CreateState(CreateStateIxArgs { nonce }));
        }
        if buf.starts_with(&ADD_TOKENS_IX_DISCM) {
            let mut reader = &buf[ADD_TOKENS_IX_DISCM.len()..];
            let delta_x = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let delta_y = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            return Ok(
                Self::AddTokens(AddTokensIxArgs {
                    delta_x,
                    delta_y,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_BUYBACK_IX_DISCM) {
            return Ok(Self::WithdrawBuyback);
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let delta_in = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let price_limit = if reader.is_empty() {
                Default::default()
            } else {
                <FixedPoint>::deserialize(&mut reader)?
            };
            let x_to_y: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    delta_in,
                    price_limit,
                    x_to_y,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_SHARES_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_SHARES_IX_DISCM.len()..];
            let shares = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            return Ok(Self::WithdrawShares(WithdrawSharesIxArgs { shares }));
        }
        if buf.starts_with(&WITHDRAW_LP_FEE_IX_DISCM) {
            return Ok(Self::WithdrawLpFee);
        }
        if buf.starts_with(&WITHDRAW_PROJECT_FEE_IX_DISCM) {
            return Ok(Self::WithdrawProjectFee);
        }
        if buf.starts_with(&CREATE_FARM_IX_DISCM) {
            let mut reader = &buf[CREATE_FARM_IX_DISCM.len()..];
            let supply = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let duration: u64 = crate::borsh_de_or_default(&mut reader)?;
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateFarm(CreateFarmIxArgs {
                    supply,
                    duration,
                    bump,
                }),
            );
        }
        if buf.starts_with(&CREATE_DUAL_FARM_IX_DISCM) {
            let mut reader = &buf[CREATE_DUAL_FARM_IX_DISCM.len()..];
            let supply_marco = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let supply_project_first = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let duration: u64 = crate::borsh_de_or_default(&mut reader)?;
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateDualFarm(CreateDualFarmIxArgs {
                    supply_marco,
                    supply_project_first,
                    duration,
                    bump,
                }),
            );
        }
        if buf.starts_with(&CREATE_TRIPLE_FARM_IX_DISCM) {
            let mut reader = &buf[CREATE_TRIPLE_FARM_IX_DISCM.len()..];
            let supply_marco = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let supply_project_first = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let supply_project_second = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let duration: u64 = crate::borsh_de_or_default(&mut reader)?;
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateTripleFarm(CreateTripleFarmIxArgs {
                    supply_marco,
                    supply_project_first,
                    supply_project_second,
                    duration,
                    bump,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_REWARDS_IX_DISCM) {
            return Ok(Self::WithdrawRewards);
        }
        if buf.starts_with(&CLOSE_POOL_IX_DISCM) {
            return Ok(Self::ClosePool);
        }
        if buf.starts_with(&WITHDRAW_MERCANTI_FEE_IX_DISCM) {
            return Ok(Self::WithdrawMercantiFee);
        }
        if buf.starts_with(&ADD_SUPPLY_IX_DISCM) {
            let mut reader = &buf[ADD_SUPPLY_IX_DISCM.len()..];
            let supply_marco = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let supply_project_first = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let supply_project_second = if reader.is_empty() {
                Default::default()
            } else {
                <Token>::deserialize(&mut reader)?
            };
            let duration: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddSupply(AddSupplyIxArgs {
                    supply_marco,
                    supply_project_first,
                    supply_project_second,
                    duration,
                }),
            );
        }
        if buf.starts_with(&UPDATE_FEES_IX_DISCM) {
            let mut reader = &buf[UPDATE_FEES_IX_DISCM.len()..];
            let new_buyback_fee = if reader.is_empty() {
                Default::default()
            } else {
                <FixedPoint>::deserialize(&mut reader)?
            };
            let new_project_fee = if reader.is_empty() {
                Default::default()
            } else {
                <FixedPoint>::deserialize(&mut reader)?
            };
            let new_provider_fee = if reader.is_empty() {
                Default::default()
            } else {
                <FixedPoint>::deserialize(&mut reader)?
            };
            let new_mercanti_fee = if reader.is_empty() {
                Default::default()
            } else {
                <FixedPoint>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateFees(UpdateFeesIxArgs {
                    new_buyback_fee,
                    new_project_fee,
                    new_provider_fee,
                    new_mercanti_fee,
                }),
            );
        }
        if buf.starts_with(&RESET_FARM_IX_DISCM) {
            return Ok(Self::ResetFarm);
        }
        if buf.starts_with(&UPDATE_REWARD_TOKENS_IX_DISCM) {
            return Ok(Self::UpdateRewardTokens);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreatePool(args) => {
                writer.write_all(&CREATE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lp_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.buyback_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.project_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.mercanti_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.initial_token_x, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.initial_token_y, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                Ok(())
            }
            Self::CreateProvider(args) => {
                writer.write_all(&CREATE_PROVIDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_x_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_y_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                Ok(())
            }
            Self::CreateState(args) => {
                writer.write_all(&CREATE_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.nonce, &mut writer)?;
                Ok(())
            }
            Self::AddTokens(args) => {
                writer.write_all(&ADD_TOKENS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.delta_x, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.delta_y, &mut writer)?;
                Ok(())
            }
            Self::WithdrawBuyback => writer.write_all(&WITHDRAW_BUYBACK_IX_DISCM),
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.delta_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.x_to_y, &mut writer)?;
                Ok(())
            }
            Self::WithdrawShares(args) => {
                writer.write_all(&WITHDRAW_SHARES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                Ok(())
            }
            Self::WithdrawLpFee => writer.write_all(&WITHDRAW_LP_FEE_IX_DISCM),
            Self::WithdrawProjectFee => writer.write_all(&WITHDRAW_PROJECT_FEE_IX_DISCM),
            Self::CreateFarm(args) => {
                writer.write_all(&CREATE_FARM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.supply, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.duration, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                Ok(())
            }
            Self::CreateDualFarm(args) => {
                writer.write_all(&CREATE_DUAL_FARM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.supply_marco, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.supply_project_first,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.duration, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                Ok(())
            }
            Self::CreateTripleFarm(args) => {
                writer.write_all(&CREATE_TRIPLE_FARM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.supply_marco, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.supply_project_first,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.supply_project_second,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.duration, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                Ok(())
            }
            Self::WithdrawRewards => writer.write_all(&WITHDRAW_REWARDS_IX_DISCM),
            Self::ClosePool => writer.write_all(&CLOSE_POOL_IX_DISCM),
            Self::WithdrawMercantiFee => {
                writer.write_all(&WITHDRAW_MERCANTI_FEE_IX_DISCM)
            }
            Self::AddSupply(args) => {
                writer.write_all(&ADD_SUPPLY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.supply_marco, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.supply_project_first,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.supply_project_second,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.duration, &mut writer)?;
                Ok(())
            }
            Self::UpdateFees(args) => {
                writer.write_all(&UPDATE_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_buyback_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_project_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_provider_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_mercanti_fee, &mut writer)?;
                Ok(())
            }
            Self::ResetFarm => writer.write_all(&RESET_FARM_IX_DISCM),
            Self::UpdateRewardTokens => writer.write_all(&UPDATE_REWARD_TOKENS_IX_DISCM),
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
pub const CREATE_POOL_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct CreatePoolAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub pool_x_account: &'me AccountInfo<'info>,
    pub pool_y_account: &'me AccountInfo<'info>,
    pub admin_x_account: &'me AccountInfo<'info>,
    pub admin_y_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub project_owner: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePoolKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub admin_x_account: Pubkey,
    pub admin_y_account: Pubkey,
    pub admin: Pubkey,
    pub project_owner: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreatePoolAccounts<'_, '_>> for CreatePoolKeys {
    fn from(accounts: CreatePoolAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            pool_x_account: *accounts.pool_x_account.key,
            pool_y_account: *accounts.pool_y_account.key,
            admin_x_account: *accounts.admin_x_account.key,
            admin_y_account: *accounts.admin_y_account.key,
            admin: *accounts.admin.key,
            project_owner: *accounts.project_owner.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreatePoolKeys> for [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_x_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_y_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.project_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]> for CreatePoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            token_x: pubkeys[2],
            token_y: pubkeys[3],
            pool_x_account: pubkeys[4],
            pool_y_account: pubkeys[5],
            admin_x_account: pubkeys[6],
            admin_y_account: pubkeys[7],
            admin: pubkeys[8],
            project_owner: pubkeys[9],
            program_authority: pubkeys[10],
            system_program: pubkeys[11],
            token_program: pubkeys[12],
            rent: pubkeys[13],
        }
    }
}
impl<'info> From<CreatePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.pool_x_account.clone(),
            accounts.pool_y_account.clone(),
            accounts.admin_x_account.clone(),
            accounts.admin_y_account.clone(),
            accounts.admin.clone(),
            accounts.project_owner.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]>
for CreatePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            token_x: &arr[2],
            token_y: &arr[3],
            pool_x_account: &arr[4],
            pool_y_account: &arr[5],
            admin_x_account: &arr[6],
            admin_y_account: &arr[7],
            admin: &arr[8],
            project_owner: &arr[9],
            program_authority: &arr[10],
            system_program: &arr[11],
            token_program: &arr[12],
            rent: &arr[13],
        }
    }
}
pub const CREATE_POOL_IX_DISCM: [u8; 8usize] = [233, 146, 209, 142, 207, 104, 64, 188];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePoolIxArgs {
    pub lp_fee: FixedPoint,
    pub buyback_fee: FixedPoint,
    pub project_fee: FixedPoint,
    pub mercanti_fee: FixedPoint,
    pub initial_token_x: Token,
    pub initial_token_y: Token,
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePoolIxData(pub CreatePoolIxArgs);
impl From<CreatePoolIxArgs> for CreatePoolIxData {
    fn from(args: CreatePoolIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let lp_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let buyback_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let project_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let mercanti_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let initial_token_x = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let initial_token_y = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreatePoolIxArgs {
                lp_fee,
                buyback_fee,
                project_fee,
                mercanti_fee,
                initial_token_x,
                initial_token_y,
                bump,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lp_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.buyback_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.project_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.mercanti_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.initial_token_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.initial_token_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePoolKeys,
    args: CreatePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_pool_ix(
    keys: CreatePoolKeys,
    args: CreatePoolIxArgs,
) -> std::io::Result<Instruction> {
    create_pool_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys, args)
}
pub fn create_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_pool_invoke(
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
) -> ProgramResult {
    create_pool_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args)
}
pub fn create_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_pool_invoke_signed(
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_pool_invoke_signed_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_pool_verify_account_keys(
    accounts: CreatePoolAccounts<'_, '_>,
    keys: CreatePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.pool_x_account.key, keys.pool_x_account),
        (*accounts.pool_y_account.key, keys.pool_y_account),
        (*accounts.admin_x_account.key, keys.admin_x_account),
        (*accounts.admin_y_account.key, keys.admin_y_account),
        (*accounts.admin.key, keys.admin),
        (*accounts.project_owner.key, keys.project_owner),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_pool_verify_writable_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.pool_x_account,
        accounts.pool_y_account,
        accounts.admin_x_account,
        accounts.admin_y_account,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_pool_verify_signer_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.pool_x_account,
        accounts.pool_y_account,
        accounts.admin,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_pool_verify_account_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_pool_verify_writable_privileges(accounts)?;
    create_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PROVIDER_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct CreateProviderAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub farm: &'me AccountInfo<'info>,
    pub provider: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub pool_x_account: &'me AccountInfo<'info>,
    pub pool_y_account: &'me AccountInfo<'info>,
    pub owner_x_account: &'me AccountInfo<'info>,
    pub owner_y_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateProviderKeys {
    pub pool: Pubkey,
    pub farm: Pubkey,
    pub provider: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub owner_x_account: Pubkey,
    pub owner_y_account: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateProviderAccounts<'_, '_>> for CreateProviderKeys {
    fn from(accounts: CreateProviderAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            farm: *accounts.farm.key,
            provider: *accounts.provider.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            pool_x_account: *accounts.pool_x_account.key,
            pool_y_account: *accounts.pool_y_account.key,
            owner_x_account: *accounts.owner_x_account.key,
            owner_y_account: *accounts.owner_y_account.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateProviderKeys> for [AccountMeta; CREATE_PROVIDER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateProviderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_PROVIDER_IX_ACCOUNTS_LEN]> for CreateProviderKeys {
    fn from(pubkeys: [Pubkey; CREATE_PROVIDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            farm: pubkeys[1],
            provider: pubkeys[2],
            token_x: pubkeys[3],
            token_y: pubkeys[4],
            pool_x_account: pubkeys[5],
            pool_y_account: pubkeys[6],
            owner_x_account: pubkeys[7],
            owner_y_account: pubkeys[8],
            owner: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            rent: pubkeys[12],
        }
    }
}
impl<'info> From<CreateProviderAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PROVIDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateProviderAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.farm.clone(),
            accounts.provider.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.pool_x_account.clone(),
            accounts.pool_y_account.clone(),
            accounts.owner_x_account.clone(),
            accounts.owner_y_account.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_PROVIDER_IX_ACCOUNTS_LEN]>
for CreateProviderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_PROVIDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            farm: &arr[1],
            provider: &arr[2],
            token_x: &arr[3],
            token_y: &arr[4],
            pool_x_account: &arr[5],
            pool_y_account: &arr[6],
            owner_x_account: &arr[7],
            owner_y_account: &arr[8],
            owner: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            rent: &arr[12],
        }
    }
}
pub const CREATE_PROVIDER_IX_DISCM: [u8; 8usize] = [74, 53, 211, 174, 38, 168, 227, 177];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateProviderIxArgs {
    pub token_x_amount: Token,
    pub token_y_amount: Token,
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateProviderIxData(pub CreateProviderIxArgs);
impl From<CreateProviderIxArgs> for CreateProviderIxData {
    fn from(args: CreateProviderIxArgs) -> Self {
        Self(args)
    }
}
impl CreateProviderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PROVIDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_x_amount = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let token_y_amount = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateProviderIxArgs {
                token_x_amount,
                token_y_amount,
                bump,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PROVIDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_x_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_y_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_provider_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateProviderKeys,
    args: CreateProviderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PROVIDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateProviderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_provider_ix(
    keys: CreateProviderKeys,
    args: CreateProviderIxArgs,
) -> std::io::Result<Instruction> {
    create_provider_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys, args)
}
pub fn create_provider_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateProviderAccounts<'_, '_>,
    args: CreateProviderIxArgs,
) -> ProgramResult {
    let keys: CreateProviderKeys = accounts.into();
    let ix = create_provider_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_provider_invoke(
    accounts: CreateProviderAccounts<'_, '_>,
    args: CreateProviderIxArgs,
) -> ProgramResult {
    create_provider_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args)
}
pub fn create_provider_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateProviderAccounts<'_, '_>,
    args: CreateProviderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateProviderKeys = accounts.into();
    let ix = create_provider_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_provider_invoke_signed(
    accounts: CreateProviderAccounts<'_, '_>,
    args: CreateProviderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_provider_invoke_signed_with_program_id(
        BONKSWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_provider_verify_account_keys(
    accounts: CreateProviderAccounts<'_, '_>,
    keys: CreateProviderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.farm.key, keys.farm),
        (*accounts.provider.key, keys.provider),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.pool_x_account.key, keys.pool_x_account),
        (*accounts.pool_y_account.key, keys.pool_y_account),
        (*accounts.owner_x_account.key, keys.owner_x_account),
        (*accounts.owner_y_account.key, keys.owner_y_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_provider_verify_writable_privileges<'me, 'info>(
    accounts: CreateProviderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.farm,
        accounts.provider,
        accounts.pool_x_account,
        accounts.pool_y_account,
        accounts.owner_x_account,
        accounts.owner_y_account,
        accounts.owner,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_provider_verify_signer_privileges<'me, 'info>(
    accounts: CreateProviderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_provider_verify_account_privileges<'me, 'info>(
    accounts: CreateProviderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_provider_verify_writable_privileges(accounts)?;
    create_provider_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_STATE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CreateStateAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateStateKeys {
    pub state: Pubkey,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateStateAccounts<'_, '_>> for CreateStateKeys {
    fn from(accounts: CreateStateAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            admin: *accounts.admin.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateStateKeys> for [AccountMeta; CREATE_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
impl From<[Pubkey; CREATE_STATE_IX_ACCOUNTS_LEN]> for CreateStateKeys {
    fn from(pubkeys: [Pubkey; CREATE_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            admin: pubkeys[1],
            program_authority: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CreateStateAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateStateAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.admin.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_STATE_IX_ACCOUNTS_LEN]>
for CreateStateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            admin: &arr[1],
            program_authority: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CREATE_STATE_IX_DISCM: [u8; 8usize] = [214, 211, 209, 79, 107, 105, 247, 222];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateStateIxArgs {
    pub nonce: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateStateIxData(pub CreateStateIxArgs);
impl From<CreateStateIxArgs> for CreateStateIxData {
    fn from(args: CreateStateIxArgs) -> Self {
        Self(args)
    }
}
impl CreateStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let nonce: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(CreateStateIxArgs { nonce }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.nonce, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_state_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateStateKeys,
    args: CreateStateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_STATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateStateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_state_ix(
    keys: CreateStateKeys,
    args: CreateStateIxArgs,
) -> std::io::Result<Instruction> {
    create_state_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys, args)
}
pub fn create_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateStateAccounts<'_, '_>,
    args: CreateStateIxArgs,
) -> ProgramResult {
    let keys: CreateStateKeys = accounts.into();
    let ix = create_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_state_invoke(
    accounts: CreateStateAccounts<'_, '_>,
    args: CreateStateIxArgs,
) -> ProgramResult {
    create_state_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args)
}
pub fn create_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateStateAccounts<'_, '_>,
    args: CreateStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateStateKeys = accounts.into();
    let ix = create_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_state_invoke_signed(
    accounts: CreateStateAccounts<'_, '_>,
    args: CreateStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_state_invoke_signed_with_program_id(
        BONKSWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_state_verify_account_keys(
    accounts: CreateStateAccounts<'_, '_>,
    keys: CreateStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.admin.key, keys.admin),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_state_verify_writable_privileges<'me, 'info>(
    accounts: CreateStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_state_verify_signer_privileges<'me, 'info>(
    accounts: CreateStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_state_verify_account_privileges<'me, 'info>(
    accounts: CreateStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_state_verify_writable_privileges(accounts)?;
    create_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_TOKENS_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct AddTokensAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub farm: &'me AccountInfo<'info>,
    pub provider: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub token_marco: &'me AccountInfo<'info>,
    pub token_project_first: &'me AccountInfo<'info>,
    pub token_project_second: &'me AccountInfo<'info>,
    pub owner_x_account: &'me AccountInfo<'info>,
    pub owner_y_account: &'me AccountInfo<'info>,
    pub pool_x_account: &'me AccountInfo<'info>,
    pub pool_y_account: &'me AccountInfo<'info>,
    pub owner_marco_account: &'me AccountInfo<'info>,
    pub owner_project_first_account: &'me AccountInfo<'info>,
    pub owner_project_second_account: &'me AccountInfo<'info>,
    pub token_marco_account: &'me AccountInfo<'info>,
    pub token_project_first_account: &'me AccountInfo<'info>,
    pub token_project_second_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddTokensKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub farm: Pubkey,
    pub provider: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub token_marco: Pubkey,
    pub token_project_first: Pubkey,
    pub token_project_second: Pubkey,
    pub owner_x_account: Pubkey,
    pub owner_y_account: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub owner_marco_account: Pubkey,
    pub owner_project_first_account: Pubkey,
    pub owner_project_second_account: Pubkey,
    pub token_marco_account: Pubkey,
    pub token_project_first_account: Pubkey,
    pub token_project_second_account: Pubkey,
    pub owner: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<AddTokensAccounts<'_, '_>> for AddTokensKeys {
    fn from(accounts: AddTokensAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            farm: *accounts.farm.key,
            provider: *accounts.provider.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            token_marco: *accounts.token_marco.key,
            token_project_first: *accounts.token_project_first.key,
            token_project_second: *accounts.token_project_second.key,
            owner_x_account: *accounts.owner_x_account.key,
            owner_y_account: *accounts.owner_y_account.key,
            pool_x_account: *accounts.pool_x_account.key,
            pool_y_account: *accounts.pool_y_account.key,
            owner_marco_account: *accounts.owner_marco_account.key,
            owner_project_first_account: *accounts.owner_project_first_account.key,
            owner_project_second_account: *accounts.owner_project_second_account.key,
            token_marco_account: *accounts.token_marco_account.key,
            token_project_first_account: *accounts.token_project_first_account.key,
            token_project_second_account: *accounts.token_project_second_account.key,
            owner: *accounts.owner.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<AddTokensKeys> for [AccountMeta; ADD_TOKENS_IX_ACCOUNTS_LEN] {
    fn from(keys: AddTokensKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_first,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_second,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
impl From<[Pubkey; ADD_TOKENS_IX_ACCOUNTS_LEN]> for AddTokensKeys {
    fn from(pubkeys: [Pubkey; ADD_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            farm: pubkeys[2],
            provider: pubkeys[3],
            token_x: pubkeys[4],
            token_y: pubkeys[5],
            token_marco: pubkeys[6],
            token_project_first: pubkeys[7],
            token_project_second: pubkeys[8],
            owner_x_account: pubkeys[9],
            owner_y_account: pubkeys[10],
            pool_x_account: pubkeys[11],
            pool_y_account: pubkeys[12],
            owner_marco_account: pubkeys[13],
            owner_project_first_account: pubkeys[14],
            owner_project_second_account: pubkeys[15],
            token_marco_account: pubkeys[16],
            token_project_first_account: pubkeys[17],
            token_project_second_account: pubkeys[18],
            owner: pubkeys[19],
            program_authority: pubkeys[20],
            system_program: pubkeys[21],
            token_program: pubkeys[22],
            associated_token_program: pubkeys[23],
            rent: pubkeys[24],
        }
    }
}
impl<'info> From<AddTokensAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_TOKENS_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddTokensAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.farm.clone(),
            accounts.provider.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.token_marco.clone(),
            accounts.token_project_first.clone(),
            accounts.token_project_second.clone(),
            accounts.owner_x_account.clone(),
            accounts.owner_y_account.clone(),
            accounts.pool_x_account.clone(),
            accounts.pool_y_account.clone(),
            accounts.owner_marco_account.clone(),
            accounts.owner_project_first_account.clone(),
            accounts.owner_project_second_account.clone(),
            accounts.token_marco_account.clone(),
            accounts.token_project_first_account.clone(),
            accounts.token_project_second_account.clone(),
            accounts.owner.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_TOKENS_IX_ACCOUNTS_LEN]>
for AddTokensAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            farm: &arr[2],
            provider: &arr[3],
            token_x: &arr[4],
            token_y: &arr[5],
            token_marco: &arr[6],
            token_project_first: &arr[7],
            token_project_second: &arr[8],
            owner_x_account: &arr[9],
            owner_y_account: &arr[10],
            pool_x_account: &arr[11],
            pool_y_account: &arr[12],
            owner_marco_account: &arr[13],
            owner_project_first_account: &arr[14],
            owner_project_second_account: &arr[15],
            token_marco_account: &arr[16],
            token_project_first_account: &arr[17],
            token_project_second_account: &arr[18],
            owner: &arr[19],
            program_authority: &arr[20],
            system_program: &arr[21],
            token_program: &arr[22],
            associated_token_program: &arr[23],
            rent: &arr[24],
        }
    }
}
pub const ADD_TOKENS_IX_DISCM: [u8; 8usize] = [28, 218, 30, 209, 175, 155, 153, 240];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddTokensIxArgs {
    pub delta_x: Token,
    pub delta_y: Token,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddTokensIxData(pub AddTokensIxArgs);
impl From<AddTokensIxArgs> for AddTokensIxData {
    fn from(args: AddTokensIxArgs) -> Self {
        Self(args)
    }
}
impl AddTokensIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_TOKENS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let delta_x = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let delta_y = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        Ok(
            Self(AddTokensIxArgs {
                delta_x,
                delta_y,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_TOKENS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.delta_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.delta_y, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_tokens_ix_with_program_id(
    program_id: Pubkey,
    keys: AddTokensKeys,
    args: AddTokensIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_TOKENS_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddTokensIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_tokens_ix(
    keys: AddTokensKeys,
    args: AddTokensIxArgs,
) -> std::io::Result<Instruction> {
    add_tokens_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys, args)
}
pub fn add_tokens_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddTokensAccounts<'_, '_>,
    args: AddTokensIxArgs,
) -> ProgramResult {
    let keys: AddTokensKeys = accounts.into();
    let ix = add_tokens_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_tokens_invoke(
    accounts: AddTokensAccounts<'_, '_>,
    args: AddTokensIxArgs,
) -> ProgramResult {
    add_tokens_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args)
}
pub fn add_tokens_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddTokensAccounts<'_, '_>,
    args: AddTokensIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddTokensKeys = accounts.into();
    let ix = add_tokens_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_tokens_invoke_signed(
    accounts: AddTokensAccounts<'_, '_>,
    args: AddTokensIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_tokens_invoke_signed_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_tokens_verify_account_keys(
    accounts: AddTokensAccounts<'_, '_>,
    keys: AddTokensKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.farm.key, keys.farm),
        (*accounts.provider.key, keys.provider),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.token_marco.key, keys.token_marco),
        (*accounts.token_project_first.key, keys.token_project_first),
        (*accounts.token_project_second.key, keys.token_project_second),
        (*accounts.owner_x_account.key, keys.owner_x_account),
        (*accounts.owner_y_account.key, keys.owner_y_account),
        (*accounts.pool_x_account.key, keys.pool_x_account),
        (*accounts.pool_y_account.key, keys.pool_y_account),
        (*accounts.owner_marco_account.key, keys.owner_marco_account),
        (*accounts.owner_project_first_account.key, keys.owner_project_first_account),
        (*accounts.owner_project_second_account.key, keys.owner_project_second_account),
        (*accounts.token_marco_account.key, keys.token_marco_account),
        (*accounts.token_project_first_account.key, keys.token_project_first_account),
        (*accounts.token_project_second_account.key, keys.token_project_second_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.program_authority.key, keys.program_authority),
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
pub fn add_tokens_verify_writable_privileges<'me, 'info>(
    accounts: AddTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.farm,
        accounts.provider,
        accounts.token_marco,
        accounts.token_project_first,
        accounts.token_project_second,
        accounts.owner_x_account,
        accounts.owner_y_account,
        accounts.pool_x_account,
        accounts.pool_y_account,
        accounts.owner_marco_account,
        accounts.owner_project_first_account,
        accounts.owner_project_second_account,
        accounts.token_marco_account,
        accounts.token_project_first_account,
        accounts.token_project_second_account,
        accounts.owner,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_tokens_verify_signer_privileges<'me, 'info>(
    accounts: AddTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_tokens_verify_account_privileges<'me, 'info>(
    accounts: AddTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_tokens_verify_writable_privileges(accounts)?;
    add_tokens_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_BUYBACK_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawBuybackAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub buyback_x_account: &'me AccountInfo<'info>,
    pub buyback_y_account: &'me AccountInfo<'info>,
    pub pool_x_account: &'me AccountInfo<'info>,
    pub pool_y_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawBuybackKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub buyback_x_account: Pubkey,
    pub buyback_y_account: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<WithdrawBuybackAccounts<'_, '_>> for WithdrawBuybackKeys {
    fn from(accounts: WithdrawBuybackAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            buyback_x_account: *accounts.buyback_x_account.key,
            buyback_y_account: *accounts.buyback_y_account.key,
            pool_x_account: *accounts.pool_x_account.key,
            pool_y_account: *accounts.pool_y_account.key,
            admin: *accounts.admin.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<WithdrawBuybackKeys> for [AccountMeta; WITHDRAW_BUYBACK_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawBuybackKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buyback_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyback_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
impl From<[Pubkey; WITHDRAW_BUYBACK_IX_ACCOUNTS_LEN]> for WithdrawBuybackKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_BUYBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            token_x: pubkeys[2],
            token_y: pubkeys[3],
            buyback_x_account: pubkeys[4],
            buyback_y_account: pubkeys[5],
            pool_x_account: pubkeys[6],
            pool_y_account: pubkeys[7],
            admin: pubkeys[8],
            program_authority: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            rent: pubkeys[13],
        }
    }
}
impl<'info> From<WithdrawBuybackAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_BUYBACK_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawBuybackAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.buyback_x_account.clone(),
            accounts.buyback_y_account.clone(),
            accounts.pool_x_account.clone(),
            accounts.pool_y_account.clone(),
            accounts.admin.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_BUYBACK_IX_ACCOUNTS_LEN]>
for WithdrawBuybackAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_BUYBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            token_x: &arr[2],
            token_y: &arr[3],
            buyback_x_account: &arr[4],
            buyback_y_account: &arr[5],
            pool_x_account: &arr[6],
            pool_y_account: &arr[7],
            admin: &arr[8],
            program_authority: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
            rent: &arr[13],
        }
    }
}
pub const WITHDRAW_BUYBACK_IX_DISCM: [u8; 8usize] = [188, 75, 30, 198, 99, 43, 12, 54];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawBuybackIxData;
impl WithdrawBuybackIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_BUYBACK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_BUYBACK_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_buyback_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawBuybackKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_BUYBACK_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawBuybackIxData.try_to_vec()?,
    })
}
pub fn withdraw_buyback_ix(keys: WithdrawBuybackKeys) -> std::io::Result<Instruction> {
    withdraw_buyback_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys)
}
pub fn withdraw_buyback_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawBuybackAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawBuybackKeys = accounts.into();
    let ix = withdraw_buyback_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_buyback_invoke(
    accounts: WithdrawBuybackAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_buyback_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts)
}
pub fn withdraw_buyback_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawBuybackAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawBuybackKeys = accounts.into();
    let ix = withdraw_buyback_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_buyback_invoke_signed(
    accounts: WithdrawBuybackAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_buyback_invoke_signed_with_program_id(BONKSWAP_PROGRAM_ID, accounts, seeds)
}
pub fn withdraw_buyback_verify_account_keys(
    accounts: WithdrawBuybackAccounts<'_, '_>,
    keys: WithdrawBuybackKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.buyback_x_account.key, keys.buyback_x_account),
        (*accounts.buyback_y_account.key, keys.buyback_y_account),
        (*accounts.pool_x_account.key, keys.pool_x_account),
        (*accounts.pool_y_account.key, keys.pool_y_account),
        (*accounts.admin.key, keys.admin),
        (*accounts.program_authority.key, keys.program_authority),
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
pub fn withdraw_buyback_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawBuybackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.buyback_x_account,
        accounts.buyback_y_account,
        accounts.pool_x_account,
        accounts.pool_y_account,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_buyback_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawBuybackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_buyback_verify_account_privileges<'me, 'info>(
    accounts: WithdrawBuybackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_buyback_verify_writable_privileges(accounts)?;
    withdraw_buyback_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub pool_x_account: &'me AccountInfo<'info>,
    pub pool_y_account: &'me AccountInfo<'info>,
    pub swapper_x_account: &'me AccountInfo<'info>,
    pub swapper_y_account: &'me AccountInfo<'info>,
    pub swapper: &'me AccountInfo<'info>,
    pub referrer_x_account: &'me AccountInfo<'info>,
    pub referrer_y_account: &'me AccountInfo<'info>,
    pub referrer: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub swapper_x_account: Pubkey,
    pub swapper_y_account: Pubkey,
    pub swapper: Pubkey,
    pub referrer_x_account: Pubkey,
    pub referrer_y_account: Pubkey,
    pub referrer: Pubkey,
    pub program_authority: Pubkey,
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
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            pool_x_account: *accounts.pool_x_account.key,
            pool_y_account: *accounts.pool_y_account.key,
            swapper_x_account: *accounts.swapper_x_account.key,
            swapper_y_account: *accounts.swapper_y_account.key,
            swapper: *accounts.swapper.key,
            referrer_x_account: *accounts.referrer_x_account.key,
            referrer_y_account: *accounts.referrer_y_account.key,
            referrer: *accounts.referrer.key,
            program_authority: *accounts.program_authority.key,
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
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swapper_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swapper_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swapper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
            token_x: pubkeys[2],
            token_y: pubkeys[3],
            pool_x_account: pubkeys[4],
            pool_y_account: pubkeys[5],
            swapper_x_account: pubkeys[6],
            swapper_y_account: pubkeys[7],
            swapper: pubkeys[8],
            referrer_x_account: pubkeys[9],
            referrer_y_account: pubkeys[10],
            referrer: pubkeys[11],
            program_authority: pubkeys[12],
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
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.pool_x_account.clone(),
            accounts.pool_y_account.clone(),
            accounts.swapper_x_account.clone(),
            accounts.swapper_y_account.clone(),
            accounts.swapper.clone(),
            accounts.referrer_x_account.clone(),
            accounts.referrer_y_account.clone(),
            accounts.referrer.clone(),
            accounts.program_authority.clone(),
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
            token_x: &arr[2],
            token_y: &arr[3],
            pool_x_account: &arr[4],
            pool_y_account: &arr[5],
            swapper_x_account: &arr[6],
            swapper_y_account: &arr[7],
            swapper: &arr[8],
            referrer_x_account: &arr[9],
            referrer_y_account: &arr[10],
            referrer: &arr[11],
            program_authority: &arr[12],
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
    pub delta_in: Token,
    pub price_limit: FixedPoint,
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
        let delta_in = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let price_limit = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let x_to_y: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                delta_in,
                price_limit,
                x_to_y,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.delta_in, &mut writer)?;
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
    swap_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.pool_x_account.key, keys.pool_x_account),
        (*accounts.pool_y_account.key, keys.pool_y_account),
        (*accounts.swapper_x_account.key, keys.swapper_x_account),
        (*accounts.swapper_y_account.key, keys.swapper_y_account),
        (*accounts.swapper.key, keys.swapper),
        (*accounts.referrer_x_account.key, keys.referrer_x_account),
        (*accounts.referrer_y_account.key, keys.referrer_y_account),
        (*accounts.referrer.key, keys.referrer),
        (*accounts.program_authority.key, keys.program_authority),
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
pub fn swap_verify_writable_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.pool_x_account,
        accounts.pool_y_account,
        accounts.swapper_x_account,
        accounts.swapper_y_account,
        accounts.swapper,
        accounts.referrer_x_account,
        accounts.referrer_y_account,
        accounts.referrer,
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
    for should_be_signer in [accounts.swapper] {
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
pub const WITHDRAW_SHARES_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawSharesAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub farm: &'me AccountInfo<'info>,
    pub provider: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub token_marco: &'me AccountInfo<'info>,
    pub token_project_first: &'me AccountInfo<'info>,
    pub token_project_second: &'me AccountInfo<'info>,
    pub pool_x_account: &'me AccountInfo<'info>,
    pub pool_y_account: &'me AccountInfo<'info>,
    pub token_marco_account: &'me AccountInfo<'info>,
    pub token_project_first_account: &'me AccountInfo<'info>,
    pub token_project_second_account: &'me AccountInfo<'info>,
    pub owner_x_account: &'me AccountInfo<'info>,
    pub owner_y_account: &'me AccountInfo<'info>,
    pub owner_marco_account: &'me AccountInfo<'info>,
    pub owner_project_first_account: &'me AccountInfo<'info>,
    pub owner_project_second_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawSharesKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub farm: Pubkey,
    pub provider: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub token_marco: Pubkey,
    pub token_project_first: Pubkey,
    pub token_project_second: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub token_marco_account: Pubkey,
    pub token_project_first_account: Pubkey,
    pub token_project_second_account: Pubkey,
    pub owner_x_account: Pubkey,
    pub owner_y_account: Pubkey,
    pub owner_marco_account: Pubkey,
    pub owner_project_first_account: Pubkey,
    pub owner_project_second_account: Pubkey,
    pub owner: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<WithdrawSharesAccounts<'_, '_>> for WithdrawSharesKeys {
    fn from(accounts: WithdrawSharesAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            farm: *accounts.farm.key,
            provider: *accounts.provider.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            token_marco: *accounts.token_marco.key,
            token_project_first: *accounts.token_project_first.key,
            token_project_second: *accounts.token_project_second.key,
            pool_x_account: *accounts.pool_x_account.key,
            pool_y_account: *accounts.pool_y_account.key,
            token_marco_account: *accounts.token_marco_account.key,
            token_project_first_account: *accounts.token_project_first_account.key,
            token_project_second_account: *accounts.token_project_second_account.key,
            owner_x_account: *accounts.owner_x_account.key,
            owner_y_account: *accounts.owner_y_account.key,
            owner_marco_account: *accounts.owner_marco_account.key,
            owner_project_first_account: *accounts.owner_project_first_account.key,
            owner_project_second_account: *accounts.owner_project_second_account.key,
            owner: *accounts.owner.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<WithdrawSharesKeys> for [AccountMeta; WITHDRAW_SHARES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawSharesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_first,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_second,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
impl From<[Pubkey; WITHDRAW_SHARES_IX_ACCOUNTS_LEN]> for WithdrawSharesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_SHARES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            farm: pubkeys[2],
            provider: pubkeys[3],
            token_x: pubkeys[4],
            token_y: pubkeys[5],
            token_marco: pubkeys[6],
            token_project_first: pubkeys[7],
            token_project_second: pubkeys[8],
            pool_x_account: pubkeys[9],
            pool_y_account: pubkeys[10],
            token_marco_account: pubkeys[11],
            token_project_first_account: pubkeys[12],
            token_project_second_account: pubkeys[13],
            owner_x_account: pubkeys[14],
            owner_y_account: pubkeys[15],
            owner_marco_account: pubkeys[16],
            owner_project_first_account: pubkeys[17],
            owner_project_second_account: pubkeys[18],
            owner: pubkeys[19],
            program_authority: pubkeys[20],
            system_program: pubkeys[21],
            token_program: pubkeys[22],
            associated_token_program: pubkeys[23],
            rent: pubkeys[24],
        }
    }
}
impl<'info> From<WithdrawSharesAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_SHARES_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawSharesAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.farm.clone(),
            accounts.provider.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.token_marco.clone(),
            accounts.token_project_first.clone(),
            accounts.token_project_second.clone(),
            accounts.pool_x_account.clone(),
            accounts.pool_y_account.clone(),
            accounts.token_marco_account.clone(),
            accounts.token_project_first_account.clone(),
            accounts.token_project_second_account.clone(),
            accounts.owner_x_account.clone(),
            accounts.owner_y_account.clone(),
            accounts.owner_marco_account.clone(),
            accounts.owner_project_first_account.clone(),
            accounts.owner_project_second_account.clone(),
            accounts.owner.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_SHARES_IX_ACCOUNTS_LEN]>
for WithdrawSharesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_SHARES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            farm: &arr[2],
            provider: &arr[3],
            token_x: &arr[4],
            token_y: &arr[5],
            token_marco: &arr[6],
            token_project_first: &arr[7],
            token_project_second: &arr[8],
            pool_x_account: &arr[9],
            pool_y_account: &arr[10],
            token_marco_account: &arr[11],
            token_project_first_account: &arr[12],
            token_project_second_account: &arr[13],
            owner_x_account: &arr[14],
            owner_y_account: &arr[15],
            owner_marco_account: &arr[16],
            owner_project_first_account: &arr[17],
            owner_project_second_account: &arr[18],
            owner: &arr[19],
            program_authority: &arr[20],
            system_program: &arr[21],
            token_program: &arr[22],
            associated_token_program: &arr[23],
            rent: &arr[24],
        }
    }
}
pub const WITHDRAW_SHARES_IX_DISCM: [u8; 8usize] = [
    176, 104, 154, 105, 250, 80, 68, 244,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawSharesIxArgs {
    pub shares: Token,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawSharesIxData(pub WithdrawSharesIxArgs);
impl From<WithdrawSharesIxArgs> for WithdrawSharesIxData {
    fn from(args: WithdrawSharesIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawSharesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_SHARES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        Ok(Self(WithdrawSharesIxArgs { shares }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_SHARES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_shares_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawSharesKeys,
    args: WithdrawSharesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_SHARES_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawSharesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_shares_ix(
    keys: WithdrawSharesKeys,
    args: WithdrawSharesIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_shares_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys, args)
}
pub fn withdraw_shares_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawSharesAccounts<'_, '_>,
    args: WithdrawSharesIxArgs,
) -> ProgramResult {
    let keys: WithdrawSharesKeys = accounts.into();
    let ix = withdraw_shares_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_shares_invoke(
    accounts: WithdrawSharesAccounts<'_, '_>,
    args: WithdrawSharesIxArgs,
) -> ProgramResult {
    withdraw_shares_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args)
}
pub fn withdraw_shares_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawSharesAccounts<'_, '_>,
    args: WithdrawSharesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawSharesKeys = accounts.into();
    let ix = withdraw_shares_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_shares_invoke_signed(
    accounts: WithdrawSharesAccounts<'_, '_>,
    args: WithdrawSharesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_shares_invoke_signed_with_program_id(
        BONKSWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_shares_verify_account_keys(
    accounts: WithdrawSharesAccounts<'_, '_>,
    keys: WithdrawSharesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.farm.key, keys.farm),
        (*accounts.provider.key, keys.provider),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.token_marco.key, keys.token_marco),
        (*accounts.token_project_first.key, keys.token_project_first),
        (*accounts.token_project_second.key, keys.token_project_second),
        (*accounts.pool_x_account.key, keys.pool_x_account),
        (*accounts.pool_y_account.key, keys.pool_y_account),
        (*accounts.token_marco_account.key, keys.token_marco_account),
        (*accounts.token_project_first_account.key, keys.token_project_first_account),
        (*accounts.token_project_second_account.key, keys.token_project_second_account),
        (*accounts.owner_x_account.key, keys.owner_x_account),
        (*accounts.owner_y_account.key, keys.owner_y_account),
        (*accounts.owner_marco_account.key, keys.owner_marco_account),
        (*accounts.owner_project_first_account.key, keys.owner_project_first_account),
        (*accounts.owner_project_second_account.key, keys.owner_project_second_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.program_authority.key, keys.program_authority),
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
pub fn withdraw_shares_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.farm,
        accounts.provider,
        accounts.token_marco,
        accounts.token_project_first,
        accounts.token_project_second,
        accounts.pool_x_account,
        accounts.pool_y_account,
        accounts.token_marco_account,
        accounts.token_project_first_account,
        accounts.token_project_second_account,
        accounts.owner_x_account,
        accounts.owner_y_account,
        accounts.owner_marco_account,
        accounts.owner_project_first_account,
        accounts.owner_project_second_account,
        accounts.owner,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_shares_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_shares_verify_account_privileges<'me, 'info>(
    accounts: WithdrawSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_shares_verify_writable_privileges(accounts)?;
    withdraw_shares_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_LP_FEE_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawLpFeeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub provider: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub owner_x_account: &'me AccountInfo<'info>,
    pub owner_y_account: &'me AccountInfo<'info>,
    pub pool_x_account: &'me AccountInfo<'info>,
    pub pool_y_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawLpFeeKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub provider: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub owner_x_account: Pubkey,
    pub owner_y_account: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub owner: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<WithdrawLpFeeAccounts<'_, '_>> for WithdrawLpFeeKeys {
    fn from(accounts: WithdrawLpFeeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            provider: *accounts.provider.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            owner_x_account: *accounts.owner_x_account.key,
            owner_y_account: *accounts.owner_y_account.key,
            pool_x_account: *accounts.pool_x_account.key,
            pool_y_account: *accounts.pool_y_account.key,
            owner: *accounts.owner.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<WithdrawLpFeeKeys> for [AccountMeta; WITHDRAW_LP_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawLpFeeKeys) -> Self {
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
                pubkey: keys.provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
impl From<[Pubkey; WITHDRAW_LP_FEE_IX_ACCOUNTS_LEN]> for WithdrawLpFeeKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_LP_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            provider: pubkeys[2],
            token_x: pubkeys[3],
            token_y: pubkeys[4],
            owner_x_account: pubkeys[5],
            owner_y_account: pubkeys[6],
            pool_x_account: pubkeys[7],
            pool_y_account: pubkeys[8],
            owner: pubkeys[9],
            program_authority: pubkeys[10],
            system_program: pubkeys[11],
            token_program: pubkeys[12],
            associated_token_program: pubkeys[13],
            rent: pubkeys[14],
        }
    }
}
impl<'info> From<WithdrawLpFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_LP_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawLpFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.provider.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.owner_x_account.clone(),
            accounts.owner_y_account.clone(),
            accounts.pool_x_account.clone(),
            accounts.pool_y_account.clone(),
            accounts.owner.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_LP_FEE_IX_ACCOUNTS_LEN]>
for WithdrawLpFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_LP_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            provider: &arr[2],
            token_x: &arr[3],
            token_y: &arr[4],
            owner_x_account: &arr[5],
            owner_y_account: &arr[6],
            pool_x_account: &arr[7],
            pool_y_account: &arr[8],
            owner: &arr[9],
            program_authority: &arr[10],
            system_program: &arr[11],
            token_program: &arr[12],
            associated_token_program: &arr[13],
            rent: &arr[14],
        }
    }
}
pub const WITHDRAW_LP_FEE_IX_DISCM: [u8; 8usize] = [149, 161, 2, 213, 195, 147, 42, 65];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawLpFeeIxData;
impl WithdrawLpFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_LP_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_LP_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_lp_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawLpFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_LP_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawLpFeeIxData.try_to_vec()?,
    })
}
pub fn withdraw_lp_fee_ix(keys: WithdrawLpFeeKeys) -> std::io::Result<Instruction> {
    withdraw_lp_fee_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys)
}
pub fn withdraw_lp_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawLpFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawLpFeeKeys = accounts.into();
    let ix = withdraw_lp_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_lp_fee_invoke(accounts: WithdrawLpFeeAccounts<'_, '_>) -> ProgramResult {
    withdraw_lp_fee_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts)
}
pub fn withdraw_lp_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawLpFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawLpFeeKeys = accounts.into();
    let ix = withdraw_lp_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_lp_fee_invoke_signed(
    accounts: WithdrawLpFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_lp_fee_invoke_signed_with_program_id(BONKSWAP_PROGRAM_ID, accounts, seeds)
}
pub fn withdraw_lp_fee_verify_account_keys(
    accounts: WithdrawLpFeeAccounts<'_, '_>,
    keys: WithdrawLpFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.provider.key, keys.provider),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.owner_x_account.key, keys.owner_x_account),
        (*accounts.owner_y_account.key, keys.owner_y_account),
        (*accounts.pool_x_account.key, keys.pool_x_account),
        (*accounts.pool_y_account.key, keys.pool_y_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.program_authority.key, keys.program_authority),
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
pub fn withdraw_lp_fee_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawLpFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.provider,
        accounts.owner_x_account,
        accounts.owner_y_account,
        accounts.pool_x_account,
        accounts.pool_y_account,
        accounts.owner,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_lp_fee_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawLpFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_lp_fee_verify_account_privileges<'me, 'info>(
    accounts: WithdrawLpFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_lp_fee_verify_writable_privileges(accounts)?;
    withdraw_lp_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_PROJECT_FEE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawProjectFeeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub project_owner_x_account: &'me AccountInfo<'info>,
    pub project_owner_y_account: &'me AccountInfo<'info>,
    pub pool_x_account: &'me AccountInfo<'info>,
    pub pool_y_account: &'me AccountInfo<'info>,
    pub project_owner: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawProjectFeeKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub project_owner_x_account: Pubkey,
    pub project_owner_y_account: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub project_owner: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<WithdrawProjectFeeAccounts<'_, '_>> for WithdrawProjectFeeKeys {
    fn from(accounts: WithdrawProjectFeeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            project_owner_x_account: *accounts.project_owner_x_account.key,
            project_owner_y_account: *accounts.project_owner_y_account.key,
            pool_x_account: *accounts.pool_x_account.key,
            pool_y_account: *accounts.pool_y_account.key,
            project_owner: *accounts.project_owner.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<WithdrawProjectFeeKeys>
for [AccountMeta; WITHDRAW_PROJECT_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawProjectFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.project_owner_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.project_owner_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.project_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
impl From<[Pubkey; WITHDRAW_PROJECT_FEE_IX_ACCOUNTS_LEN]> for WithdrawProjectFeeKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PROJECT_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            token_x: pubkeys[2],
            token_y: pubkeys[3],
            project_owner_x_account: pubkeys[4],
            project_owner_y_account: pubkeys[5],
            pool_x_account: pubkeys[6],
            pool_y_account: pubkeys[7],
            project_owner: pubkeys[8],
            program_authority: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            rent: pubkeys[13],
        }
    }
}
impl<'info> From<WithdrawProjectFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PROJECT_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawProjectFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.project_owner_x_account.clone(),
            accounts.project_owner_y_account.clone(),
            accounts.pool_x_account.clone(),
            accounts.pool_y_account.clone(),
            accounts.project_owner.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_PROJECT_FEE_IX_ACCOUNTS_LEN]>
for WithdrawProjectFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_PROJECT_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            token_x: &arr[2],
            token_y: &arr[3],
            project_owner_x_account: &arr[4],
            project_owner_y_account: &arr[5],
            pool_x_account: &arr[6],
            pool_y_account: &arr[7],
            project_owner: &arr[8],
            program_authority: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
            rent: &arr[13],
        }
    }
}
pub const WITHDRAW_PROJECT_FEE_IX_DISCM: [u8; 8usize] = [
    130, 201, 142, 156, 159, 207, 168, 22,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawProjectFeeIxData;
impl WithdrawProjectFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_PROJECT_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_PROJECT_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_project_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawProjectFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PROJECT_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawProjectFeeIxData.try_to_vec()?,
    })
}
pub fn withdraw_project_fee_ix(
    keys: WithdrawProjectFeeKeys,
) -> std::io::Result<Instruction> {
    withdraw_project_fee_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys)
}
pub fn withdraw_project_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProjectFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawProjectFeeKeys = accounts.into();
    let ix = withdraw_project_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_project_fee_invoke(
    accounts: WithdrawProjectFeeAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_project_fee_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts)
}
pub fn withdraw_project_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProjectFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawProjectFeeKeys = accounts.into();
    let ix = withdraw_project_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_project_fee_invoke_signed(
    accounts: WithdrawProjectFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_project_fee_invoke_signed_with_program_id(
        BONKSWAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn withdraw_project_fee_verify_account_keys(
    accounts: WithdrawProjectFeeAccounts<'_, '_>,
    keys: WithdrawProjectFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.project_owner_x_account.key, keys.project_owner_x_account),
        (*accounts.project_owner_y_account.key, keys.project_owner_y_account),
        (*accounts.pool_x_account.key, keys.pool_x_account),
        (*accounts.pool_y_account.key, keys.pool_y_account),
        (*accounts.project_owner.key, keys.project_owner),
        (*accounts.program_authority.key, keys.program_authority),
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
pub fn withdraw_project_fee_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawProjectFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.project_owner_x_account,
        accounts.project_owner_y_account,
        accounts.pool_x_account,
        accounts.pool_y_account,
        accounts.project_owner,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_project_fee_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawProjectFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.project_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_project_fee_verify_account_privileges<'me, 'info>(
    accounts: WithdrawProjectFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_project_fee_verify_writable_privileges(accounts)?;
    withdraw_project_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_FARM_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct CreateFarmAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub farm: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub token_marco: &'me AccountInfo<'info>,
    pub token_marco_account: &'me AccountInfo<'info>,
    pub admin_marco_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateFarmKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub farm: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub token_marco: Pubkey,
    pub token_marco_account: Pubkey,
    pub admin_marco_account: Pubkey,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateFarmAccounts<'_, '_>> for CreateFarmKeys {
    fn from(accounts: CreateFarmAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            farm: *accounts.farm.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            token_marco: *accounts.token_marco.key,
            token_marco_account: *accounts.token_marco_account.key,
            admin_marco_account: *accounts.admin_marco_account.key,
            admin: *accounts.admin.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateFarmKeys> for [AccountMeta; CREATE_FARM_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateFarmKeys) -> Self {
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
                pubkey: keys.farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_FARM_IX_ACCOUNTS_LEN]> for CreateFarmKeys {
    fn from(pubkeys: [Pubkey; CREATE_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            farm: pubkeys[2],
            token_x: pubkeys[3],
            token_y: pubkeys[4],
            token_marco: pubkeys[5],
            token_marco_account: pubkeys[6],
            admin_marco_account: pubkeys[7],
            admin: pubkeys[8],
            program_authority: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            rent: pubkeys[12],
        }
    }
}
impl<'info> From<CreateFarmAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_FARM_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateFarmAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.farm.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.token_marco.clone(),
            accounts.token_marco_account.clone(),
            accounts.admin_marco_account.clone(),
            accounts.admin.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_FARM_IX_ACCOUNTS_LEN]>
for CreateFarmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            farm: &arr[2],
            token_x: &arr[3],
            token_y: &arr[4],
            token_marco: &arr[5],
            token_marco_account: &arr[6],
            admin_marco_account: &arr[7],
            admin: &arr[8],
            program_authority: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            rent: &arr[12],
        }
    }
}
pub const CREATE_FARM_IX_DISCM: [u8; 8usize] = [74, 59, 128, 160, 87, 174, 153, 194];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateFarmIxArgs {
    pub supply: Token,
    pub duration: u64,
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateFarmIxData(pub CreateFarmIxArgs);
impl From<CreateFarmIxArgs> for CreateFarmIxData {
    fn from(args: CreateFarmIxArgs) -> Self {
        Self(args)
    }
}
impl CreateFarmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_FARM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let supply = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateFarmIxArgs {
                supply,
                duration,
                bump,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_FARM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_farm_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateFarmKeys,
    args: CreateFarmIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_FARM_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateFarmIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_farm_ix(
    keys: CreateFarmKeys,
    args: CreateFarmIxArgs,
) -> std::io::Result<Instruction> {
    create_farm_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys, args)
}
pub fn create_farm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateFarmAccounts<'_, '_>,
    args: CreateFarmIxArgs,
) -> ProgramResult {
    let keys: CreateFarmKeys = accounts.into();
    let ix = create_farm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_farm_invoke(
    accounts: CreateFarmAccounts<'_, '_>,
    args: CreateFarmIxArgs,
) -> ProgramResult {
    create_farm_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args)
}
pub fn create_farm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateFarmAccounts<'_, '_>,
    args: CreateFarmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateFarmKeys = accounts.into();
    let ix = create_farm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_farm_invoke_signed(
    accounts: CreateFarmAccounts<'_, '_>,
    args: CreateFarmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_farm_invoke_signed_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_farm_verify_account_keys(
    accounts: CreateFarmAccounts<'_, '_>,
    keys: CreateFarmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.farm.key, keys.farm),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.token_marco.key, keys.token_marco),
        (*accounts.token_marco_account.key, keys.token_marco_account),
        (*accounts.admin_marco_account.key, keys.admin_marco_account),
        (*accounts.admin.key, keys.admin),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_farm_verify_writable_privileges<'me, 'info>(
    accounts: CreateFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm,
        accounts.token_marco_account,
        accounts.admin_marco_account,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_farm_verify_signer_privileges<'me, 'info>(
    accounts: CreateFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.token_marco_account, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_farm_verify_account_privileges<'me, 'info>(
    accounts: CreateFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_farm_verify_writable_privileges(accounts)?;
    create_farm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_DUAL_FARM_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct CreateDualFarmAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub farm: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub token_marco: &'me AccountInfo<'info>,
    pub token_project_first: &'me AccountInfo<'info>,
    pub token_marco_account: &'me AccountInfo<'info>,
    pub token_project_first_account: &'me AccountInfo<'info>,
    pub admin_marco_account: &'me AccountInfo<'info>,
    pub admin_project_first_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateDualFarmKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub farm: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub token_marco: Pubkey,
    pub token_project_first: Pubkey,
    pub token_marco_account: Pubkey,
    pub token_project_first_account: Pubkey,
    pub admin_marco_account: Pubkey,
    pub admin_project_first_account: Pubkey,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateDualFarmAccounts<'_, '_>> for CreateDualFarmKeys {
    fn from(accounts: CreateDualFarmAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            farm: *accounts.farm.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            token_marco: *accounts.token_marco.key,
            token_project_first: *accounts.token_project_first.key,
            token_marco_account: *accounts.token_marco_account.key,
            token_project_first_account: *accounts.token_project_first_account.key,
            admin_marco_account: *accounts.admin_marco_account.key,
            admin_project_first_account: *accounts.admin_project_first_account.key,
            admin: *accounts.admin.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateDualFarmKeys> for [AccountMeta; CREATE_DUAL_FARM_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateDualFarmKeys) -> Self {
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
                pubkey: keys.farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_project_first,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_first_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_DUAL_FARM_IX_ACCOUNTS_LEN]> for CreateDualFarmKeys {
    fn from(pubkeys: [Pubkey; CREATE_DUAL_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            farm: pubkeys[2],
            token_x: pubkeys[3],
            token_y: pubkeys[4],
            token_marco: pubkeys[5],
            token_project_first: pubkeys[6],
            token_marco_account: pubkeys[7],
            token_project_first_account: pubkeys[8],
            admin_marco_account: pubkeys[9],
            admin_project_first_account: pubkeys[10],
            admin: pubkeys[11],
            program_authority: pubkeys[12],
            system_program: pubkeys[13],
            token_program: pubkeys[14],
            rent: pubkeys[15],
        }
    }
}
impl<'info> From<CreateDualFarmAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_DUAL_FARM_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateDualFarmAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.farm.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.token_marco.clone(),
            accounts.token_project_first.clone(),
            accounts.token_marco_account.clone(),
            accounts.token_project_first_account.clone(),
            accounts.admin_marco_account.clone(),
            accounts.admin_project_first_account.clone(),
            accounts.admin.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_DUAL_FARM_IX_ACCOUNTS_LEN]>
for CreateDualFarmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_DUAL_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            farm: &arr[2],
            token_x: &arr[3],
            token_y: &arr[4],
            token_marco: &arr[5],
            token_project_first: &arr[6],
            token_marco_account: &arr[7],
            token_project_first_account: &arr[8],
            admin_marco_account: &arr[9],
            admin_project_first_account: &arr[10],
            admin: &arr[11],
            program_authority: &arr[12],
            system_program: &arr[13],
            token_program: &arr[14],
            rent: &arr[15],
        }
    }
}
pub const CREATE_DUAL_FARM_IX_DISCM: [u8; 8usize] = [
    42, 180, 103, 138, 206, 43, 208, 98,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateDualFarmIxArgs {
    pub supply_marco: Token,
    pub supply_project_first: Token,
    pub duration: u64,
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateDualFarmIxData(pub CreateDualFarmIxArgs);
impl From<CreateDualFarmIxArgs> for CreateDualFarmIxData {
    fn from(args: CreateDualFarmIxArgs) -> Self {
        Self(args)
    }
}
impl CreateDualFarmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_DUAL_FARM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let supply_marco = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let supply_project_first = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateDualFarmIxArgs {
                supply_marco,
                supply_project_first,
                duration,
                bump,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_DUAL_FARM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.supply_marco, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.supply_project_first, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_dual_farm_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateDualFarmKeys,
    args: CreateDualFarmIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_DUAL_FARM_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateDualFarmIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_dual_farm_ix(
    keys: CreateDualFarmKeys,
    args: CreateDualFarmIxArgs,
) -> std::io::Result<Instruction> {
    create_dual_farm_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys, args)
}
pub fn create_dual_farm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateDualFarmAccounts<'_, '_>,
    args: CreateDualFarmIxArgs,
) -> ProgramResult {
    let keys: CreateDualFarmKeys = accounts.into();
    let ix = create_dual_farm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_dual_farm_invoke(
    accounts: CreateDualFarmAccounts<'_, '_>,
    args: CreateDualFarmIxArgs,
) -> ProgramResult {
    create_dual_farm_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args)
}
pub fn create_dual_farm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateDualFarmAccounts<'_, '_>,
    args: CreateDualFarmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateDualFarmKeys = accounts.into();
    let ix = create_dual_farm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_dual_farm_invoke_signed(
    accounts: CreateDualFarmAccounts<'_, '_>,
    args: CreateDualFarmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_dual_farm_invoke_signed_with_program_id(
        BONKSWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_dual_farm_verify_account_keys(
    accounts: CreateDualFarmAccounts<'_, '_>,
    keys: CreateDualFarmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.farm.key, keys.farm),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.token_marco.key, keys.token_marco),
        (*accounts.token_project_first.key, keys.token_project_first),
        (*accounts.token_marco_account.key, keys.token_marco_account),
        (*accounts.token_project_first_account.key, keys.token_project_first_account),
        (*accounts.admin_marco_account.key, keys.admin_marco_account),
        (*accounts.admin_project_first_account.key, keys.admin_project_first_account),
        (*accounts.admin.key, keys.admin),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_dual_farm_verify_writable_privileges<'me, 'info>(
    accounts: CreateDualFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm,
        accounts.token_marco_account,
        accounts.token_project_first_account,
        accounts.admin_marco_account,
        accounts.admin_project_first_account,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_dual_farm_verify_signer_privileges<'me, 'info>(
    accounts: CreateDualFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.token_marco_account,
        accounts.token_project_first_account,
        accounts.admin,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_dual_farm_verify_account_privileges<'me, 'info>(
    accounts: CreateDualFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_dual_farm_verify_writable_privileges(accounts)?;
    create_dual_farm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TRIPLE_FARM_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct CreateTripleFarmAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub farm: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub token_marco: &'me AccountInfo<'info>,
    pub token_project_first: &'me AccountInfo<'info>,
    pub token_project_second: &'me AccountInfo<'info>,
    pub token_marco_account: &'me AccountInfo<'info>,
    pub token_project_first_account: &'me AccountInfo<'info>,
    pub token_project_second_account: &'me AccountInfo<'info>,
    pub admin_marco_account: &'me AccountInfo<'info>,
    pub admin_project_first_account: &'me AccountInfo<'info>,
    pub admin_project_second_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTripleFarmKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub farm: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub token_marco: Pubkey,
    pub token_project_first: Pubkey,
    pub token_project_second: Pubkey,
    pub token_marco_account: Pubkey,
    pub token_project_first_account: Pubkey,
    pub token_project_second_account: Pubkey,
    pub admin_marco_account: Pubkey,
    pub admin_project_first_account: Pubkey,
    pub admin_project_second_account: Pubkey,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateTripleFarmAccounts<'_, '_>> for CreateTripleFarmKeys {
    fn from(accounts: CreateTripleFarmAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            farm: *accounts.farm.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            token_marco: *accounts.token_marco.key,
            token_project_first: *accounts.token_project_first.key,
            token_project_second: *accounts.token_project_second.key,
            token_marco_account: *accounts.token_marco_account.key,
            token_project_first_account: *accounts.token_project_first_account.key,
            token_project_second_account: *accounts.token_project_second_account.key,
            admin_marco_account: *accounts.admin_marco_account.key,
            admin_project_first_account: *accounts.admin_project_first_account.key,
            admin_project_second_account: *accounts.admin_project_second_account.key,
            admin: *accounts.admin.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateTripleFarmKeys> for [AccountMeta; CREATE_TRIPLE_FARM_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTripleFarmKeys) -> Self {
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
                pubkey: keys.farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_project_first,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_project_second,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_first_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_second_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_TRIPLE_FARM_IX_ACCOUNTS_LEN]> for CreateTripleFarmKeys {
    fn from(pubkeys: [Pubkey; CREATE_TRIPLE_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            farm: pubkeys[2],
            token_x: pubkeys[3],
            token_y: pubkeys[4],
            token_marco: pubkeys[5],
            token_project_first: pubkeys[6],
            token_project_second: pubkeys[7],
            token_marco_account: pubkeys[8],
            token_project_first_account: pubkeys[9],
            token_project_second_account: pubkeys[10],
            admin_marco_account: pubkeys[11],
            admin_project_first_account: pubkeys[12],
            admin_project_second_account: pubkeys[13],
            admin: pubkeys[14],
            program_authority: pubkeys[15],
            system_program: pubkeys[16],
            token_program: pubkeys[17],
            rent: pubkeys[18],
        }
    }
}
impl<'info> From<CreateTripleFarmAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TRIPLE_FARM_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTripleFarmAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.farm.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.token_marco.clone(),
            accounts.token_project_first.clone(),
            accounts.token_project_second.clone(),
            accounts.token_marco_account.clone(),
            accounts.token_project_first_account.clone(),
            accounts.token_project_second_account.clone(),
            accounts.admin_marco_account.clone(),
            accounts.admin_project_first_account.clone(),
            accounts.admin_project_second_account.clone(),
            accounts.admin.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TRIPLE_FARM_IX_ACCOUNTS_LEN]>
for CreateTripleFarmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_TRIPLE_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            farm: &arr[2],
            token_x: &arr[3],
            token_y: &arr[4],
            token_marco: &arr[5],
            token_project_first: &arr[6],
            token_project_second: &arr[7],
            token_marco_account: &arr[8],
            token_project_first_account: &arr[9],
            token_project_second_account: &arr[10],
            admin_marco_account: &arr[11],
            admin_project_first_account: &arr[12],
            admin_project_second_account: &arr[13],
            admin: &arr[14],
            program_authority: &arr[15],
            system_program: &arr[16],
            token_program: &arr[17],
            rent: &arr[18],
        }
    }
}
pub const CREATE_TRIPLE_FARM_IX_DISCM: [u8; 8usize] = [
    154, 26, 180, 145, 18, 201, 135, 171,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTripleFarmIxArgs {
    pub supply_marco: Token,
    pub supply_project_first: Token,
    pub supply_project_second: Token,
    pub duration: u64,
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTripleFarmIxData(pub CreateTripleFarmIxArgs);
impl From<CreateTripleFarmIxArgs> for CreateTripleFarmIxData {
    fn from(args: CreateTripleFarmIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTripleFarmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TRIPLE_FARM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let supply_marco = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let supply_project_first = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let supply_project_second = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateTripleFarmIxArgs {
                supply_marco,
                supply_project_first,
                supply_project_second,
                duration,
                bump,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TRIPLE_FARM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.supply_marco, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.supply_project_first, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.supply_project_second, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_triple_farm_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTripleFarmKeys,
    args: CreateTripleFarmIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TRIPLE_FARM_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateTripleFarmIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_triple_farm_ix(
    keys: CreateTripleFarmKeys,
    args: CreateTripleFarmIxArgs,
) -> std::io::Result<Instruction> {
    create_triple_farm_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys, args)
}
pub fn create_triple_farm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTripleFarmAccounts<'_, '_>,
    args: CreateTripleFarmIxArgs,
) -> ProgramResult {
    let keys: CreateTripleFarmKeys = accounts.into();
    let ix = create_triple_farm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_triple_farm_invoke(
    accounts: CreateTripleFarmAccounts<'_, '_>,
    args: CreateTripleFarmIxArgs,
) -> ProgramResult {
    create_triple_farm_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args)
}
pub fn create_triple_farm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTripleFarmAccounts<'_, '_>,
    args: CreateTripleFarmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTripleFarmKeys = accounts.into();
    let ix = create_triple_farm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_triple_farm_invoke_signed(
    accounts: CreateTripleFarmAccounts<'_, '_>,
    args: CreateTripleFarmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_triple_farm_invoke_signed_with_program_id(
        BONKSWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_triple_farm_verify_account_keys(
    accounts: CreateTripleFarmAccounts<'_, '_>,
    keys: CreateTripleFarmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.farm.key, keys.farm),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.token_marco.key, keys.token_marco),
        (*accounts.token_project_first.key, keys.token_project_first),
        (*accounts.token_project_second.key, keys.token_project_second),
        (*accounts.token_marco_account.key, keys.token_marco_account),
        (*accounts.token_project_first_account.key, keys.token_project_first_account),
        (*accounts.token_project_second_account.key, keys.token_project_second_account),
        (*accounts.admin_marco_account.key, keys.admin_marco_account),
        (*accounts.admin_project_first_account.key, keys.admin_project_first_account),
        (*accounts.admin_project_second_account.key, keys.admin_project_second_account),
        (*accounts.admin.key, keys.admin),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_triple_farm_verify_writable_privileges<'me, 'info>(
    accounts: CreateTripleFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm,
        accounts.token_marco_account,
        accounts.token_project_first_account,
        accounts.token_project_second_account,
        accounts.admin_marco_account,
        accounts.admin_project_first_account,
        accounts.admin_project_second_account,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_triple_farm_verify_signer_privileges<'me, 'info>(
    accounts: CreateTripleFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.token_marco_account,
        accounts.token_project_first_account,
        accounts.token_project_second_account,
        accounts.admin,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_triple_farm_verify_account_privileges<'me, 'info>(
    accounts: CreateTripleFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_triple_farm_verify_writable_privileges(accounts)?;
    create_triple_farm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_REWARDS_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawRewardsAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub farm: &'me AccountInfo<'info>,
    pub provider: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub token_marco: &'me AccountInfo<'info>,
    pub token_project_first: &'me AccountInfo<'info>,
    pub token_project_second: &'me AccountInfo<'info>,
    pub token_marco_account: &'me AccountInfo<'info>,
    pub token_project_first_account: &'me AccountInfo<'info>,
    pub token_project_second_account: &'me AccountInfo<'info>,
    pub owner_marco_account: &'me AccountInfo<'info>,
    pub owner_project_first_account: &'me AccountInfo<'info>,
    pub owner_project_second_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawRewardsKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub farm: Pubkey,
    pub provider: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub token_marco: Pubkey,
    pub token_project_first: Pubkey,
    pub token_project_second: Pubkey,
    pub token_marco_account: Pubkey,
    pub token_project_first_account: Pubkey,
    pub token_project_second_account: Pubkey,
    pub owner_marco_account: Pubkey,
    pub owner_project_first_account: Pubkey,
    pub owner_project_second_account: Pubkey,
    pub owner: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<WithdrawRewardsAccounts<'_, '_>> for WithdrawRewardsKeys {
    fn from(accounts: WithdrawRewardsAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            farm: *accounts.farm.key,
            provider: *accounts.provider.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            token_marco: *accounts.token_marco.key,
            token_project_first: *accounts.token_project_first.key,
            token_project_second: *accounts.token_project_second.key,
            token_marco_account: *accounts.token_marco_account.key,
            token_project_first_account: *accounts.token_project_first_account.key,
            token_project_second_account: *accounts.token_project_second_account.key,
            owner_marco_account: *accounts.owner_marco_account.key,
            owner_project_first_account: *accounts.owner_project_first_account.key,
            owner_project_second_account: *accounts.owner_project_second_account.key,
            owner: *accounts.owner.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<WithdrawRewardsKeys> for [AccountMeta; WITHDRAW_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawRewardsKeys) -> Self {
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
                pubkey: keys.farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_first,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_second,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
impl From<[Pubkey; WITHDRAW_REWARDS_IX_ACCOUNTS_LEN]> for WithdrawRewardsKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            farm: pubkeys[2],
            provider: pubkeys[3],
            token_x: pubkeys[4],
            token_y: pubkeys[5],
            token_marco: pubkeys[6],
            token_project_first: pubkeys[7],
            token_project_second: pubkeys[8],
            token_marco_account: pubkeys[9],
            token_project_first_account: pubkeys[10],
            token_project_second_account: pubkeys[11],
            owner_marco_account: pubkeys[12],
            owner_project_first_account: pubkeys[13],
            owner_project_second_account: pubkeys[14],
            owner: pubkeys[15],
            program_authority: pubkeys[16],
            system_program: pubkeys[17],
            token_program: pubkeys[18],
            associated_token_program: pubkeys[19],
            rent: pubkeys[20],
        }
    }
}
impl<'info> From<WithdrawRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.farm.clone(),
            accounts.provider.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.token_marco.clone(),
            accounts.token_project_first.clone(),
            accounts.token_project_second.clone(),
            accounts.token_marco_account.clone(),
            accounts.token_project_first_account.clone(),
            accounts.token_project_second_account.clone(),
            accounts.owner_marco_account.clone(),
            accounts.owner_project_first_account.clone(),
            accounts.owner_project_second_account.clone(),
            accounts.owner.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_REWARDS_IX_ACCOUNTS_LEN]>
for WithdrawRewardsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            farm: &arr[2],
            provider: &arr[3],
            token_x: &arr[4],
            token_y: &arr[5],
            token_marco: &arr[6],
            token_project_first: &arr[7],
            token_project_second: &arr[8],
            token_marco_account: &arr[9],
            token_project_first_account: &arr[10],
            token_project_second_account: &arr[11],
            owner_marco_account: &arr[12],
            owner_project_first_account: &arr[13],
            owner_project_second_account: &arr[14],
            owner: &arr[15],
            program_authority: &arr[16],
            system_program: &arr[17],
            token_program: &arr[18],
            associated_token_program: &arr[19],
            rent: &arr[20],
        }
    }
}
pub const WITHDRAW_REWARDS_IX_DISCM: [u8; 8usize] = [
    10, 214, 219, 139, 205, 22, 251, 21,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawRewardsIxData;
impl WithdrawRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawRewardsIxData.try_to_vec()?,
    })
}
pub fn withdraw_rewards_ix(keys: WithdrawRewardsKeys) -> std::io::Result<Instruction> {
    withdraw_rewards_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys)
}
pub fn withdraw_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawRewardsKeys = accounts.into();
    let ix = withdraw_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_rewards_invoke(
    accounts: WithdrawRewardsAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_rewards_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts)
}
pub fn withdraw_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawRewardsKeys = accounts.into();
    let ix = withdraw_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_rewards_invoke_signed(
    accounts: WithdrawRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_rewards_invoke_signed_with_program_id(BONKSWAP_PROGRAM_ID, accounts, seeds)
}
pub fn withdraw_rewards_verify_account_keys(
    accounts: WithdrawRewardsAccounts<'_, '_>,
    keys: WithdrawRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.farm.key, keys.farm),
        (*accounts.provider.key, keys.provider),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.token_marco.key, keys.token_marco),
        (*accounts.token_project_first.key, keys.token_project_first),
        (*accounts.token_project_second.key, keys.token_project_second),
        (*accounts.token_marco_account.key, keys.token_marco_account),
        (*accounts.token_project_first_account.key, keys.token_project_first_account),
        (*accounts.token_project_second_account.key, keys.token_project_second_account),
        (*accounts.owner_marco_account.key, keys.owner_marco_account),
        (*accounts.owner_project_first_account.key, keys.owner_project_first_account),
        (*accounts.owner_project_second_account.key, keys.owner_project_second_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.program_authority.key, keys.program_authority),
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
pub fn withdraw_rewards_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm,
        accounts.provider,
        accounts.token_marco,
        accounts.token_project_first,
        accounts.token_project_second,
        accounts.token_marco_account,
        accounts.token_project_first_account,
        accounts.token_project_second_account,
        accounts.owner_marco_account,
        accounts.owner_project_first_account,
        accounts.owner_project_second_account,
        accounts.owner,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_rewards_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_rewards_verify_account_privileges<'me, 'info>(
    accounts: WithdrawRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_rewards_verify_writable_privileges(accounts)?;
    withdraw_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_POOL_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct ClosePoolAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub farm: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub token_marco_account: &'me AccountInfo<'info>,
    pub token_project_first_account: &'me AccountInfo<'info>,
    pub token_project_second_account: &'me AccountInfo<'info>,
    pub pool_x_account: &'me AccountInfo<'info>,
    pub pool_y_account: &'me AccountInfo<'info>,
    pub buyback_x_account: &'me AccountInfo<'info>,
    pub buyback_y_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePoolKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub farm: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub token_marco_account: Pubkey,
    pub token_project_first_account: Pubkey,
    pub token_project_second_account: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub buyback_x_account: Pubkey,
    pub buyback_y_account: Pubkey,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClosePoolAccounts<'_, '_>> for ClosePoolKeys {
    fn from(accounts: ClosePoolAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            farm: *accounts.farm.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            token_marco_account: *accounts.token_marco_account.key,
            token_project_first_account: *accounts.token_project_first_account.key,
            token_project_second_account: *accounts.token_project_second_account.key,
            pool_x_account: *accounts.pool_x_account.key,
            pool_y_account: *accounts.pool_y_account.key,
            buyback_x_account: *accounts.buyback_x_account.key,
            buyback_y_account: *accounts.buyback_y_account.key,
            admin: *accounts.admin.key,
            program_authority: *accounts.program_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClosePoolKeys> for [AccountMeta; CLOSE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyback_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyback_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
impl From<[Pubkey; CLOSE_POOL_IX_ACCOUNTS_LEN]> for ClosePoolKeys {
    fn from(pubkeys: [Pubkey; CLOSE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            farm: pubkeys[2],
            token_x: pubkeys[3],
            token_y: pubkeys[4],
            token_marco_account: pubkeys[5],
            token_project_first_account: pubkeys[6],
            token_project_second_account: pubkeys[7],
            pool_x_account: pubkeys[8],
            pool_y_account: pubkeys[9],
            buyback_x_account: pubkeys[10],
            buyback_y_account: pubkeys[11],
            admin: pubkeys[12],
            program_authority: pubkeys[13],
            token_program: pubkeys[14],
        }
    }
}
impl<'info> From<ClosePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.farm.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.token_marco_account.clone(),
            accounts.token_project_first_account.clone(),
            accounts.token_project_second_account.clone(),
            accounts.pool_x_account.clone(),
            accounts.pool_y_account.clone(),
            accounts.buyback_x_account.clone(),
            accounts.buyback_y_account.clone(),
            accounts.admin.clone(),
            accounts.program_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POOL_IX_ACCOUNTS_LEN]>
for ClosePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            farm: &arr[2],
            token_x: &arr[3],
            token_y: &arr[4],
            token_marco_account: &arr[5],
            token_project_first_account: &arr[6],
            token_project_second_account: &arr[7],
            pool_x_account: &arr[8],
            pool_y_account: &arr[9],
            buyback_x_account: &arr[10],
            buyback_y_account: &arr[11],
            admin: &arr[12],
            program_authority: &arr[13],
            token_program: &arr[14],
        }
    }
}
pub const CLOSE_POOL_IX_DISCM: [u8; 8usize] = [140, 189, 209, 23, 239, 62, 239, 11];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePoolIxData;
impl ClosePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePoolIxData.try_to_vec()?,
    })
}
pub fn close_pool_ix(keys: ClosePoolKeys) -> std::io::Result<Instruction> {
    close_pool_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys)
}
pub fn close_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePoolKeys = accounts.into();
    let ix = close_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_pool_invoke(accounts: ClosePoolAccounts<'_, '_>) -> ProgramResult {
    close_pool_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts)
}
pub fn close_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePoolKeys = accounts.into();
    let ix = close_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_pool_invoke_signed(
    accounts: ClosePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_pool_invoke_signed_with_program_id(BONKSWAP_PROGRAM_ID, accounts, seeds)
}
pub fn close_pool_verify_account_keys(
    accounts: ClosePoolAccounts<'_, '_>,
    keys: ClosePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.farm.key, keys.farm),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.token_marco_account.key, keys.token_marco_account),
        (*accounts.token_project_first_account.key, keys.token_project_first_account),
        (*accounts.token_project_second_account.key, keys.token_project_second_account),
        (*accounts.pool_x_account.key, keys.pool_x_account),
        (*accounts.pool_y_account.key, keys.pool_y_account),
        (*accounts.buyback_x_account.key, keys.buyback_x_account),
        (*accounts.buyback_y_account.key, keys.buyback_y_account),
        (*accounts.admin.key, keys.admin),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_pool_verify_writable_privileges<'me, 'info>(
    accounts: ClosePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.farm,
        accounts.token_marco_account,
        accounts.token_project_first_account,
        accounts.token_project_second_account,
        accounts.pool_x_account,
        accounts.pool_y_account,
        accounts.buyback_x_account,
        accounts.buyback_y_account,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_pool_verify_signer_privileges<'me, 'info>(
    accounts: ClosePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_pool_verify_account_privileges<'me, 'info>(
    accounts: ClosePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_pool_verify_writable_privileges(accounts)?;
    close_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_MERCANTI_FEE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawMercantiFeeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub mercanti_x_account: &'me AccountInfo<'info>,
    pub mercanti_y_account: &'me AccountInfo<'info>,
    pub pool_x_account: &'me AccountInfo<'info>,
    pub pool_y_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawMercantiFeeKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub mercanti_x_account: Pubkey,
    pub mercanti_y_account: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawMercantiFeeAccounts<'_, '_>> for WithdrawMercantiFeeKeys {
    fn from(accounts: WithdrawMercantiFeeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            mercanti_x_account: *accounts.mercanti_x_account.key,
            mercanti_y_account: *accounts.mercanti_y_account.key,
            pool_x_account: *accounts.pool_x_account.key,
            pool_y_account: *accounts.pool_y_account.key,
            admin: *accounts.admin.key,
            program_authority: *accounts.program_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawMercantiFeeKeys>
for [AccountMeta; WITHDRAW_MERCANTI_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawMercantiFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mercanti_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mercanti_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
impl From<[Pubkey; WITHDRAW_MERCANTI_FEE_IX_ACCOUNTS_LEN]> for WithdrawMercantiFeeKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_MERCANTI_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            token_x: pubkeys[2],
            token_y: pubkeys[3],
            mercanti_x_account: pubkeys[4],
            mercanti_y_account: pubkeys[5],
            pool_x_account: pubkeys[6],
            pool_y_account: pubkeys[7],
            admin: pubkeys[8],
            program_authority: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<WithdrawMercantiFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_MERCANTI_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawMercantiFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.mercanti_x_account.clone(),
            accounts.mercanti_y_account.clone(),
            accounts.pool_x_account.clone(),
            accounts.pool_y_account.clone(),
            accounts.admin.clone(),
            accounts.program_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_MERCANTI_FEE_IX_ACCOUNTS_LEN]>
for WithdrawMercantiFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_MERCANTI_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            token_x: &arr[2],
            token_y: &arr[3],
            mercanti_x_account: &arr[4],
            mercanti_y_account: &arr[5],
            pool_x_account: &arr[6],
            pool_y_account: &arr[7],
            admin: &arr[8],
            program_authority: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const WITHDRAW_MERCANTI_FEE_IX_DISCM: [u8; 8usize] = [
    253, 229, 129, 37, 47, 72, 11, 240,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawMercantiFeeIxData;
impl WithdrawMercantiFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_MERCANTI_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_MERCANTI_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_mercanti_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawMercantiFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_MERCANTI_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawMercantiFeeIxData.try_to_vec()?,
    })
}
pub fn withdraw_mercanti_fee_ix(
    keys: WithdrawMercantiFeeKeys,
) -> std::io::Result<Instruction> {
    withdraw_mercanti_fee_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys)
}
pub fn withdraw_mercanti_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawMercantiFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawMercantiFeeKeys = accounts.into();
    let ix = withdraw_mercanti_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_mercanti_fee_invoke(
    accounts: WithdrawMercantiFeeAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_mercanti_fee_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts)
}
pub fn withdraw_mercanti_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawMercantiFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawMercantiFeeKeys = accounts.into();
    let ix = withdraw_mercanti_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_mercanti_fee_invoke_signed(
    accounts: WithdrawMercantiFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_mercanti_fee_invoke_signed_with_program_id(
        BONKSWAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn withdraw_mercanti_fee_verify_account_keys(
    accounts: WithdrawMercantiFeeAccounts<'_, '_>,
    keys: WithdrawMercantiFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.mercanti_x_account.key, keys.mercanti_x_account),
        (*accounts.mercanti_y_account.key, keys.mercanti_y_account),
        (*accounts.pool_x_account.key, keys.pool_x_account),
        (*accounts.pool_y_account.key, keys.pool_y_account),
        (*accounts.admin.key, keys.admin),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_mercanti_fee_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawMercantiFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.mercanti_x_account,
        accounts.mercanti_y_account,
        accounts.pool_x_account,
        accounts.pool_y_account,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_mercanti_fee_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawMercantiFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_mercanti_fee_verify_account_privileges<'me, 'info>(
    accounts: WithdrawMercantiFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_mercanti_fee_verify_writable_privileges(accounts)?;
    withdraw_mercanti_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_SUPPLY_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct AddSupplyAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub farm: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub token_marco_account: &'me AccountInfo<'info>,
    pub token_project_first_account: &'me AccountInfo<'info>,
    pub token_project_second_account: &'me AccountInfo<'info>,
    pub admin_marco_account: &'me AccountInfo<'info>,
    pub admin_project_first_account: &'me AccountInfo<'info>,
    pub admin_project_second_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddSupplyKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub farm: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub token_marco_account: Pubkey,
    pub token_project_first_account: Pubkey,
    pub token_project_second_account: Pubkey,
    pub admin_marco_account: Pubkey,
    pub admin_project_first_account: Pubkey,
    pub admin_project_second_account: Pubkey,
    pub admin: Pubkey,
    pub token_program: Pubkey,
}
impl From<AddSupplyAccounts<'_, '_>> for AddSupplyKeys {
    fn from(accounts: AddSupplyAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            farm: *accounts.farm.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            token_marco_account: *accounts.token_marco_account.key,
            token_project_first_account: *accounts.token_project_first_account.key,
            token_project_second_account: *accounts.token_project_second_account.key,
            admin_marco_account: *accounts.admin_marco_account.key,
            admin_project_first_account: *accounts.admin_project_first_account.key,
            admin_project_second_account: *accounts.admin_project_second_account.key,
            admin: *accounts.admin.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<AddSupplyKeys> for [AccountMeta; ADD_SUPPLY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddSupplyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
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
impl From<[Pubkey; ADD_SUPPLY_IX_ACCOUNTS_LEN]> for AddSupplyKeys {
    fn from(pubkeys: [Pubkey; ADD_SUPPLY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            farm: pubkeys[2],
            token_x: pubkeys[3],
            token_y: pubkeys[4],
            token_marco_account: pubkeys[5],
            token_project_first_account: pubkeys[6],
            token_project_second_account: pubkeys[7],
            admin_marco_account: pubkeys[8],
            admin_project_first_account: pubkeys[9],
            admin_project_second_account: pubkeys[10],
            admin: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<AddSupplyAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_SUPPLY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddSupplyAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.farm.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.token_marco_account.clone(),
            accounts.token_project_first_account.clone(),
            accounts.token_project_second_account.clone(),
            accounts.admin_marco_account.clone(),
            accounts.admin_project_first_account.clone(),
            accounts.admin_project_second_account.clone(),
            accounts.admin.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_SUPPLY_IX_ACCOUNTS_LEN]>
for AddSupplyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_SUPPLY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            farm: &arr[2],
            token_x: &arr[3],
            token_y: &arr[4],
            token_marco_account: &arr[5],
            token_project_first_account: &arr[6],
            token_project_second_account: &arr[7],
            admin_marco_account: &arr[8],
            admin_project_first_account: &arr[9],
            admin_project_second_account: &arr[10],
            admin: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const ADD_SUPPLY_IX_DISCM: [u8; 8usize] = [80, 102, 70, 57, 235, 88, 239, 8];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddSupplyIxArgs {
    pub supply_marco: Token,
    pub supply_project_first: Token,
    pub supply_project_second: Token,
    pub duration: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddSupplyIxData(pub AddSupplyIxArgs);
impl From<AddSupplyIxArgs> for AddSupplyIxData {
    fn from(args: AddSupplyIxArgs) -> Self {
        Self(args)
    }
}
impl AddSupplyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_SUPPLY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let supply_marco = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let supply_project_first = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let supply_project_second = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddSupplyIxArgs {
                supply_marco,
                supply_project_first,
                supply_project_second,
                duration,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_SUPPLY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.supply_marco, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.supply_project_first, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.supply_project_second, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.duration, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_supply_ix_with_program_id(
    program_id: Pubkey,
    keys: AddSupplyKeys,
    args: AddSupplyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_SUPPLY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddSupplyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_supply_ix(
    keys: AddSupplyKeys,
    args: AddSupplyIxArgs,
) -> std::io::Result<Instruction> {
    add_supply_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys, args)
}
pub fn add_supply_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddSupplyAccounts<'_, '_>,
    args: AddSupplyIxArgs,
) -> ProgramResult {
    let keys: AddSupplyKeys = accounts.into();
    let ix = add_supply_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_supply_invoke(
    accounts: AddSupplyAccounts<'_, '_>,
    args: AddSupplyIxArgs,
) -> ProgramResult {
    add_supply_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args)
}
pub fn add_supply_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddSupplyAccounts<'_, '_>,
    args: AddSupplyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddSupplyKeys = accounts.into();
    let ix = add_supply_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_supply_invoke_signed(
    accounts: AddSupplyAccounts<'_, '_>,
    args: AddSupplyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_supply_invoke_signed_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_supply_verify_account_keys(
    accounts: AddSupplyAccounts<'_, '_>,
    keys: AddSupplyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.farm.key, keys.farm),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.token_marco_account.key, keys.token_marco_account),
        (*accounts.token_project_first_account.key, keys.token_project_first_account),
        (*accounts.token_project_second_account.key, keys.token_project_second_account),
        (*accounts.admin_marco_account.key, keys.admin_marco_account),
        (*accounts.admin_project_first_account.key, keys.admin_project_first_account),
        (*accounts.admin_project_second_account.key, keys.admin_project_second_account),
        (*accounts.admin.key, keys.admin),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_supply_verify_writable_privileges<'me, 'info>(
    accounts: AddSupplyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.farm,
        accounts.token_marco_account,
        accounts.token_project_first_account,
        accounts.token_project_second_account,
        accounts.admin_marco_account,
        accounts.admin_project_first_account,
        accounts.admin_project_second_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_supply_verify_signer_privileges<'me, 'info>(
    accounts: AddSupplyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_supply_verify_account_privileges<'me, 'info>(
    accounts: AddSupplyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_supply_verify_writable_privileges(accounts)?;
    add_supply_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FEES_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeesAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeesKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
}
impl From<UpdateFeesAccounts<'_, '_>> for UpdateFeesKeys {
    fn from(accounts: UpdateFeesAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            admin: *accounts.admin.key,
            program_authority: *accounts.program_authority.key,
        }
    }
}
impl From<UpdateFeesKeys> for [AccountMeta; UPDATE_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FEES_IX_ACCOUNTS_LEN]> for UpdateFeesKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            token_x: pubkeys[2],
            token_y: pubkeys[3],
            admin: pubkeys[4],
            program_authority: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.admin.clone(),
            accounts.program_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FEES_IX_ACCOUNTS_LEN]>
for UpdateFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            token_x: &arr[2],
            token_y: &arr[3],
            admin: &arr[4],
            program_authority: &arr[5],
        }
    }
}
pub const UPDATE_FEES_IX_DISCM: [u8; 8usize] = [225, 27, 13, 6, 69, 84, 172, 191];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeesIxArgs {
    pub new_buyback_fee: FixedPoint,
    pub new_project_fee: FixedPoint,
    pub new_provider_fee: FixedPoint,
    pub new_mercanti_fee: FixedPoint,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeesIxData(pub UpdateFeesIxArgs);
impl From<UpdateFeesIxArgs> for UpdateFeesIxData {
    fn from(args: UpdateFeesIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_buyback_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let new_project_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let new_provider_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let new_mercanti_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateFeesIxArgs {
                new_buyback_fee,
                new_project_fee,
                new_provider_fee,
                new_mercanti_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_buyback_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_project_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_provider_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_mercanti_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeesKeys,
    args: UpdateFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_fees_ix(
    keys: UpdateFeesKeys,
    args: UpdateFeesIxArgs,
) -> std::io::Result<Instruction> {
    update_fees_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys, args)
}
pub fn update_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeesAccounts<'_, '_>,
    args: UpdateFeesIxArgs,
) -> ProgramResult {
    let keys: UpdateFeesKeys = accounts.into();
    let ix = update_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fees_invoke(
    accounts: UpdateFeesAccounts<'_, '_>,
    args: UpdateFeesIxArgs,
) -> ProgramResult {
    update_fees_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args)
}
pub fn update_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeesAccounts<'_, '_>,
    args: UpdateFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeesKeys = accounts.into();
    let ix = update_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fees_invoke_signed(
    accounts: UpdateFeesAccounts<'_, '_>,
    args: UpdateFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fees_invoke_signed_with_program_id(BONKSWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_fees_verify_account_keys(
    accounts: UpdateFeesAccounts<'_, '_>,
    keys: UpdateFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.admin.key, keys.admin),
        (*accounts.program_authority.key, keys.program_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fees_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fees_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fees_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fees_verify_writable_privileges(accounts)?;
    update_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RESET_FARM_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct ResetFarmAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub farm: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub token_marco: &'me AccountInfo<'info>,
    pub token_marco_account: &'me AccountInfo<'info>,
    pub token_project_first_account: &'me AccountInfo<'info>,
    pub token_project_second_account: &'me AccountInfo<'info>,
    pub admin_marco_account: &'me AccountInfo<'info>,
    pub admin_project_first_account: &'me AccountInfo<'info>,
    pub admin_project_second_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResetFarmKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub farm: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub token_marco: Pubkey,
    pub token_marco_account: Pubkey,
    pub token_project_first_account: Pubkey,
    pub token_project_second_account: Pubkey,
    pub admin_marco_account: Pubkey,
    pub admin_project_first_account: Pubkey,
    pub admin_project_second_account: Pubkey,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<ResetFarmAccounts<'_, '_>> for ResetFarmKeys {
    fn from(accounts: ResetFarmAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            farm: *accounts.farm.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            token_marco: *accounts.token_marco.key,
            token_marco_account: *accounts.token_marco_account.key,
            token_project_first_account: *accounts.token_project_first_account.key,
            token_project_second_account: *accounts.token_project_second_account.key,
            admin_marco_account: *accounts.admin_marco_account.key,
            admin_project_first_account: *accounts.admin_project_first_account.key,
            admin_project_second_account: *accounts.admin_project_second_account.key,
            admin: *accounts.admin.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<ResetFarmKeys> for [AccountMeta; RESET_FARM_IX_ACCOUNTS_LEN] {
    fn from(keys: ResetFarmKeys) -> Self {
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
                pubkey: keys.farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; RESET_FARM_IX_ACCOUNTS_LEN]> for ResetFarmKeys {
    fn from(pubkeys: [Pubkey; RESET_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            farm: pubkeys[2],
            token_x: pubkeys[3],
            token_y: pubkeys[4],
            token_marco: pubkeys[5],
            token_marco_account: pubkeys[6],
            token_project_first_account: pubkeys[7],
            token_project_second_account: pubkeys[8],
            admin_marco_account: pubkeys[9],
            admin_project_first_account: pubkeys[10],
            admin_project_second_account: pubkeys[11],
            admin: pubkeys[12],
            program_authority: pubkeys[13],
            system_program: pubkeys[14],
            token_program: pubkeys[15],
            rent: pubkeys[16],
        }
    }
}
impl<'info> From<ResetFarmAccounts<'_, 'info>>
for [AccountInfo<'info>; RESET_FARM_IX_ACCOUNTS_LEN] {
    fn from(accounts: ResetFarmAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.farm.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.token_marco.clone(),
            accounts.token_marco_account.clone(),
            accounts.token_project_first_account.clone(),
            accounts.token_project_second_account.clone(),
            accounts.admin_marco_account.clone(),
            accounts.admin_project_first_account.clone(),
            accounts.admin_project_second_account.clone(),
            accounts.admin.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESET_FARM_IX_ACCOUNTS_LEN]>
for ResetFarmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; RESET_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            farm: &arr[2],
            token_x: &arr[3],
            token_y: &arr[4],
            token_marco: &arr[5],
            token_marco_account: &arr[6],
            token_project_first_account: &arr[7],
            token_project_second_account: &arr[8],
            admin_marco_account: &arr[9],
            admin_project_first_account: &arr[10],
            admin_project_second_account: &arr[11],
            admin: &arr[12],
            program_authority: &arr[13],
            system_program: &arr[14],
            token_program: &arr[15],
            rent: &arr[16],
        }
    }
}
pub const RESET_FARM_IX_DISCM: [u8; 8usize] = [47, 77, 233, 117, 118, 55, 61, 113];
#[derive(Clone, Debug, PartialEq)]
pub struct ResetFarmIxData;
impl ResetFarmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESET_FARM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESET_FARM_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn reset_farm_ix_with_program_id(
    program_id: Pubkey,
    keys: ResetFarmKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RESET_FARM_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ResetFarmIxData.try_to_vec()?,
    })
}
pub fn reset_farm_ix(keys: ResetFarmKeys) -> std::io::Result<Instruction> {
    reset_farm_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys)
}
pub fn reset_farm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ResetFarmAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ResetFarmKeys = accounts.into();
    let ix = reset_farm_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn reset_farm_invoke(accounts: ResetFarmAccounts<'_, '_>) -> ProgramResult {
    reset_farm_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts)
}
pub fn reset_farm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ResetFarmAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ResetFarmKeys = accounts.into();
    let ix = reset_farm_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn reset_farm_invoke_signed(
    accounts: ResetFarmAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    reset_farm_invoke_signed_with_program_id(BONKSWAP_PROGRAM_ID, accounts, seeds)
}
pub fn reset_farm_verify_account_keys(
    accounts: ResetFarmAccounts<'_, '_>,
    keys: ResetFarmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.farm.key, keys.farm),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.token_marco.key, keys.token_marco),
        (*accounts.token_marco_account.key, keys.token_marco_account),
        (*accounts.token_project_first_account.key, keys.token_project_first_account),
        (*accounts.token_project_second_account.key, keys.token_project_second_account),
        (*accounts.admin_marco_account.key, keys.admin_marco_account),
        (*accounts.admin_project_first_account.key, keys.admin_project_first_account),
        (*accounts.admin_project_second_account.key, keys.admin_project_second_account),
        (*accounts.admin.key, keys.admin),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn reset_farm_verify_writable_privileges<'me, 'info>(
    accounts: ResetFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm,
        accounts.token_marco_account,
        accounts.token_project_first_account,
        accounts.token_project_second_account,
        accounts.admin_marco_account,
        accounts.admin_project_first_account,
        accounts.admin_project_second_account,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn reset_farm_verify_signer_privileges<'me, 'info>(
    accounts: ResetFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn reset_farm_verify_account_privileges<'me, 'info>(
    accounts: ResetFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    reset_farm_verify_writable_privileges(accounts)?;
    reset_farm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REWARD_TOKENS_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRewardTokensAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub farm: &'me AccountInfo<'info>,
    pub token_marco_account: &'me AccountInfo<'info>,
    pub token_project_first_account: &'me AccountInfo<'info>,
    pub token_project_second_account: &'me AccountInfo<'info>,
    pub token_marco: &'me AccountInfo<'info>,
    pub new_token_marco_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRewardTokensKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub farm: Pubkey,
    pub token_marco_account: Pubkey,
    pub token_project_first_account: Pubkey,
    pub token_project_second_account: Pubkey,
    pub token_marco: Pubkey,
    pub new_token_marco_account: Pubkey,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<UpdateRewardTokensAccounts<'_, '_>> for UpdateRewardTokensKeys {
    fn from(accounts: UpdateRewardTokensAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            farm: *accounts.farm.key,
            token_marco_account: *accounts.token_marco_account.key,
            token_project_first_account: *accounts.token_project_first_account.key,
            token_project_second_account: *accounts.token_project_second_account.key,
            token_marco: *accounts.token_marco.key,
            new_token_marco_account: *accounts.new_token_marco_account.key,
            admin: *accounts.admin.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<UpdateRewardTokensKeys>
for [AccountMeta; UPDATE_REWARD_TOKENS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRewardTokensKeys) -> Self {
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
                pubkey: keys.farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_marco_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_first_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_project_second_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_marco,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_token_marco_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
        ]
    }
}
impl From<[Pubkey; UPDATE_REWARD_TOKENS_IX_ACCOUNTS_LEN]> for UpdateRewardTokensKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REWARD_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            farm: pubkeys[2],
            token_marco_account: pubkeys[3],
            token_project_first_account: pubkeys[4],
            token_project_second_account: pubkeys[5],
            token_marco: pubkeys[6],
            new_token_marco_account: pubkeys[7],
            admin: pubkeys[8],
            program_authority: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<UpdateRewardTokensAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REWARD_TOKENS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRewardTokensAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.farm.clone(),
            accounts.token_marco_account.clone(),
            accounts.token_project_first_account.clone(),
            accounts.token_project_second_account.clone(),
            accounts.token_marco.clone(),
            accounts.new_token_marco_account.clone(),
            accounts.admin.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_REWARD_TOKENS_IX_ACCOUNTS_LEN]>
for UpdateRewardTokensAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_REWARD_TOKENS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            farm: &arr[2],
            token_marco_account: &arr[3],
            token_project_first_account: &arr[4],
            token_project_second_account: &arr[5],
            token_marco: &arr[6],
            new_token_marco_account: &arr[7],
            admin: &arr[8],
            program_authority: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const UPDATE_REWARD_TOKENS_IX_DISCM: [u8; 8usize] = [
    249, 236, 71, 74, 104, 58, 225, 28,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardTokensIxData;
impl UpdateRewardTokensIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REWARD_TOKENS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REWARD_TOKENS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_reward_tokens_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRewardTokensKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REWARD_TOKENS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateRewardTokensIxData.try_to_vec()?,
    })
}
pub fn update_reward_tokens_ix(
    keys: UpdateRewardTokensKeys,
) -> std::io::Result<Instruction> {
    update_reward_tokens_ix_with_program_id(BONKSWAP_PROGRAM_ID, keys)
}
pub fn update_reward_tokens_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardTokensAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateRewardTokensKeys = accounts.into();
    let ix = update_reward_tokens_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_reward_tokens_invoke(
    accounts: UpdateRewardTokensAccounts<'_, '_>,
) -> ProgramResult {
    update_reward_tokens_invoke_with_program_id(BONKSWAP_PROGRAM_ID, accounts)
}
pub fn update_reward_tokens_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardTokensAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRewardTokensKeys = accounts.into();
    let ix = update_reward_tokens_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_reward_tokens_invoke_signed(
    accounts: UpdateRewardTokensAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_reward_tokens_invoke_signed_with_program_id(
        BONKSWAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_reward_tokens_verify_account_keys(
    accounts: UpdateRewardTokensAccounts<'_, '_>,
    keys: UpdateRewardTokensKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.farm.key, keys.farm),
        (*accounts.token_marco_account.key, keys.token_marco_account),
        (*accounts.token_project_first_account.key, keys.token_project_first_account),
        (*accounts.token_project_second_account.key, keys.token_project_second_account),
        (*accounts.token_marco.key, keys.token_marco),
        (*accounts.new_token_marco_account.key, keys.new_token_marco_account),
        (*accounts.admin.key, keys.admin),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_reward_tokens_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRewardTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm,
        accounts.token_marco_account,
        accounts.token_project_first_account,
        accounts.token_project_second_account,
        accounts.new_token_marco_account,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_reward_tokens_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRewardTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.new_token_marco_account, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_reward_tokens_verify_account_privileges<'me, 'info>(
    accounts: UpdateRewardTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_reward_tokens_verify_writable_privileges(accounts)?;
    update_reward_tokens_verify_signer_privileges(accounts)?;
    Ok(())
}
