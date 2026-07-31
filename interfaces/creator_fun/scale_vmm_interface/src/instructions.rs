use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum ScaleVmmProgramIx {
    Buy(BuyIxArgs),
    Create(CreateIxArgs),
    GetPlatformConfig,
    QuoteBuy(QuoteBuyIxArgs),
    QuoteSell(QuoteSellIxArgs),
    Sell(SellIxArgs),
    SetFeeBeneficiary(SetFeeBeneficiaryIxArgs),
    SetGraduationThreshold(SetGraduationThresholdIxArgs),
    SetPlatformBaseToken(SetPlatformBaseTokenIxArgs),
    SetPlatformFee(SetPlatformFeeIxArgs),
    TransferAuthority(TransferAuthorityIxArgs),
}
impl ScaleVmmProgramIx {
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
        if buf.starts_with(&CREATE_IX_DISCM) {
            let mut reader = &buf[CREATE_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <CreateParams>::deserialize(&mut reader)?
            };
            return Ok(Self::Create(CreateIxArgs { params }));
        }
        if buf.starts_with(&GET_PLATFORM_CONFIG_IX_DISCM) {
            return Ok(Self::GetPlatformConfig);
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
        if buf.starts_with(&SET_FEE_BENEFICIARY_IX_DISCM) {
            let mut reader = &buf[SET_FEE_BENEFICIARY_IX_DISCM.len()..];
            let beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetFeeBeneficiary(SetFeeBeneficiaryIxArgs {
                    beneficiary,
                }),
            );
        }
        if buf.starts_with(&SET_GRADUATION_THRESHOLD_IX_DISCM) {
            let mut reader = &buf[SET_GRADUATION_THRESHOLD_IX_DISCM.len()..];
            let threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetGraduationThreshold(SetGraduationThresholdIxArgs {
                    threshold,
                }),
            );
        }
        if buf.starts_with(&SET_PLATFORM_BASE_TOKEN_IX_DISCM) {
            let mut reader = &buf[SET_PLATFORM_BASE_TOKEN_IX_DISCM.len()..];
            let base_token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetPlatformBaseToken(SetPlatformBaseTokenIxArgs {
                    base_token,
                }),
            );
        }
        if buf.starts_with(&SET_PLATFORM_FEE_IX_DISCM) {
            let mut reader = &buf[SET_PLATFORM_FEE_IX_DISCM.len()..];
            let fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetPlatformFee(SetPlatformFeeIxArgs { fee_bps }));
        }
        if buf.starts_with(&TRANSFER_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[TRANSFER_AUTHORITY_IX_DISCM.len()..];
            let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TransferAuthority(TransferAuthorityIxArgs {
                    new_authority,
                }),
            );
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
            Self::Create(args) => {
                writer.write_all(&CREATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::GetPlatformConfig => writer.write_all(&GET_PLATFORM_CONFIG_IX_DISCM),
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
            Self::SetFeeBeneficiary(args) => {
                writer.write_all(&SET_FEE_BENEFICIARY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.beneficiary, &mut writer)?;
                Ok(())
            }
            Self::SetGraduationThreshold(args) => {
                writer.write_all(&SET_GRADUATION_THRESHOLD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.threshold, &mut writer)?;
                Ok(())
            }
            Self::SetPlatformBaseToken(args) => {
                writer.write_all(&SET_PLATFORM_BASE_TOKEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_token, &mut writer)?;
                Ok(())
            }
            Self::SetPlatformFee(args) => {
                writer.write_all(&SET_PLATFORM_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_bps, &mut writer)?;
                Ok(())
            }
            Self::TransferAuthority(args) => {
                writer.write_all(&TRANSFER_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_authority, &mut writer)?;
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
pub const BUY_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct BuyAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub user_ta_a: &'me AccountInfo<'info>,
    pub user_ta_b: &'me AccountInfo<'info>,
    pub vault_a: &'me AccountInfo<'info>,
    pub vault_b: &'me AccountInfo<'info>,
    pub platform_fee_ta_a: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub amm_pool: &'me AccountInfo<'info>,
    pub amm_vault_a: &'me AccountInfo<'info>,
    pub amm_vault_b: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyKeys {
    pub pair: Pubkey,
    pub user: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub user_ta_a: Pubkey,
    pub user_ta_b: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub platform_fee_ta_a: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub system_program: Pubkey,
    pub config: Pubkey,
    pub amm_program: Pubkey,
    pub amm_pool: Pubkey,
    pub amm_vault_a: Pubkey,
    pub amm_vault_b: Pubkey,
    pub amm_config: Pubkey,
}
impl From<BuyAccounts<'_, '_>> for BuyKeys {
    fn from(accounts: BuyAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            user: *accounts.user.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            user_ta_a: *accounts.user_ta_a.key,
            user_ta_b: *accounts.user_ta_b.key,
            vault_a: *accounts.vault_a.key,
            vault_b: *accounts.vault_b.key,
            platform_fee_ta_a: *accounts.platform_fee_ta_a.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            system_program: *accounts.system_program.key,
            config: *accounts.config.key,
            amm_program: *accounts.amm_program.key,
            amm_pool: *accounts.amm_pool.key,
            amm_vault_a: *accounts.amm_vault_a.key,
            amm_vault_b: *accounts.amm_vault_b.key,
            amm_config: *accounts.amm_config.key,
        }
    }
}
impl From<BuyKeys> for [AccountMeta; BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.platform_fee_ta_a,
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
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BUY_IX_ACCOUNTS_LEN]> for BuyKeys {
    fn from(pubkeys: [Pubkey; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            user: pubkeys[1],
            mint_a: pubkeys[2],
            mint_b: pubkeys[3],
            user_ta_a: pubkeys[4],
            user_ta_b: pubkeys[5],
            vault_a: pubkeys[6],
            vault_b: pubkeys[7],
            platform_fee_ta_a: pubkeys[8],
            token_program_a: pubkeys[9],
            token_program_b: pubkeys[10],
            system_program: pubkeys[11],
            config: pubkeys[12],
            amm_program: pubkeys[13],
            amm_pool: pubkeys[14],
            amm_vault_a: pubkeys[15],
            amm_vault_b: pubkeys[16],
            amm_config: pubkeys[17],
        }
    }
}
impl<'info> From<BuyAccounts<'_, 'info>> for [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.user.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.user_ta_a.clone(),
            accounts.user_ta_b.clone(),
            accounts.vault_a.clone(),
            accounts.vault_b.clone(),
            accounts.platform_fee_ta_a.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.system_program.clone(),
            accounts.config.clone(),
            accounts.amm_program.clone(),
            accounts.amm_pool.clone(),
            accounts.amm_vault_a.clone(),
            accounts.amm_vault_b.clone(),
            accounts.amm_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]>
for BuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            user: &arr[1],
            mint_a: &arr[2],
            mint_b: &arr[3],
            user_ta_a: &arr[4],
            user_ta_b: &arr[5],
            vault_a: &arr[6],
            vault_b: &arr[7],
            platform_fee_ta_a: &arr[8],
            token_program_a: &arr[9],
            token_program_b: &arr[10],
            system_program: &arr[11],
            config: &arr[12],
            amm_program: &arr[13],
            amm_pool: &arr[14],
            amm_vault_a: &arr[15],
            amm_vault_b: &arr[16],
            amm_config: &arr[17],
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
    buy_ix_with_program_id(SCALE_VMM_PROGRAM_ID, keys, args)
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
    buy_invoke_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args)
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
    buy_invoke_signed_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn buy_verify_account_keys(
    accounts: BuyAccounts<'_, '_>,
    keys: BuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.user.key, keys.user),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.user_ta_a.key, keys.user_ta_a),
        (*accounts.user_ta_b.key, keys.user_ta_b),
        (*accounts.vault_a.key, keys.vault_a),
        (*accounts.vault_b.key, keys.vault_b),
        (*accounts.platform_fee_ta_a.key, keys.platform_fee_ta_a),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.config.key, keys.config),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.amm_pool.key, keys.amm_pool),
        (*accounts.amm_vault_a.key, keys.amm_vault_a),
        (*accounts.amm_vault_b.key, keys.amm_vault_b),
        (*accounts.amm_config.key, keys.amm_config),
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
        accounts.pair,
        accounts.user,
        accounts.user_ta_a,
        accounts.user_ta_b,
        accounts.vault_a,
        accounts.vault_b,
        accounts.platform_fee_ta_a,
        accounts.amm_pool,
        accounts.amm_vault_a,
        accounts.amm_vault_b,
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
pub const CREATE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CreateAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub token_wallet_authority: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub token_wallet_b: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub vault_a: &'me AccountInfo<'info>,
    pub vault_b: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateKeys {
    pub payer: Pubkey,
    pub token_wallet_authority: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub token_wallet_b: Pubkey,
    pub pair: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub system_program: Pubkey,
    pub config: Pubkey,
}
impl From<CreateAccounts<'_, '_>> for CreateKeys {
    fn from(accounts: CreateAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            token_wallet_authority: *accounts.token_wallet_authority.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            token_wallet_b: *accounts.token_wallet_b.key,
            pair: *accounts.pair.key,
            vault_a: *accounts.vault_a.key,
            vault_b: *accounts.vault_b.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            system_program: *accounts.system_program.key,
            config: *accounts.config.key,
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
                pubkey: keys.pair,
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
                pubkey: keys.config,
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
            token_wallet_authority: pubkeys[1],
            mint_a: pubkeys[2],
            mint_b: pubkeys[3],
            token_wallet_b: pubkeys[4],
            pair: pubkeys[5],
            vault_a: pubkeys[6],
            vault_b: pubkeys[7],
            token_program_a: pubkeys[8],
            token_program_b: pubkeys[9],
            system_program: pubkeys[10],
            config: pubkeys[11],
        }
    }
}
impl<'info> From<CreateAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.token_wallet_authority.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.token_wallet_b.clone(),
            accounts.pair.clone(),
            accounts.vault_a.clone(),
            accounts.vault_b.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.system_program.clone(),
            accounts.config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]>
for CreateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            token_wallet_authority: &arr[1],
            mint_a: &arr[2],
            mint_b: &arr[3],
            token_wallet_b: &arr[4],
            pair: &arr[5],
            vault_a: &arr[6],
            vault_b: &arr[7],
            token_program_a: &arr[8],
            token_program_b: &arr[9],
            system_program: &arr[10],
            config: &arr[11],
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
    create_ix_with_program_id(SCALE_VMM_PROGRAM_ID, keys, args)
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
    create_invoke_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args)
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
    create_invoke_signed_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_verify_account_keys(
    accounts: CreateAccounts<'_, '_>,
    keys: CreateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.token_wallet_authority.key, keys.token_wallet_authority),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.token_wallet_b.key, keys.token_wallet_b),
        (*accounts.pair.key, keys.pair),
        (*accounts.vault_a.key, keys.vault_a),
        (*accounts.vault_b.key, keys.vault_b),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.config.key, keys.config),
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
        accounts.pair,
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
    for should_be_signer in [accounts.payer, accounts.token_wallet_authority] {
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
pub const GET_PLATFORM_CONFIG_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct GetPlatformConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetPlatformConfigKeys {
    pub config: Pubkey,
}
impl From<GetPlatformConfigAccounts<'_, '_>> for GetPlatformConfigKeys {
    fn from(accounts: GetPlatformConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
        }
    }
}
impl From<GetPlatformConfigKeys> for [AccountMeta; GET_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: GetPlatformConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]> for GetPlatformConfigKeys {
    fn from(pubkeys: [Pubkey; GET_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self { config: pubkeys[0] }
    }
}
impl<'info> From<GetPlatformConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; GET_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetPlatformConfigAccounts<'_, 'info>) -> Self {
        [accounts.config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]>
for GetPlatformConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; GET_PLATFORM_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self { config: &arr[0] }
    }
}
pub const GET_PLATFORM_CONFIG_IX_DISCM: [u8; 8usize] = [
    114, 136, 38, 81, 204, 245, 250, 146,
];
#[derive(Clone, Debug, PartialEq)]
pub struct GetPlatformConfigIxData;
impl GetPlatformConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_PLATFORM_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_PLATFORM_CONFIG_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_platform_config_ix_with_program_id(
    program_id: Pubkey,
    keys: GetPlatformConfigKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GetPlatformConfigIxData.try_to_vec()?,
    })
}
pub fn get_platform_config_ix(
    keys: GetPlatformConfigKeys,
) -> std::io::Result<Instruction> {
    get_platform_config_ix_with_program_id(SCALE_VMM_PROGRAM_ID, keys)
}
pub fn get_platform_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetPlatformConfigAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GetPlatformConfigKeys = accounts.into();
    let ix = get_platform_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_platform_config_invoke(
    accounts: GetPlatformConfigAccounts<'_, '_>,
) -> ProgramResult {
    get_platform_config_invoke_with_program_id(SCALE_VMM_PROGRAM_ID, accounts)
}
pub fn get_platform_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetPlatformConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetPlatformConfigKeys = accounts.into();
    let ix = get_platform_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_platform_config_invoke_signed(
    accounts: GetPlatformConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_platform_config_invoke_signed_with_program_id(
        SCALE_VMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn get_platform_config_verify_account_keys(
    accounts: GetPlatformConfigAccounts<'_, '_>,
    keys: GetPlatformConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.config.key, keys.config)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const QUOTE_BUY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct QuoteBuyAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QuoteBuyKeys {
    pub pair: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub config: Pubkey,
}
impl From<QuoteBuyAccounts<'_, '_>> for QuoteBuyKeys {
    fn from(accounts: QuoteBuyAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            config: *accounts.config.key,
        }
    }
}
impl From<QuoteBuyKeys> for [AccountMeta; QUOTE_BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: QuoteBuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
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
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; QUOTE_BUY_IX_ACCOUNTS_LEN]> for QuoteBuyKeys {
    fn from(pubkeys: [Pubkey; QUOTE_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            mint_a: pubkeys[1],
            mint_b: pubkeys[2],
            config: pubkeys[3],
        }
    }
}
impl<'info> From<QuoteBuyAccounts<'_, 'info>>
for [AccountInfo<'info>; QUOTE_BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: QuoteBuyAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; QUOTE_BUY_IX_ACCOUNTS_LEN]>
for QuoteBuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; QUOTE_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            mint_a: &arr[1],
            mint_b: &arr[2],
            config: &arr[3],
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
    quote_buy_ix_with_program_id(SCALE_VMM_PROGRAM_ID, keys, args)
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
    quote_buy_invoke_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args)
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
    quote_buy_invoke_signed_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn quote_buy_verify_account_keys(
    accounts: QuoteBuyAccounts<'_, '_>,
    keys: QuoteBuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.config.key, keys.config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const QUOTE_SELL_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct QuoteSellAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QuoteSellKeys {
    pub pair: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub config: Pubkey,
}
impl From<QuoteSellAccounts<'_, '_>> for QuoteSellKeys {
    fn from(accounts: QuoteSellAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            config: *accounts.config.key,
        }
    }
}
impl From<QuoteSellKeys> for [AccountMeta; QUOTE_SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: QuoteSellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
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
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; QUOTE_SELL_IX_ACCOUNTS_LEN]> for QuoteSellKeys {
    fn from(pubkeys: [Pubkey; QUOTE_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            mint_a: pubkeys[1],
            mint_b: pubkeys[2],
            config: pubkeys[3],
        }
    }
}
impl<'info> From<QuoteSellAccounts<'_, 'info>>
for [AccountInfo<'info>; QUOTE_SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: QuoteSellAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; QUOTE_SELL_IX_ACCOUNTS_LEN]>
for QuoteSellAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; QUOTE_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            mint_a: &arr[1],
            mint_b: &arr[2],
            config: &arr[3],
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
    quote_sell_ix_with_program_id(SCALE_VMM_PROGRAM_ID, keys, args)
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
    quote_sell_invoke_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args)
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
    quote_sell_invoke_signed_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn quote_sell_verify_account_keys(
    accounts: QuoteSellAccounts<'_, '_>,
    keys: QuoteSellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.config.key, keys.config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const SELL_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct SellAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub user_ta_a: &'me AccountInfo<'info>,
    pub user_ta_b: &'me AccountInfo<'info>,
    pub vault_a: &'me AccountInfo<'info>,
    pub vault_b: &'me AccountInfo<'info>,
    pub platform_fee_ta_a: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub amm_pool: &'me AccountInfo<'info>,
    pub amm_vault_a: &'me AccountInfo<'info>,
    pub amm_vault_b: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellKeys {
    pub pair: Pubkey,
    pub user: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub user_ta_a: Pubkey,
    pub user_ta_b: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub platform_fee_ta_a: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub system_program: Pubkey,
    pub config: Pubkey,
    pub amm_program: Pubkey,
    pub amm_pool: Pubkey,
    pub amm_vault_a: Pubkey,
    pub amm_vault_b: Pubkey,
    pub amm_config: Pubkey,
}
impl From<SellAccounts<'_, '_>> for SellKeys {
    fn from(accounts: SellAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            user: *accounts.user.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            user_ta_a: *accounts.user_ta_a.key,
            user_ta_b: *accounts.user_ta_b.key,
            vault_a: *accounts.vault_a.key,
            vault_b: *accounts.vault_b.key,
            platform_fee_ta_a: *accounts.platform_fee_ta_a.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            system_program: *accounts.system_program.key,
            config: *accounts.config.key,
            amm_program: *accounts.amm_program.key,
            amm_pool: *accounts.amm_pool.key,
            amm_vault_a: *accounts.amm_vault_a.key,
            amm_vault_b: *accounts.amm_vault_b.key,
            amm_config: *accounts.amm_config.key,
        }
    }
}
impl From<SellKeys> for [AccountMeta; SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: SellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.platform_fee_ta_a,
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
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SELL_IX_ACCOUNTS_LEN]> for SellKeys {
    fn from(pubkeys: [Pubkey; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            user: pubkeys[1],
            mint_a: pubkeys[2],
            mint_b: pubkeys[3],
            user_ta_a: pubkeys[4],
            user_ta_b: pubkeys[5],
            vault_a: pubkeys[6],
            vault_b: pubkeys[7],
            platform_fee_ta_a: pubkeys[8],
            token_program_a: pubkeys[9],
            token_program_b: pubkeys[10],
            system_program: pubkeys[11],
            config: pubkeys[12],
            amm_program: pubkeys[13],
            amm_pool: pubkeys[14],
            amm_vault_a: pubkeys[15],
            amm_vault_b: pubkeys[16],
            amm_config: pubkeys[17],
        }
    }
}
impl<'info> From<SellAccounts<'_, 'info>>
for [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.user.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.user_ta_a.clone(),
            accounts.user_ta_b.clone(),
            accounts.vault_a.clone(),
            accounts.vault_b.clone(),
            accounts.platform_fee_ta_a.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.system_program.clone(),
            accounts.config.clone(),
            accounts.amm_program.clone(),
            accounts.amm_pool.clone(),
            accounts.amm_vault_a.clone(),
            accounts.amm_vault_b.clone(),
            accounts.amm_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]>
for SellAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            user: &arr[1],
            mint_a: &arr[2],
            mint_b: &arr[3],
            user_ta_a: &arr[4],
            user_ta_b: &arr[5],
            vault_a: &arr[6],
            vault_b: &arr[7],
            platform_fee_ta_a: &arr[8],
            token_program_a: &arr[9],
            token_program_b: &arr[10],
            system_program: &arr[11],
            config: &arr[12],
            amm_program: &arr[13],
            amm_pool: &arr[14],
            amm_vault_a: &arr[15],
            amm_vault_b: &arr[16],
            amm_config: &arr[17],
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
    sell_ix_with_program_id(SCALE_VMM_PROGRAM_ID, keys, args)
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
    sell_invoke_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args)
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
    sell_invoke_signed_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn sell_verify_account_keys(
    accounts: SellAccounts<'_, '_>,
    keys: SellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.user.key, keys.user),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.user_ta_a.key, keys.user_ta_a),
        (*accounts.user_ta_b.key, keys.user_ta_b),
        (*accounts.vault_a.key, keys.vault_a),
        (*accounts.vault_b.key, keys.vault_b),
        (*accounts.platform_fee_ta_a.key, keys.platform_fee_ta_a),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.config.key, keys.config),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.amm_pool.key, keys.amm_pool),
        (*accounts.amm_vault_a.key, keys.amm_vault_a),
        (*accounts.amm_vault_b.key, keys.amm_vault_b),
        (*accounts.amm_config.key, keys.amm_config),
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
        accounts.pair,
        accounts.user,
        accounts.user_ta_a,
        accounts.user_ta_b,
        accounts.vault_a,
        accounts.vault_b,
        accounts.platform_fee_ta_a,
        accounts.amm_pool,
        accounts.amm_vault_a,
        accounts.amm_vault_b,
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
pub const SET_FEE_BENEFICIARY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeBeneficiaryAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetFeeBeneficiaryKeys {
    pub authority: Pubkey,
    pub config: Pubkey,
}
impl From<SetFeeBeneficiaryAccounts<'_, '_>> for SetFeeBeneficiaryKeys {
    fn from(accounts: SetFeeBeneficiaryAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            config: *accounts.config.key,
        }
    }
}
impl From<SetFeeBeneficiaryKeys> for [AccountMeta; SET_FEE_BENEFICIARY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFeeBeneficiaryKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_FEE_BENEFICIARY_IX_ACCOUNTS_LEN]> for SetFeeBeneficiaryKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_BENEFICIARY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            config: pubkeys[1],
        }
    }
}
impl<'info> From<SetFeeBeneficiaryAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_FEE_BENEFICIARY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetFeeBeneficiaryAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_BENEFICIARY_IX_ACCOUNTS_LEN]>
for SetFeeBeneficiaryAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_FEE_BENEFICIARY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            config: &arr[1],
        }
    }
}
pub const SET_FEE_BENEFICIARY_IX_DISCM: [u8; 8usize] = [
    74, 105, 83, 74, 154, 205, 249, 217,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetFeeBeneficiaryIxArgs {
    pub beneficiary: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeBeneficiaryIxData(pub SetFeeBeneficiaryIxArgs);
impl From<SetFeeBeneficiaryIxArgs> for SetFeeBeneficiaryIxData {
    fn from(args: SetFeeBeneficiaryIxArgs) -> Self {
        Self(args)
    }
}
impl SetFeeBeneficiaryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_FEE_BENEFICIARY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetFeeBeneficiaryIxArgs {
                beneficiary,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_BENEFICIARY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.beneficiary, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_fee_beneficiary_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFeeBeneficiaryKeys,
    args: SetFeeBeneficiaryIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FEE_BENEFICIARY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetFeeBeneficiaryIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fee_beneficiary_ix(
    keys: SetFeeBeneficiaryKeys,
    args: SetFeeBeneficiaryIxArgs,
) -> std::io::Result<Instruction> {
    set_fee_beneficiary_ix_with_program_id(SCALE_VMM_PROGRAM_ID, keys, args)
}
pub fn set_fee_beneficiary_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeBeneficiaryAccounts<'_, '_>,
    args: SetFeeBeneficiaryIxArgs,
) -> ProgramResult {
    let keys: SetFeeBeneficiaryKeys = accounts.into();
    let ix = set_fee_beneficiary_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_fee_beneficiary_invoke(
    accounts: SetFeeBeneficiaryAccounts<'_, '_>,
    args: SetFeeBeneficiaryIxArgs,
) -> ProgramResult {
    set_fee_beneficiary_invoke_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args)
}
pub fn set_fee_beneficiary_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeBeneficiaryAccounts<'_, '_>,
    args: SetFeeBeneficiaryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFeeBeneficiaryKeys = accounts.into();
    let ix = set_fee_beneficiary_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_fee_beneficiary_invoke_signed(
    accounts: SetFeeBeneficiaryAccounts<'_, '_>,
    args: SetFeeBeneficiaryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_fee_beneficiary_invoke_signed_with_program_id(
        SCALE_VMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_fee_beneficiary_verify_account_keys(
    accounts: SetFeeBeneficiaryAccounts<'_, '_>,
    keys: SetFeeBeneficiaryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.config.key, keys.config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_fee_beneficiary_verify_writable_privileges<'me, 'info>(
    accounts: SetFeeBeneficiaryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_fee_beneficiary_verify_signer_privileges<'me, 'info>(
    accounts: SetFeeBeneficiaryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_fee_beneficiary_verify_account_privileges<'me, 'info>(
    accounts: SetFeeBeneficiaryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_fee_beneficiary_verify_writable_privileges(accounts)?;
    set_fee_beneficiary_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_GRADUATION_THRESHOLD_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetGraduationThresholdAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetGraduationThresholdKeys {
    pub authority: Pubkey,
    pub config: Pubkey,
}
impl From<SetGraduationThresholdAccounts<'_, '_>> for SetGraduationThresholdKeys {
    fn from(accounts: SetGraduationThresholdAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            config: *accounts.config.key,
        }
    }
}
impl From<SetGraduationThresholdKeys>
for [AccountMeta; SET_GRADUATION_THRESHOLD_IX_ACCOUNTS_LEN] {
    fn from(keys: SetGraduationThresholdKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_GRADUATION_THRESHOLD_IX_ACCOUNTS_LEN]>
for SetGraduationThresholdKeys {
    fn from(pubkeys: [Pubkey; SET_GRADUATION_THRESHOLD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            config: pubkeys[1],
        }
    }
}
impl<'info> From<SetGraduationThresholdAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_GRADUATION_THRESHOLD_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetGraduationThresholdAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.config.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_GRADUATION_THRESHOLD_IX_ACCOUNTS_LEN]>
for SetGraduationThresholdAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_GRADUATION_THRESHOLD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            config: &arr[1],
        }
    }
}
pub const SET_GRADUATION_THRESHOLD_IX_DISCM: [u8; 8usize] = [
    221, 151, 101, 206, 50, 192, 31, 12,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetGraduationThresholdIxArgs {
    pub threshold: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetGraduationThresholdIxData(pub SetGraduationThresholdIxArgs);
impl From<SetGraduationThresholdIxArgs> for SetGraduationThresholdIxData {
    fn from(args: SetGraduationThresholdIxArgs) -> Self {
        Self(args)
    }
}
impl SetGraduationThresholdIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_GRADUATION_THRESHOLD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetGraduationThresholdIxArgs {
                threshold,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_GRADUATION_THRESHOLD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.threshold, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_graduation_threshold_ix_with_program_id(
    program_id: Pubkey,
    keys: SetGraduationThresholdKeys,
    args: SetGraduationThresholdIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_GRADUATION_THRESHOLD_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetGraduationThresholdIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_graduation_threshold_ix(
    keys: SetGraduationThresholdKeys,
    args: SetGraduationThresholdIxArgs,
) -> std::io::Result<Instruction> {
    set_graduation_threshold_ix_with_program_id(SCALE_VMM_PROGRAM_ID, keys, args)
}
pub fn set_graduation_threshold_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetGraduationThresholdAccounts<'_, '_>,
    args: SetGraduationThresholdIxArgs,
) -> ProgramResult {
    let keys: SetGraduationThresholdKeys = accounts.into();
    let ix = set_graduation_threshold_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_graduation_threshold_invoke(
    accounts: SetGraduationThresholdAccounts<'_, '_>,
    args: SetGraduationThresholdIxArgs,
) -> ProgramResult {
    set_graduation_threshold_invoke_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args)
}
pub fn set_graduation_threshold_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetGraduationThresholdAccounts<'_, '_>,
    args: SetGraduationThresholdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetGraduationThresholdKeys = accounts.into();
    let ix = set_graduation_threshold_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_graduation_threshold_invoke_signed(
    accounts: SetGraduationThresholdAccounts<'_, '_>,
    args: SetGraduationThresholdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_graduation_threshold_invoke_signed_with_program_id(
        SCALE_VMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_graduation_threshold_verify_account_keys(
    accounts: SetGraduationThresholdAccounts<'_, '_>,
    keys: SetGraduationThresholdKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.config.key, keys.config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_graduation_threshold_verify_writable_privileges<'me, 'info>(
    accounts: SetGraduationThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_graduation_threshold_verify_signer_privileges<'me, 'info>(
    accounts: SetGraduationThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_graduation_threshold_verify_account_privileges<'me, 'info>(
    accounts: SetGraduationThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_graduation_threshold_verify_writable_privileges(accounts)?;
    set_graduation_threshold_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PLATFORM_BASE_TOKEN_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetPlatformBaseTokenAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program_data: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPlatformBaseTokenKeys {
    pub authority: Pubkey,
    pub config: Pubkey,
    pub system_program: Pubkey,
    pub program_data: Pubkey,
}
impl From<SetPlatformBaseTokenAccounts<'_, '_>> for SetPlatformBaseTokenKeys {
    fn from(accounts: SetPlatformBaseTokenAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            config: *accounts.config.key,
            system_program: *accounts.system_program.key,
            program_data: *accounts.program_data.key,
        }
    }
}
impl From<SetPlatformBaseTokenKeys>
for [AccountMeta; SET_PLATFORM_BASE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPlatformBaseTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
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
        ]
    }
}
impl From<[Pubkey; SET_PLATFORM_BASE_TOKEN_IX_ACCOUNTS_LEN]>
for SetPlatformBaseTokenKeys {
    fn from(pubkeys: [Pubkey; SET_PLATFORM_BASE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            config: pubkeys[1],
            system_program: pubkeys[2],
            program_data: pubkeys[3],
        }
    }
}
impl<'info> From<SetPlatformBaseTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PLATFORM_BASE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPlatformBaseTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.config.clone(),
            accounts.system_program.clone(),
            accounts.program_data.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PLATFORM_BASE_TOKEN_IX_ACCOUNTS_LEN]>
for SetPlatformBaseTokenAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PLATFORM_BASE_TOKEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            config: &arr[1],
            system_program: &arr[2],
            program_data: &arr[3],
        }
    }
}
pub const SET_PLATFORM_BASE_TOKEN_IX_DISCM: [u8; 8usize] = [
    157, 110, 249, 237, 128, 5, 212, 233,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPlatformBaseTokenIxArgs {
    pub base_token: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPlatformBaseTokenIxData(pub SetPlatformBaseTokenIxArgs);
impl From<SetPlatformBaseTokenIxArgs> for SetPlatformBaseTokenIxData {
    fn from(args: SetPlatformBaseTokenIxArgs) -> Self {
        Self(args)
    }
}
impl SetPlatformBaseTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PLATFORM_BASE_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetPlatformBaseTokenIxArgs {
                base_token,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PLATFORM_BASE_TOKEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_token, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_platform_base_token_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPlatformBaseTokenKeys,
    args: SetPlatformBaseTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PLATFORM_BASE_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPlatformBaseTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_platform_base_token_ix(
    keys: SetPlatformBaseTokenKeys,
    args: SetPlatformBaseTokenIxArgs,
) -> std::io::Result<Instruction> {
    set_platform_base_token_ix_with_program_id(SCALE_VMM_PROGRAM_ID, keys, args)
}
pub fn set_platform_base_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPlatformBaseTokenAccounts<'_, '_>,
    args: SetPlatformBaseTokenIxArgs,
) -> ProgramResult {
    let keys: SetPlatformBaseTokenKeys = accounts.into();
    let ix = set_platform_base_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_platform_base_token_invoke(
    accounts: SetPlatformBaseTokenAccounts<'_, '_>,
    args: SetPlatformBaseTokenIxArgs,
) -> ProgramResult {
    set_platform_base_token_invoke_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args)
}
pub fn set_platform_base_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPlatformBaseTokenAccounts<'_, '_>,
    args: SetPlatformBaseTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPlatformBaseTokenKeys = accounts.into();
    let ix = set_platform_base_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_platform_base_token_invoke_signed(
    accounts: SetPlatformBaseTokenAccounts<'_, '_>,
    args: SetPlatformBaseTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_platform_base_token_invoke_signed_with_program_id(
        SCALE_VMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_platform_base_token_verify_account_keys(
    accounts: SetPlatformBaseTokenAccounts<'_, '_>,
    keys: SetPlatformBaseTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.config.key, keys.config),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program_data.key, keys.program_data),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_platform_base_token_verify_writable_privileges<'me, 'info>(
    accounts: SetPlatformBaseTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_platform_base_token_verify_signer_privileges<'me, 'info>(
    accounts: SetPlatformBaseTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_platform_base_token_verify_account_privileges<'me, 'info>(
    accounts: SetPlatformBaseTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_platform_base_token_verify_writable_privileges(accounts)?;
    set_platform_base_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PLATFORM_FEE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetPlatformFeeAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPlatformFeeKeys {
    pub authority: Pubkey,
    pub config: Pubkey,
}
impl From<SetPlatformFeeAccounts<'_, '_>> for SetPlatformFeeKeys {
    fn from(accounts: SetPlatformFeeAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            config: *accounts.config.key,
        }
    }
}
impl From<SetPlatformFeeKeys> for [AccountMeta; SET_PLATFORM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPlatformFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_PLATFORM_FEE_IX_ACCOUNTS_LEN]> for SetPlatformFeeKeys {
    fn from(pubkeys: [Pubkey; SET_PLATFORM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            config: pubkeys[1],
        }
    }
}
impl<'info> From<SetPlatformFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PLATFORM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPlatformFeeAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PLATFORM_FEE_IX_ACCOUNTS_LEN]>
for SetPlatformFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_PLATFORM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            config: &arr[1],
        }
    }
}
pub const SET_PLATFORM_FEE_IX_DISCM: [u8; 8usize] = [
    19, 70, 111, 182, 156, 58, 208, 203,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPlatformFeeIxArgs {
    pub fee_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPlatformFeeIxData(pub SetPlatformFeeIxArgs);
impl From<SetPlatformFeeIxArgs> for SetPlatformFeeIxData {
    fn from(args: SetPlatformFeeIxArgs) -> Self {
        Self(args)
    }
}
impl SetPlatformFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PLATFORM_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetPlatformFeeIxArgs { fee_bps }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PLATFORM_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_platform_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPlatformFeeKeys,
    args: SetPlatformFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PLATFORM_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPlatformFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_platform_fee_ix(
    keys: SetPlatformFeeKeys,
    args: SetPlatformFeeIxArgs,
) -> std::io::Result<Instruction> {
    set_platform_fee_ix_with_program_id(SCALE_VMM_PROGRAM_ID, keys, args)
}
pub fn set_platform_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPlatformFeeAccounts<'_, '_>,
    args: SetPlatformFeeIxArgs,
) -> ProgramResult {
    let keys: SetPlatformFeeKeys = accounts.into();
    let ix = set_platform_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_platform_fee_invoke(
    accounts: SetPlatformFeeAccounts<'_, '_>,
    args: SetPlatformFeeIxArgs,
) -> ProgramResult {
    set_platform_fee_invoke_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args)
}
pub fn set_platform_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPlatformFeeAccounts<'_, '_>,
    args: SetPlatformFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPlatformFeeKeys = accounts.into();
    let ix = set_platform_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_platform_fee_invoke_signed(
    accounts: SetPlatformFeeAccounts<'_, '_>,
    args: SetPlatformFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_platform_fee_invoke_signed_with_program_id(
        SCALE_VMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_platform_fee_verify_account_keys(
    accounts: SetPlatformFeeAccounts<'_, '_>,
    keys: SetPlatformFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.config.key, keys.config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_platform_fee_verify_writable_privileges<'me, 'info>(
    accounts: SetPlatformFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_platform_fee_verify_signer_privileges<'me, 'info>(
    accounts: SetPlatformFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_platform_fee_verify_account_privileges<'me, 'info>(
    accounts: SetPlatformFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_platform_fee_verify_writable_privileges(accounts)?;
    set_platform_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct TransferAuthorityAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferAuthorityKeys {
    pub authority: Pubkey,
    pub config: Pubkey,
}
impl From<TransferAuthorityAccounts<'_, '_>> for TransferAuthorityKeys {
    fn from(accounts: TransferAuthorityAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            config: *accounts.config.key,
        }
    }
}
impl From<TransferAuthorityKeys> for [AccountMeta; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN]> for TransferAuthorityKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            config: pubkeys[1],
        }
    }
}
impl<'info> From<TransferAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN]>
for TransferAuthorityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            config: &arr[1],
        }
    }
}
pub const TRANSFER_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    48, 169, 76, 72, 229, 180, 55, 161,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferAuthorityIxArgs {
    pub new_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferAuthorityIxData(pub TransferAuthorityIxArgs);
impl From<TransferAuthorityIxArgs> for TransferAuthorityIxData {
    fn from(args: TransferAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl TransferAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TransferAuthorityIxArgs {
                new_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferAuthorityKeys,
    args: TransferAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: TransferAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_authority_ix(
    keys: TransferAuthorityKeys,
    args: TransferAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    transfer_authority_ix_with_program_id(SCALE_VMM_PROGRAM_ID, keys, args)
}
pub fn transfer_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferAuthorityAccounts<'_, '_>,
    args: TransferAuthorityIxArgs,
) -> ProgramResult {
    let keys: TransferAuthorityKeys = accounts.into();
    let ix = transfer_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_authority_invoke(
    accounts: TransferAuthorityAccounts<'_, '_>,
    args: TransferAuthorityIxArgs,
) -> ProgramResult {
    transfer_authority_invoke_with_program_id(SCALE_VMM_PROGRAM_ID, accounts, args)
}
pub fn transfer_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferAuthorityAccounts<'_, '_>,
    args: TransferAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferAuthorityKeys = accounts.into();
    let ix = transfer_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_authority_invoke_signed(
    accounts: TransferAuthorityAccounts<'_, '_>,
    args: TransferAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_authority_invoke_signed_with_program_id(
        SCALE_VMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn transfer_authority_verify_account_keys(
    accounts: TransferAuthorityAccounts<'_, '_>,
    keys: TransferAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.config.key, keys.config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_authority_verify_writable_privileges<'me, 'info>(
    accounts: TransferAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_authority_verify_signer_privileges<'me, 'info>(
    accounts: TransferAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_authority_verify_account_privileges<'me, 'info>(
    accounts: TransferAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_authority_verify_writable_privileges(accounts)?;
    transfer_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
