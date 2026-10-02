use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::Instruction;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum OnedexProgramIx {
    AdminUpdateAdminAuth,
    AdminUpdateFeeSetting(AdminUpdateFeeSettingIxArgs),
    AdminUpdateSwapFeeTier(AdminUpdateSwapFeeTierIxArgs),
    AdminWithdrawToken(AdminWithdrawTokenIxArgs),
    CreateMetadataState(CreateMetadataStateIxArgs),
    CreatePoolState(CreatePoolStateIxArgs),
    ExitPool(ExitPoolIxArgs),
    JoinPool(JoinPoolIxArgs),
    SwapExactAmountIn(SwapExactAmountInIxArgs),
    SwapExactAmountOut(SwapExactAmountOutIxArgs),
}
impl OnedexProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ADMIN_UPDATE_ADMIN_AUTH_IX_DISCM) {
            return Ok(Self::AdminUpdateAdminAuth);
        }
        if buf.starts_with(&ADMIN_UPDATE_FEE_SETTING_IX_DISCM) {
            let mut reader = &buf[ADMIN_UPDATE_FEE_SETTING_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AdminUpdateFeeSetting(AdminUpdateFeeSettingIxArgs {
                    field_0,
                    field_1,
                    field_2,
                }),
            );
        }
        if buf.starts_with(&ADMIN_UPDATE_SWAP_FEE_TIER_IX_DISCM) {
            let mut reader = &buf[ADMIN_UPDATE_SWAP_FEE_TIER_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AdminUpdateSwapFeeTier(AdminUpdateSwapFeeTierIxArgs {
                    field_0,
                    field_1,
                }),
            );
        }
        if buf.starts_with(&ADMIN_WITHDRAW_TOKEN_IX_DISCM) {
            let mut reader = &buf[ADMIN_WITHDRAW_TOKEN_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AdminWithdrawToken(AdminWithdrawTokenIxArgs {
                    field_0,
                }),
            );
        }
        if buf.starts_with(&CREATE_METADATA_STATE_IX_DISCM) {
            let mut reader = &buf[CREATE_METADATA_STATE_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateMetadataState(CreateMetadataStateIxArgs {
                    field_0,
                    field_1,
                    field_2,
                }),
            );
        }
        if buf.starts_with(&CREATE_POOL_STATE_IX_DISCM) {
            let mut reader = &buf[CREATE_POOL_STATE_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u128 = crate::borsh_de_or_default(&mut reader)?;
            let field_3: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreatePoolState(CreatePoolStateIxArgs {
                    field_0,
                    field_1,
                    field_2,
                    field_3,
                }),
            );
        }
        if buf.starts_with(&EXIT_POOL_IX_DISCM) {
            let mut reader = &buf[EXIT_POOL_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ExitPool(ExitPoolIxArgs {
                    field_0,
                    field_1,
                    field_2,
                }),
            );
        }
        if buf.starts_with(&JOIN_POOL_IX_DISCM) {
            let mut reader = &buf[JOIN_POOL_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::JoinPool(JoinPoolIxArgs {
                    field_0,
                    field_1,
                    field_2,
                }),
            );
        }
        if buf.starts_with(&SWAP_EXACT_AMOUNT_IN_IX_DISCM) {
            let mut reader = &buf[SWAP_EXACT_AMOUNT_IN_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapExactAmountIn(SwapExactAmountInIxArgs {
                    field_0,
                    field_1,
                }),
            );
        }
        if buf.starts_with(&SWAP_EXACT_AMOUNT_OUT_IX_DISCM) {
            let mut reader = &buf[SWAP_EXACT_AMOUNT_OUT_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapExactAmountOut(SwapExactAmountOutIxArgs {
                    field_0,
                    field_1,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AdminUpdateAdminAuth => {
                writer.write_all(&ADMIN_UPDATE_ADMIN_AUTH_IX_DISCM)
            }
            Self::AdminUpdateFeeSetting(args) => {
                writer.write_all(&ADMIN_UPDATE_FEE_SETTING_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                Ok(())
            }
            Self::AdminUpdateSwapFeeTier(args) => {
                writer.write_all(&ADMIN_UPDATE_SWAP_FEE_TIER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                Ok(())
            }
            Self::AdminWithdrawToken(args) => {
                writer.write_all(&ADMIN_WITHDRAW_TOKEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                Ok(())
            }
            Self::CreateMetadataState(args) => {
                writer.write_all(&CREATE_METADATA_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                Ok(())
            }
            Self::CreatePoolState(args) => {
                writer.write_all(&CREATE_POOL_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_3, &mut writer)?;
                Ok(())
            }
            Self::ExitPool(args) => {
                writer.write_all(&EXIT_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                Ok(())
            }
            Self::JoinPool(args) => {
                writer.write_all(&JOIN_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                Ok(())
            }
            Self::SwapExactAmountIn(args) => {
                writer.write_all(&SWAP_EXACT_AMOUNT_IN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                Ok(())
            }
            Self::SwapExactAmountOut(args) => {
                writer.write_all(&SWAP_EXACT_AMOUNT_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
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
pub const ADMIN_UPDATE_ADMIN_AUTH_IX_DISCM: [u8; 8usize] = [
    30, 100, 56, 131, 184, 138, 52, 136,
];
#[derive(Clone, Debug, PartialEq)]
pub struct AdminUpdateAdminAuthIxData;
impl AdminUpdateAdminAuthIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_UPDATE_ADMIN_AUTH_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_UPDATE_ADMIN_AUTH_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_update_admin_auth_ix_with_program_id(
    program_id: Pubkey,
) -> std::io::Result<Instruction> {
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: AdminUpdateAdminAuthIxData.try_to_vec()?,
    })
}
pub fn admin_update_admin_auth_ix() -> std::io::Result<Instruction> {
    admin_update_admin_auth_ix_with_program_id(ONEDEX_PROGRAM_ID)
}
pub fn admin_update_admin_auth_invoke_with_program_id(
    program_id: Pubkey,
) -> ProgramResult {
    let ix = admin_update_admin_auth_ix_with_program_id(program_id)?;
    invoke(&ix, &[])
}
pub fn admin_update_admin_auth_invoke() -> ProgramResult {
    admin_update_admin_auth_invoke_with_program_id(ONEDEX_PROGRAM_ID)
}
pub fn admin_update_admin_auth_invoke_signed_with_program_id(
    program_id: Pubkey,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = admin_update_admin_auth_ix_with_program_id(program_id)?;
    invoke_signed(&ix, &[], seeds)
}
pub fn admin_update_admin_auth_invoke_signed(seeds: &[&[&[u8]]]) -> ProgramResult {
    admin_update_admin_auth_invoke_signed_with_program_id(ONEDEX_PROGRAM_ID, seeds)
}
pub const ADMIN_UPDATE_FEE_SETTING_IX_DISCM: [u8; 8usize] = [
    53, 21, 69, 171, 237, 25, 215, 172,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminUpdateFeeSettingIxArgs {
    pub field_0: u64,
    pub field_1: u64,
    pub field_2: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminUpdateFeeSettingIxData(pub AdminUpdateFeeSettingIxArgs);
impl From<AdminUpdateFeeSettingIxArgs> for AdminUpdateFeeSettingIxData {
    fn from(args: AdminUpdateFeeSettingIxArgs) -> Self {
        Self(args)
    }
}
impl AdminUpdateFeeSettingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_UPDATE_FEE_SETTING_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AdminUpdateFeeSettingIxArgs {
                field_0,
                field_1,
                field_2,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_UPDATE_FEE_SETTING_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_update_fee_setting_ix_with_program_id(
    program_id: Pubkey,
    args: AdminUpdateFeeSettingIxArgs,
) -> std::io::Result<Instruction> {
    let data: AdminUpdateFeeSettingIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: data.try_to_vec()?,
    })
}
pub fn admin_update_fee_setting_ix(
    args: AdminUpdateFeeSettingIxArgs,
) -> std::io::Result<Instruction> {
    admin_update_fee_setting_ix_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn admin_update_fee_setting_invoke_with_program_id(
    program_id: Pubkey,
    args: AdminUpdateFeeSettingIxArgs,
) -> ProgramResult {
    let ix = admin_update_fee_setting_ix_with_program_id(program_id, args)?;
    invoke(&ix, &[])
}
pub fn admin_update_fee_setting_invoke(
    args: AdminUpdateFeeSettingIxArgs,
) -> ProgramResult {
    admin_update_fee_setting_invoke_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn admin_update_fee_setting_invoke_signed_with_program_id(
    program_id: Pubkey,
    args: AdminUpdateFeeSettingIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = admin_update_fee_setting_ix_with_program_id(program_id, args)?;
    invoke_signed(&ix, &[], seeds)
}
pub fn admin_update_fee_setting_invoke_signed(
    args: AdminUpdateFeeSettingIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_update_fee_setting_invoke_signed_with_program_id(
        ONEDEX_PROGRAM_ID,
        args,
        seeds,
    )
}
pub const ADMIN_UPDATE_SWAP_FEE_TIER_IX_DISCM: [u8; 8usize] = [
    185, 254, 220, 30, 30, 157, 253, 34,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminUpdateSwapFeeTierIxArgs {
    pub field_0: u64,
    pub field_1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminUpdateSwapFeeTierIxData(pub AdminUpdateSwapFeeTierIxArgs);
impl From<AdminUpdateSwapFeeTierIxArgs> for AdminUpdateSwapFeeTierIxData {
    fn from(args: AdminUpdateSwapFeeTierIxArgs) -> Self {
        Self(args)
    }
}
impl AdminUpdateSwapFeeTierIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_UPDATE_SWAP_FEE_TIER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AdminUpdateSwapFeeTierIxArgs {
                field_0,
                field_1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_UPDATE_SWAP_FEE_TIER_IX_DISCM)?;
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
pub fn admin_update_swap_fee_tier_ix_with_program_id(
    program_id: Pubkey,
    args: AdminUpdateSwapFeeTierIxArgs,
) -> std::io::Result<Instruction> {
    let data: AdminUpdateSwapFeeTierIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: data.try_to_vec()?,
    })
}
pub fn admin_update_swap_fee_tier_ix(
    args: AdminUpdateSwapFeeTierIxArgs,
) -> std::io::Result<Instruction> {
    admin_update_swap_fee_tier_ix_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn admin_update_swap_fee_tier_invoke_with_program_id(
    program_id: Pubkey,
    args: AdminUpdateSwapFeeTierIxArgs,
) -> ProgramResult {
    let ix = admin_update_swap_fee_tier_ix_with_program_id(program_id, args)?;
    invoke(&ix, &[])
}
pub fn admin_update_swap_fee_tier_invoke(
    args: AdminUpdateSwapFeeTierIxArgs,
) -> ProgramResult {
    admin_update_swap_fee_tier_invoke_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn admin_update_swap_fee_tier_invoke_signed_with_program_id(
    program_id: Pubkey,
    args: AdminUpdateSwapFeeTierIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = admin_update_swap_fee_tier_ix_with_program_id(program_id, args)?;
    invoke_signed(&ix, &[], seeds)
}
pub fn admin_update_swap_fee_tier_invoke_signed(
    args: AdminUpdateSwapFeeTierIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_update_swap_fee_tier_invoke_signed_with_program_id(
        ONEDEX_PROGRAM_ID,
        args,
        seeds,
    )
}
pub const ADMIN_WITHDRAW_TOKEN_IX_DISCM: [u8; 8usize] = [
    121, 185, 235, 148, 189, 56, 49, 11,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminWithdrawTokenIxArgs {
    pub field_0: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminWithdrawTokenIxData(pub AdminWithdrawTokenIxArgs);
impl From<AdminWithdrawTokenIxArgs> for AdminWithdrawTokenIxData {
    fn from(args: AdminWithdrawTokenIxArgs) -> Self {
        Self(args)
    }
}
impl AdminWithdrawTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_WITHDRAW_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AdminWithdrawTokenIxArgs {
                field_0,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_WITHDRAW_TOKEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_withdraw_token_ix_with_program_id(
    program_id: Pubkey,
    args: AdminWithdrawTokenIxArgs,
) -> std::io::Result<Instruction> {
    let data: AdminWithdrawTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: data.try_to_vec()?,
    })
}
pub fn admin_withdraw_token_ix(
    args: AdminWithdrawTokenIxArgs,
) -> std::io::Result<Instruction> {
    admin_withdraw_token_ix_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn admin_withdraw_token_invoke_with_program_id(
    program_id: Pubkey,
    args: AdminWithdrawTokenIxArgs,
) -> ProgramResult {
    let ix = admin_withdraw_token_ix_with_program_id(program_id, args)?;
    invoke(&ix, &[])
}
pub fn admin_withdraw_token_invoke(args: AdminWithdrawTokenIxArgs) -> ProgramResult {
    admin_withdraw_token_invoke_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn admin_withdraw_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    args: AdminWithdrawTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = admin_withdraw_token_ix_with_program_id(program_id, args)?;
    invoke_signed(&ix, &[], seeds)
}
pub fn admin_withdraw_token_invoke_signed(
    args: AdminWithdrawTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_withdraw_token_invoke_signed_with_program_id(ONEDEX_PROGRAM_ID, args, seeds)
}
pub const CREATE_METADATA_STATE_IX_DISCM: [u8; 8usize] = [
    158, 208, 225, 158, 166, 29, 134, 196,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMetadataStateIxArgs {
    pub field_0: u64,
    pub field_1: u64,
    pub field_2: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMetadataStateIxData(pub CreateMetadataStateIxArgs);
impl From<CreateMetadataStateIxArgs> for CreateMetadataStateIxData {
    fn from(args: CreateMetadataStateIxArgs) -> Self {
        Self(args)
    }
}
impl CreateMetadataStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_METADATA_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateMetadataStateIxArgs {
                field_0,
                field_1,
                field_2,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_METADATA_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_metadata_state_ix_with_program_id(
    program_id: Pubkey,
    args: CreateMetadataStateIxArgs,
) -> std::io::Result<Instruction> {
    let data: CreateMetadataStateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: data.try_to_vec()?,
    })
}
pub fn create_metadata_state_ix(
    args: CreateMetadataStateIxArgs,
) -> std::io::Result<Instruction> {
    create_metadata_state_ix_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn create_metadata_state_invoke_with_program_id(
    program_id: Pubkey,
    args: CreateMetadataStateIxArgs,
) -> ProgramResult {
    let ix = create_metadata_state_ix_with_program_id(program_id, args)?;
    invoke(&ix, &[])
}
pub fn create_metadata_state_invoke(args: CreateMetadataStateIxArgs) -> ProgramResult {
    create_metadata_state_invoke_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn create_metadata_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    args: CreateMetadataStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_metadata_state_ix_with_program_id(program_id, args)?;
    invoke_signed(&ix, &[], seeds)
}
pub fn create_metadata_state_invoke_signed(
    args: CreateMetadataStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_metadata_state_invoke_signed_with_program_id(ONEDEX_PROGRAM_ID, args, seeds)
}
pub const CREATE_POOL_STATE_IX_DISCM: [u8; 8usize] = [
    155, 42, 146, 16, 193, 134, 186, 237,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePoolStateIxArgs {
    pub field_0: u64,
    pub field_1: u64,
    pub field_2: u128,
    pub field_3: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePoolStateIxData(pub CreatePoolStateIxArgs);
impl From<CreatePoolStateIxArgs> for CreatePoolStateIxData {
    fn from(args: CreatePoolStateIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePoolStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POOL_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u128 = crate::borsh_de_or_default(&mut reader)?;
        let field_3: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreatePoolStateIxArgs {
                field_0,
                field_1,
                field_2,
                field_3,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POOL_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_3, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_pool_state_ix_with_program_id(
    program_id: Pubkey,
    args: CreatePoolStateIxArgs,
) -> std::io::Result<Instruction> {
    let data: CreatePoolStateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: data.try_to_vec()?,
    })
}
pub fn create_pool_state_ix(
    args: CreatePoolStateIxArgs,
) -> std::io::Result<Instruction> {
    create_pool_state_ix_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn create_pool_state_invoke_with_program_id(
    program_id: Pubkey,
    args: CreatePoolStateIxArgs,
) -> ProgramResult {
    let ix = create_pool_state_ix_with_program_id(program_id, args)?;
    invoke(&ix, &[])
}
pub fn create_pool_state_invoke(args: CreatePoolStateIxArgs) -> ProgramResult {
    create_pool_state_invoke_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn create_pool_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    args: CreatePoolStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_pool_state_ix_with_program_id(program_id, args)?;
    invoke_signed(&ix, &[], seeds)
}
pub fn create_pool_state_invoke_signed(
    args: CreatePoolStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_pool_state_invoke_signed_with_program_id(ONEDEX_PROGRAM_ID, args, seeds)
}
pub const EXIT_POOL_IX_DISCM: [u8; 8usize] = [6, 156, 87, 187, 195, 101, 222, 143];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExitPoolIxArgs {
    pub field_0: u64,
    pub field_1: u64,
    pub field_2: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExitPoolIxData(pub ExitPoolIxArgs);
impl From<ExitPoolIxArgs> for ExitPoolIxData {
    fn from(args: ExitPoolIxArgs) -> Self {
        Self(args)
    }
}
impl ExitPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXIT_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExitPoolIxArgs {
                field_0,
                field_1,
                field_2,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXIT_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn exit_pool_ix_with_program_id(
    program_id: Pubkey,
    args: ExitPoolIxArgs,
) -> std::io::Result<Instruction> {
    let data: ExitPoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: data.try_to_vec()?,
    })
}
pub fn exit_pool_ix(args: ExitPoolIxArgs) -> std::io::Result<Instruction> {
    exit_pool_ix_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn exit_pool_invoke_with_program_id(
    program_id: Pubkey,
    args: ExitPoolIxArgs,
) -> ProgramResult {
    let ix = exit_pool_ix_with_program_id(program_id, args)?;
    invoke(&ix, &[])
}
pub fn exit_pool_invoke(args: ExitPoolIxArgs) -> ProgramResult {
    exit_pool_invoke_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn exit_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    args: ExitPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = exit_pool_ix_with_program_id(program_id, args)?;
    invoke_signed(&ix, &[], seeds)
}
pub fn exit_pool_invoke_signed(
    args: ExitPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    exit_pool_invoke_signed_with_program_id(ONEDEX_PROGRAM_ID, args, seeds)
}
pub const JOIN_POOL_IX_DISCM: [u8; 8usize] = [14, 65, 62, 16, 116, 17, 195, 107];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JoinPoolIxArgs {
    pub field_0: u64,
    pub field_1: u64,
    pub field_2: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct JoinPoolIxData(pub JoinPoolIxArgs);
impl From<JoinPoolIxArgs> for JoinPoolIxData {
    fn from(args: JoinPoolIxArgs) -> Self {
        Self(args)
    }
}
impl JoinPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != JOIN_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(JoinPoolIxArgs {
                field_0,
                field_1,
                field_2,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&JOIN_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn join_pool_ix_with_program_id(
    program_id: Pubkey,
    args: JoinPoolIxArgs,
) -> std::io::Result<Instruction> {
    let data: JoinPoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: data.try_to_vec()?,
    })
}
pub fn join_pool_ix(args: JoinPoolIxArgs) -> std::io::Result<Instruction> {
    join_pool_ix_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn join_pool_invoke_with_program_id(
    program_id: Pubkey,
    args: JoinPoolIxArgs,
) -> ProgramResult {
    let ix = join_pool_ix_with_program_id(program_id, args)?;
    invoke(&ix, &[])
}
pub fn join_pool_invoke(args: JoinPoolIxArgs) -> ProgramResult {
    join_pool_invoke_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn join_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    args: JoinPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = join_pool_ix_with_program_id(program_id, args)?;
    invoke_signed(&ix, &[], seeds)
}
pub fn join_pool_invoke_signed(
    args: JoinPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    join_pool_invoke_signed_with_program_id(ONEDEX_PROGRAM_ID, args, seeds)
}
pub const SWAP_EXACT_AMOUNT_IN_IX_DISCM: [u8; 8usize] = [
    8, 151, 245, 76, 172, 203, 144, 39,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExactAmountInIxArgs {
    pub field_0: u64,
    pub field_1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactAmountInIxData(pub SwapExactAmountInIxArgs);
impl From<SwapExactAmountInIxArgs> for SwapExactAmountInIxData {
    fn from(args: SwapExactAmountInIxArgs) -> Self {
        Self(args)
    }
}
impl SwapExactAmountInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_AMOUNT_IN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapExactAmountInIxArgs {
                field_0,
                field_1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_AMOUNT_IN_IX_DISCM)?;
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
pub fn swap_exact_amount_in_ix_with_program_id(
    program_id: Pubkey,
    args: SwapExactAmountInIxArgs,
) -> std::io::Result<Instruction> {
    let data: SwapExactAmountInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_amount_in_ix(
    args: SwapExactAmountInIxArgs,
) -> std::io::Result<Instruction> {
    swap_exact_amount_in_ix_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn swap_exact_amount_in_invoke_with_program_id(
    program_id: Pubkey,
    args: SwapExactAmountInIxArgs,
) -> ProgramResult {
    let ix = swap_exact_amount_in_ix_with_program_id(program_id, args)?;
    invoke(&ix, &[])
}
pub fn swap_exact_amount_in_invoke(args: SwapExactAmountInIxArgs) -> ProgramResult {
    swap_exact_amount_in_invoke_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn swap_exact_amount_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    args: SwapExactAmountInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_exact_amount_in_ix_with_program_id(program_id, args)?;
    invoke_signed(&ix, &[], seeds)
}
pub fn swap_exact_amount_in_invoke_signed(
    args: SwapExactAmountInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exact_amount_in_invoke_signed_with_program_id(ONEDEX_PROGRAM_ID, args, seeds)
}
pub const SWAP_EXACT_AMOUNT_OUT_IX_DISCM: [u8; 8usize] = [
    250, 23, 49, 134, 38, 84, 219, 125,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExactAmountOutIxArgs {
    pub field_0: u64,
    pub field_1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactAmountOutIxData(pub SwapExactAmountOutIxArgs);
impl From<SwapExactAmountOutIxArgs> for SwapExactAmountOutIxData {
    fn from(args: SwapExactAmountOutIxArgs) -> Self {
        Self(args)
    }
}
impl SwapExactAmountOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_AMOUNT_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapExactAmountOutIxArgs {
                field_0,
                field_1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_AMOUNT_OUT_IX_DISCM)?;
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
pub fn swap_exact_amount_out_ix_with_program_id(
    program_id: Pubkey,
    args: SwapExactAmountOutIxArgs,
) -> std::io::Result<Instruction> {
    let data: SwapExactAmountOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_amount_out_ix(
    args: SwapExactAmountOutIxArgs,
) -> std::io::Result<Instruction> {
    swap_exact_amount_out_ix_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn swap_exact_amount_out_invoke_with_program_id(
    program_id: Pubkey,
    args: SwapExactAmountOutIxArgs,
) -> ProgramResult {
    let ix = swap_exact_amount_out_ix_with_program_id(program_id, args)?;
    invoke(&ix, &[])
}
pub fn swap_exact_amount_out_invoke(args: SwapExactAmountOutIxArgs) -> ProgramResult {
    swap_exact_amount_out_invoke_with_program_id(ONEDEX_PROGRAM_ID, args)
}
pub fn swap_exact_amount_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    args: SwapExactAmountOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_exact_amount_out_ix_with_program_id(program_id, args)?;
    invoke_signed(&ix, &[], seeds)
}
pub fn swap_exact_amount_out_invoke_signed(
    args: SwapExactAmountOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exact_amount_out_invoke_signed_with_program_id(ONEDEX_PROGRAM_ID, args, seeds)
}
