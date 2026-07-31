use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum OndoGmProgramIx {
    AddToWhitelist(AddToWhitelistIxArgs),
    BatchCloseAttestationAccounts,
    BurnUsdon(BurnUsdonIxArgs),
    CloseAttestationAccount(CloseAttestationAccountIxArgs),
    EnableOraclePrice(EnableOraclePriceIxArgs),
    GrantGmtokenFactoryRole(GrantGmtokenFactoryRoleIxArgs),
    GrantGmtokenManagerRole(GrantGmtokenManagerRoleIxArgs),
    GrantGmtokenRole(GrantGmtokenRoleIxArgs),
    GrantRole(GrantRoleIxArgs),
    GrantSanityConfigurerRole(GrantSanityConfigurerRoleIxArgs),
    GrantSanitySetterRole(GrantSanitySetterRoleIxArgs),
    GrantUsdonRole(GrantUsdonRoleIxArgs),
    InitMint(InitMintIxArgs),
    InitMintDelegate(InitMintDelegateIxArgs),
    InitializeGmtokenManager(InitializeGmtokenManagerIxArgs),
    InitializeSanityCheck(InitializeSanityCheckIxArgs),
    InitializeTokenLimit(InitializeTokenLimitIxArgs),
    InitializeUsdonManager(InitializeUsdonManagerIxArgs),
    InitializeUser(InitializeUserIxArgs),
    MintGm(MintGmIxArgs),
    MintUsdon(MintUsdonIxArgs),
    MintWithUsdc(MintWithUsdcIxArgs),
    MintWithUsdon(MintWithUsdonIxArgs),
    PauseGlobalMinting,
    PauseGlobalMintingAdmin,
    PauseGlobalRedemption,
    PauseGlobalRedemptionAdmin,
    PauseToken,
    PauseTokenFactory,
    PauseTokenFactoryAdmin,
    PauseTokenMinting,
    PauseTokenMintingAdmin,
    PauseTokenRedemption,
    PauseTokenRedemptionAdmin,
    RedeemForUsdc(RedeemForUsdcIxArgs),
    RedeemForUsdon(RedeemForUsdonIxArgs),
    RemoveFromWhitelist(RemoveFromWhitelistIxArgs),
    ResumeGlobalMinting,
    ResumeGlobalRedemption,
    ResumeToken,
    ResumeTokenFactory,
    ResumeTokenMinting,
    ResumeTokenRedemption,
    RetrieveTokens(RetrieveTokensIxArgs),
    RevokeGmtokenFactoryRole,
    RevokeGmtokenManagerRole,
    RevokeGmtokenRole,
    RevokeRole(RevokeRoleIxArgs),
    RevokeSanityConfigurerRole,
    RevokeSanitySetterRole,
    RevokeUsdonRole,
    SetAllowedDeviationBps(SetAllowedDeviationBpsIxArgs),
    SetAttestationSignerSecp(SetAttestationSignerSecpIxArgs),
    SetLastPrice(SetLastPriceIxArgs),
    SetMaxTimeDelay(SetMaxTimeDelayIxArgs),
    SetOndoUserLimits(SetOndoUserLimitsIxArgs),
    SetOraclePriceMaxAge(SetOraclePriceMaxAgeIxArgs),
    SetTokenLimit(SetTokenLimitIxArgs),
    SetUsdcPriceUpdateAddress(SetUsdcPriceUpdateAddressIxArgs),
    UpdateScaledUiMultiplier(UpdateScaledUiMultiplierIxArgs),
    UpdateTokenMetadata(UpdateTokenMetadataIxArgs),
}
impl OndoGmProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ADD_TO_WHITELIST_IX_DISCM) {
            let mut reader = &buf[ADD_TO_WHITELIST_IX_DISCM.len()..];
            let address_to_whitelist: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddToWhitelist(AddToWhitelistIxArgs {
                    address_to_whitelist,
                }),
            );
        }
        if buf.starts_with(&BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_DISCM) {
            return Ok(Self::BatchCloseAttestationAccounts);
        }
        if buf.starts_with(&BURN_USDON_IX_DISCM) {
            let mut reader = &buf[BURN_USDON_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::BurnUsdon(BurnUsdonIxArgs { amount }));
        }
        if buf.starts_with(&CLOSE_ATTESTATION_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[CLOSE_ATTESTATION_ACCOUNT_IX_DISCM.len()..];
            let _attestation_id: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CloseAttestationAccount(CloseAttestationAccountIxArgs {
                    _attestation_id,
                }),
            );
        }
        if buf.starts_with(&ENABLE_ORACLE_PRICE_IX_DISCM) {
            let mut reader = &buf[ENABLE_ORACLE_PRICE_IX_DISCM.len()..];
            let is_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::EnableOraclePrice(EnableOraclePriceIxArgs {
                    is_enabled,
                }),
            );
        }
        if buf.starts_with(&GRANT_GMTOKEN_FACTORY_ROLE_IX_DISCM) {
            let mut reader = &buf[GRANT_GMTOKEN_FACTORY_ROLE_IX_DISCM.len()..];
            let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
            let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::GrantGmtokenFactoryRole(GrantGmtokenFactoryRoleIxArgs {
                    role,
                    user,
                }),
            );
        }
        if buf.starts_with(&GRANT_GMTOKEN_MANAGER_ROLE_IX_DISCM) {
            let mut reader = &buf[GRANT_GMTOKEN_MANAGER_ROLE_IX_DISCM.len()..];
            let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
            let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::GrantGmtokenManagerRole(GrantGmtokenManagerRoleIxArgs {
                    role,
                    user,
                }),
            );
        }
        if buf.starts_with(&GRANT_GMTOKEN_ROLE_IX_DISCM) {
            let mut reader = &buf[GRANT_GMTOKEN_ROLE_IX_DISCM.len()..];
            let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
            let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::GrantGmtokenRole(GrantGmtokenRoleIxArgs {
                    role,
                    user,
                }),
            );
        }
        if buf.starts_with(&GRANT_ROLE_IX_DISCM) {
            let mut reader = &buf[GRANT_ROLE_IX_DISCM.len()..];
            let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
            let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::GrantRole(GrantRoleIxArgs { role, user }));
        }
        if buf.starts_with(&GRANT_SANITY_CONFIGURER_ROLE_IX_DISCM) {
            let mut reader = &buf[GRANT_SANITY_CONFIGURER_ROLE_IX_DISCM.len()..];
            let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
            let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::GrantSanityConfigurerRole(GrantSanityConfigurerRoleIxArgs {
                    role,
                    user,
                }),
            );
        }
        if buf.starts_with(&GRANT_SANITY_SETTER_ROLE_IX_DISCM) {
            let mut reader = &buf[GRANT_SANITY_SETTER_ROLE_IX_DISCM.len()..];
            let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
            let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::GrantSanitySetterRole(GrantSanitySetterRoleIxArgs {
                    role,
                    user,
                }),
            );
        }
        if buf.starts_with(&GRANT_USDON_ROLE_IX_DISCM) {
            let mut reader = &buf[GRANT_USDON_ROLE_IX_DISCM.len()..];
            let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
            let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::GrantUsdonRole(GrantUsdonRoleIxArgs { role, user }));
        }
        if buf.starts_with(&INIT_MINT_IX_DISCM) {
            let mut reader = &buf[INIT_MINT_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            let freeze_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitMint(InitMintIxArgs {
                    name,
                    symbol,
                    uri,
                    freeze_authority,
                }),
            );
        }
        if buf.starts_with(&INIT_MINT_DELEGATE_IX_DISCM) {
            let mut reader = &buf[INIT_MINT_DELEGATE_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            let freeze_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitMintDelegate(InitMintDelegateIxArgs {
                    name,
                    symbol,
                    uri,
                    freeze_authority,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_GMTOKEN_MANAGER_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_GMTOKEN_MANAGER_IX_DISCM.len()..];
            let factory_paused: bool = crate::borsh_de_or_default(&mut reader)?;
            let redemptions_paused: bool = crate::borsh_de_or_default(&mut reader)?;
            let minting_paused: bool = crate::borsh_de_or_default(&mut reader)?;
            let attestation_signer_secp: [u8; 20] = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let trading_hours_offset: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeGmtokenManager(InitializeGmtokenManagerIxArgs {
                    factory_paused,
                    redemptions_paused,
                    minting_paused,
                    attestation_signer_secp,
                    trading_hours_offset,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_SANITY_CHECK_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_SANITY_CHECK_IX_DISCM.len()..];
            let last_price: u64 = crate::borsh_de_or_default(&mut reader)?;
            let allowed_deviation_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_time_delay: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeSanityCheck(InitializeSanityCheckIxArgs {
                    last_price,
                    allowed_deviation_bps,
                    max_time_delay,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_TOKEN_LIMIT_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_TOKEN_LIMIT_IX_DISCM.len()..];
            let rate_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let limit_window: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let default_user_rate_limit: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let default_limit_window: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::InitializeTokenLimit(InitializeTokenLimitIxArgs {
                    rate_limit,
                    limit_window,
                    default_user_rate_limit,
                    default_limit_window,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_USDON_MANAGER_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_USDON_MANAGER_IX_DISCM.len()..];
            let oracle_price_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
            let oracle_price_max_age: u64 = crate::borsh_de_or_default(&mut reader)?;
            let usdc_price_update_address: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::InitializeUsdonManager(InitializeUsdonManagerIxArgs {
                    oracle_price_enabled,
                    oracle_price_max_age,
                    usdc_price_update_address,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_USER_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_USER_IX_DISCM.len()..];
            let rate_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let limit_window: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeUser(InitializeUserIxArgs {
                    rate_limit,
                    limit_window,
                }),
            );
        }
        if buf.starts_with(&MINT_GM_IX_DISCM) {
            let mut reader = &buf[MINT_GM_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::MintGm(MintGmIxArgs { amount }));
        }
        if buf.starts_with(&MINT_USDON_IX_DISCM) {
            let mut reader = &buf[MINT_USDON_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::MintUsdon(MintUsdonIxArgs { amount }));
        }
        if buf.starts_with(&MINT_WITH_USDC_IX_DISCM) {
            let mut reader = &buf[MINT_WITH_USDC_IX_DISCM.len()..];
            let attestation_id: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
            let price: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let expiration: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::MintWithUsdc(MintWithUsdcIxArgs {
                    attestation_id,
                    price,
                    amount,
                    expiration,
                }),
            );
        }
        if buf.starts_with(&MINT_WITH_USDON_IX_DISCM) {
            let mut reader = &buf[MINT_WITH_USDON_IX_DISCM.len()..];
            let attestation_id: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
            let price: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let expiration: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::MintWithUsdon(MintWithUsdonIxArgs {
                    attestation_id,
                    price,
                    amount,
                    expiration,
                }),
            );
        }
        if buf.starts_with(&PAUSE_GLOBAL_MINTING_IX_DISCM) {
            return Ok(Self::PauseGlobalMinting);
        }
        if buf.starts_with(&PAUSE_GLOBAL_MINTING_ADMIN_IX_DISCM) {
            return Ok(Self::PauseGlobalMintingAdmin);
        }
        if buf.starts_with(&PAUSE_GLOBAL_REDEMPTION_IX_DISCM) {
            return Ok(Self::PauseGlobalRedemption);
        }
        if buf.starts_with(&PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_DISCM) {
            return Ok(Self::PauseGlobalRedemptionAdmin);
        }
        if buf.starts_with(&PAUSE_TOKEN_IX_DISCM) {
            return Ok(Self::PauseToken);
        }
        if buf.starts_with(&PAUSE_TOKEN_FACTORY_IX_DISCM) {
            return Ok(Self::PauseTokenFactory);
        }
        if buf.starts_with(&PAUSE_TOKEN_FACTORY_ADMIN_IX_DISCM) {
            return Ok(Self::PauseTokenFactoryAdmin);
        }
        if buf.starts_with(&PAUSE_TOKEN_MINTING_IX_DISCM) {
            return Ok(Self::PauseTokenMinting);
        }
        if buf.starts_with(&PAUSE_TOKEN_MINTING_ADMIN_IX_DISCM) {
            return Ok(Self::PauseTokenMintingAdmin);
        }
        if buf.starts_with(&PAUSE_TOKEN_REDEMPTION_IX_DISCM) {
            return Ok(Self::PauseTokenRedemption);
        }
        if buf.starts_with(&PAUSE_TOKEN_REDEMPTION_ADMIN_IX_DISCM) {
            return Ok(Self::PauseTokenRedemptionAdmin);
        }
        if buf.starts_with(&REDEEM_FOR_USDC_IX_DISCM) {
            let mut reader = &buf[REDEEM_FOR_USDC_IX_DISCM.len()..];
            let attestation_id: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
            let price: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let expiration: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RedeemForUsdc(RedeemForUsdcIxArgs {
                    attestation_id,
                    price,
                    amount,
                    expiration,
                }),
            );
        }
        if buf.starts_with(&REDEEM_FOR_USDON_IX_DISCM) {
            let mut reader = &buf[REDEEM_FOR_USDON_IX_DISCM.len()..];
            let attestation_id: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
            let price: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let expiration: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RedeemForUsdon(RedeemForUsdonIxArgs {
                    attestation_id,
                    price,
                    amount,
                    expiration,
                }),
            );
        }
        if buf.starts_with(&REMOVE_FROM_WHITELIST_IX_DISCM) {
            let mut reader = &buf[REMOVE_FROM_WHITELIST_IX_DISCM.len()..];
            let address_to_remove: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemoveFromWhitelist(RemoveFromWhitelistIxArgs {
                    address_to_remove,
                }),
            );
        }
        if buf.starts_with(&RESUME_GLOBAL_MINTING_IX_DISCM) {
            return Ok(Self::ResumeGlobalMinting);
        }
        if buf.starts_with(&RESUME_GLOBAL_REDEMPTION_IX_DISCM) {
            return Ok(Self::ResumeGlobalRedemption);
        }
        if buf.starts_with(&RESUME_TOKEN_IX_DISCM) {
            return Ok(Self::ResumeToken);
        }
        if buf.starts_with(&RESUME_TOKEN_FACTORY_IX_DISCM) {
            return Ok(Self::ResumeTokenFactory);
        }
        if buf.starts_with(&RESUME_TOKEN_MINTING_IX_DISCM) {
            return Ok(Self::ResumeTokenMinting);
        }
        if buf.starts_with(&RESUME_TOKEN_REDEMPTION_IX_DISCM) {
            return Ok(Self::ResumeTokenRedemption);
        }
        if buf.starts_with(&RETRIEVE_TOKENS_IX_DISCM) {
            let mut reader = &buf[RETRIEVE_TOKENS_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::RetrieveTokens(RetrieveTokensIxArgs { amount }));
        }
        if buf.starts_with(&REVOKE_GMTOKEN_FACTORY_ROLE_IX_DISCM) {
            return Ok(Self::RevokeGmtokenFactoryRole);
        }
        if buf.starts_with(&REVOKE_GMTOKEN_MANAGER_ROLE_IX_DISCM) {
            return Ok(Self::RevokeGmtokenManagerRole);
        }
        if buf.starts_with(&REVOKE_GMTOKEN_ROLE_IX_DISCM) {
            return Ok(Self::RevokeGmtokenRole);
        }
        if buf.starts_with(&REVOKE_ROLE_IX_DISCM) {
            let mut reader = &buf[REVOKE_ROLE_IX_DISCM.len()..];
            let _role: RoleType = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::RevokeRole(RevokeRoleIxArgs { _role }));
        }
        if buf.starts_with(&REVOKE_SANITY_CONFIGURER_ROLE_IX_DISCM) {
            return Ok(Self::RevokeSanityConfigurerRole);
        }
        if buf.starts_with(&REVOKE_SANITY_SETTER_ROLE_IX_DISCM) {
            return Ok(Self::RevokeSanitySetterRole);
        }
        if buf.starts_with(&REVOKE_USDON_ROLE_IX_DISCM) {
            return Ok(Self::RevokeUsdonRole);
        }
        if buf.starts_with(&SET_ALLOWED_DEVIATION_BPS_IX_DISCM) {
            let mut reader = &buf[SET_ALLOWED_DEVIATION_BPS_IX_DISCM.len()..];
            let allowed_deviation_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetAllowedDeviationBps(SetAllowedDeviationBpsIxArgs {
                    allowed_deviation_bps,
                }),
            );
        }
        if buf.starts_with(&SET_ATTESTATION_SIGNER_SECP_IX_DISCM) {
            let mut reader = &buf[SET_ATTESTATION_SIGNER_SECP_IX_DISCM.len()..];
            let attestation_signer_secp: [u8; 20] = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetAttestationSignerSecp(SetAttestationSignerSecpIxArgs {
                    attestation_signer_secp,
                }),
            );
        }
        if buf.starts_with(&SET_LAST_PRICE_IX_DISCM) {
            let mut reader = &buf[SET_LAST_PRICE_IX_DISCM.len()..];
            let last_price: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetLastPrice(SetLastPriceIxArgs { last_price }));
        }
        if buf.starts_with(&SET_MAX_TIME_DELAY_IX_DISCM) {
            let mut reader = &buf[SET_MAX_TIME_DELAY_IX_DISCM.len()..];
            let max_time_delay: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetMaxTimeDelay(SetMaxTimeDelayIxArgs {
                    max_time_delay,
                }),
            );
        }
        if buf.starts_with(&SET_ONDO_USER_LIMITS_IX_DISCM) {
            let mut reader = &buf[SET_ONDO_USER_LIMITS_IX_DISCM.len()..];
            let rate_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
            let limit_window: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetOndoUserLimits(SetOndoUserLimitsIxArgs {
                    rate_limit,
                    limit_window,
                }),
            );
        }
        if buf.starts_with(&SET_ORACLE_PRICE_MAX_AGE_IX_DISCM) {
            let mut reader = &buf[SET_ORACLE_PRICE_MAX_AGE_IX_DISCM.len()..];
            let oracle_price_max_age: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetOraclePriceMaxAge(SetOraclePriceMaxAgeIxArgs {
                    oracle_price_max_age,
                }),
            );
        }
        if buf.starts_with(&SET_TOKEN_LIMIT_IX_DISCM) {
            let mut reader = &buf[SET_TOKEN_LIMIT_IX_DISCM.len()..];
            let rate_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let limit_window: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let default_user_rate_limit: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let default_user_limit_window: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetTokenLimit(SetTokenLimitIxArgs {
                    rate_limit,
                    limit_window,
                    default_user_rate_limit,
                    default_user_limit_window,
                }),
            );
        }
        if buf.starts_with(&SET_USDC_PRICE_UPDATE_ADDRESS_IX_DISCM) {
            let mut reader = &buf[SET_USDC_PRICE_UPDATE_ADDRESS_IX_DISCM.len()..];
            let new_price_update_address: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetUsdcPriceUpdateAddress(SetUsdcPriceUpdateAddressIxArgs {
                    new_price_update_address,
                }),
            );
        }
        if buf.starts_with(&UPDATE_SCALED_UI_MULTIPLIER_IX_DISCM) {
            let mut reader = &buf[UPDATE_SCALED_UI_MULTIPLIER_IX_DISCM.len()..];
            let new_multiplier: f64 = crate::borsh_de_or_default(&mut reader)?;
            let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateScaledUiMultiplier(UpdateScaledUiMultiplierIxArgs {
                    new_multiplier,
                    timestamp,
                }),
            );
        }
        if buf.starts_with(&UPDATE_TOKEN_METADATA_IX_DISCM) {
            let mut reader = &buf[UPDATE_TOKEN_METADATA_IX_DISCM.len()..];
            let new_name: Option<String> = crate::borsh_de_or_default(&mut reader)?;
            let new_symbol: Option<String> = crate::borsh_de_or_default(&mut reader)?;
            let new_uri: Option<String> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateTokenMetadata(UpdateTokenMetadataIxArgs {
                    new_name,
                    new_symbol,
                    new_uri,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AddToWhitelist(args) => {
                writer.write_all(&ADD_TO_WHITELIST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.address_to_whitelist,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::BatchCloseAttestationAccounts => {
                writer.write_all(&BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_DISCM)
            }
            Self::BurnUsdon(args) => {
                writer.write_all(&BURN_USDON_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::CloseAttestationAccount(args) => {
                writer.write_all(&CLOSE_ATTESTATION_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args._attestation_id, &mut writer)?;
                Ok(())
            }
            Self::EnableOraclePrice(args) => {
                writer.write_all(&ENABLE_ORACLE_PRICE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.is_enabled, &mut writer)?;
                Ok(())
            }
            Self::GrantGmtokenFactoryRole(args) => {
                writer.write_all(&GRANT_GMTOKEN_FACTORY_ROLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.role, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.user, &mut writer)?;
                Ok(())
            }
            Self::GrantGmtokenManagerRole(args) => {
                writer.write_all(&GRANT_GMTOKEN_MANAGER_ROLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.role, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.user, &mut writer)?;
                Ok(())
            }
            Self::GrantGmtokenRole(args) => {
                writer.write_all(&GRANT_GMTOKEN_ROLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.role, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.user, &mut writer)?;
                Ok(())
            }
            Self::GrantRole(args) => {
                writer.write_all(&GRANT_ROLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.role, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.user, &mut writer)?;
                Ok(())
            }
            Self::GrantSanityConfigurerRole(args) => {
                writer.write_all(&GRANT_SANITY_CONFIGURER_ROLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.role, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.user, &mut writer)?;
                Ok(())
            }
            Self::GrantSanitySetterRole(args) => {
                writer.write_all(&GRANT_SANITY_SETTER_ROLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.role, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.user, &mut writer)?;
                Ok(())
            }
            Self::GrantUsdonRole(args) => {
                writer.write_all(&GRANT_USDON_ROLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.role, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.user, &mut writer)?;
                Ok(())
            }
            Self::InitMint(args) => {
                writer.write_all(&INIT_MINT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.freeze_authority, &mut writer)?;
                Ok(())
            }
            Self::InitMintDelegate(args) => {
                writer.write_all(&INIT_MINT_DELEGATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.freeze_authority, &mut writer)?;
                Ok(())
            }
            Self::InitializeGmtokenManager(args) => {
                writer.write_all(&INITIALIZE_GMTOKEN_MANAGER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.factory_paused, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.redemptions_paused, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minting_paused, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.attestation_signer_secp,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.trading_hours_offset,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitializeSanityCheck(args) => {
                writer.write_all(&INITIALIZE_SANITY_CHECK_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.last_price, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.allowed_deviation_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.max_time_delay, &mut writer)?;
                Ok(())
            }
            Self::InitializeTokenLimit(args) => {
                writer.write_all(&INITIALIZE_TOKEN_LIMIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.rate_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.limit_window, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.default_user_rate_limit,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.default_limit_window,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitializeUsdonManager(args) => {
                writer.write_all(&INITIALIZE_USDON_MANAGER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.oracle_price_enabled,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.oracle_price_max_age,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.usdc_price_update_address,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitializeUser(args) => {
                writer.write_all(&INITIALIZE_USER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.rate_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.limit_window, &mut writer)?;
                Ok(())
            }
            Self::MintGm(args) => {
                writer.write_all(&MINT_GM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::MintUsdon(args) => {
                writer.write_all(&MINT_USDON_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::MintWithUsdc(args) => {
                writer.write_all(&MINT_WITH_USDC_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.attestation_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.expiration, &mut writer)?;
                Ok(())
            }
            Self::MintWithUsdon(args) => {
                writer.write_all(&MINT_WITH_USDON_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.attestation_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.expiration, &mut writer)?;
                Ok(())
            }
            Self::PauseGlobalMinting => writer.write_all(&PAUSE_GLOBAL_MINTING_IX_DISCM),
            Self::PauseGlobalMintingAdmin => {
                writer.write_all(&PAUSE_GLOBAL_MINTING_ADMIN_IX_DISCM)
            }
            Self::PauseGlobalRedemption => {
                writer.write_all(&PAUSE_GLOBAL_REDEMPTION_IX_DISCM)
            }
            Self::PauseGlobalRedemptionAdmin => {
                writer.write_all(&PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_DISCM)
            }
            Self::PauseToken => writer.write_all(&PAUSE_TOKEN_IX_DISCM),
            Self::PauseTokenFactory => writer.write_all(&PAUSE_TOKEN_FACTORY_IX_DISCM),
            Self::PauseTokenFactoryAdmin => {
                writer.write_all(&PAUSE_TOKEN_FACTORY_ADMIN_IX_DISCM)
            }
            Self::PauseTokenMinting => writer.write_all(&PAUSE_TOKEN_MINTING_IX_DISCM),
            Self::PauseTokenMintingAdmin => {
                writer.write_all(&PAUSE_TOKEN_MINTING_ADMIN_IX_DISCM)
            }
            Self::PauseTokenRedemption => {
                writer.write_all(&PAUSE_TOKEN_REDEMPTION_IX_DISCM)
            }
            Self::PauseTokenRedemptionAdmin => {
                writer.write_all(&PAUSE_TOKEN_REDEMPTION_ADMIN_IX_DISCM)
            }
            Self::RedeemForUsdc(args) => {
                writer.write_all(&REDEEM_FOR_USDC_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.attestation_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.expiration, &mut writer)?;
                Ok(())
            }
            Self::RedeemForUsdon(args) => {
                writer.write_all(&REDEEM_FOR_USDON_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.attestation_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.expiration, &mut writer)?;
                Ok(())
            }
            Self::RemoveFromWhitelist(args) => {
                writer.write_all(&REMOVE_FROM_WHITELIST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.address_to_remove, &mut writer)?;
                Ok(())
            }
            Self::ResumeGlobalMinting => {
                writer.write_all(&RESUME_GLOBAL_MINTING_IX_DISCM)
            }
            Self::ResumeGlobalRedemption => {
                writer.write_all(&RESUME_GLOBAL_REDEMPTION_IX_DISCM)
            }
            Self::ResumeToken => writer.write_all(&RESUME_TOKEN_IX_DISCM),
            Self::ResumeTokenFactory => writer.write_all(&RESUME_TOKEN_FACTORY_IX_DISCM),
            Self::ResumeTokenMinting => writer.write_all(&RESUME_TOKEN_MINTING_IX_DISCM),
            Self::ResumeTokenRedemption => {
                writer.write_all(&RESUME_TOKEN_REDEMPTION_IX_DISCM)
            }
            Self::RetrieveTokens(args) => {
                writer.write_all(&RETRIEVE_TOKENS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::RevokeGmtokenFactoryRole => {
                writer.write_all(&REVOKE_GMTOKEN_FACTORY_ROLE_IX_DISCM)
            }
            Self::RevokeGmtokenManagerRole => {
                writer.write_all(&REVOKE_GMTOKEN_MANAGER_ROLE_IX_DISCM)
            }
            Self::RevokeGmtokenRole => writer.write_all(&REVOKE_GMTOKEN_ROLE_IX_DISCM),
            Self::RevokeRole(args) => {
                writer.write_all(&REVOKE_ROLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args._role, &mut writer)?;
                Ok(())
            }
            Self::RevokeSanityConfigurerRole => {
                writer.write_all(&REVOKE_SANITY_CONFIGURER_ROLE_IX_DISCM)
            }
            Self::RevokeSanitySetterRole => {
                writer.write_all(&REVOKE_SANITY_SETTER_ROLE_IX_DISCM)
            }
            Self::RevokeUsdonRole => writer.write_all(&REVOKE_USDON_ROLE_IX_DISCM),
            Self::SetAllowedDeviationBps(args) => {
                writer.write_all(&SET_ALLOWED_DEVIATION_BPS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.allowed_deviation_bps,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetAttestationSignerSecp(args) => {
                writer.write_all(&SET_ATTESTATION_SIGNER_SECP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.attestation_signer_secp,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetLastPrice(args) => {
                writer.write_all(&SET_LAST_PRICE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.last_price, &mut writer)?;
                Ok(())
            }
            Self::SetMaxTimeDelay(args) => {
                writer.write_all(&SET_MAX_TIME_DELAY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_time_delay, &mut writer)?;
                Ok(())
            }
            Self::SetOndoUserLimits(args) => {
                writer.write_all(&SET_ONDO_USER_LIMITS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.rate_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.limit_window, &mut writer)?;
                Ok(())
            }
            Self::SetOraclePriceMaxAge(args) => {
                writer.write_all(&SET_ORACLE_PRICE_MAX_AGE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.oracle_price_max_age,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetTokenLimit(args) => {
                writer.write_all(&SET_TOKEN_LIMIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.rate_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.limit_window, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.default_user_rate_limit,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.default_user_limit_window,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetUsdcPriceUpdateAddress(args) => {
                writer.write_all(&SET_USDC_PRICE_UPDATE_ADDRESS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_price_update_address,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateScaledUiMultiplier(args) => {
                writer.write_all(&UPDATE_SCALED_UI_MULTIPLIER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_multiplier, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.timestamp, &mut writer)?;
                Ok(())
            }
            Self::UpdateTokenMetadata(args) => {
                writer.write_all(&UPDATE_TOKEN_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_uri, &mut writer)?;
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
pub const ADD_TO_WHITELIST_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct AddToWhitelistAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub whitelist: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddToWhitelistKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub whitelist: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddToWhitelistAccounts<'_, '_>> for AddToWhitelistKeys {
    fn from(accounts: AddToWhitelistAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            whitelist: *accounts.whitelist.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddToWhitelistKeys> for [AccountMeta; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(keys: AddToWhitelistKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whitelist,
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
impl From<[Pubkey; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN]> for AddToWhitelistKeys {
    fn from(pubkeys: [Pubkey; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            whitelist: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<AddToWhitelistAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddToWhitelistAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.whitelist.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN]>
for AddToWhitelistAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            whitelist: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const ADD_TO_WHITELIST_IX_DISCM: [u8; 8usize] = [157, 211, 52, 54, 144, 81, 5, 55];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddToWhitelistIxArgs {
    pub address_to_whitelist: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddToWhitelistIxData(pub AddToWhitelistIxArgs);
impl From<AddToWhitelistIxArgs> for AddToWhitelistIxData {
    fn from(args: AddToWhitelistIxArgs) -> Self {
        Self(args)
    }
}
impl AddToWhitelistIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_TO_WHITELIST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let address_to_whitelist: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddToWhitelistIxArgs {
                address_to_whitelist,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_TO_WHITELIST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.address_to_whitelist, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_to_whitelist_ix_with_program_id(
    program_id: Pubkey,
    keys: AddToWhitelistKeys,
    args: AddToWhitelistIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddToWhitelistIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_to_whitelist_ix(
    keys: AddToWhitelistKeys,
    args: AddToWhitelistIxArgs,
) -> std::io::Result<Instruction> {
    add_to_whitelist_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn add_to_whitelist_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddToWhitelistAccounts<'_, '_>,
    args: AddToWhitelistIxArgs,
) -> ProgramResult {
    let keys: AddToWhitelistKeys = accounts.into();
    let ix = add_to_whitelist_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_to_whitelist_invoke(
    accounts: AddToWhitelistAccounts<'_, '_>,
    args: AddToWhitelistIxArgs,
) -> ProgramResult {
    add_to_whitelist_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn add_to_whitelist_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddToWhitelistAccounts<'_, '_>,
    args: AddToWhitelistIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddToWhitelistKeys = accounts.into();
    let ix = add_to_whitelist_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_to_whitelist_invoke_signed(
    accounts: AddToWhitelistAccounts<'_, '_>,
    args: AddToWhitelistIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_to_whitelist_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_to_whitelist_verify_account_keys(
    accounts: AddToWhitelistAccounts<'_, '_>,
    keys: AddToWhitelistKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.whitelist.key, keys.whitelist),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_to_whitelist_verify_writable_privileges<'me, 'info>(
    accounts: AddToWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.whitelist] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_to_whitelist_verify_signer_privileges<'me, 'info>(
    accounts: AddToWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_to_whitelist_verify_account_privileges<'me, 'info>(
    accounts: AddToWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_to_whitelist_verify_writable_privileges(accounts)?;
    add_to_whitelist_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct BatchCloseAttestationAccountsAccounts<'me, 'info> {
    pub closer: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BatchCloseAttestationAccountsKeys {
    pub closer: Pubkey,
    pub recipient: Pubkey,
    pub system_program: Pubkey,
}
impl From<BatchCloseAttestationAccountsAccounts<'_, '_>>
for BatchCloseAttestationAccountsKeys {
    fn from(accounts: BatchCloseAttestationAccountsAccounts) -> Self {
        Self {
            closer: *accounts.closer.key,
            recipient: *accounts.recipient.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<BatchCloseAttestationAccountsKeys>
for [AccountMeta; BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_ACCOUNTS_LEN] {
    fn from(keys: BatchCloseAttestationAccountsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.closer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient,
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
impl From<[Pubkey; BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_ACCOUNTS_LEN]>
for BatchCloseAttestationAccountsKeys {
    fn from(
        pubkeys: [Pubkey; BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            closer: pubkeys[0],
            recipient: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<BatchCloseAttestationAccountsAccounts<'_, 'info>>
for [AccountInfo<'info>; BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: BatchCloseAttestationAccountsAccounts<'_, 'info>) -> Self {
        [
            accounts.closer.clone(),
            accounts.recipient.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_ACCOUNTS_LEN]>
for BatchCloseAttestationAccountsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            closer: &arr[0],
            recipient: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_DISCM: [u8; 8usize] = [
    73, 167, 240, 82, 80, 48, 205, 207,
];
#[derive(Clone, Debug, PartialEq)]
pub struct BatchCloseAttestationAccountsIxData;
impl BatchCloseAttestationAccountsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn batch_close_attestation_accounts_ix_with_program_id(
    program_id: Pubkey,
    keys: BatchCloseAttestationAccountsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BATCH_CLOSE_ATTESTATION_ACCOUNTS_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: BatchCloseAttestationAccountsIxData.try_to_vec()?,
    })
}
pub fn batch_close_attestation_accounts_ix(
    keys: BatchCloseAttestationAccountsKeys,
) -> std::io::Result<Instruction> {
    batch_close_attestation_accounts_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn batch_close_attestation_accounts_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BatchCloseAttestationAccountsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: BatchCloseAttestationAccountsKeys = accounts.into();
    let ix = batch_close_attestation_accounts_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn batch_close_attestation_accounts_invoke(
    accounts: BatchCloseAttestationAccountsAccounts<'_, '_>,
) -> ProgramResult {
    batch_close_attestation_accounts_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn batch_close_attestation_accounts_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BatchCloseAttestationAccountsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BatchCloseAttestationAccountsKeys = accounts.into();
    let ix = batch_close_attestation_accounts_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn batch_close_attestation_accounts_invoke_signed(
    accounts: BatchCloseAttestationAccountsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    batch_close_attestation_accounts_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn batch_close_attestation_accounts_verify_account_keys(
    accounts: BatchCloseAttestationAccountsAccounts<'_, '_>,
    keys: BatchCloseAttestationAccountsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.closer.key, keys.closer),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn batch_close_attestation_accounts_verify_writable_privileges<'me, 'info>(
    accounts: BatchCloseAttestationAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.recipient] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn batch_close_attestation_accounts_verify_signer_privileges<'me, 'info>(
    accounts: BatchCloseAttestationAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.closer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn batch_close_attestation_accounts_verify_account_privileges<'me, 'info>(
    accounts: BatchCloseAttestationAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    batch_close_attestation_accounts_verify_writable_privileges(accounts)?;
    batch_close_attestation_accounts_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BURN_USDON_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct BurnUsdonAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub permanent_delegate: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BurnUsdonKeys {
    pub authority: Pubkey,
    pub permanent_delegate: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub authority_role_account: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub destination: Pubkey,
}
impl From<BurnUsdonAccounts<'_, '_>> for BurnUsdonKeys {
    fn from(accounts: BurnUsdonAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            permanent_delegate: *accounts.permanent_delegate.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            authority_role_account: *accounts.authority_role_account.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            destination: *accounts.destination.key,
        }
    }
}
impl From<BurnUsdonKeys> for [AccountMeta; BURN_USDON_IX_ACCOUNTS_LEN] {
    fn from(keys: BurnUsdonKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.permanent_delegate,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
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
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; BURN_USDON_IX_ACCOUNTS_LEN]> for BurnUsdonKeys {
    fn from(pubkeys: [Pubkey; BURN_USDON_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            permanent_delegate: pubkeys[1],
            usdon_manager_state: pubkeys[2],
            authority_role_account: pubkeys[3],
            mint: pubkeys[4],
            token_program: pubkeys[5],
            destination: pubkeys[6],
        }
    }
}
impl<'info> From<BurnUsdonAccounts<'_, 'info>>
for [AccountInfo<'info>; BURN_USDON_IX_ACCOUNTS_LEN] {
    fn from(accounts: BurnUsdonAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.permanent_delegate.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.authority_role_account.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.destination.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BURN_USDON_IX_ACCOUNTS_LEN]>
for BurnUsdonAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BURN_USDON_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            permanent_delegate: &arr[1],
            usdon_manager_state: &arr[2],
            authority_role_account: &arr[3],
            mint: &arr[4],
            token_program: &arr[5],
            destination: &arr[6],
        }
    }
}
pub const BURN_USDON_IX_DISCM: [u8; 8usize] = [173, 55, 240, 198, 34, 223, 191, 82];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BurnUsdonIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BurnUsdonIxData(pub BurnUsdonIxArgs);
impl From<BurnUsdonIxArgs> for BurnUsdonIxData {
    fn from(args: BurnUsdonIxArgs) -> Self {
        Self(args)
    }
}
impl BurnUsdonIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BURN_USDON_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(BurnUsdonIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BURN_USDON_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn burn_usdon_ix_with_program_id(
    program_id: Pubkey,
    keys: BurnUsdonKeys,
    args: BurnUsdonIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BURN_USDON_IX_ACCOUNTS_LEN] = keys.into();
    let data: BurnUsdonIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn burn_usdon_ix(
    keys: BurnUsdonKeys,
    args: BurnUsdonIxArgs,
) -> std::io::Result<Instruction> {
    burn_usdon_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn burn_usdon_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BurnUsdonAccounts<'_, '_>,
    args: BurnUsdonIxArgs,
) -> ProgramResult {
    let keys: BurnUsdonKeys = accounts.into();
    let ix = burn_usdon_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn burn_usdon_invoke(
    accounts: BurnUsdonAccounts<'_, '_>,
    args: BurnUsdonIxArgs,
) -> ProgramResult {
    burn_usdon_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn burn_usdon_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BurnUsdonAccounts<'_, '_>,
    args: BurnUsdonIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BurnUsdonKeys = accounts.into();
    let ix = burn_usdon_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn burn_usdon_invoke_signed(
    accounts: BurnUsdonAccounts<'_, '_>,
    args: BurnUsdonIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    burn_usdon_invoke_signed_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args, seeds)
}
pub fn burn_usdon_verify_account_keys(
    accounts: BurnUsdonAccounts<'_, '_>,
    keys: BurnUsdonKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.permanent_delegate.key, keys.permanent_delegate),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.destination.key, keys.destination),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn burn_usdon_verify_writable_privileges<'me, 'info>(
    accounts: BurnUsdonAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.mint, accounts.destination] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn burn_usdon_verify_signer_privileges<'me, 'info>(
    accounts: BurnUsdonAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn burn_usdon_verify_account_privileges<'me, 'info>(
    accounts: BurnUsdonAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    burn_usdon_verify_writable_privileges(accounts)?;
    burn_usdon_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_ATTESTATION_ACCOUNT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CloseAttestationAccountAccounts<'me, 'info> {
    pub closer: &'me AccountInfo<'info>,
    pub attestation: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseAttestationAccountKeys {
    pub closer: Pubkey,
    pub attestation: Pubkey,
    pub recipient: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseAttestationAccountAccounts<'_, '_>> for CloseAttestationAccountKeys {
    fn from(accounts: CloseAttestationAccountAccounts) -> Self {
        Self {
            closer: *accounts.closer.key,
            attestation: *accounts.attestation.key,
            recipient: *accounts.recipient.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseAttestationAccountKeys>
for [AccountMeta; CLOSE_ATTESTATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseAttestationAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.closer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.attestation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient,
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
impl From<[Pubkey; CLOSE_ATTESTATION_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseAttestationAccountKeys {
    fn from(pubkeys: [Pubkey; CLOSE_ATTESTATION_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            closer: pubkeys[0],
            attestation: pubkeys[1],
            recipient: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CloseAttestationAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_ATTESTATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseAttestationAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.closer.clone(),
            accounts.attestation.clone(),
            accounts.recipient.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_ATTESTATION_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseAttestationAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_ATTESTATION_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            closer: &arr[0],
            attestation: &arr[1],
            recipient: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CLOSE_ATTESTATION_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    201, 34, 214, 89, 249, 220, 97, 101,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CloseAttestationAccountIxArgs {
    pub _attestation_id: [u8; 16],
}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseAttestationAccountIxData(pub CloseAttestationAccountIxArgs);
impl From<CloseAttestationAccountIxArgs> for CloseAttestationAccountIxData {
    fn from(args: CloseAttestationAccountIxArgs) -> Self {
        Self(args)
    }
}
impl CloseAttestationAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_ATTESTATION_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let _attestation_id: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CloseAttestationAccountIxArgs {
                _attestation_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_ATTESTATION_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0._attestation_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_attestation_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseAttestationAccountKeys,
    args: CloseAttestationAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_ATTESTATION_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CloseAttestationAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn close_attestation_account_ix(
    keys: CloseAttestationAccountKeys,
    args: CloseAttestationAccountIxArgs,
) -> std::io::Result<Instruction> {
    close_attestation_account_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn close_attestation_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseAttestationAccountAccounts<'_, '_>,
    args: CloseAttestationAccountIxArgs,
) -> ProgramResult {
    let keys: CloseAttestationAccountKeys = accounts.into();
    let ix = close_attestation_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_attestation_account_invoke(
    accounts: CloseAttestationAccountAccounts<'_, '_>,
    args: CloseAttestationAccountIxArgs,
) -> ProgramResult {
    close_attestation_account_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn close_attestation_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseAttestationAccountAccounts<'_, '_>,
    args: CloseAttestationAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseAttestationAccountKeys = accounts.into();
    let ix = close_attestation_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_attestation_account_invoke_signed(
    accounts: CloseAttestationAccountAccounts<'_, '_>,
    args: CloseAttestationAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_attestation_account_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn close_attestation_account_verify_account_keys(
    accounts: CloseAttestationAccountAccounts<'_, '_>,
    keys: CloseAttestationAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.closer.key, keys.closer),
        (*accounts.attestation.key, keys.attestation),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_attestation_account_verify_writable_privileges<'me, 'info>(
    accounts: CloseAttestationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.attestation, accounts.recipient] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_attestation_account_verify_signer_privileges<'me, 'info>(
    accounts: CloseAttestationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.closer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_attestation_account_verify_account_privileges<'me, 'info>(
    accounts: CloseAttestationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_attestation_account_verify_writable_privileges(accounts)?;
    close_attestation_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ENABLE_ORACLE_PRICE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct EnableOraclePriceAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EnableOraclePriceKeys {
    pub authority: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub authority_role_account: Pubkey,
}
impl From<EnableOraclePriceAccounts<'_, '_>> for EnableOraclePriceKeys {
    fn from(accounts: EnableOraclePriceAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            authority_role_account: *accounts.authority_role_account.key,
        }
    }
}
impl From<EnableOraclePriceKeys> for [AccountMeta; ENABLE_ORACLE_PRICE_IX_ACCOUNTS_LEN] {
    fn from(keys: EnableOraclePriceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ENABLE_ORACLE_PRICE_IX_ACCOUNTS_LEN]> for EnableOraclePriceKeys {
    fn from(pubkeys: [Pubkey; ENABLE_ORACLE_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            usdon_manager_state: pubkeys[1],
            authority_role_account: pubkeys[2],
        }
    }
}
impl<'info> From<EnableOraclePriceAccounts<'_, 'info>>
for [AccountInfo<'info>; ENABLE_ORACLE_PRICE_IX_ACCOUNTS_LEN] {
    fn from(accounts: EnableOraclePriceAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.authority_role_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ENABLE_ORACLE_PRICE_IX_ACCOUNTS_LEN]>
for EnableOraclePriceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ENABLE_ORACLE_PRICE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            usdon_manager_state: &arr[1],
            authority_role_account: &arr[2],
        }
    }
}
pub const ENABLE_ORACLE_PRICE_IX_DISCM: [u8; 8usize] = [
    31, 98, 115, 216, 241, 185, 41, 109,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EnableOraclePriceIxArgs {
    pub is_enabled: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnableOraclePriceIxData(pub EnableOraclePriceIxArgs);
impl From<EnableOraclePriceIxArgs> for EnableOraclePriceIxData {
    fn from(args: EnableOraclePriceIxArgs) -> Self {
        Self(args)
    }
}
impl EnableOraclePriceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ENABLE_ORACLE_PRICE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let is_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(EnableOraclePriceIxArgs {
                is_enabled,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ENABLE_ORACLE_PRICE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.is_enabled, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn enable_oracle_price_ix_with_program_id(
    program_id: Pubkey,
    keys: EnableOraclePriceKeys,
    args: EnableOraclePriceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ENABLE_ORACLE_PRICE_IX_ACCOUNTS_LEN] = keys.into();
    let data: EnableOraclePriceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn enable_oracle_price_ix(
    keys: EnableOraclePriceKeys,
    args: EnableOraclePriceIxArgs,
) -> std::io::Result<Instruction> {
    enable_oracle_price_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn enable_oracle_price_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EnableOraclePriceAccounts<'_, '_>,
    args: EnableOraclePriceIxArgs,
) -> ProgramResult {
    let keys: EnableOraclePriceKeys = accounts.into();
    let ix = enable_oracle_price_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn enable_oracle_price_invoke(
    accounts: EnableOraclePriceAccounts<'_, '_>,
    args: EnableOraclePriceIxArgs,
) -> ProgramResult {
    enable_oracle_price_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn enable_oracle_price_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EnableOraclePriceAccounts<'_, '_>,
    args: EnableOraclePriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EnableOraclePriceKeys = accounts.into();
    let ix = enable_oracle_price_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn enable_oracle_price_invoke_signed(
    accounts: EnableOraclePriceAccounts<'_, '_>,
    args: EnableOraclePriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    enable_oracle_price_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn enable_oracle_price_verify_account_keys(
    accounts: EnableOraclePriceAccounts<'_, '_>,
    keys: EnableOraclePriceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
        (*accounts.authority_role_account.key, keys.authority_role_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn enable_oracle_price_verify_writable_privileges<'me, 'info>(
    accounts: EnableOraclePriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.usdon_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn enable_oracle_price_verify_signer_privileges<'me, 'info>(
    accounts: EnableOraclePriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn enable_oracle_price_verify_account_privileges<'me, 'info>(
    accounts: EnableOraclePriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    enable_oracle_price_verify_writable_privileges(accounts)?;
    enable_oracle_price_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GRANT_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct GrantGmtokenFactoryRoleAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub role_to_grant: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GrantGmtokenFactoryRoleKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub role_to_grant: Pubkey,
    pub system_program: Pubkey,
}
impl From<GrantGmtokenFactoryRoleAccounts<'_, '_>> for GrantGmtokenFactoryRoleKeys {
    fn from(accounts: GrantGmtokenFactoryRoleAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            role_to_grant: *accounts.role_to_grant.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<GrantGmtokenFactoryRoleKeys>
for [AccountMeta; GRANT_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: GrantGmtokenFactoryRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_grant,
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
impl From<[Pubkey; GRANT_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN]>
for GrantGmtokenFactoryRoleKeys {
    fn from(pubkeys: [Pubkey; GRANT_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            role_to_grant: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<GrantGmtokenFactoryRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; GRANT_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GrantGmtokenFactoryRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.role_to_grant.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; GRANT_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN]>
for GrantGmtokenFactoryRoleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; GRANT_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            role_to_grant: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const GRANT_GMTOKEN_FACTORY_ROLE_IX_DISCM: [u8; 8usize] = [
    219, 108, 46, 79, 46, 104, 74, 179,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrantGmtokenFactoryRoleIxArgs {
    pub role: RoleType,
    pub user: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GrantGmtokenFactoryRoleIxData(pub GrantGmtokenFactoryRoleIxArgs);
impl From<GrantGmtokenFactoryRoleIxArgs> for GrantGmtokenFactoryRoleIxData {
    fn from(args: GrantGmtokenFactoryRoleIxArgs) -> Self {
        Self(args)
    }
}
impl GrantGmtokenFactoryRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GRANT_GMTOKEN_FACTORY_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(GrantGmtokenFactoryRoleIxArgs {
                role,
                user,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GRANT_GMTOKEN_FACTORY_ROLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.role, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.user, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn grant_gmtoken_factory_role_ix_with_program_id(
    program_id: Pubkey,
    keys: GrantGmtokenFactoryRoleKeys,
    args: GrantGmtokenFactoryRoleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GRANT_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: GrantGmtokenFactoryRoleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn grant_gmtoken_factory_role_ix(
    keys: GrantGmtokenFactoryRoleKeys,
    args: GrantGmtokenFactoryRoleIxArgs,
) -> std::io::Result<Instruction> {
    grant_gmtoken_factory_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn grant_gmtoken_factory_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GrantGmtokenFactoryRoleAccounts<'_, '_>,
    args: GrantGmtokenFactoryRoleIxArgs,
) -> ProgramResult {
    let keys: GrantGmtokenFactoryRoleKeys = accounts.into();
    let ix = grant_gmtoken_factory_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn grant_gmtoken_factory_role_invoke(
    accounts: GrantGmtokenFactoryRoleAccounts<'_, '_>,
    args: GrantGmtokenFactoryRoleIxArgs,
) -> ProgramResult {
    grant_gmtoken_factory_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn grant_gmtoken_factory_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GrantGmtokenFactoryRoleAccounts<'_, '_>,
    args: GrantGmtokenFactoryRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GrantGmtokenFactoryRoleKeys = accounts.into();
    let ix = grant_gmtoken_factory_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn grant_gmtoken_factory_role_invoke_signed(
    accounts: GrantGmtokenFactoryRoleAccounts<'_, '_>,
    args: GrantGmtokenFactoryRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    grant_gmtoken_factory_role_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn grant_gmtoken_factory_role_verify_account_keys(
    accounts: GrantGmtokenFactoryRoleAccounts<'_, '_>,
    keys: GrantGmtokenFactoryRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.role_to_grant.key, keys.role_to_grant),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn grant_gmtoken_factory_role_verify_writable_privileges<'me, 'info>(
    accounts: GrantGmtokenFactoryRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.role_to_grant] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn grant_gmtoken_factory_role_verify_signer_privileges<'me, 'info>(
    accounts: GrantGmtokenFactoryRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn grant_gmtoken_factory_role_verify_account_privileges<'me, 'info>(
    accounts: GrantGmtokenFactoryRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    grant_gmtoken_factory_role_verify_writable_privileges(accounts)?;
    grant_gmtoken_factory_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GRANT_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct GrantGmtokenManagerRoleAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub role_to_grant: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GrantGmtokenManagerRoleKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub role_to_grant: Pubkey,
    pub system_program: Pubkey,
}
impl From<GrantGmtokenManagerRoleAccounts<'_, '_>> for GrantGmtokenManagerRoleKeys {
    fn from(accounts: GrantGmtokenManagerRoleAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            role_to_grant: *accounts.role_to_grant.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<GrantGmtokenManagerRoleKeys>
for [AccountMeta; GRANT_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: GrantGmtokenManagerRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_grant,
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
impl From<[Pubkey; GRANT_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN]>
for GrantGmtokenManagerRoleKeys {
    fn from(pubkeys: [Pubkey; GRANT_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            role_to_grant: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<GrantGmtokenManagerRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; GRANT_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GrantGmtokenManagerRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.role_to_grant.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; GRANT_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN]>
for GrantGmtokenManagerRoleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; GRANT_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            role_to_grant: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const GRANT_GMTOKEN_MANAGER_ROLE_IX_DISCM: [u8; 8usize] = [
    245, 248, 145, 72, 159, 93, 154, 176,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrantGmtokenManagerRoleIxArgs {
    pub role: RoleType,
    pub user: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GrantGmtokenManagerRoleIxData(pub GrantGmtokenManagerRoleIxArgs);
impl From<GrantGmtokenManagerRoleIxArgs> for GrantGmtokenManagerRoleIxData {
    fn from(args: GrantGmtokenManagerRoleIxArgs) -> Self {
        Self(args)
    }
}
impl GrantGmtokenManagerRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GRANT_GMTOKEN_MANAGER_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(GrantGmtokenManagerRoleIxArgs {
                role,
                user,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GRANT_GMTOKEN_MANAGER_ROLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.role, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.user, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn grant_gmtoken_manager_role_ix_with_program_id(
    program_id: Pubkey,
    keys: GrantGmtokenManagerRoleKeys,
    args: GrantGmtokenManagerRoleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GRANT_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: GrantGmtokenManagerRoleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn grant_gmtoken_manager_role_ix(
    keys: GrantGmtokenManagerRoleKeys,
    args: GrantGmtokenManagerRoleIxArgs,
) -> std::io::Result<Instruction> {
    grant_gmtoken_manager_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn grant_gmtoken_manager_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GrantGmtokenManagerRoleAccounts<'_, '_>,
    args: GrantGmtokenManagerRoleIxArgs,
) -> ProgramResult {
    let keys: GrantGmtokenManagerRoleKeys = accounts.into();
    let ix = grant_gmtoken_manager_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn grant_gmtoken_manager_role_invoke(
    accounts: GrantGmtokenManagerRoleAccounts<'_, '_>,
    args: GrantGmtokenManagerRoleIxArgs,
) -> ProgramResult {
    grant_gmtoken_manager_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn grant_gmtoken_manager_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GrantGmtokenManagerRoleAccounts<'_, '_>,
    args: GrantGmtokenManagerRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GrantGmtokenManagerRoleKeys = accounts.into();
    let ix = grant_gmtoken_manager_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn grant_gmtoken_manager_role_invoke_signed(
    accounts: GrantGmtokenManagerRoleAccounts<'_, '_>,
    args: GrantGmtokenManagerRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    grant_gmtoken_manager_role_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn grant_gmtoken_manager_role_verify_account_keys(
    accounts: GrantGmtokenManagerRoleAccounts<'_, '_>,
    keys: GrantGmtokenManagerRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.role_to_grant.key, keys.role_to_grant),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn grant_gmtoken_manager_role_verify_writable_privileges<'me, 'info>(
    accounts: GrantGmtokenManagerRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.role_to_grant] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn grant_gmtoken_manager_role_verify_signer_privileges<'me, 'info>(
    accounts: GrantGmtokenManagerRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn grant_gmtoken_manager_role_verify_account_privileges<'me, 'info>(
    accounts: GrantGmtokenManagerRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    grant_gmtoken_manager_role_verify_writable_privileges(accounts)?;
    grant_gmtoken_manager_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GRANT_GMTOKEN_ROLE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct GrantGmtokenRoleAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub role_to_grant: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GrantGmtokenRoleKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub role_to_grant: Pubkey,
    pub system_program: Pubkey,
}
impl From<GrantGmtokenRoleAccounts<'_, '_>> for GrantGmtokenRoleKeys {
    fn from(accounts: GrantGmtokenRoleAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            role_to_grant: *accounts.role_to_grant.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<GrantGmtokenRoleKeys> for [AccountMeta; GRANT_GMTOKEN_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: GrantGmtokenRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_grant,
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
impl From<[Pubkey; GRANT_GMTOKEN_ROLE_IX_ACCOUNTS_LEN]> for GrantGmtokenRoleKeys {
    fn from(pubkeys: [Pubkey; GRANT_GMTOKEN_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            role_to_grant: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<GrantGmtokenRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; GRANT_GMTOKEN_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GrantGmtokenRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.role_to_grant.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GRANT_GMTOKEN_ROLE_IX_ACCOUNTS_LEN]>
for GrantGmtokenRoleAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GRANT_GMTOKEN_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            role_to_grant: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const GRANT_GMTOKEN_ROLE_IX_DISCM: [u8; 8usize] = [80, 114, 198, 5, 12, 2, 84, 105];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrantGmtokenRoleIxArgs {
    pub role: RoleType,
    pub user: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GrantGmtokenRoleIxData(pub GrantGmtokenRoleIxArgs);
impl From<GrantGmtokenRoleIxArgs> for GrantGmtokenRoleIxData {
    fn from(args: GrantGmtokenRoleIxArgs) -> Self {
        Self(args)
    }
}
impl GrantGmtokenRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GRANT_GMTOKEN_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(GrantGmtokenRoleIxArgs {
                role,
                user,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GRANT_GMTOKEN_ROLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.role, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.user, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn grant_gmtoken_role_ix_with_program_id(
    program_id: Pubkey,
    keys: GrantGmtokenRoleKeys,
    args: GrantGmtokenRoleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GRANT_GMTOKEN_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: GrantGmtokenRoleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn grant_gmtoken_role_ix(
    keys: GrantGmtokenRoleKeys,
    args: GrantGmtokenRoleIxArgs,
) -> std::io::Result<Instruction> {
    grant_gmtoken_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn grant_gmtoken_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GrantGmtokenRoleAccounts<'_, '_>,
    args: GrantGmtokenRoleIxArgs,
) -> ProgramResult {
    let keys: GrantGmtokenRoleKeys = accounts.into();
    let ix = grant_gmtoken_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn grant_gmtoken_role_invoke(
    accounts: GrantGmtokenRoleAccounts<'_, '_>,
    args: GrantGmtokenRoleIxArgs,
) -> ProgramResult {
    grant_gmtoken_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn grant_gmtoken_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GrantGmtokenRoleAccounts<'_, '_>,
    args: GrantGmtokenRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GrantGmtokenRoleKeys = accounts.into();
    let ix = grant_gmtoken_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn grant_gmtoken_role_invoke_signed(
    accounts: GrantGmtokenRoleAccounts<'_, '_>,
    args: GrantGmtokenRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    grant_gmtoken_role_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn grant_gmtoken_role_verify_account_keys(
    accounts: GrantGmtokenRoleAccounts<'_, '_>,
    keys: GrantGmtokenRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.role_to_grant.key, keys.role_to_grant),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn grant_gmtoken_role_verify_writable_privileges<'me, 'info>(
    accounts: GrantGmtokenRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.role_to_grant] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn grant_gmtoken_role_verify_signer_privileges<'me, 'info>(
    accounts: GrantGmtokenRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn grant_gmtoken_role_verify_account_privileges<'me, 'info>(
    accounts: GrantGmtokenRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    grant_gmtoken_role_verify_writable_privileges(accounts)?;
    grant_gmtoken_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GRANT_ROLE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct GrantRoleAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub role_to_grant: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub program_data: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GrantRoleKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub role_to_grant: Pubkey,
    pub system_program: Pubkey,
    pub program: Pubkey,
    pub program_data: Pubkey,
}
impl From<GrantRoleAccounts<'_, '_>> for GrantRoleKeys {
    fn from(accounts: GrantRoleAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            role_to_grant: *accounts.role_to_grant.key,
            system_program: *accounts.system_program.key,
            program: *accounts.program.key,
            program_data: *accounts.program_data.key,
        }
    }
}
impl From<GrantRoleKeys> for [AccountMeta; GRANT_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: GrantRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_grant,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.program_data,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GRANT_ROLE_IX_ACCOUNTS_LEN]> for GrantRoleKeys {
    fn from(pubkeys: [Pubkey; GRANT_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            role_to_grant: pubkeys[2],
            system_program: pubkeys[3],
            program: pubkeys[4],
            program_data: pubkeys[5],
        }
    }
}
impl<'info> From<GrantRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; GRANT_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GrantRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.role_to_grant.clone(),
            accounts.system_program.clone(),
            accounts.program.clone(),
            accounts.program_data.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GRANT_ROLE_IX_ACCOUNTS_LEN]>
for GrantRoleAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GRANT_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            role_to_grant: &arr[2],
            system_program: &arr[3],
            program: &arr[4],
            program_data: &arr[5],
        }
    }
}
pub const GRANT_ROLE_IX_DISCM: [u8; 8usize] = [218, 234, 128, 15, 82, 33, 236, 253];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrantRoleIxArgs {
    pub role: RoleType,
    pub user: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GrantRoleIxData(pub GrantRoleIxArgs);
impl From<GrantRoleIxArgs> for GrantRoleIxData {
    fn from(args: GrantRoleIxArgs) -> Self {
        Self(args)
    }
}
impl GrantRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GRANT_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(GrantRoleIxArgs { role, user }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GRANT_ROLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.role, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.user, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn grant_role_ix_with_program_id(
    program_id: Pubkey,
    keys: GrantRoleKeys,
    args: GrantRoleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GRANT_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: GrantRoleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn grant_role_ix(
    keys: GrantRoleKeys,
    args: GrantRoleIxArgs,
) -> std::io::Result<Instruction> {
    grant_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn grant_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GrantRoleAccounts<'_, '_>,
    args: GrantRoleIxArgs,
) -> ProgramResult {
    let keys: GrantRoleKeys = accounts.into();
    let ix = grant_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn grant_role_invoke(
    accounts: GrantRoleAccounts<'_, '_>,
    args: GrantRoleIxArgs,
) -> ProgramResult {
    grant_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn grant_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GrantRoleAccounts<'_, '_>,
    args: GrantRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GrantRoleKeys = accounts.into();
    let ix = grant_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn grant_role_invoke_signed(
    accounts: GrantRoleAccounts<'_, '_>,
    args: GrantRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    grant_role_invoke_signed_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args, seeds)
}
pub fn grant_role_verify_account_keys(
    accounts: GrantRoleAccounts<'_, '_>,
    keys: GrantRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.role_to_grant.key, keys.role_to_grant),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program.key, keys.program),
        (*accounts.program_data.key, keys.program_data),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn grant_role_verify_writable_privileges<'me, 'info>(
    accounts: GrantRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.role_to_grant] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn grant_role_verify_signer_privileges<'me, 'info>(
    accounts: GrantRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn grant_role_verify_account_privileges<'me, 'info>(
    accounts: GrantRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    grant_role_verify_writable_privileges(accounts)?;
    grant_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GRANT_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct GrantSanityConfigurerRoleAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub role_to_grant: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GrantSanityConfigurerRoleKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub role_to_grant: Pubkey,
    pub system_program: Pubkey,
}
impl From<GrantSanityConfigurerRoleAccounts<'_, '_>> for GrantSanityConfigurerRoleKeys {
    fn from(accounts: GrantSanityConfigurerRoleAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            role_to_grant: *accounts.role_to_grant.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<GrantSanityConfigurerRoleKeys>
for [AccountMeta; GRANT_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: GrantSanityConfigurerRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_grant,
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
impl From<[Pubkey; GRANT_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN]>
for GrantSanityConfigurerRoleKeys {
    fn from(pubkeys: [Pubkey; GRANT_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            role_to_grant: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<GrantSanityConfigurerRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; GRANT_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GrantSanityConfigurerRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.role_to_grant.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; GRANT_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN]>
for GrantSanityConfigurerRoleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; GRANT_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            role_to_grant: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const GRANT_SANITY_CONFIGURER_ROLE_IX_DISCM: [u8; 8usize] = [
    22, 112, 48, 145, 46, 77, 61, 33,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrantSanityConfigurerRoleIxArgs {
    pub role: RoleType,
    pub user: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GrantSanityConfigurerRoleIxData(pub GrantSanityConfigurerRoleIxArgs);
impl From<GrantSanityConfigurerRoleIxArgs> for GrantSanityConfigurerRoleIxData {
    fn from(args: GrantSanityConfigurerRoleIxArgs) -> Self {
        Self(args)
    }
}
impl GrantSanityConfigurerRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GRANT_SANITY_CONFIGURER_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(GrantSanityConfigurerRoleIxArgs {
                role,
                user,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GRANT_SANITY_CONFIGURER_ROLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.role, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.user, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn grant_sanity_configurer_role_ix_with_program_id(
    program_id: Pubkey,
    keys: GrantSanityConfigurerRoleKeys,
    args: GrantSanityConfigurerRoleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GRANT_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: GrantSanityConfigurerRoleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn grant_sanity_configurer_role_ix(
    keys: GrantSanityConfigurerRoleKeys,
    args: GrantSanityConfigurerRoleIxArgs,
) -> std::io::Result<Instruction> {
    grant_sanity_configurer_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn grant_sanity_configurer_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GrantSanityConfigurerRoleAccounts<'_, '_>,
    args: GrantSanityConfigurerRoleIxArgs,
) -> ProgramResult {
    let keys: GrantSanityConfigurerRoleKeys = accounts.into();
    let ix = grant_sanity_configurer_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn grant_sanity_configurer_role_invoke(
    accounts: GrantSanityConfigurerRoleAccounts<'_, '_>,
    args: GrantSanityConfigurerRoleIxArgs,
) -> ProgramResult {
    grant_sanity_configurer_role_invoke_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn grant_sanity_configurer_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GrantSanityConfigurerRoleAccounts<'_, '_>,
    args: GrantSanityConfigurerRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GrantSanityConfigurerRoleKeys = accounts.into();
    let ix = grant_sanity_configurer_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn grant_sanity_configurer_role_invoke_signed(
    accounts: GrantSanityConfigurerRoleAccounts<'_, '_>,
    args: GrantSanityConfigurerRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    grant_sanity_configurer_role_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn grant_sanity_configurer_role_verify_account_keys(
    accounts: GrantSanityConfigurerRoleAccounts<'_, '_>,
    keys: GrantSanityConfigurerRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.role_to_grant.key, keys.role_to_grant),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn grant_sanity_configurer_role_verify_writable_privileges<'me, 'info>(
    accounts: GrantSanityConfigurerRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.role_to_grant] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn grant_sanity_configurer_role_verify_signer_privileges<'me, 'info>(
    accounts: GrantSanityConfigurerRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn grant_sanity_configurer_role_verify_account_privileges<'me, 'info>(
    accounts: GrantSanityConfigurerRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    grant_sanity_configurer_role_verify_writable_privileges(accounts)?;
    grant_sanity_configurer_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GRANT_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct GrantSanitySetterRoleAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub role_to_grant: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GrantSanitySetterRoleKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub role_to_grant: Pubkey,
    pub system_program: Pubkey,
}
impl From<GrantSanitySetterRoleAccounts<'_, '_>> for GrantSanitySetterRoleKeys {
    fn from(accounts: GrantSanitySetterRoleAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            role_to_grant: *accounts.role_to_grant.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<GrantSanitySetterRoleKeys>
for [AccountMeta; GRANT_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: GrantSanitySetterRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_grant,
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
impl From<[Pubkey; GRANT_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN]>
for GrantSanitySetterRoleKeys {
    fn from(pubkeys: [Pubkey; GRANT_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            role_to_grant: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<GrantSanitySetterRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; GRANT_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GrantSanitySetterRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.role_to_grant.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; GRANT_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN]>
for GrantSanitySetterRoleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; GRANT_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            role_to_grant: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const GRANT_SANITY_SETTER_ROLE_IX_DISCM: [u8; 8usize] = [
    209, 154, 110, 150, 18, 113, 63, 227,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrantSanitySetterRoleIxArgs {
    pub role: RoleType,
    pub user: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GrantSanitySetterRoleIxData(pub GrantSanitySetterRoleIxArgs);
impl From<GrantSanitySetterRoleIxArgs> for GrantSanitySetterRoleIxData {
    fn from(args: GrantSanitySetterRoleIxArgs) -> Self {
        Self(args)
    }
}
impl GrantSanitySetterRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GRANT_SANITY_SETTER_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(GrantSanitySetterRoleIxArgs {
                role,
                user,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GRANT_SANITY_SETTER_ROLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.role, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.user, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn grant_sanity_setter_role_ix_with_program_id(
    program_id: Pubkey,
    keys: GrantSanitySetterRoleKeys,
    args: GrantSanitySetterRoleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GRANT_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: GrantSanitySetterRoleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn grant_sanity_setter_role_ix(
    keys: GrantSanitySetterRoleKeys,
    args: GrantSanitySetterRoleIxArgs,
) -> std::io::Result<Instruction> {
    grant_sanity_setter_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn grant_sanity_setter_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GrantSanitySetterRoleAccounts<'_, '_>,
    args: GrantSanitySetterRoleIxArgs,
) -> ProgramResult {
    let keys: GrantSanitySetterRoleKeys = accounts.into();
    let ix = grant_sanity_setter_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn grant_sanity_setter_role_invoke(
    accounts: GrantSanitySetterRoleAccounts<'_, '_>,
    args: GrantSanitySetterRoleIxArgs,
) -> ProgramResult {
    grant_sanity_setter_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn grant_sanity_setter_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GrantSanitySetterRoleAccounts<'_, '_>,
    args: GrantSanitySetterRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GrantSanitySetterRoleKeys = accounts.into();
    let ix = grant_sanity_setter_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn grant_sanity_setter_role_invoke_signed(
    accounts: GrantSanitySetterRoleAccounts<'_, '_>,
    args: GrantSanitySetterRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    grant_sanity_setter_role_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn grant_sanity_setter_role_verify_account_keys(
    accounts: GrantSanitySetterRoleAccounts<'_, '_>,
    keys: GrantSanitySetterRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.role_to_grant.key, keys.role_to_grant),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn grant_sanity_setter_role_verify_writable_privileges<'me, 'info>(
    accounts: GrantSanitySetterRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.role_to_grant] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn grant_sanity_setter_role_verify_signer_privileges<'me, 'info>(
    accounts: GrantSanitySetterRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn grant_sanity_setter_role_verify_account_privileges<'me, 'info>(
    accounts: GrantSanitySetterRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    grant_sanity_setter_role_verify_writable_privileges(accounts)?;
    grant_sanity_setter_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GRANT_USDON_ROLE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct GrantUsdonRoleAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub role_to_grant: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GrantUsdonRoleKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub role_to_grant: Pubkey,
    pub system_program: Pubkey,
}
impl From<GrantUsdonRoleAccounts<'_, '_>> for GrantUsdonRoleKeys {
    fn from(accounts: GrantUsdonRoleAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            role_to_grant: *accounts.role_to_grant.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<GrantUsdonRoleKeys> for [AccountMeta; GRANT_USDON_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: GrantUsdonRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_grant,
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
impl From<[Pubkey; GRANT_USDON_ROLE_IX_ACCOUNTS_LEN]> for GrantUsdonRoleKeys {
    fn from(pubkeys: [Pubkey; GRANT_USDON_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            role_to_grant: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<GrantUsdonRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; GRANT_USDON_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GrantUsdonRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.role_to_grant.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GRANT_USDON_ROLE_IX_ACCOUNTS_LEN]>
for GrantUsdonRoleAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GRANT_USDON_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            role_to_grant: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const GRANT_USDON_ROLE_IX_DISCM: [u8; 8usize] = [107, 112, 6, 81, 156, 182, 208, 9];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrantUsdonRoleIxArgs {
    pub role: RoleType,
    pub user: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GrantUsdonRoleIxData(pub GrantUsdonRoleIxArgs);
impl From<GrantUsdonRoleIxArgs> for GrantUsdonRoleIxData {
    fn from(args: GrantUsdonRoleIxArgs) -> Self {
        Self(args)
    }
}
impl GrantUsdonRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GRANT_USDON_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(GrantUsdonRoleIxArgs { role, user }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GRANT_USDON_ROLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.role, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.user, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn grant_usdon_role_ix_with_program_id(
    program_id: Pubkey,
    keys: GrantUsdonRoleKeys,
    args: GrantUsdonRoleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GRANT_USDON_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: GrantUsdonRoleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn grant_usdon_role_ix(
    keys: GrantUsdonRoleKeys,
    args: GrantUsdonRoleIxArgs,
) -> std::io::Result<Instruction> {
    grant_usdon_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn grant_usdon_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GrantUsdonRoleAccounts<'_, '_>,
    args: GrantUsdonRoleIxArgs,
) -> ProgramResult {
    let keys: GrantUsdonRoleKeys = accounts.into();
    let ix = grant_usdon_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn grant_usdon_role_invoke(
    accounts: GrantUsdonRoleAccounts<'_, '_>,
    args: GrantUsdonRoleIxArgs,
) -> ProgramResult {
    grant_usdon_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn grant_usdon_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GrantUsdonRoleAccounts<'_, '_>,
    args: GrantUsdonRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GrantUsdonRoleKeys = accounts.into();
    let ix = grant_usdon_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn grant_usdon_role_invoke_signed(
    accounts: GrantUsdonRoleAccounts<'_, '_>,
    args: GrantUsdonRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    grant_usdon_role_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn grant_usdon_role_verify_account_keys(
    accounts: GrantUsdonRoleAccounts<'_, '_>,
    keys: GrantUsdonRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.role_to_grant.key, keys.role_to_grant),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn grant_usdon_role_verify_writable_privileges<'me, 'info>(
    accounts: GrantUsdonRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.role_to_grant] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn grant_usdon_role_verify_signer_privileges<'me, 'info>(
    accounts: GrantUsdonRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn grant_usdon_role_verify_account_privileges<'me, 'info>(
    accounts: GrantUsdonRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    grant_usdon_role_verify_writable_privileges(accounts)?;
    grant_usdon_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_MINT_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitMintAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitMintKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub gmtoken_manager_state: Pubkey,
}
impl From<InitMintAccounts<'_, '_>> for InitMintKeys {
    fn from(accounts: InitMintAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
        }
    }
}
impl From<InitMintKeys> for [AccountMeta; INIT_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitMintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
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
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_MINT_IX_ACCOUNTS_LEN]> for InitMintKeys {
    fn from(pubkeys: [Pubkey; INIT_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            mint: pubkeys[3],
            mint_authority: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
            gmtoken_manager_state: pubkeys[7],
        }
    }
}
impl<'info> From<InitMintAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_MINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitMintAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.gmtoken_manager_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_MINT_IX_ACCOUNTS_LEN]>
for InitMintAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            mint: &arr[3],
            mint_authority: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
            gmtoken_manager_state: &arr[7],
        }
    }
}
pub const INIT_MINT_IX_DISCM: [u8; 8usize] = [126, 176, 233, 16, 66, 117, 209, 125];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitMintIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub freeze_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitMintIxData(pub InitMintIxArgs);
impl From<InitMintIxArgs> for InitMintIxData {
    fn from(args: InitMintIxArgs) -> Self {
        Self(args)
    }
}
impl InitMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_MINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        let freeze_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitMintIxArgs {
                name,
                symbol,
                uri,
                freeze_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_MINT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.freeze_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_mint_ix_with_program_id(
    program_id: Pubkey,
    keys: InitMintKeys,
    args: InitMintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitMintIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_mint_ix(
    keys: InitMintKeys,
    args: InitMintIxArgs,
) -> std::io::Result<Instruction> {
    init_mint_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn init_mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitMintAccounts<'_, '_>,
    args: InitMintIxArgs,
) -> ProgramResult {
    let keys: InitMintKeys = accounts.into();
    let ix = init_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_mint_invoke(
    accounts: InitMintAccounts<'_, '_>,
    args: InitMintIxArgs,
) -> ProgramResult {
    init_mint_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn init_mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitMintAccounts<'_, '_>,
    args: InitMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitMintKeys = accounts.into();
    let ix = init_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_mint_invoke_signed(
    accounts: InitMintAccounts<'_, '_>,
    args: InitMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_mint_invoke_signed_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args, seeds)
}
pub fn init_mint_verify_account_keys(
    accounts: InitMintAccounts<'_, '_>,
    keys: InitMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_mint_verify_writable_privileges<'me, 'info>(
    accounts: InitMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_mint_verify_signer_privileges<'me, 'info>(
    accounts: InitMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority, accounts.mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_mint_verify_account_privileges<'me, 'info>(
    accounts: InitMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_mint_verify_writable_privileges(accounts)?;
    init_mint_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_MINT_DELEGATE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitMintDelegateAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitMintDelegateKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub gmtoken_manager_state: Pubkey,
}
impl From<InitMintDelegateAccounts<'_, '_>> for InitMintDelegateKeys {
    fn from(accounts: InitMintDelegateAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
        }
    }
}
impl From<InitMintDelegateKeys> for [AccountMeta; INIT_MINT_DELEGATE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitMintDelegateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
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
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_MINT_DELEGATE_IX_ACCOUNTS_LEN]> for InitMintDelegateKeys {
    fn from(pubkeys: [Pubkey; INIT_MINT_DELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            mint: pubkeys[3],
            mint_authority: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
            gmtoken_manager_state: pubkeys[7],
        }
    }
}
impl<'info> From<InitMintDelegateAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_MINT_DELEGATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitMintDelegateAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.gmtoken_manager_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_MINT_DELEGATE_IX_ACCOUNTS_LEN]>
for InitMintDelegateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_MINT_DELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            mint: &arr[3],
            mint_authority: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
            gmtoken_manager_state: &arr[7],
        }
    }
}
pub const INIT_MINT_DELEGATE_IX_DISCM: [u8; 8usize] = [
    66, 23, 77, 148, 164, 53, 165, 83,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitMintDelegateIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub freeze_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitMintDelegateIxData(pub InitMintDelegateIxArgs);
impl From<InitMintDelegateIxArgs> for InitMintDelegateIxData {
    fn from(args: InitMintDelegateIxArgs) -> Self {
        Self(args)
    }
}
impl InitMintDelegateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_MINT_DELEGATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        let freeze_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitMintDelegateIxArgs {
                name,
                symbol,
                uri,
                freeze_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_MINT_DELEGATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.freeze_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_mint_delegate_ix_with_program_id(
    program_id: Pubkey,
    keys: InitMintDelegateKeys,
    args: InitMintDelegateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_MINT_DELEGATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitMintDelegateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_mint_delegate_ix(
    keys: InitMintDelegateKeys,
    args: InitMintDelegateIxArgs,
) -> std::io::Result<Instruction> {
    init_mint_delegate_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn init_mint_delegate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitMintDelegateAccounts<'_, '_>,
    args: InitMintDelegateIxArgs,
) -> ProgramResult {
    let keys: InitMintDelegateKeys = accounts.into();
    let ix = init_mint_delegate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_mint_delegate_invoke(
    accounts: InitMintDelegateAccounts<'_, '_>,
    args: InitMintDelegateIxArgs,
) -> ProgramResult {
    init_mint_delegate_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn init_mint_delegate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitMintDelegateAccounts<'_, '_>,
    args: InitMintDelegateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitMintDelegateKeys = accounts.into();
    let ix = init_mint_delegate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_mint_delegate_invoke_signed(
    accounts: InitMintDelegateAccounts<'_, '_>,
    args: InitMintDelegateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_mint_delegate_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_mint_delegate_verify_account_keys(
    accounts: InitMintDelegateAccounts<'_, '_>,
    keys: InitMintDelegateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_mint_delegate_verify_writable_privileges<'me, 'info>(
    accounts: InitMintDelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_mint_delegate_verify_signer_privileges<'me, 'info>(
    accounts: InitMintDelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority, accounts.mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_mint_delegate_verify_account_privileges<'me, 'info>(
    accounts: InitMintDelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_mint_delegate_verify_writable_privileges(accounts)?;
    init_mint_delegate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_GMTOKEN_MANAGER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializeGmtokenManagerAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeGmtokenManagerKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub gmtoken_manager_state: Pubkey,
    pub authority_role_account: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeGmtokenManagerAccounts<'_, '_>> for InitializeGmtokenManagerKeys {
    fn from(accounts: InitializeGmtokenManagerAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
            authority_role_account: *accounts.authority_role_account.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeGmtokenManagerKeys>
for [AccountMeta; INITIALIZE_GMTOKEN_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeGmtokenManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
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
impl From<[Pubkey; INITIALIZE_GMTOKEN_MANAGER_IX_ACCOUNTS_LEN]>
for InitializeGmtokenManagerKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_GMTOKEN_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            gmtoken_manager_state: pubkeys[2],
            authority_role_account: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<InitializeGmtokenManagerAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_GMTOKEN_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeGmtokenManagerAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.gmtoken_manager_state.clone(),
            accounts.authority_role_account.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_GMTOKEN_MANAGER_IX_ACCOUNTS_LEN]>
for InitializeGmtokenManagerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_GMTOKEN_MANAGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            gmtoken_manager_state: &arr[2],
            authority_role_account: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const INITIALIZE_GMTOKEN_MANAGER_IX_DISCM: [u8; 8usize] = [
    118, 57, 145, 134, 233, 31, 145, 47,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeGmtokenManagerIxArgs {
    pub factory_paused: bool,
    pub redemptions_paused: bool,
    pub minting_paused: bool,
    pub attestation_signer_secp: [u8; 20],
    pub trading_hours_offset: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeGmtokenManagerIxData(pub InitializeGmtokenManagerIxArgs);
impl From<InitializeGmtokenManagerIxArgs> for InitializeGmtokenManagerIxData {
    fn from(args: InitializeGmtokenManagerIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeGmtokenManagerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_GMTOKEN_MANAGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let factory_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let redemptions_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let minting_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let attestation_signer_secp: [u8; 20] = crate::borsh_de_or_default(&mut reader)?;
        let trading_hours_offset: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeGmtokenManagerIxArgs {
                factory_paused,
                redemptions_paused,
                minting_paused,
                attestation_signer_secp,
                trading_hours_offset,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_GMTOKEN_MANAGER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.factory_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.redemptions_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minting_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.attestation_signer_secp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.trading_hours_offset, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_gmtoken_manager_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeGmtokenManagerKeys,
    args: InitializeGmtokenManagerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_GMTOKEN_MANAGER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeGmtokenManagerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_gmtoken_manager_ix(
    keys: InitializeGmtokenManagerKeys,
    args: InitializeGmtokenManagerIxArgs,
) -> std::io::Result<Instruction> {
    initialize_gmtoken_manager_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn initialize_gmtoken_manager_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeGmtokenManagerAccounts<'_, '_>,
    args: InitializeGmtokenManagerIxArgs,
) -> ProgramResult {
    let keys: InitializeGmtokenManagerKeys = accounts.into();
    let ix = initialize_gmtoken_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_gmtoken_manager_invoke(
    accounts: InitializeGmtokenManagerAccounts<'_, '_>,
    args: InitializeGmtokenManagerIxArgs,
) -> ProgramResult {
    initialize_gmtoken_manager_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn initialize_gmtoken_manager_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeGmtokenManagerAccounts<'_, '_>,
    args: InitializeGmtokenManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeGmtokenManagerKeys = accounts.into();
    let ix = initialize_gmtoken_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_gmtoken_manager_invoke_signed(
    accounts: InitializeGmtokenManagerAccounts<'_, '_>,
    args: InitializeGmtokenManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_gmtoken_manager_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_gmtoken_manager_verify_account_keys(
    accounts: InitializeGmtokenManagerAccounts<'_, '_>,
    keys: InitializeGmtokenManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_gmtoken_manager_verify_writable_privileges<'me, 'info>(
    accounts: InitializeGmtokenManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.gmtoken_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_gmtoken_manager_verify_signer_privileges<'me, 'info>(
    accounts: InitializeGmtokenManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_gmtoken_manager_verify_account_privileges<'me, 'info>(
    accounts: InitializeGmtokenManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_gmtoken_manager_verify_writable_privileges(accounts)?;
    initialize_gmtoken_manager_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_SANITY_CHECK_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializeSanityCheckAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub sanity_check: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeSanityCheckKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub sanity_check: Pubkey,
    pub authority_role_account: Pubkey,
    pub mint: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeSanityCheckAccounts<'_, '_>> for InitializeSanityCheckKeys {
    fn from(accounts: InitializeSanityCheckAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            sanity_check: *accounts.sanity_check.key,
            authority_role_account: *accounts.authority_role_account.key,
            mint: *accounts.mint.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeSanityCheckKeys>
for [AccountMeta; INITIALIZE_SANITY_CHECK_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeSanityCheckKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sanity_check,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
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
impl From<[Pubkey; INITIALIZE_SANITY_CHECK_IX_ACCOUNTS_LEN]>
for InitializeSanityCheckKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_SANITY_CHECK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            sanity_check: pubkeys[2],
            authority_role_account: pubkeys[3],
            mint: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<InitializeSanityCheckAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_SANITY_CHECK_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeSanityCheckAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.sanity_check.clone(),
            accounts.authority_role_account.clone(),
            accounts.mint.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_SANITY_CHECK_IX_ACCOUNTS_LEN]>
for InitializeSanityCheckAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_SANITY_CHECK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            sanity_check: &arr[2],
            authority_role_account: &arr[3],
            mint: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const INITIALIZE_SANITY_CHECK_IX_DISCM: [u8; 8usize] = [
    99, 169, 10, 74, 68, 74, 22, 72,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeSanityCheckIxArgs {
    pub last_price: u64,
    pub allowed_deviation_bps: u64,
    pub max_time_delay: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeSanityCheckIxData(pub InitializeSanityCheckIxArgs);
impl From<InitializeSanityCheckIxArgs> for InitializeSanityCheckIxData {
    fn from(args: InitializeSanityCheckIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeSanityCheckIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_SANITY_CHECK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let last_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let allowed_deviation_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_time_delay: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeSanityCheckIxArgs {
                last_price,
                allowed_deviation_bps,
                max_time_delay,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_SANITY_CHECK_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.last_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.allowed_deviation_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_time_delay, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_sanity_check_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeSanityCheckKeys,
    args: InitializeSanityCheckIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_SANITY_CHECK_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeSanityCheckIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_sanity_check_ix(
    keys: InitializeSanityCheckKeys,
    args: InitializeSanityCheckIxArgs,
) -> std::io::Result<Instruction> {
    initialize_sanity_check_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn initialize_sanity_check_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeSanityCheckAccounts<'_, '_>,
    args: InitializeSanityCheckIxArgs,
) -> ProgramResult {
    let keys: InitializeSanityCheckKeys = accounts.into();
    let ix = initialize_sanity_check_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_sanity_check_invoke(
    accounts: InitializeSanityCheckAccounts<'_, '_>,
    args: InitializeSanityCheckIxArgs,
) -> ProgramResult {
    initialize_sanity_check_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn initialize_sanity_check_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeSanityCheckAccounts<'_, '_>,
    args: InitializeSanityCheckIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeSanityCheckKeys = accounts.into();
    let ix = initialize_sanity_check_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_sanity_check_invoke_signed(
    accounts: InitializeSanityCheckAccounts<'_, '_>,
    args: InitializeSanityCheckIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_sanity_check_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_sanity_check_verify_account_keys(
    accounts: InitializeSanityCheckAccounts<'_, '_>,
    keys: InitializeSanityCheckKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.sanity_check.key, keys.sanity_check),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_sanity_check_verify_writable_privileges<'me, 'info>(
    accounts: InitializeSanityCheckAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.sanity_check] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_sanity_check_verify_signer_privileges<'me, 'info>(
    accounts: InitializeSanityCheckAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_sanity_check_verify_account_privileges<'me, 'info>(
    accounts: InitializeSanityCheckAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_sanity_check_verify_writable_privileges(accounts)?;
    initialize_sanity_check_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_TOKEN_LIMIT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializeTokenLimitAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_limit: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeTokenLimitKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub token_limit: Pubkey,
    pub authority_role_account: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeTokenLimitAccounts<'_, '_>> for InitializeTokenLimitKeys {
    fn from(accounts: InitializeTokenLimitAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            mint: *accounts.mint.key,
            token_limit: *accounts.token_limit.key,
            authority_role_account: *accounts.authority_role_account.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeTokenLimitKeys>
for [AccountMeta; INITIALIZE_TOKEN_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeTokenLimitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_limit,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
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
impl From<[Pubkey; INITIALIZE_TOKEN_LIMIT_IX_ACCOUNTS_LEN]>
for InitializeTokenLimitKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_TOKEN_LIMIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            mint: pubkeys[2],
            token_limit: pubkeys[3],
            authority_role_account: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<InitializeTokenLimitAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_TOKEN_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeTokenLimitAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.mint.clone(),
            accounts.token_limit.clone(),
            accounts.authority_role_account.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_TOKEN_LIMIT_IX_ACCOUNTS_LEN]>
for InitializeTokenLimitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_TOKEN_LIMIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            mint: &arr[2],
            token_limit: &arr[3],
            authority_role_account: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const INITIALIZE_TOKEN_LIMIT_IX_DISCM: [u8; 8usize] = [
    187, 218, 162, 146, 4, 49, 163, 240,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeTokenLimitIxArgs {
    pub rate_limit: Option<u64>,
    pub limit_window: Option<u64>,
    pub default_user_rate_limit: Option<u64>,
    pub default_limit_window: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTokenLimitIxData(pub InitializeTokenLimitIxArgs);
impl From<InitializeTokenLimitIxArgs> for InitializeTokenLimitIxData {
    fn from(args: InitializeTokenLimitIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeTokenLimitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_TOKEN_LIMIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let rate_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let limit_window: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let default_user_rate_limit: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let default_limit_window: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeTokenLimitIxArgs {
                rate_limit,
                limit_window,
                default_user_rate_limit,
                default_limit_window,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_TOKEN_LIMIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.rate_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.limit_window, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.default_user_rate_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.default_limit_window, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_token_limit_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeTokenLimitKeys,
    args: InitializeTokenLimitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_TOKEN_LIMIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeTokenLimitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_token_limit_ix(
    keys: InitializeTokenLimitKeys,
    args: InitializeTokenLimitIxArgs,
) -> std::io::Result<Instruction> {
    initialize_token_limit_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn initialize_token_limit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTokenLimitAccounts<'_, '_>,
    args: InitializeTokenLimitIxArgs,
) -> ProgramResult {
    let keys: InitializeTokenLimitKeys = accounts.into();
    let ix = initialize_token_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_token_limit_invoke(
    accounts: InitializeTokenLimitAccounts<'_, '_>,
    args: InitializeTokenLimitIxArgs,
) -> ProgramResult {
    initialize_token_limit_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn initialize_token_limit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTokenLimitAccounts<'_, '_>,
    args: InitializeTokenLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeTokenLimitKeys = accounts.into();
    let ix = initialize_token_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_token_limit_invoke_signed(
    accounts: InitializeTokenLimitAccounts<'_, '_>,
    args: InitializeTokenLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_token_limit_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_token_limit_verify_account_keys(
    accounts: InitializeTokenLimitAccounts<'_, '_>,
    keys: InitializeTokenLimitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_limit.key, keys.token_limit),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_token_limit_verify_writable_privileges<'me, 'info>(
    accounts: InitializeTokenLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.token_limit] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_token_limit_verify_signer_privileges<'me, 'info>(
    accounts: InitializeTokenLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_token_limit_verify_account_privileges<'me, 'info>(
    accounts: InitializeTokenLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_token_limit_verify_writable_privileges(accounts)?;
    initialize_token_limit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_USDON_MANAGER_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializeUsdonManagerAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub usdon_mint: &'me AccountInfo<'info>,
    pub usdon_vault: &'me AccountInfo<'info>,
    pub usdc_vault: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeUsdonManagerKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub mint_authority: Pubkey,
    pub usdon_mint: Pubkey,
    pub usdon_vault: Pubkey,
    pub usdc_vault: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub authority_role_account: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeUsdonManagerAccounts<'_, '_>> for InitializeUsdonManagerKeys {
    fn from(accounts: InitializeUsdonManagerAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            mint_authority: *accounts.mint_authority.key,
            usdon_mint: *accounts.usdon_mint.key,
            usdon_vault: *accounts.usdon_vault.key,
            usdc_vault: *accounts.usdc_vault.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            authority_role_account: *accounts.authority_role_account.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeUsdonManagerKeys>
for [AccountMeta; INITIALIZE_USDON_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeUsdonManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdon_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdon_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
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
impl From<[Pubkey; INITIALIZE_USDON_MANAGER_IX_ACCOUNTS_LEN]>
for InitializeUsdonManagerKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_USDON_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            mint_authority: pubkeys[2],
            usdon_mint: pubkeys[3],
            usdon_vault: pubkeys[4],
            usdc_vault: pubkeys[5],
            usdon_manager_state: pubkeys[6],
            authority_role_account: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializeUsdonManagerAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_USDON_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeUsdonManagerAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.mint_authority.clone(),
            accounts.usdon_mint.clone(),
            accounts.usdon_vault.clone(),
            accounts.usdc_vault.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.authority_role_account.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_USDON_MANAGER_IX_ACCOUNTS_LEN]>
for InitializeUsdonManagerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_USDON_MANAGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            mint_authority: &arr[2],
            usdon_mint: &arr[3],
            usdon_vault: &arr[4],
            usdc_vault: &arr[5],
            usdon_manager_state: &arr[6],
            authority_role_account: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const INITIALIZE_USDON_MANAGER_IX_DISCM: [u8; 8usize] = [
    9, 32, 232, 105, 2, 166, 13, 47,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeUsdonManagerIxArgs {
    pub oracle_price_enabled: bool,
    pub oracle_price_max_age: u64,
    pub usdc_price_update_address: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeUsdonManagerIxData(pub InitializeUsdonManagerIxArgs);
impl From<InitializeUsdonManagerIxArgs> for InitializeUsdonManagerIxData {
    fn from(args: InitializeUsdonManagerIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeUsdonManagerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_USDON_MANAGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let oracle_price_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let oracle_price_max_age: u64 = crate::borsh_de_or_default(&mut reader)?;
        let usdc_price_update_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeUsdonManagerIxArgs {
                oracle_price_enabled,
                oracle_price_max_age,
                usdc_price_update_address,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_USDON_MANAGER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_price_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_price_max_age, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.usdc_price_update_address,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_usdon_manager_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeUsdonManagerKeys,
    args: InitializeUsdonManagerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_USDON_MANAGER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeUsdonManagerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_usdon_manager_ix(
    keys: InitializeUsdonManagerKeys,
    args: InitializeUsdonManagerIxArgs,
) -> std::io::Result<Instruction> {
    initialize_usdon_manager_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn initialize_usdon_manager_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeUsdonManagerAccounts<'_, '_>,
    args: InitializeUsdonManagerIxArgs,
) -> ProgramResult {
    let keys: InitializeUsdonManagerKeys = accounts.into();
    let ix = initialize_usdon_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_usdon_manager_invoke(
    accounts: InitializeUsdonManagerAccounts<'_, '_>,
    args: InitializeUsdonManagerIxArgs,
) -> ProgramResult {
    initialize_usdon_manager_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn initialize_usdon_manager_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeUsdonManagerAccounts<'_, '_>,
    args: InitializeUsdonManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeUsdonManagerKeys = accounts.into();
    let ix = initialize_usdon_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_usdon_manager_invoke_signed(
    accounts: InitializeUsdonManagerAccounts<'_, '_>,
    args: InitializeUsdonManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_usdon_manager_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_usdon_manager_verify_account_keys(
    accounts: InitializeUsdonManagerAccounts<'_, '_>,
    keys: InitializeUsdonManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.usdon_mint.key, keys.usdon_mint),
        (*accounts.usdon_vault.key, keys.usdon_vault),
        (*accounts.usdc_vault.key, keys.usdc_vault),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_usdon_manager_verify_writable_privileges<'me, 'info>(
    accounts: InitializeUsdonManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.usdon_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_usdon_manager_verify_signer_privileges<'me, 'info>(
    accounts: InitializeUsdonManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_usdon_manager_verify_account_privileges<'me, 'info>(
    accounts: InitializeUsdonManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_usdon_manager_verify_writable_privileges(accounts)?;
    initialize_usdon_manager_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_USER_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeUserAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub ondo_user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeUserKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub user: Pubkey,
    pub mint: Pubkey,
    pub authority_role_account: Pubkey,
    pub ondo_user: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeUserAccounts<'_, '_>> for InitializeUserKeys {
    fn from(accounts: InitializeUserAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            user: *accounts.user.key,
            mint: *accounts.mint.key,
            authority_role_account: *accounts.authority_role_account.key,
            ondo_user: *accounts.ondo_user.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeUserKeys> for [AccountMeta; INITIALIZE_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeUserKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ondo_user,
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
impl From<[Pubkey; INITIALIZE_USER_IX_ACCOUNTS_LEN]> for InitializeUserKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            user: pubkeys[2],
            mint: pubkeys[3],
            authority_role_account: pubkeys[4],
            ondo_user: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeUserAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeUserAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.user.clone(),
            accounts.mint.clone(),
            accounts.authority_role_account.clone(),
            accounts.ondo_user.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN]>
for InitializeUserAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            user: &arr[2],
            mint: &arr[3],
            authority_role_account: &arr[4],
            ondo_user: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const INITIALIZE_USER_IX_DISCM: [u8; 8usize] = [111, 17, 185, 250, 60, 122, 38, 254];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeUserIxArgs {
    pub rate_limit: Option<u64>,
    pub limit_window: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeUserIxData(pub InitializeUserIxArgs);
impl From<InitializeUserIxArgs> for InitializeUserIxData {
    fn from(args: InitializeUserIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeUserIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_USER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let rate_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let limit_window: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeUserIxArgs {
                rate_limit,
                limit_window,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_USER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.rate_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.limit_window, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_user_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeUserKeys,
    args: InitializeUserIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_USER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeUserIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_user_ix(
    keys: InitializeUserKeys,
    args: InitializeUserIxArgs,
) -> std::io::Result<Instruction> {
    initialize_user_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn initialize_user_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeUserAccounts<'_, '_>,
    args: InitializeUserIxArgs,
) -> ProgramResult {
    let keys: InitializeUserKeys = accounts.into();
    let ix = initialize_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_user_invoke(
    accounts: InitializeUserAccounts<'_, '_>,
    args: InitializeUserIxArgs,
) -> ProgramResult {
    initialize_user_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn initialize_user_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeUserAccounts<'_, '_>,
    args: InitializeUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeUserKeys = accounts.into();
    let ix = initialize_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_user_invoke_signed(
    accounts: InitializeUserAccounts<'_, '_>,
    args: InitializeUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_user_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_user_verify_account_keys(
    accounts: InitializeUserAccounts<'_, '_>,
    keys: InitializeUserKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.user.key, keys.user),
        (*accounts.mint.key, keys.mint),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.ondo_user.key, keys.ondo_user),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_user_verify_writable_privileges<'me, 'info>(
    accounts: InitializeUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.ondo_user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_user_verify_signer_privileges<'me, 'info>(
    accounts: InitializeUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_user_verify_account_privileges<'me, 'info>(
    accounts: InitializeUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_user_verify_writable_privileges(accounts)?;
    initialize_user_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_GM_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct MintGmAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub oracle_sanity_check: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintGmKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub user: Pubkey,
    pub authority_role_account: Pubkey,
    pub oracle_sanity_check: Pubkey,
    pub mint_authority: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<MintGmAccounts<'_, '_>> for MintGmKeys {
    fn from(accounts: MintGmAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            user: *accounts.user.key,
            authority_role_account: *accounts.authority_role_account.key,
            oracle_sanity_check: *accounts.oracle_sanity_check.key,
            mint_authority: *accounts.mint_authority.key,
            mint: *accounts.mint.key,
            destination: *accounts.destination.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MintGmKeys> for [AccountMeta; MINT_GM_IX_ACCOUNTS_LEN] {
    fn from(keys: MintGmKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_sanity_check,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
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
impl From<[Pubkey; MINT_GM_IX_ACCOUNTS_LEN]> for MintGmKeys {
    fn from(pubkeys: [Pubkey; MINT_GM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            user: pubkeys[2],
            authority_role_account: pubkeys[3],
            oracle_sanity_check: pubkeys[4],
            mint_authority: pubkeys[5],
            mint: pubkeys[6],
            destination: pubkeys[7],
            usdon_manager_state: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
        }
    }
}
impl<'info> From<MintGmAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_GM_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintGmAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.user.clone(),
            accounts.authority_role_account.clone(),
            accounts.oracle_sanity_check.clone(),
            accounts.mint_authority.clone(),
            accounts.mint.clone(),
            accounts.destination.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_GM_IX_ACCOUNTS_LEN]>
for MintGmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_GM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            user: &arr[2],
            authority_role_account: &arr[3],
            oracle_sanity_check: &arr[4],
            mint_authority: &arr[5],
            mint: &arr[6],
            destination: &arr[7],
            usdon_manager_state: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
        }
    }
}
pub const MINT_GM_IX_DISCM: [u8; 8usize] = [117, 223, 58, 111, 44, 36, 16, 43];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintGmIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintGmIxData(pub MintGmIxArgs);
impl From<MintGmIxArgs> for MintGmIxData {
    fn from(args: MintGmIxArgs) -> Self {
        Self(args)
    }
}
impl MintGmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_GM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(MintGmIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_GM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_gm_ix_with_program_id(
    program_id: Pubkey,
    keys: MintGmKeys,
    args: MintGmIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_GM_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintGmIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_gm_ix(keys: MintGmKeys, args: MintGmIxArgs) -> std::io::Result<Instruction> {
    mint_gm_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn mint_gm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintGmAccounts<'_, '_>,
    args: MintGmIxArgs,
) -> ProgramResult {
    let keys: MintGmKeys = accounts.into();
    let ix = mint_gm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_gm_invoke(
    accounts: MintGmAccounts<'_, '_>,
    args: MintGmIxArgs,
) -> ProgramResult {
    mint_gm_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn mint_gm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintGmAccounts<'_, '_>,
    args: MintGmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintGmKeys = accounts.into();
    let ix = mint_gm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_gm_invoke_signed(
    accounts: MintGmAccounts<'_, '_>,
    args: MintGmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_gm_invoke_signed_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args, seeds)
}
pub fn mint_gm_verify_account_keys(
    accounts: MintGmAccounts<'_, '_>,
    keys: MintGmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.user.key, keys.user),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.oracle_sanity_check.key, keys.oracle_sanity_check),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.mint.key, keys.mint),
        (*accounts.destination.key, keys.destination),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
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
pub fn mint_gm_verify_writable_privileges<'me, 'info>(
    accounts: MintGmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.oracle_sanity_check,
        accounts.mint,
        accounts.destination,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_gm_verify_signer_privileges<'me, 'info>(
    accounts: MintGmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_gm_verify_account_privileges<'me, 'info>(
    accounts: MintGmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_gm_verify_writable_privileges(accounts)?;
    mint_gm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_USDON_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct MintUsdonAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintUsdonKeys {
    pub authority: Pubkey,
    pub mint_authority: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub authority_role_account: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub destination: Pubkey,
}
impl From<MintUsdonAccounts<'_, '_>> for MintUsdonKeys {
    fn from(accounts: MintUsdonAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            mint_authority: *accounts.mint_authority.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            authority_role_account: *accounts.authority_role_account.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            destination: *accounts.destination.key,
        }
    }
}
impl From<MintUsdonKeys> for [AccountMeta; MINT_USDON_IX_ACCOUNTS_LEN] {
    fn from(keys: MintUsdonKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
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
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; MINT_USDON_IX_ACCOUNTS_LEN]> for MintUsdonKeys {
    fn from(pubkeys: [Pubkey; MINT_USDON_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            mint_authority: pubkeys[1],
            usdon_manager_state: pubkeys[2],
            authority_role_account: pubkeys[3],
            mint: pubkeys[4],
            token_program: pubkeys[5],
            destination: pubkeys[6],
        }
    }
}
impl<'info> From<MintUsdonAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_USDON_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintUsdonAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.mint_authority.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.authority_role_account.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.destination.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_USDON_IX_ACCOUNTS_LEN]>
for MintUsdonAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_USDON_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            mint_authority: &arr[1],
            usdon_manager_state: &arr[2],
            authority_role_account: &arr[3],
            mint: &arr[4],
            token_program: &arr[5],
            destination: &arr[6],
        }
    }
}
pub const MINT_USDON_IX_DISCM: [u8; 8usize] = [178, 178, 234, 133, 225, 144, 48, 129];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintUsdonIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintUsdonIxData(pub MintUsdonIxArgs);
impl From<MintUsdonIxArgs> for MintUsdonIxData {
    fn from(args: MintUsdonIxArgs) -> Self {
        Self(args)
    }
}
impl MintUsdonIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_USDON_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(MintUsdonIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_USDON_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_usdon_ix_with_program_id(
    program_id: Pubkey,
    keys: MintUsdonKeys,
    args: MintUsdonIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_USDON_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintUsdonIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_usdon_ix(
    keys: MintUsdonKeys,
    args: MintUsdonIxArgs,
) -> std::io::Result<Instruction> {
    mint_usdon_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn mint_usdon_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintUsdonAccounts<'_, '_>,
    args: MintUsdonIxArgs,
) -> ProgramResult {
    let keys: MintUsdonKeys = accounts.into();
    let ix = mint_usdon_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_usdon_invoke(
    accounts: MintUsdonAccounts<'_, '_>,
    args: MintUsdonIxArgs,
) -> ProgramResult {
    mint_usdon_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn mint_usdon_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintUsdonAccounts<'_, '_>,
    args: MintUsdonIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintUsdonKeys = accounts.into();
    let ix = mint_usdon_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_usdon_invoke_signed(
    accounts: MintUsdonAccounts<'_, '_>,
    args: MintUsdonIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_usdon_invoke_signed_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args, seeds)
}
pub fn mint_usdon_verify_account_keys(
    accounts: MintUsdonAccounts<'_, '_>,
    keys: MintUsdonKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.destination.key, keys.destination),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_usdon_verify_writable_privileges<'me, 'info>(
    accounts: MintUsdonAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.mint, accounts.destination] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_usdon_verify_signer_privileges<'me, 'info>(
    accounts: MintUsdonAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_usdon_verify_account_privileges<'me, 'info>(
    accounts: MintUsdonAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_usdon_verify_writable_privileges(accounts)?;
    mint_usdon_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_WITH_USDC_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct MintWithUsdcAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub ondo_user: &'me AccountInfo<'info>,
    pub token_limit_account: &'me AccountInfo<'info>,
    pub sanity_check_account: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub attestation_id_account: &'me AccountInfo<'info>,
    pub whitelist: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub spl_token_program: &'me AccountInfo<'info>,
    pub usdc_price_update: &'me AccountInfo<'info>,
    pub usdc_vault: &'me AccountInfo<'info>,
    pub usdon_vault: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub user_usdc_token_account: &'me AccountInfo<'info>,
    pub usdon_mint: &'me AccountInfo<'info>,
    pub user_usdon_token_account: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
    pub instructions: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintWithUsdcKeys {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub ondo_user: Pubkey,
    pub token_limit_account: Pubkey,
    pub sanity_check_account: Pubkey,
    pub user_token_account: Pubkey,
    pub attestation_id_account: Pubkey,
    pub whitelist: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub spl_token_program: Pubkey,
    pub usdc_price_update: Pubkey,
    pub usdc_vault: Pubkey,
    pub usdon_vault: Pubkey,
    pub usdc_mint: Pubkey,
    pub user_usdc_token_account: Pubkey,
    pub usdon_mint: Pubkey,
    pub user_usdon_token_account: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub gmtoken_manager_state: Pubkey,
    pub instructions: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MintWithUsdcAccounts<'_, '_>> for MintWithUsdcKeys {
    fn from(accounts: MintWithUsdcAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            ondo_user: *accounts.ondo_user.key,
            token_limit_account: *accounts.token_limit_account.key,
            sanity_check_account: *accounts.sanity_check_account.key,
            user_token_account: *accounts.user_token_account.key,
            attestation_id_account: *accounts.attestation_id_account.key,
            whitelist: *accounts.whitelist.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            spl_token_program: *accounts.spl_token_program.key,
            usdc_price_update: *accounts.usdc_price_update.key,
            usdc_vault: *accounts.usdc_vault.key,
            usdon_vault: *accounts.usdon_vault.key,
            usdc_mint: *accounts.usdc_mint.key,
            user_usdc_token_account: *accounts.user_usdc_token_account.key,
            usdon_mint: *accounts.usdon_mint.key,
            user_usdon_token_account: *accounts.user_usdon_token_account.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
            instructions: *accounts.instructions.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MintWithUsdcKeys> for [AccountMeta; MINT_WITH_USDC_IX_ACCOUNTS_LEN] {
    fn from(keys: MintWithUsdcKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ondo_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_limit_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sanity_check_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.attestation_id_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.whitelist,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.spl_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_price_update,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdon_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_usdc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdon_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_usdon_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instructions,
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
impl From<[Pubkey; MINT_WITH_USDC_IX_ACCOUNTS_LEN]> for MintWithUsdcKeys {
    fn from(pubkeys: [Pubkey; MINT_WITH_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            mint: pubkeys[1],
            mint_authority: pubkeys[2],
            ondo_user: pubkeys[3],
            token_limit_account: pubkeys[4],
            sanity_check_account: pubkeys[5],
            user_token_account: pubkeys[6],
            attestation_id_account: pubkeys[7],
            whitelist: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            associated_token_program: pubkeys[11],
            spl_token_program: pubkeys[12],
            usdc_price_update: pubkeys[13],
            usdc_vault: pubkeys[14],
            usdon_vault: pubkeys[15],
            usdc_mint: pubkeys[16],
            user_usdc_token_account: pubkeys[17],
            usdon_mint: pubkeys[18],
            user_usdon_token_account: pubkeys[19],
            usdon_manager_state: pubkeys[20],
            gmtoken_manager_state: pubkeys[21],
            instructions: pubkeys[22],
            event_authority: pubkeys[23],
            program: pubkeys[24],
        }
    }
}
impl<'info> From<MintWithUsdcAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_WITH_USDC_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintWithUsdcAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.ondo_user.clone(),
            accounts.token_limit_account.clone(),
            accounts.sanity_check_account.clone(),
            accounts.user_token_account.clone(),
            accounts.attestation_id_account.clone(),
            accounts.whitelist.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.spl_token_program.clone(),
            accounts.usdc_price_update.clone(),
            accounts.usdc_vault.clone(),
            accounts.usdon_vault.clone(),
            accounts.usdc_mint.clone(),
            accounts.user_usdc_token_account.clone(),
            accounts.usdon_mint.clone(),
            accounts.user_usdon_token_account.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.gmtoken_manager_state.clone(),
            accounts.instructions.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_WITH_USDC_IX_ACCOUNTS_LEN]>
for MintWithUsdcAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_WITH_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            mint: &arr[1],
            mint_authority: &arr[2],
            ondo_user: &arr[3],
            token_limit_account: &arr[4],
            sanity_check_account: &arr[5],
            user_token_account: &arr[6],
            attestation_id_account: &arr[7],
            whitelist: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            associated_token_program: &arr[11],
            spl_token_program: &arr[12],
            usdc_price_update: &arr[13],
            usdc_vault: &arr[14],
            usdon_vault: &arr[15],
            usdc_mint: &arr[16],
            user_usdc_token_account: &arr[17],
            usdon_mint: &arr[18],
            user_usdon_token_account: &arr[19],
            usdon_manager_state: &arr[20],
            gmtoken_manager_state: &arr[21],
            instructions: &arr[22],
            event_authority: &arr[23],
            program: &arr[24],
        }
    }
}
pub const MINT_WITH_USDC_IX_DISCM: [u8; 8usize] = [128, 28, 133, 173, 71, 142, 185, 206];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintWithUsdcIxArgs {
    pub attestation_id: [u8; 16],
    pub price: u64,
    pub amount: u64,
    pub expiration: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintWithUsdcIxData(pub MintWithUsdcIxArgs);
impl From<MintWithUsdcIxArgs> for MintWithUsdcIxData {
    fn from(args: MintWithUsdcIxArgs) -> Self {
        Self(args)
    }
}
impl MintWithUsdcIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_WITH_USDC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let attestation_id: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expiration: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(MintWithUsdcIxArgs {
                attestation_id,
                price,
                amount,
                expiration,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_WITH_USDC_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.attestation_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.expiration, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_with_usdc_ix_with_program_id(
    program_id: Pubkey,
    keys: MintWithUsdcKeys,
    args: MintWithUsdcIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_WITH_USDC_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintWithUsdcIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_with_usdc_ix(
    keys: MintWithUsdcKeys,
    args: MintWithUsdcIxArgs,
) -> std::io::Result<Instruction> {
    mint_with_usdc_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn mint_with_usdc_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintWithUsdcAccounts<'_, '_>,
    args: MintWithUsdcIxArgs,
) -> ProgramResult {
    let keys: MintWithUsdcKeys = accounts.into();
    let ix = mint_with_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_with_usdc_invoke(
    accounts: MintWithUsdcAccounts<'_, '_>,
    args: MintWithUsdcIxArgs,
) -> ProgramResult {
    mint_with_usdc_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn mint_with_usdc_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintWithUsdcAccounts<'_, '_>,
    args: MintWithUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintWithUsdcKeys = accounts.into();
    let ix = mint_with_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_with_usdc_invoke_signed(
    accounts: MintWithUsdcAccounts<'_, '_>,
    args: MintWithUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_with_usdc_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mint_with_usdc_verify_account_keys(
    accounts: MintWithUsdcAccounts<'_, '_>,
    keys: MintWithUsdcKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.ondo_user.key, keys.ondo_user),
        (*accounts.token_limit_account.key, keys.token_limit_account),
        (*accounts.sanity_check_account.key, keys.sanity_check_account),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.attestation_id_account.key, keys.attestation_id_account),
        (*accounts.whitelist.key, keys.whitelist),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.spl_token_program.key, keys.spl_token_program),
        (*accounts.usdc_price_update.key, keys.usdc_price_update),
        (*accounts.usdc_vault.key, keys.usdc_vault),
        (*accounts.usdon_vault.key, keys.usdon_vault),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.user_usdc_token_account.key, keys.user_usdc_token_account),
        (*accounts.usdon_mint.key, keys.usdon_mint),
        (*accounts.user_usdon_token_account.key, keys.user_usdon_token_account),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
        (*accounts.instructions.key, keys.instructions),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_with_usdc_verify_writable_privileges<'me, 'info>(
    accounts: MintWithUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.mint,
        accounts.ondo_user,
        accounts.token_limit_account,
        accounts.sanity_check_account,
        accounts.user_token_account,
        accounts.attestation_id_account,
        accounts.usdc_vault,
        accounts.usdon_vault,
        accounts.user_usdc_token_account,
        accounts.usdon_mint,
        accounts.user_usdon_token_account,
        accounts.gmtoken_manager_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_with_usdc_verify_signer_privileges<'me, 'info>(
    accounts: MintWithUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_with_usdc_verify_account_privileges<'me, 'info>(
    accounts: MintWithUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_with_usdc_verify_writable_privileges(accounts)?;
    mint_with_usdc_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_WITH_USDON_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct MintWithUsdonAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub ondo_user: &'me AccountInfo<'info>,
    pub token_limit_account: &'me AccountInfo<'info>,
    pub sanity_check_account: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub attestation_id_account: &'me AccountInfo<'info>,
    pub whitelist: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub usdon_vault: &'me AccountInfo<'info>,
    pub usdon_mint: &'me AccountInfo<'info>,
    pub user_usdon_token_account: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
    pub instructions: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintWithUsdonKeys {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub ondo_user: Pubkey,
    pub token_limit_account: Pubkey,
    pub sanity_check_account: Pubkey,
    pub user_token_account: Pubkey,
    pub attestation_id_account: Pubkey,
    pub whitelist: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub usdon_vault: Pubkey,
    pub usdon_mint: Pubkey,
    pub user_usdon_token_account: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub gmtoken_manager_state: Pubkey,
    pub instructions: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MintWithUsdonAccounts<'_, '_>> for MintWithUsdonKeys {
    fn from(accounts: MintWithUsdonAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            ondo_user: *accounts.ondo_user.key,
            token_limit_account: *accounts.token_limit_account.key,
            sanity_check_account: *accounts.sanity_check_account.key,
            user_token_account: *accounts.user_token_account.key,
            attestation_id_account: *accounts.attestation_id_account.key,
            whitelist: *accounts.whitelist.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            usdon_vault: *accounts.usdon_vault.key,
            usdon_mint: *accounts.usdon_mint.key,
            user_usdon_token_account: *accounts.user_usdon_token_account.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
            instructions: *accounts.instructions.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MintWithUsdonKeys> for [AccountMeta; MINT_WITH_USDON_IX_ACCOUNTS_LEN] {
    fn from(keys: MintWithUsdonKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ondo_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_limit_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sanity_check_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.attestation_id_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.whitelist,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdon_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdon_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_usdon_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instructions,
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
impl From<[Pubkey; MINT_WITH_USDON_IX_ACCOUNTS_LEN]> for MintWithUsdonKeys {
    fn from(pubkeys: [Pubkey; MINT_WITH_USDON_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            mint: pubkeys[1],
            mint_authority: pubkeys[2],
            ondo_user: pubkeys[3],
            token_limit_account: pubkeys[4],
            sanity_check_account: pubkeys[5],
            user_token_account: pubkeys[6],
            attestation_id_account: pubkeys[7],
            whitelist: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            associated_token_program: pubkeys[11],
            usdon_vault: pubkeys[12],
            usdon_mint: pubkeys[13],
            user_usdon_token_account: pubkeys[14],
            usdon_manager_state: pubkeys[15],
            gmtoken_manager_state: pubkeys[16],
            instructions: pubkeys[17],
            event_authority: pubkeys[18],
            program: pubkeys[19],
        }
    }
}
impl<'info> From<MintWithUsdonAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_WITH_USDON_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintWithUsdonAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.ondo_user.clone(),
            accounts.token_limit_account.clone(),
            accounts.sanity_check_account.clone(),
            accounts.user_token_account.clone(),
            accounts.attestation_id_account.clone(),
            accounts.whitelist.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.usdon_vault.clone(),
            accounts.usdon_mint.clone(),
            accounts.user_usdon_token_account.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.gmtoken_manager_state.clone(),
            accounts.instructions.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_WITH_USDON_IX_ACCOUNTS_LEN]>
for MintWithUsdonAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_WITH_USDON_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            mint: &arr[1],
            mint_authority: &arr[2],
            ondo_user: &arr[3],
            token_limit_account: &arr[4],
            sanity_check_account: &arr[5],
            user_token_account: &arr[6],
            attestation_id_account: &arr[7],
            whitelist: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            associated_token_program: &arr[11],
            usdon_vault: &arr[12],
            usdon_mint: &arr[13],
            user_usdon_token_account: &arr[14],
            usdon_manager_state: &arr[15],
            gmtoken_manager_state: &arr[16],
            instructions: &arr[17],
            event_authority: &arr[18],
            program: &arr[19],
        }
    }
}
pub const MINT_WITH_USDON_IX_DISCM: [u8; 8usize] = [25, 116, 92, 45, 43, 188, 95, 58];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintWithUsdonIxArgs {
    pub attestation_id: [u8; 16],
    pub price: u64,
    pub amount: u64,
    pub expiration: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintWithUsdonIxData(pub MintWithUsdonIxArgs);
impl From<MintWithUsdonIxArgs> for MintWithUsdonIxData {
    fn from(args: MintWithUsdonIxArgs) -> Self {
        Self(args)
    }
}
impl MintWithUsdonIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_WITH_USDON_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let attestation_id: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expiration: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(MintWithUsdonIxArgs {
                attestation_id,
                price,
                amount,
                expiration,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_WITH_USDON_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.attestation_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.expiration, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_with_usdon_ix_with_program_id(
    program_id: Pubkey,
    keys: MintWithUsdonKeys,
    args: MintWithUsdonIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_WITH_USDON_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintWithUsdonIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_with_usdon_ix(
    keys: MintWithUsdonKeys,
    args: MintWithUsdonIxArgs,
) -> std::io::Result<Instruction> {
    mint_with_usdon_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn mint_with_usdon_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintWithUsdonAccounts<'_, '_>,
    args: MintWithUsdonIxArgs,
) -> ProgramResult {
    let keys: MintWithUsdonKeys = accounts.into();
    let ix = mint_with_usdon_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_with_usdon_invoke(
    accounts: MintWithUsdonAccounts<'_, '_>,
    args: MintWithUsdonIxArgs,
) -> ProgramResult {
    mint_with_usdon_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn mint_with_usdon_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintWithUsdonAccounts<'_, '_>,
    args: MintWithUsdonIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintWithUsdonKeys = accounts.into();
    let ix = mint_with_usdon_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_with_usdon_invoke_signed(
    accounts: MintWithUsdonAccounts<'_, '_>,
    args: MintWithUsdonIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_with_usdon_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mint_with_usdon_verify_account_keys(
    accounts: MintWithUsdonAccounts<'_, '_>,
    keys: MintWithUsdonKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.ondo_user.key, keys.ondo_user),
        (*accounts.token_limit_account.key, keys.token_limit_account),
        (*accounts.sanity_check_account.key, keys.sanity_check_account),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.attestation_id_account.key, keys.attestation_id_account),
        (*accounts.whitelist.key, keys.whitelist),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.usdon_vault.key, keys.usdon_vault),
        (*accounts.usdon_mint.key, keys.usdon_mint),
        (*accounts.user_usdon_token_account.key, keys.user_usdon_token_account),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
        (*accounts.instructions.key, keys.instructions),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_with_usdon_verify_writable_privileges<'me, 'info>(
    accounts: MintWithUsdonAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.mint,
        accounts.ondo_user,
        accounts.token_limit_account,
        accounts.sanity_check_account,
        accounts.user_token_account,
        accounts.attestation_id_account,
        accounts.usdon_vault,
        accounts.usdon_mint,
        accounts.user_usdon_token_account,
        accounts.gmtoken_manager_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_with_usdon_verify_signer_privileges<'me, 'info>(
    accounts: MintWithUsdonAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_with_usdon_verify_account_privileges<'me, 'info>(
    accounts: MintWithUsdonAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_with_usdon_verify_writable_privileges(accounts)?;
    mint_with_usdon_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_GLOBAL_MINTING_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseGlobalMintingAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseGlobalMintingKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub gmtoken_manager_state: Pubkey,
}
impl From<PauseGlobalMintingAccounts<'_, '_>> for PauseGlobalMintingKeys {
    fn from(accounts: PauseGlobalMintingAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
        }
    }
}
impl From<PauseGlobalMintingKeys>
for [AccountMeta; PAUSE_GLOBAL_MINTING_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseGlobalMintingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_GLOBAL_MINTING_IX_ACCOUNTS_LEN]> for PauseGlobalMintingKeys {
    fn from(pubkeys: [Pubkey; PAUSE_GLOBAL_MINTING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            gmtoken_manager_state: pubkeys[2],
        }
    }
}
impl<'info> From<PauseGlobalMintingAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_GLOBAL_MINTING_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseGlobalMintingAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.gmtoken_manager_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_GLOBAL_MINTING_IX_ACCOUNTS_LEN]>
for PauseGlobalMintingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAUSE_GLOBAL_MINTING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            gmtoken_manager_state: &arr[2],
        }
    }
}
pub const PAUSE_GLOBAL_MINTING_IX_DISCM: [u8; 8usize] = [
    191, 70, 149, 85, 20, 141, 242, 239,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseGlobalMintingIxData;
impl PauseGlobalMintingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_GLOBAL_MINTING_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_GLOBAL_MINTING_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_global_minting_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseGlobalMintingKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_GLOBAL_MINTING_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseGlobalMintingIxData.try_to_vec()?,
    })
}
pub fn pause_global_minting_ix(
    keys: PauseGlobalMintingKeys,
) -> std::io::Result<Instruction> {
    pause_global_minting_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn pause_global_minting_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseGlobalMintingAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseGlobalMintingKeys = accounts.into();
    let ix = pause_global_minting_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_global_minting_invoke(
    accounts: PauseGlobalMintingAccounts<'_, '_>,
) -> ProgramResult {
    pause_global_minting_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn pause_global_minting_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseGlobalMintingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseGlobalMintingKeys = accounts.into();
    let ix = pause_global_minting_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_global_minting_invoke_signed(
    accounts: PauseGlobalMintingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_global_minting_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_global_minting_verify_account_keys(
    accounts: PauseGlobalMintingAccounts<'_, '_>,
    keys: PauseGlobalMintingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_global_minting_verify_writable_privileges<'me, 'info>(
    accounts: PauseGlobalMintingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.gmtoken_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_global_minting_verify_signer_privileges<'me, 'info>(
    accounts: PauseGlobalMintingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_global_minting_verify_account_privileges<'me, 'info>(
    accounts: PauseGlobalMintingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_global_minting_verify_writable_privileges(accounts)?;
    pause_global_minting_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_GLOBAL_MINTING_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseGlobalMintingAdminAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseGlobalMintingAdminKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub gmtoken_manager_state: Pubkey,
}
impl From<PauseGlobalMintingAdminAccounts<'_, '_>> for PauseGlobalMintingAdminKeys {
    fn from(accounts: PauseGlobalMintingAdminAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
        }
    }
}
impl From<PauseGlobalMintingAdminKeys>
for [AccountMeta; PAUSE_GLOBAL_MINTING_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseGlobalMintingAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_GLOBAL_MINTING_ADMIN_IX_ACCOUNTS_LEN]>
for PauseGlobalMintingAdminKeys {
    fn from(pubkeys: [Pubkey; PAUSE_GLOBAL_MINTING_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            gmtoken_manager_state: pubkeys[2],
        }
    }
}
impl<'info> From<PauseGlobalMintingAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_GLOBAL_MINTING_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseGlobalMintingAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.gmtoken_manager_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PAUSE_GLOBAL_MINTING_ADMIN_IX_ACCOUNTS_LEN]>
for PauseGlobalMintingAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAUSE_GLOBAL_MINTING_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            gmtoken_manager_state: &arr[2],
        }
    }
}
pub const PAUSE_GLOBAL_MINTING_ADMIN_IX_DISCM: [u8; 8usize] = [
    20, 43, 34, 201, 214, 148, 151, 75,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseGlobalMintingAdminIxData;
impl PauseGlobalMintingAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_GLOBAL_MINTING_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_GLOBAL_MINTING_ADMIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_global_minting_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseGlobalMintingAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_GLOBAL_MINTING_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseGlobalMintingAdminIxData.try_to_vec()?,
    })
}
pub fn pause_global_minting_admin_ix(
    keys: PauseGlobalMintingAdminKeys,
) -> std::io::Result<Instruction> {
    pause_global_minting_admin_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn pause_global_minting_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseGlobalMintingAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseGlobalMintingAdminKeys = accounts.into();
    let ix = pause_global_minting_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_global_minting_admin_invoke(
    accounts: PauseGlobalMintingAdminAccounts<'_, '_>,
) -> ProgramResult {
    pause_global_minting_admin_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn pause_global_minting_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseGlobalMintingAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseGlobalMintingAdminKeys = accounts.into();
    let ix = pause_global_minting_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_global_minting_admin_invoke_signed(
    accounts: PauseGlobalMintingAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_global_minting_admin_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_global_minting_admin_verify_account_keys(
    accounts: PauseGlobalMintingAdminAccounts<'_, '_>,
    keys: PauseGlobalMintingAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_global_minting_admin_verify_writable_privileges<'me, 'info>(
    accounts: PauseGlobalMintingAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.gmtoken_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_global_minting_admin_verify_signer_privileges<'me, 'info>(
    accounts: PauseGlobalMintingAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_global_minting_admin_verify_account_privileges<'me, 'info>(
    accounts: PauseGlobalMintingAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_global_minting_admin_verify_writable_privileges(accounts)?;
    pause_global_minting_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseGlobalRedemptionAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseGlobalRedemptionKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub gmtoken_manager_state: Pubkey,
}
impl From<PauseGlobalRedemptionAccounts<'_, '_>> for PauseGlobalRedemptionKeys {
    fn from(accounts: PauseGlobalRedemptionAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
        }
    }
}
impl From<PauseGlobalRedemptionKeys>
for [AccountMeta; PAUSE_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseGlobalRedemptionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN]>
for PauseGlobalRedemptionKeys {
    fn from(pubkeys: [Pubkey; PAUSE_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            gmtoken_manager_state: pubkeys[2],
        }
    }
}
impl<'info> From<PauseGlobalRedemptionAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseGlobalRedemptionAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.gmtoken_manager_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN]>
for PauseGlobalRedemptionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAUSE_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            gmtoken_manager_state: &arr[2],
        }
    }
}
pub const PAUSE_GLOBAL_REDEMPTION_IX_DISCM: [u8; 8usize] = [
    169, 5, 1, 19, 183, 71, 133, 147,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseGlobalRedemptionIxData;
impl PauseGlobalRedemptionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_GLOBAL_REDEMPTION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_GLOBAL_REDEMPTION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_global_redemption_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseGlobalRedemptionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseGlobalRedemptionIxData.try_to_vec()?,
    })
}
pub fn pause_global_redemption_ix(
    keys: PauseGlobalRedemptionKeys,
) -> std::io::Result<Instruction> {
    pause_global_redemption_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn pause_global_redemption_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseGlobalRedemptionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseGlobalRedemptionKeys = accounts.into();
    let ix = pause_global_redemption_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_global_redemption_invoke(
    accounts: PauseGlobalRedemptionAccounts<'_, '_>,
) -> ProgramResult {
    pause_global_redemption_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn pause_global_redemption_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseGlobalRedemptionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseGlobalRedemptionKeys = accounts.into();
    let ix = pause_global_redemption_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_global_redemption_invoke_signed(
    accounts: PauseGlobalRedemptionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_global_redemption_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_global_redemption_verify_account_keys(
    accounts: PauseGlobalRedemptionAccounts<'_, '_>,
    keys: PauseGlobalRedemptionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_global_redemption_verify_writable_privileges<'me, 'info>(
    accounts: PauseGlobalRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.gmtoken_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_global_redemption_verify_signer_privileges<'me, 'info>(
    accounts: PauseGlobalRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_global_redemption_verify_account_privileges<'me, 'info>(
    accounts: PauseGlobalRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_global_redemption_verify_writable_privileges(accounts)?;
    pause_global_redemption_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseGlobalRedemptionAdminAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseGlobalRedemptionAdminKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub gmtoken_manager_state: Pubkey,
}
impl From<PauseGlobalRedemptionAdminAccounts<'_, '_>>
for PauseGlobalRedemptionAdminKeys {
    fn from(accounts: PauseGlobalRedemptionAdminAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
        }
    }
}
impl From<PauseGlobalRedemptionAdminKeys>
for [AccountMeta; PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseGlobalRedemptionAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN]>
for PauseGlobalRedemptionAdminKeys {
    fn from(pubkeys: [Pubkey; PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            gmtoken_manager_state: pubkeys[2],
        }
    }
}
impl<'info> From<PauseGlobalRedemptionAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseGlobalRedemptionAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.gmtoken_manager_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN]>
for PauseGlobalRedemptionAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            gmtoken_manager_state: &arr[2],
        }
    }
}
pub const PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_DISCM: [u8; 8usize] = [
    77, 80, 68, 48, 7, 94, 111, 183,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseGlobalRedemptionAdminIxData;
impl PauseGlobalRedemptionAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_global_redemption_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseGlobalRedemptionAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_GLOBAL_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseGlobalRedemptionAdminIxData.try_to_vec()?,
    })
}
pub fn pause_global_redemption_admin_ix(
    keys: PauseGlobalRedemptionAdminKeys,
) -> std::io::Result<Instruction> {
    pause_global_redemption_admin_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn pause_global_redemption_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseGlobalRedemptionAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseGlobalRedemptionAdminKeys = accounts.into();
    let ix = pause_global_redemption_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_global_redemption_admin_invoke(
    accounts: PauseGlobalRedemptionAdminAccounts<'_, '_>,
) -> ProgramResult {
    pause_global_redemption_admin_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn pause_global_redemption_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseGlobalRedemptionAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseGlobalRedemptionAdminKeys = accounts.into();
    let ix = pause_global_redemption_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_global_redemption_admin_invoke_signed(
    accounts: PauseGlobalRedemptionAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_global_redemption_admin_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_global_redemption_admin_verify_account_keys(
    accounts: PauseGlobalRedemptionAdminAccounts<'_, '_>,
    keys: PauseGlobalRedemptionAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_global_redemption_admin_verify_writable_privileges<'me, 'info>(
    accounts: PauseGlobalRedemptionAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.gmtoken_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_global_redemption_admin_verify_signer_privileges<'me, 'info>(
    accounts: PauseGlobalRedemptionAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_global_redemption_admin_verify_account_privileges<'me, 'info>(
    accounts: PauseGlobalRedemptionAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_global_redemption_admin_verify_writable_privileges(accounts)?;
    pause_global_redemption_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_TOKEN_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct PauseTokenAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseTokenKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<PauseTokenAccounts<'_, '_>> for PauseTokenKeys {
    fn from(accounts: PauseTokenAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<PauseTokenKeys> for [AccountMeta; PAUSE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
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
impl From<[Pubkey; PAUSE_TOKEN_IX_ACCOUNTS_LEN]> for PauseTokenKeys {
    fn from(pubkeys: [Pubkey; PAUSE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            mint: pubkeys[2],
            mint_authority: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<PauseTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_TOKEN_IX_ACCOUNTS_LEN]>
for PauseTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAUSE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            mint: &arr[2],
            mint_authority: &arr[3],
            token_program: &arr[4],
        }
    }
}
pub const PAUSE_TOKEN_IX_DISCM: [u8; 8usize] = [226, 150, 72, 211, 159, 51, 226, 39];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseTokenIxData;
impl PauseTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_TOKEN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_token_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseTokenKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseTokenIxData.try_to_vec()?,
    })
}
pub fn pause_token_ix(keys: PauseTokenKeys) -> std::io::Result<Instruction> {
    pause_token_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn pause_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseTokenKeys = accounts.into();
    let ix = pause_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_token_invoke(accounts: PauseTokenAccounts<'_, '_>) -> ProgramResult {
    pause_token_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn pause_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseTokenKeys = accounts.into();
    let ix = pause_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_token_invoke_signed(
    accounts: PauseTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_token_invoke_signed_with_program_id(ONDO_GM_PROGRAM_ID, accounts, seeds)
}
pub fn pause_token_verify_account_keys(
    accounts: PauseTokenAccounts<'_, '_>,
    keys: PauseTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_token_verify_writable_privileges<'me, 'info>(
    accounts: PauseTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_token_verify_signer_privileges<'me, 'info>(
    accounts: PauseTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_token_verify_account_privileges<'me, 'info>(
    accounts: PauseTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_token_verify_writable_privileges(accounts)?;
    pause_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_TOKEN_FACTORY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseTokenFactoryAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseTokenFactoryKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub gmtoken_manager_state: Pubkey,
}
impl From<PauseTokenFactoryAccounts<'_, '_>> for PauseTokenFactoryKeys {
    fn from(accounts: PauseTokenFactoryAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
        }
    }
}
impl From<PauseTokenFactoryKeys> for [AccountMeta; PAUSE_TOKEN_FACTORY_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseTokenFactoryKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_TOKEN_FACTORY_IX_ACCOUNTS_LEN]> for PauseTokenFactoryKeys {
    fn from(pubkeys: [Pubkey; PAUSE_TOKEN_FACTORY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            gmtoken_manager_state: pubkeys[2],
        }
    }
}
impl<'info> From<PauseTokenFactoryAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_TOKEN_FACTORY_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseTokenFactoryAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.gmtoken_manager_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_TOKEN_FACTORY_IX_ACCOUNTS_LEN]>
for PauseTokenFactoryAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAUSE_TOKEN_FACTORY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            gmtoken_manager_state: &arr[2],
        }
    }
}
pub const PAUSE_TOKEN_FACTORY_IX_DISCM: [u8; 8usize] = [
    79, 227, 221, 11, 182, 89, 193, 150,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseTokenFactoryIxData;
impl PauseTokenFactoryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_TOKEN_FACTORY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_TOKEN_FACTORY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_token_factory_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseTokenFactoryKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_TOKEN_FACTORY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseTokenFactoryIxData.try_to_vec()?,
    })
}
pub fn pause_token_factory_ix(
    keys: PauseTokenFactoryKeys,
) -> std::io::Result<Instruction> {
    pause_token_factory_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn pause_token_factory_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenFactoryAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseTokenFactoryKeys = accounts.into();
    let ix = pause_token_factory_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_token_factory_invoke(
    accounts: PauseTokenFactoryAccounts<'_, '_>,
) -> ProgramResult {
    pause_token_factory_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn pause_token_factory_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenFactoryAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseTokenFactoryKeys = accounts.into();
    let ix = pause_token_factory_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_token_factory_invoke_signed(
    accounts: PauseTokenFactoryAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_token_factory_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_token_factory_verify_account_keys(
    accounts: PauseTokenFactoryAccounts<'_, '_>,
    keys: PauseTokenFactoryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_token_factory_verify_writable_privileges<'me, 'info>(
    accounts: PauseTokenFactoryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.gmtoken_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_token_factory_verify_signer_privileges<'me, 'info>(
    accounts: PauseTokenFactoryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_token_factory_verify_account_privileges<'me, 'info>(
    accounts: PauseTokenFactoryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_token_factory_verify_writable_privileges(accounts)?;
    pause_token_factory_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_TOKEN_FACTORY_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseTokenFactoryAdminAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseTokenFactoryAdminKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub gmtoken_manager_state: Pubkey,
}
impl From<PauseTokenFactoryAdminAccounts<'_, '_>> for PauseTokenFactoryAdminKeys {
    fn from(accounts: PauseTokenFactoryAdminAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
        }
    }
}
impl From<PauseTokenFactoryAdminKeys>
for [AccountMeta; PAUSE_TOKEN_FACTORY_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseTokenFactoryAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_TOKEN_FACTORY_ADMIN_IX_ACCOUNTS_LEN]>
for PauseTokenFactoryAdminKeys {
    fn from(pubkeys: [Pubkey; PAUSE_TOKEN_FACTORY_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            gmtoken_manager_state: pubkeys[2],
        }
    }
}
impl<'info> From<PauseTokenFactoryAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_TOKEN_FACTORY_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseTokenFactoryAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.gmtoken_manager_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PAUSE_TOKEN_FACTORY_ADMIN_IX_ACCOUNTS_LEN]>
for PauseTokenFactoryAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAUSE_TOKEN_FACTORY_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            gmtoken_manager_state: &arr[2],
        }
    }
}
pub const PAUSE_TOKEN_FACTORY_ADMIN_IX_DISCM: [u8; 8usize] = [
    244, 155, 26, 17, 39, 152, 51, 18,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseTokenFactoryAdminIxData;
impl PauseTokenFactoryAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_TOKEN_FACTORY_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_TOKEN_FACTORY_ADMIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_token_factory_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseTokenFactoryAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_TOKEN_FACTORY_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseTokenFactoryAdminIxData.try_to_vec()?,
    })
}
pub fn pause_token_factory_admin_ix(
    keys: PauseTokenFactoryAdminKeys,
) -> std::io::Result<Instruction> {
    pause_token_factory_admin_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn pause_token_factory_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenFactoryAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseTokenFactoryAdminKeys = accounts.into();
    let ix = pause_token_factory_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_token_factory_admin_invoke(
    accounts: PauseTokenFactoryAdminAccounts<'_, '_>,
) -> ProgramResult {
    pause_token_factory_admin_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn pause_token_factory_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenFactoryAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseTokenFactoryAdminKeys = accounts.into();
    let ix = pause_token_factory_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_token_factory_admin_invoke_signed(
    accounts: PauseTokenFactoryAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_token_factory_admin_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_token_factory_admin_verify_account_keys(
    accounts: PauseTokenFactoryAdminAccounts<'_, '_>,
    keys: PauseTokenFactoryAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_token_factory_admin_verify_writable_privileges<'me, 'info>(
    accounts: PauseTokenFactoryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.gmtoken_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_token_factory_admin_verify_signer_privileges<'me, 'info>(
    accounts: PauseTokenFactoryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_token_factory_admin_verify_account_privileges<'me, 'info>(
    accounts: PauseTokenFactoryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_token_factory_admin_verify_writable_privileges(accounts)?;
    pause_token_factory_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_TOKEN_MINTING_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseTokenMintingAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub token_limit_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseTokenMintingKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub token_limit_account: Pubkey,
}
impl From<PauseTokenMintingAccounts<'_, '_>> for PauseTokenMintingKeys {
    fn from(accounts: PauseTokenMintingAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            token_limit_account: *accounts.token_limit_account.key,
        }
    }
}
impl From<PauseTokenMintingKeys> for [AccountMeta; PAUSE_TOKEN_MINTING_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseTokenMintingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_limit_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_TOKEN_MINTING_IX_ACCOUNTS_LEN]> for PauseTokenMintingKeys {
    fn from(pubkeys: [Pubkey; PAUSE_TOKEN_MINTING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            token_limit_account: pubkeys[2],
        }
    }
}
impl<'info> From<PauseTokenMintingAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_TOKEN_MINTING_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseTokenMintingAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.token_limit_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_TOKEN_MINTING_IX_ACCOUNTS_LEN]>
for PauseTokenMintingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAUSE_TOKEN_MINTING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            token_limit_account: &arr[2],
        }
    }
}
pub const PAUSE_TOKEN_MINTING_IX_DISCM: [u8; 8usize] = [
    105, 219, 64, 183, 163, 148, 17, 126,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseTokenMintingIxData;
impl PauseTokenMintingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_TOKEN_MINTING_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_TOKEN_MINTING_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_token_minting_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseTokenMintingKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_TOKEN_MINTING_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseTokenMintingIxData.try_to_vec()?,
    })
}
pub fn pause_token_minting_ix(
    keys: PauseTokenMintingKeys,
) -> std::io::Result<Instruction> {
    pause_token_minting_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn pause_token_minting_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenMintingAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseTokenMintingKeys = accounts.into();
    let ix = pause_token_minting_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_token_minting_invoke(
    accounts: PauseTokenMintingAccounts<'_, '_>,
) -> ProgramResult {
    pause_token_minting_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn pause_token_minting_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenMintingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseTokenMintingKeys = accounts.into();
    let ix = pause_token_minting_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_token_minting_invoke_signed(
    accounts: PauseTokenMintingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_token_minting_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_token_minting_verify_account_keys(
    accounts: PauseTokenMintingAccounts<'_, '_>,
    keys: PauseTokenMintingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.token_limit_account.key, keys.token_limit_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_token_minting_verify_writable_privileges<'me, 'info>(
    accounts: PauseTokenMintingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_limit_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_token_minting_verify_signer_privileges<'me, 'info>(
    accounts: PauseTokenMintingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_token_minting_verify_account_privileges<'me, 'info>(
    accounts: PauseTokenMintingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_token_minting_verify_writable_privileges(accounts)?;
    pause_token_minting_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_TOKEN_MINTING_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseTokenMintingAdminAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub token_limit_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseTokenMintingAdminKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub token_limit_account: Pubkey,
}
impl From<PauseTokenMintingAdminAccounts<'_, '_>> for PauseTokenMintingAdminKeys {
    fn from(accounts: PauseTokenMintingAdminAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            token_limit_account: *accounts.token_limit_account.key,
        }
    }
}
impl From<PauseTokenMintingAdminKeys>
for [AccountMeta; PAUSE_TOKEN_MINTING_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseTokenMintingAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_limit_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_TOKEN_MINTING_ADMIN_IX_ACCOUNTS_LEN]>
for PauseTokenMintingAdminKeys {
    fn from(pubkeys: [Pubkey; PAUSE_TOKEN_MINTING_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            token_limit_account: pubkeys[2],
        }
    }
}
impl<'info> From<PauseTokenMintingAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_TOKEN_MINTING_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseTokenMintingAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.token_limit_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PAUSE_TOKEN_MINTING_ADMIN_IX_ACCOUNTS_LEN]>
for PauseTokenMintingAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAUSE_TOKEN_MINTING_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            token_limit_account: &arr[2],
        }
    }
}
pub const PAUSE_TOKEN_MINTING_ADMIN_IX_DISCM: [u8; 8usize] = [
    8, 3, 163, 16, 201, 93, 94, 145,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseTokenMintingAdminIxData;
impl PauseTokenMintingAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_TOKEN_MINTING_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_TOKEN_MINTING_ADMIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_token_minting_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseTokenMintingAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_TOKEN_MINTING_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseTokenMintingAdminIxData.try_to_vec()?,
    })
}
pub fn pause_token_minting_admin_ix(
    keys: PauseTokenMintingAdminKeys,
) -> std::io::Result<Instruction> {
    pause_token_minting_admin_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn pause_token_minting_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenMintingAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseTokenMintingAdminKeys = accounts.into();
    let ix = pause_token_minting_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_token_minting_admin_invoke(
    accounts: PauseTokenMintingAdminAccounts<'_, '_>,
) -> ProgramResult {
    pause_token_minting_admin_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn pause_token_minting_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenMintingAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseTokenMintingAdminKeys = accounts.into();
    let ix = pause_token_minting_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_token_minting_admin_invoke_signed(
    accounts: PauseTokenMintingAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_token_minting_admin_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_token_minting_admin_verify_account_keys(
    accounts: PauseTokenMintingAdminAccounts<'_, '_>,
    keys: PauseTokenMintingAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.token_limit_account.key, keys.token_limit_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_token_minting_admin_verify_writable_privileges<'me, 'info>(
    accounts: PauseTokenMintingAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_limit_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_token_minting_admin_verify_signer_privileges<'me, 'info>(
    accounts: PauseTokenMintingAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_token_minting_admin_verify_account_privileges<'me, 'info>(
    accounts: PauseTokenMintingAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_token_minting_admin_verify_writable_privileges(accounts)?;
    pause_token_minting_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseTokenRedemptionAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub token_limit_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseTokenRedemptionKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub token_limit_account: Pubkey,
}
impl From<PauseTokenRedemptionAccounts<'_, '_>> for PauseTokenRedemptionKeys {
    fn from(accounts: PauseTokenRedemptionAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            token_limit_account: *accounts.token_limit_account.key,
        }
    }
}
impl From<PauseTokenRedemptionKeys>
for [AccountMeta; PAUSE_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseTokenRedemptionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_limit_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN]>
for PauseTokenRedemptionKeys {
    fn from(pubkeys: [Pubkey; PAUSE_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            token_limit_account: pubkeys[2],
        }
    }
}
impl<'info> From<PauseTokenRedemptionAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseTokenRedemptionAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.token_limit_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN]>
for PauseTokenRedemptionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAUSE_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            token_limit_account: &arr[2],
        }
    }
}
pub const PAUSE_TOKEN_REDEMPTION_IX_DISCM: [u8; 8usize] = [
    232, 134, 32, 8, 70, 234, 54, 216,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseTokenRedemptionIxData;
impl PauseTokenRedemptionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_TOKEN_REDEMPTION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_TOKEN_REDEMPTION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_token_redemption_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseTokenRedemptionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseTokenRedemptionIxData.try_to_vec()?,
    })
}
pub fn pause_token_redemption_ix(
    keys: PauseTokenRedemptionKeys,
) -> std::io::Result<Instruction> {
    pause_token_redemption_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn pause_token_redemption_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenRedemptionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseTokenRedemptionKeys = accounts.into();
    let ix = pause_token_redemption_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_token_redemption_invoke(
    accounts: PauseTokenRedemptionAccounts<'_, '_>,
) -> ProgramResult {
    pause_token_redemption_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn pause_token_redemption_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenRedemptionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseTokenRedemptionKeys = accounts.into();
    let ix = pause_token_redemption_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_token_redemption_invoke_signed(
    accounts: PauseTokenRedemptionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_token_redemption_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_token_redemption_verify_account_keys(
    accounts: PauseTokenRedemptionAccounts<'_, '_>,
    keys: PauseTokenRedemptionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.token_limit_account.key, keys.token_limit_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_token_redemption_verify_writable_privileges<'me, 'info>(
    accounts: PauseTokenRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_limit_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_token_redemption_verify_signer_privileges<'me, 'info>(
    accounts: PauseTokenRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_token_redemption_verify_account_privileges<'me, 'info>(
    accounts: PauseTokenRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_token_redemption_verify_writable_privileges(accounts)?;
    pause_token_redemption_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_TOKEN_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseTokenRedemptionAdminAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub token_limit_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseTokenRedemptionAdminKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub token_limit_account: Pubkey,
}
impl From<PauseTokenRedemptionAdminAccounts<'_, '_>> for PauseTokenRedemptionAdminKeys {
    fn from(accounts: PauseTokenRedemptionAdminAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            token_limit_account: *accounts.token_limit_account.key,
        }
    }
}
impl From<PauseTokenRedemptionAdminKeys>
for [AccountMeta; PAUSE_TOKEN_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseTokenRedemptionAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_limit_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_TOKEN_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN]>
for PauseTokenRedemptionAdminKeys {
    fn from(pubkeys: [Pubkey; PAUSE_TOKEN_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            token_limit_account: pubkeys[2],
        }
    }
}
impl<'info> From<PauseTokenRedemptionAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_TOKEN_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseTokenRedemptionAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.token_limit_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PAUSE_TOKEN_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN]>
for PauseTokenRedemptionAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAUSE_TOKEN_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            token_limit_account: &arr[2],
        }
    }
}
pub const PAUSE_TOKEN_REDEMPTION_ADMIN_IX_DISCM: [u8; 8usize] = [
    62, 162, 192, 223, 22, 143, 110, 191,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseTokenRedemptionAdminIxData;
impl PauseTokenRedemptionAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_TOKEN_REDEMPTION_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_TOKEN_REDEMPTION_ADMIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_token_redemption_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseTokenRedemptionAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_TOKEN_REDEMPTION_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseTokenRedemptionAdminIxData.try_to_vec()?,
    })
}
pub fn pause_token_redemption_admin_ix(
    keys: PauseTokenRedemptionAdminKeys,
) -> std::io::Result<Instruction> {
    pause_token_redemption_admin_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn pause_token_redemption_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenRedemptionAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseTokenRedemptionAdminKeys = accounts.into();
    let ix = pause_token_redemption_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_token_redemption_admin_invoke(
    accounts: PauseTokenRedemptionAdminAccounts<'_, '_>,
) -> ProgramResult {
    pause_token_redemption_admin_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn pause_token_redemption_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseTokenRedemptionAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseTokenRedemptionAdminKeys = accounts.into();
    let ix = pause_token_redemption_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_token_redemption_admin_invoke_signed(
    accounts: PauseTokenRedemptionAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_token_redemption_admin_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_token_redemption_admin_verify_account_keys(
    accounts: PauseTokenRedemptionAdminAccounts<'_, '_>,
    keys: PauseTokenRedemptionAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.token_limit_account.key, keys.token_limit_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_token_redemption_admin_verify_writable_privileges<'me, 'info>(
    accounts: PauseTokenRedemptionAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_limit_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_token_redemption_admin_verify_signer_privileges<'me, 'info>(
    accounts: PauseTokenRedemptionAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_token_redemption_admin_verify_account_privileges<'me, 'info>(
    accounts: PauseTokenRedemptionAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_token_redemption_admin_verify_writable_privileges(accounts)?;
    pause_token_redemption_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_FOR_USDC_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct RedeemForUsdcAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub ondo_user: &'me AccountInfo<'info>,
    pub token_limit_account: &'me AccountInfo<'info>,
    pub sanity_check_account: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub attestation_id_account: &'me AccountInfo<'info>,
    pub whitelist: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub spl_token_program: &'me AccountInfo<'info>,
    pub usdc_price_update: &'me AccountInfo<'info>,
    pub usdc_vault: &'me AccountInfo<'info>,
    pub usdon_vault: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub user_usdc_token_account: &'me AccountInfo<'info>,
    pub usdon_mint: &'me AccountInfo<'info>,
    pub user_usdon_token_account: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
    pub instructions: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemForUsdcKeys {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub ondo_user: Pubkey,
    pub token_limit_account: Pubkey,
    pub sanity_check_account: Pubkey,
    pub user_token_account: Pubkey,
    pub attestation_id_account: Pubkey,
    pub whitelist: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub spl_token_program: Pubkey,
    pub usdc_price_update: Pubkey,
    pub usdc_vault: Pubkey,
    pub usdon_vault: Pubkey,
    pub usdc_mint: Pubkey,
    pub user_usdc_token_account: Pubkey,
    pub usdon_mint: Pubkey,
    pub user_usdon_token_account: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub gmtoken_manager_state: Pubkey,
    pub instructions: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RedeemForUsdcAccounts<'_, '_>> for RedeemForUsdcKeys {
    fn from(accounts: RedeemForUsdcAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            ondo_user: *accounts.ondo_user.key,
            token_limit_account: *accounts.token_limit_account.key,
            sanity_check_account: *accounts.sanity_check_account.key,
            user_token_account: *accounts.user_token_account.key,
            attestation_id_account: *accounts.attestation_id_account.key,
            whitelist: *accounts.whitelist.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            spl_token_program: *accounts.spl_token_program.key,
            usdc_price_update: *accounts.usdc_price_update.key,
            usdc_vault: *accounts.usdc_vault.key,
            usdon_vault: *accounts.usdon_vault.key,
            usdc_mint: *accounts.usdc_mint.key,
            user_usdc_token_account: *accounts.user_usdc_token_account.key,
            usdon_mint: *accounts.usdon_mint.key,
            user_usdon_token_account: *accounts.user_usdon_token_account.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
            instructions: *accounts.instructions.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RedeemForUsdcKeys> for [AccountMeta; REDEEM_FOR_USDC_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemForUsdcKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ondo_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_limit_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sanity_check_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.attestation_id_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.whitelist,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.spl_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_price_update,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdon_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_usdc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdon_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_usdon_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instructions,
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
impl From<[Pubkey; REDEEM_FOR_USDC_IX_ACCOUNTS_LEN]> for RedeemForUsdcKeys {
    fn from(pubkeys: [Pubkey; REDEEM_FOR_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            mint: pubkeys[1],
            mint_authority: pubkeys[2],
            ondo_user: pubkeys[3],
            token_limit_account: pubkeys[4],
            sanity_check_account: pubkeys[5],
            user_token_account: pubkeys[6],
            attestation_id_account: pubkeys[7],
            whitelist: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            associated_token_program: pubkeys[11],
            spl_token_program: pubkeys[12],
            usdc_price_update: pubkeys[13],
            usdc_vault: pubkeys[14],
            usdon_vault: pubkeys[15],
            usdc_mint: pubkeys[16],
            user_usdc_token_account: pubkeys[17],
            usdon_mint: pubkeys[18],
            user_usdon_token_account: pubkeys[19],
            usdon_manager_state: pubkeys[20],
            gmtoken_manager_state: pubkeys[21],
            instructions: pubkeys[22],
            event_authority: pubkeys[23],
            program: pubkeys[24],
        }
    }
}
impl<'info> From<RedeemForUsdcAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_FOR_USDC_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemForUsdcAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.ondo_user.clone(),
            accounts.token_limit_account.clone(),
            accounts.sanity_check_account.clone(),
            accounts.user_token_account.clone(),
            accounts.attestation_id_account.clone(),
            accounts.whitelist.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.spl_token_program.clone(),
            accounts.usdc_price_update.clone(),
            accounts.usdc_vault.clone(),
            accounts.usdon_vault.clone(),
            accounts.usdc_mint.clone(),
            accounts.user_usdc_token_account.clone(),
            accounts.usdon_mint.clone(),
            accounts.user_usdon_token_account.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.gmtoken_manager_state.clone(),
            accounts.instructions.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_FOR_USDC_IX_ACCOUNTS_LEN]>
for RedeemForUsdcAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REDEEM_FOR_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            mint: &arr[1],
            mint_authority: &arr[2],
            ondo_user: &arr[3],
            token_limit_account: &arr[4],
            sanity_check_account: &arr[5],
            user_token_account: &arr[6],
            attestation_id_account: &arr[7],
            whitelist: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            associated_token_program: &arr[11],
            spl_token_program: &arr[12],
            usdc_price_update: &arr[13],
            usdc_vault: &arr[14],
            usdon_vault: &arr[15],
            usdc_mint: &arr[16],
            user_usdc_token_account: &arr[17],
            usdon_mint: &arr[18],
            user_usdon_token_account: &arr[19],
            usdon_manager_state: &arr[20],
            gmtoken_manager_state: &arr[21],
            instructions: &arr[22],
            event_authority: &arr[23],
            program: &arr[24],
        }
    }
}
pub const REDEEM_FOR_USDC_IX_DISCM: [u8; 8usize] = [150, 9, 215, 220, 255, 157, 74, 78];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemForUsdcIxArgs {
    pub attestation_id: [u8; 16],
    pub price: u64,
    pub amount: u64,
    pub expiration: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemForUsdcIxData(pub RedeemForUsdcIxArgs);
impl From<RedeemForUsdcIxArgs> for RedeemForUsdcIxData {
    fn from(args: RedeemForUsdcIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemForUsdcIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_FOR_USDC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let attestation_id: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expiration: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RedeemForUsdcIxArgs {
                attestation_id,
                price,
                amount,
                expiration,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_FOR_USDC_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.attestation_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.expiration, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_for_usdc_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemForUsdcKeys,
    args: RedeemForUsdcIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_FOR_USDC_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemForUsdcIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_for_usdc_ix(
    keys: RedeemForUsdcKeys,
    args: RedeemForUsdcIxArgs,
) -> std::io::Result<Instruction> {
    redeem_for_usdc_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn redeem_for_usdc_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemForUsdcAccounts<'_, '_>,
    args: RedeemForUsdcIxArgs,
) -> ProgramResult {
    let keys: RedeemForUsdcKeys = accounts.into();
    let ix = redeem_for_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_for_usdc_invoke(
    accounts: RedeemForUsdcAccounts<'_, '_>,
    args: RedeemForUsdcIxArgs,
) -> ProgramResult {
    redeem_for_usdc_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn redeem_for_usdc_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemForUsdcAccounts<'_, '_>,
    args: RedeemForUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemForUsdcKeys = accounts.into();
    let ix = redeem_for_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_for_usdc_invoke_signed(
    accounts: RedeemForUsdcAccounts<'_, '_>,
    args: RedeemForUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_for_usdc_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn redeem_for_usdc_verify_account_keys(
    accounts: RedeemForUsdcAccounts<'_, '_>,
    keys: RedeemForUsdcKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.ondo_user.key, keys.ondo_user),
        (*accounts.token_limit_account.key, keys.token_limit_account),
        (*accounts.sanity_check_account.key, keys.sanity_check_account),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.attestation_id_account.key, keys.attestation_id_account),
        (*accounts.whitelist.key, keys.whitelist),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.spl_token_program.key, keys.spl_token_program),
        (*accounts.usdc_price_update.key, keys.usdc_price_update),
        (*accounts.usdc_vault.key, keys.usdc_vault),
        (*accounts.usdon_vault.key, keys.usdon_vault),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.user_usdc_token_account.key, keys.user_usdc_token_account),
        (*accounts.usdon_mint.key, keys.usdon_mint),
        (*accounts.user_usdon_token_account.key, keys.user_usdon_token_account),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
        (*accounts.instructions.key, keys.instructions),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_for_usdc_verify_writable_privileges<'me, 'info>(
    accounts: RedeemForUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.mint,
        accounts.ondo_user,
        accounts.token_limit_account,
        accounts.sanity_check_account,
        accounts.user_token_account,
        accounts.attestation_id_account,
        accounts.usdc_vault,
        accounts.usdon_vault,
        accounts.user_usdc_token_account,
        accounts.usdon_mint,
        accounts.user_usdon_token_account,
        accounts.gmtoken_manager_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_for_usdc_verify_signer_privileges<'me, 'info>(
    accounts: RedeemForUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_for_usdc_verify_account_privileges<'me, 'info>(
    accounts: RedeemForUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_for_usdc_verify_writable_privileges(accounts)?;
    redeem_for_usdc_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_FOR_USDON_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct RedeemForUsdonAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub ondo_user: &'me AccountInfo<'info>,
    pub token_limit_account: &'me AccountInfo<'info>,
    pub sanity_check_account: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub attestation_id_account: &'me AccountInfo<'info>,
    pub whitelist: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub usdon_vault: &'me AccountInfo<'info>,
    pub usdon_mint: &'me AccountInfo<'info>,
    pub user_usdon_token_account: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
    pub instructions: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemForUsdonKeys {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub ondo_user: Pubkey,
    pub token_limit_account: Pubkey,
    pub sanity_check_account: Pubkey,
    pub user_token_account: Pubkey,
    pub attestation_id_account: Pubkey,
    pub whitelist: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub usdon_vault: Pubkey,
    pub usdon_mint: Pubkey,
    pub user_usdon_token_account: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub gmtoken_manager_state: Pubkey,
    pub instructions: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RedeemForUsdonAccounts<'_, '_>> for RedeemForUsdonKeys {
    fn from(accounts: RedeemForUsdonAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            ondo_user: *accounts.ondo_user.key,
            token_limit_account: *accounts.token_limit_account.key,
            sanity_check_account: *accounts.sanity_check_account.key,
            user_token_account: *accounts.user_token_account.key,
            attestation_id_account: *accounts.attestation_id_account.key,
            whitelist: *accounts.whitelist.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            usdon_vault: *accounts.usdon_vault.key,
            usdon_mint: *accounts.usdon_mint.key,
            user_usdon_token_account: *accounts.user_usdon_token_account.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
            instructions: *accounts.instructions.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RedeemForUsdonKeys> for [AccountMeta; REDEEM_FOR_USDON_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemForUsdonKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ondo_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_limit_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sanity_check_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.attestation_id_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.whitelist,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdon_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdon_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_usdon_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instructions,
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
impl From<[Pubkey; REDEEM_FOR_USDON_IX_ACCOUNTS_LEN]> for RedeemForUsdonKeys {
    fn from(pubkeys: [Pubkey; REDEEM_FOR_USDON_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            mint: pubkeys[1],
            mint_authority: pubkeys[2],
            ondo_user: pubkeys[3],
            token_limit_account: pubkeys[4],
            sanity_check_account: pubkeys[5],
            user_token_account: pubkeys[6],
            attestation_id_account: pubkeys[7],
            whitelist: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            associated_token_program: pubkeys[11],
            usdon_vault: pubkeys[12],
            usdon_mint: pubkeys[13],
            user_usdon_token_account: pubkeys[14],
            usdon_manager_state: pubkeys[15],
            gmtoken_manager_state: pubkeys[16],
            instructions: pubkeys[17],
            event_authority: pubkeys[18],
            program: pubkeys[19],
        }
    }
}
impl<'info> From<RedeemForUsdonAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_FOR_USDON_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemForUsdonAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.ondo_user.clone(),
            accounts.token_limit_account.clone(),
            accounts.sanity_check_account.clone(),
            accounts.user_token_account.clone(),
            accounts.attestation_id_account.clone(),
            accounts.whitelist.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.usdon_vault.clone(),
            accounts.usdon_mint.clone(),
            accounts.user_usdon_token_account.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.gmtoken_manager_state.clone(),
            accounts.instructions.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_FOR_USDON_IX_ACCOUNTS_LEN]>
for RedeemForUsdonAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REDEEM_FOR_USDON_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            mint: &arr[1],
            mint_authority: &arr[2],
            ondo_user: &arr[3],
            token_limit_account: &arr[4],
            sanity_check_account: &arr[5],
            user_token_account: &arr[6],
            attestation_id_account: &arr[7],
            whitelist: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            associated_token_program: &arr[11],
            usdon_vault: &arr[12],
            usdon_mint: &arr[13],
            user_usdon_token_account: &arr[14],
            usdon_manager_state: &arr[15],
            gmtoken_manager_state: &arr[16],
            instructions: &arr[17],
            event_authority: &arr[18],
            program: &arr[19],
        }
    }
}
pub const REDEEM_FOR_USDON_IX_DISCM: [u8; 8usize] = [231, 121, 93, 33, 143, 252, 82, 13];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemForUsdonIxArgs {
    pub attestation_id: [u8; 16],
    pub price: u64,
    pub amount: u64,
    pub expiration: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemForUsdonIxData(pub RedeemForUsdonIxArgs);
impl From<RedeemForUsdonIxArgs> for RedeemForUsdonIxData {
    fn from(args: RedeemForUsdonIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemForUsdonIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_FOR_USDON_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let attestation_id: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expiration: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RedeemForUsdonIxArgs {
                attestation_id,
                price,
                amount,
                expiration,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_FOR_USDON_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.attestation_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.expiration, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_for_usdon_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemForUsdonKeys,
    args: RedeemForUsdonIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_FOR_USDON_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemForUsdonIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_for_usdon_ix(
    keys: RedeemForUsdonKeys,
    args: RedeemForUsdonIxArgs,
) -> std::io::Result<Instruction> {
    redeem_for_usdon_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn redeem_for_usdon_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemForUsdonAccounts<'_, '_>,
    args: RedeemForUsdonIxArgs,
) -> ProgramResult {
    let keys: RedeemForUsdonKeys = accounts.into();
    let ix = redeem_for_usdon_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_for_usdon_invoke(
    accounts: RedeemForUsdonAccounts<'_, '_>,
    args: RedeemForUsdonIxArgs,
) -> ProgramResult {
    redeem_for_usdon_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn redeem_for_usdon_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemForUsdonAccounts<'_, '_>,
    args: RedeemForUsdonIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemForUsdonKeys = accounts.into();
    let ix = redeem_for_usdon_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_for_usdon_invoke_signed(
    accounts: RedeemForUsdonAccounts<'_, '_>,
    args: RedeemForUsdonIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_for_usdon_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn redeem_for_usdon_verify_account_keys(
    accounts: RedeemForUsdonAccounts<'_, '_>,
    keys: RedeemForUsdonKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.ondo_user.key, keys.ondo_user),
        (*accounts.token_limit_account.key, keys.token_limit_account),
        (*accounts.sanity_check_account.key, keys.sanity_check_account),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.attestation_id_account.key, keys.attestation_id_account),
        (*accounts.whitelist.key, keys.whitelist),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.usdon_vault.key, keys.usdon_vault),
        (*accounts.usdon_mint.key, keys.usdon_mint),
        (*accounts.user_usdon_token_account.key, keys.user_usdon_token_account),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
        (*accounts.instructions.key, keys.instructions),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_for_usdon_verify_writable_privileges<'me, 'info>(
    accounts: RedeemForUsdonAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.mint,
        accounts.ondo_user,
        accounts.token_limit_account,
        accounts.sanity_check_account,
        accounts.user_token_account,
        accounts.attestation_id_account,
        accounts.usdon_vault,
        accounts.usdon_mint,
        accounts.user_usdon_token_account,
        accounts.gmtoken_manager_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_for_usdon_verify_signer_privileges<'me, 'info>(
    accounts: RedeemForUsdonAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_for_usdon_verify_account_privileges<'me, 'info>(
    accounts: RedeemForUsdonAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_for_usdon_verify_writable_privileges(accounts)?;
    redeem_for_usdon_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct RemoveFromWhitelistAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub whitelist: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveFromWhitelistKeys {
    pub authority: Pubkey,
    pub recipient: Pubkey,
    pub authority_role_account: Pubkey,
    pub whitelist: Pubkey,
}
impl From<RemoveFromWhitelistAccounts<'_, '_>> for RemoveFromWhitelistKeys {
    fn from(accounts: RemoveFromWhitelistAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            recipient: *accounts.recipient.key,
            authority_role_account: *accounts.authority_role_account.key,
            whitelist: *accounts.whitelist.key,
        }
    }
}
impl From<RemoveFromWhitelistKeys>
for [AccountMeta; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveFromWhitelistKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whitelist,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN]> for RemoveFromWhitelistKeys {
    fn from(pubkeys: [Pubkey; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            recipient: pubkeys[1],
            authority_role_account: pubkeys[2],
            whitelist: pubkeys[3],
        }
    }
}
impl<'info> From<RemoveFromWhitelistAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveFromWhitelistAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.recipient.clone(),
            accounts.authority_role_account.clone(),
            accounts.whitelist.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN]>
for RemoveFromWhitelistAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            recipient: &arr[1],
            authority_role_account: &arr[2],
            whitelist: &arr[3],
        }
    }
}
pub const REMOVE_FROM_WHITELIST_IX_DISCM: [u8; 8usize] = [
    7, 144, 216, 239, 243, 236, 193, 235,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveFromWhitelistIxArgs {
    pub address_to_remove: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveFromWhitelistIxData(pub RemoveFromWhitelistIxArgs);
impl From<RemoveFromWhitelistIxArgs> for RemoveFromWhitelistIxData {
    fn from(args: RemoveFromWhitelistIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveFromWhitelistIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_FROM_WHITELIST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let address_to_remove: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemoveFromWhitelistIxArgs {
                address_to_remove,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_FROM_WHITELIST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.address_to_remove, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_from_whitelist_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveFromWhitelistKeys,
    args: RemoveFromWhitelistIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveFromWhitelistIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_from_whitelist_ix(
    keys: RemoveFromWhitelistKeys,
    args: RemoveFromWhitelistIxArgs,
) -> std::io::Result<Instruction> {
    remove_from_whitelist_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn remove_from_whitelist_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveFromWhitelistAccounts<'_, '_>,
    args: RemoveFromWhitelistIxArgs,
) -> ProgramResult {
    let keys: RemoveFromWhitelistKeys = accounts.into();
    let ix = remove_from_whitelist_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_from_whitelist_invoke(
    accounts: RemoveFromWhitelistAccounts<'_, '_>,
    args: RemoveFromWhitelistIxArgs,
) -> ProgramResult {
    remove_from_whitelist_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn remove_from_whitelist_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveFromWhitelistAccounts<'_, '_>,
    args: RemoveFromWhitelistIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveFromWhitelistKeys = accounts.into();
    let ix = remove_from_whitelist_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_from_whitelist_invoke_signed(
    accounts: RemoveFromWhitelistAccounts<'_, '_>,
    args: RemoveFromWhitelistIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_from_whitelist_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_from_whitelist_verify_account_keys(
    accounts: RemoveFromWhitelistAccounts<'_, '_>,
    keys: RemoveFromWhitelistKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.whitelist.key, keys.whitelist),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_from_whitelist_verify_writable_privileges<'me, 'info>(
    accounts: RemoveFromWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.recipient, accounts.whitelist] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_from_whitelist_verify_signer_privileges<'me, 'info>(
    accounts: RemoveFromWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_from_whitelist_verify_account_privileges<'me, 'info>(
    accounts: RemoveFromWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_from_whitelist_verify_writable_privileges(accounts)?;
    remove_from_whitelist_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RESUME_GLOBAL_MINTING_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ResumeGlobalMintingAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResumeGlobalMintingKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub gmtoken_manager_state: Pubkey,
}
impl From<ResumeGlobalMintingAccounts<'_, '_>> for ResumeGlobalMintingKeys {
    fn from(accounts: ResumeGlobalMintingAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
        }
    }
}
impl From<ResumeGlobalMintingKeys>
for [AccountMeta; RESUME_GLOBAL_MINTING_IX_ACCOUNTS_LEN] {
    fn from(keys: ResumeGlobalMintingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; RESUME_GLOBAL_MINTING_IX_ACCOUNTS_LEN]> for ResumeGlobalMintingKeys {
    fn from(pubkeys: [Pubkey; RESUME_GLOBAL_MINTING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            gmtoken_manager_state: pubkeys[2],
        }
    }
}
impl<'info> From<ResumeGlobalMintingAccounts<'_, 'info>>
for [AccountInfo<'info>; RESUME_GLOBAL_MINTING_IX_ACCOUNTS_LEN] {
    fn from(accounts: ResumeGlobalMintingAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.gmtoken_manager_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESUME_GLOBAL_MINTING_IX_ACCOUNTS_LEN]>
for ResumeGlobalMintingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; RESUME_GLOBAL_MINTING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            gmtoken_manager_state: &arr[2],
        }
    }
}
pub const RESUME_GLOBAL_MINTING_IX_DISCM: [u8; 8usize] = [
    18, 40, 173, 140, 250, 6, 10, 207,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ResumeGlobalMintingIxData;
impl ResumeGlobalMintingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESUME_GLOBAL_MINTING_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESUME_GLOBAL_MINTING_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn resume_global_minting_ix_with_program_id(
    program_id: Pubkey,
    keys: ResumeGlobalMintingKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RESUME_GLOBAL_MINTING_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ResumeGlobalMintingIxData.try_to_vec()?,
    })
}
pub fn resume_global_minting_ix(
    keys: ResumeGlobalMintingKeys,
) -> std::io::Result<Instruction> {
    resume_global_minting_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn resume_global_minting_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ResumeGlobalMintingAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ResumeGlobalMintingKeys = accounts.into();
    let ix = resume_global_minting_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn resume_global_minting_invoke(
    accounts: ResumeGlobalMintingAccounts<'_, '_>,
) -> ProgramResult {
    resume_global_minting_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn resume_global_minting_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ResumeGlobalMintingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ResumeGlobalMintingKeys = accounts.into();
    let ix = resume_global_minting_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn resume_global_minting_invoke_signed(
    accounts: ResumeGlobalMintingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    resume_global_minting_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn resume_global_minting_verify_account_keys(
    accounts: ResumeGlobalMintingAccounts<'_, '_>,
    keys: ResumeGlobalMintingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn resume_global_minting_verify_writable_privileges<'me, 'info>(
    accounts: ResumeGlobalMintingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.gmtoken_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn resume_global_minting_verify_signer_privileges<'me, 'info>(
    accounts: ResumeGlobalMintingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn resume_global_minting_verify_account_privileges<'me, 'info>(
    accounts: ResumeGlobalMintingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    resume_global_minting_verify_writable_privileges(accounts)?;
    resume_global_minting_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RESUME_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ResumeGlobalRedemptionAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResumeGlobalRedemptionKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub gmtoken_manager_state: Pubkey,
}
impl From<ResumeGlobalRedemptionAccounts<'_, '_>> for ResumeGlobalRedemptionKeys {
    fn from(accounts: ResumeGlobalRedemptionAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
        }
    }
}
impl From<ResumeGlobalRedemptionKeys>
for [AccountMeta; RESUME_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN] {
    fn from(keys: ResumeGlobalRedemptionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; RESUME_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN]>
for ResumeGlobalRedemptionKeys {
    fn from(pubkeys: [Pubkey; RESUME_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            gmtoken_manager_state: pubkeys[2],
        }
    }
}
impl<'info> From<ResumeGlobalRedemptionAccounts<'_, 'info>>
for [AccountInfo<'info>; RESUME_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ResumeGlobalRedemptionAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.gmtoken_manager_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; RESUME_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN]>
for ResumeGlobalRedemptionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; RESUME_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            gmtoken_manager_state: &arr[2],
        }
    }
}
pub const RESUME_GLOBAL_REDEMPTION_IX_DISCM: [u8; 8usize] = [
    80, 47, 24, 168, 44, 185, 215, 97,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ResumeGlobalRedemptionIxData;
impl ResumeGlobalRedemptionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESUME_GLOBAL_REDEMPTION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESUME_GLOBAL_REDEMPTION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn resume_global_redemption_ix_with_program_id(
    program_id: Pubkey,
    keys: ResumeGlobalRedemptionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RESUME_GLOBAL_REDEMPTION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ResumeGlobalRedemptionIxData.try_to_vec()?,
    })
}
pub fn resume_global_redemption_ix(
    keys: ResumeGlobalRedemptionKeys,
) -> std::io::Result<Instruction> {
    resume_global_redemption_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn resume_global_redemption_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ResumeGlobalRedemptionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ResumeGlobalRedemptionKeys = accounts.into();
    let ix = resume_global_redemption_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn resume_global_redemption_invoke(
    accounts: ResumeGlobalRedemptionAccounts<'_, '_>,
) -> ProgramResult {
    resume_global_redemption_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn resume_global_redemption_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ResumeGlobalRedemptionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ResumeGlobalRedemptionKeys = accounts.into();
    let ix = resume_global_redemption_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn resume_global_redemption_invoke_signed(
    accounts: ResumeGlobalRedemptionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    resume_global_redemption_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn resume_global_redemption_verify_account_keys(
    accounts: ResumeGlobalRedemptionAccounts<'_, '_>,
    keys: ResumeGlobalRedemptionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn resume_global_redemption_verify_writable_privileges<'me, 'info>(
    accounts: ResumeGlobalRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.gmtoken_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn resume_global_redemption_verify_signer_privileges<'me, 'info>(
    accounts: ResumeGlobalRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn resume_global_redemption_verify_account_privileges<'me, 'info>(
    accounts: ResumeGlobalRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    resume_global_redemption_verify_writable_privileges(accounts)?;
    resume_global_redemption_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RESUME_TOKEN_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ResumeTokenAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResumeTokenKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<ResumeTokenAccounts<'_, '_>> for ResumeTokenKeys {
    fn from(accounts: ResumeTokenAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ResumeTokenKeys> for [AccountMeta; RESUME_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: ResumeTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
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
impl From<[Pubkey; RESUME_TOKEN_IX_ACCOUNTS_LEN]> for ResumeTokenKeys {
    fn from(pubkeys: [Pubkey; RESUME_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            mint: pubkeys[2],
            mint_authority: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<ResumeTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; RESUME_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: ResumeTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESUME_TOKEN_IX_ACCOUNTS_LEN]>
for ResumeTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; RESUME_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            mint: &arr[2],
            mint_authority: &arr[3],
            token_program: &arr[4],
        }
    }
}
pub const RESUME_TOKEN_IX_DISCM: [u8; 8usize] = [173, 116, 46, 190, 81, 191, 249, 83];
#[derive(Clone, Debug, PartialEq)]
pub struct ResumeTokenIxData;
impl ResumeTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESUME_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESUME_TOKEN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn resume_token_ix_with_program_id(
    program_id: Pubkey,
    keys: ResumeTokenKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RESUME_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ResumeTokenIxData.try_to_vec()?,
    })
}
pub fn resume_token_ix(keys: ResumeTokenKeys) -> std::io::Result<Instruction> {
    resume_token_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn resume_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ResumeTokenAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ResumeTokenKeys = accounts.into();
    let ix = resume_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn resume_token_invoke(accounts: ResumeTokenAccounts<'_, '_>) -> ProgramResult {
    resume_token_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn resume_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ResumeTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ResumeTokenKeys = accounts.into();
    let ix = resume_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn resume_token_invoke_signed(
    accounts: ResumeTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    resume_token_invoke_signed_with_program_id(ONDO_GM_PROGRAM_ID, accounts, seeds)
}
pub fn resume_token_verify_account_keys(
    accounts: ResumeTokenAccounts<'_, '_>,
    keys: ResumeTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn resume_token_verify_writable_privileges<'me, 'info>(
    accounts: ResumeTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn resume_token_verify_signer_privileges<'me, 'info>(
    accounts: ResumeTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn resume_token_verify_account_privileges<'me, 'info>(
    accounts: ResumeTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    resume_token_verify_writable_privileges(accounts)?;
    resume_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RESUME_TOKEN_FACTORY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ResumeTokenFactoryAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResumeTokenFactoryKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub gmtoken_manager_state: Pubkey,
}
impl From<ResumeTokenFactoryAccounts<'_, '_>> for ResumeTokenFactoryKeys {
    fn from(accounts: ResumeTokenFactoryAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
        }
    }
}
impl From<ResumeTokenFactoryKeys>
for [AccountMeta; RESUME_TOKEN_FACTORY_IX_ACCOUNTS_LEN] {
    fn from(keys: ResumeTokenFactoryKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; RESUME_TOKEN_FACTORY_IX_ACCOUNTS_LEN]> for ResumeTokenFactoryKeys {
    fn from(pubkeys: [Pubkey; RESUME_TOKEN_FACTORY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            gmtoken_manager_state: pubkeys[2],
        }
    }
}
impl<'info> From<ResumeTokenFactoryAccounts<'_, 'info>>
for [AccountInfo<'info>; RESUME_TOKEN_FACTORY_IX_ACCOUNTS_LEN] {
    fn from(accounts: ResumeTokenFactoryAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.gmtoken_manager_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESUME_TOKEN_FACTORY_IX_ACCOUNTS_LEN]>
for ResumeTokenFactoryAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; RESUME_TOKEN_FACTORY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            gmtoken_manager_state: &arr[2],
        }
    }
}
pub const RESUME_TOKEN_FACTORY_IX_DISCM: [u8; 8usize] = [
    235, 37, 239, 145, 244, 212, 31, 37,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ResumeTokenFactoryIxData;
impl ResumeTokenFactoryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESUME_TOKEN_FACTORY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESUME_TOKEN_FACTORY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn resume_token_factory_ix_with_program_id(
    program_id: Pubkey,
    keys: ResumeTokenFactoryKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RESUME_TOKEN_FACTORY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ResumeTokenFactoryIxData.try_to_vec()?,
    })
}
pub fn resume_token_factory_ix(
    keys: ResumeTokenFactoryKeys,
) -> std::io::Result<Instruction> {
    resume_token_factory_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn resume_token_factory_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ResumeTokenFactoryAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ResumeTokenFactoryKeys = accounts.into();
    let ix = resume_token_factory_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn resume_token_factory_invoke(
    accounts: ResumeTokenFactoryAccounts<'_, '_>,
) -> ProgramResult {
    resume_token_factory_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn resume_token_factory_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ResumeTokenFactoryAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ResumeTokenFactoryKeys = accounts.into();
    let ix = resume_token_factory_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn resume_token_factory_invoke_signed(
    accounts: ResumeTokenFactoryAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    resume_token_factory_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn resume_token_factory_verify_account_keys(
    accounts: ResumeTokenFactoryAccounts<'_, '_>,
    keys: ResumeTokenFactoryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn resume_token_factory_verify_writable_privileges<'me, 'info>(
    accounts: ResumeTokenFactoryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.gmtoken_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn resume_token_factory_verify_signer_privileges<'me, 'info>(
    accounts: ResumeTokenFactoryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn resume_token_factory_verify_account_privileges<'me, 'info>(
    accounts: ResumeTokenFactoryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    resume_token_factory_verify_writable_privileges(accounts)?;
    resume_token_factory_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RESUME_TOKEN_MINTING_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ResumeTokenMintingAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub token_limit_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResumeTokenMintingKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub token_limit_account: Pubkey,
}
impl From<ResumeTokenMintingAccounts<'_, '_>> for ResumeTokenMintingKeys {
    fn from(accounts: ResumeTokenMintingAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            token_limit_account: *accounts.token_limit_account.key,
        }
    }
}
impl From<ResumeTokenMintingKeys>
for [AccountMeta; RESUME_TOKEN_MINTING_IX_ACCOUNTS_LEN] {
    fn from(keys: ResumeTokenMintingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_limit_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; RESUME_TOKEN_MINTING_IX_ACCOUNTS_LEN]> for ResumeTokenMintingKeys {
    fn from(pubkeys: [Pubkey; RESUME_TOKEN_MINTING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            token_limit_account: pubkeys[2],
        }
    }
}
impl<'info> From<ResumeTokenMintingAccounts<'_, 'info>>
for [AccountInfo<'info>; RESUME_TOKEN_MINTING_IX_ACCOUNTS_LEN] {
    fn from(accounts: ResumeTokenMintingAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.token_limit_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESUME_TOKEN_MINTING_IX_ACCOUNTS_LEN]>
for ResumeTokenMintingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; RESUME_TOKEN_MINTING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            token_limit_account: &arr[2],
        }
    }
}
pub const RESUME_TOKEN_MINTING_IX_DISCM: [u8; 8usize] = [
    188, 102, 226, 249, 194, 29, 81, 248,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ResumeTokenMintingIxData;
impl ResumeTokenMintingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESUME_TOKEN_MINTING_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESUME_TOKEN_MINTING_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn resume_token_minting_ix_with_program_id(
    program_id: Pubkey,
    keys: ResumeTokenMintingKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RESUME_TOKEN_MINTING_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ResumeTokenMintingIxData.try_to_vec()?,
    })
}
pub fn resume_token_minting_ix(
    keys: ResumeTokenMintingKeys,
) -> std::io::Result<Instruction> {
    resume_token_minting_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn resume_token_minting_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ResumeTokenMintingAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ResumeTokenMintingKeys = accounts.into();
    let ix = resume_token_minting_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn resume_token_minting_invoke(
    accounts: ResumeTokenMintingAccounts<'_, '_>,
) -> ProgramResult {
    resume_token_minting_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn resume_token_minting_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ResumeTokenMintingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ResumeTokenMintingKeys = accounts.into();
    let ix = resume_token_minting_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn resume_token_minting_invoke_signed(
    accounts: ResumeTokenMintingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    resume_token_minting_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn resume_token_minting_verify_account_keys(
    accounts: ResumeTokenMintingAccounts<'_, '_>,
    keys: ResumeTokenMintingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.token_limit_account.key, keys.token_limit_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn resume_token_minting_verify_writable_privileges<'me, 'info>(
    accounts: ResumeTokenMintingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_limit_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn resume_token_minting_verify_signer_privileges<'me, 'info>(
    accounts: ResumeTokenMintingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn resume_token_minting_verify_account_privileges<'me, 'info>(
    accounts: ResumeTokenMintingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    resume_token_minting_verify_writable_privileges(accounts)?;
    resume_token_minting_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RESUME_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ResumeTokenRedemptionAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub token_limit_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResumeTokenRedemptionKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub token_limit_account: Pubkey,
}
impl From<ResumeTokenRedemptionAccounts<'_, '_>> for ResumeTokenRedemptionKeys {
    fn from(accounts: ResumeTokenRedemptionAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            token_limit_account: *accounts.token_limit_account.key,
        }
    }
}
impl From<ResumeTokenRedemptionKeys>
for [AccountMeta; RESUME_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN] {
    fn from(keys: ResumeTokenRedemptionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_limit_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; RESUME_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN]>
for ResumeTokenRedemptionKeys {
    fn from(pubkeys: [Pubkey; RESUME_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            token_limit_account: pubkeys[2],
        }
    }
}
impl<'info> From<ResumeTokenRedemptionAccounts<'_, 'info>>
for [AccountInfo<'info>; RESUME_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ResumeTokenRedemptionAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.token_limit_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESUME_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN]>
for ResumeTokenRedemptionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; RESUME_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            token_limit_account: &arr[2],
        }
    }
}
pub const RESUME_TOKEN_REDEMPTION_IX_DISCM: [u8; 8usize] = [
    152, 249, 213, 178, 85, 141, 122, 45,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ResumeTokenRedemptionIxData;
impl ResumeTokenRedemptionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESUME_TOKEN_REDEMPTION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESUME_TOKEN_REDEMPTION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn resume_token_redemption_ix_with_program_id(
    program_id: Pubkey,
    keys: ResumeTokenRedemptionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RESUME_TOKEN_REDEMPTION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ResumeTokenRedemptionIxData.try_to_vec()?,
    })
}
pub fn resume_token_redemption_ix(
    keys: ResumeTokenRedemptionKeys,
) -> std::io::Result<Instruction> {
    resume_token_redemption_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn resume_token_redemption_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ResumeTokenRedemptionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ResumeTokenRedemptionKeys = accounts.into();
    let ix = resume_token_redemption_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn resume_token_redemption_invoke(
    accounts: ResumeTokenRedemptionAccounts<'_, '_>,
) -> ProgramResult {
    resume_token_redemption_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn resume_token_redemption_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ResumeTokenRedemptionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ResumeTokenRedemptionKeys = accounts.into();
    let ix = resume_token_redemption_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn resume_token_redemption_invoke_signed(
    accounts: ResumeTokenRedemptionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    resume_token_redemption_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn resume_token_redemption_verify_account_keys(
    accounts: ResumeTokenRedemptionAccounts<'_, '_>,
    keys: ResumeTokenRedemptionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.token_limit_account.key, keys.token_limit_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn resume_token_redemption_verify_writable_privileges<'me, 'info>(
    accounts: ResumeTokenRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_limit_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn resume_token_redemption_verify_signer_privileges<'me, 'info>(
    accounts: ResumeTokenRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn resume_token_redemption_verify_account_privileges<'me, 'info>(
    accounts: ResumeTokenRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    resume_token_redemption_verify_writable_privileges(accounts)?;
    resume_token_redemption_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RETRIEVE_TOKENS_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct RetrieveTokensAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub source_vault: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RetrieveTokensKeys {
    pub authority: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub authority_role_account: Pubkey,
    pub token_mint: Pubkey,
    pub source_vault: Pubkey,
    pub destination: Pubkey,
    pub token_program: Pubkey,
}
impl From<RetrieveTokensAccounts<'_, '_>> for RetrieveTokensKeys {
    fn from(accounts: RetrieveTokensAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            authority_role_account: *accounts.authority_role_account.key,
            token_mint: *accounts.token_mint.key,
            source_vault: *accounts.source_vault.key,
            destination: *accounts.destination.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RetrieveTokensKeys> for [AccountMeta; RETRIEVE_TOKENS_IX_ACCOUNTS_LEN] {
    fn from(keys: RetrieveTokensKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination,
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
impl From<[Pubkey; RETRIEVE_TOKENS_IX_ACCOUNTS_LEN]> for RetrieveTokensKeys {
    fn from(pubkeys: [Pubkey; RETRIEVE_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            usdon_manager_state: pubkeys[1],
            authority_role_account: pubkeys[2],
            token_mint: pubkeys[3],
            source_vault: pubkeys[4],
            destination: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<RetrieveTokensAccounts<'_, 'info>>
for [AccountInfo<'info>; RETRIEVE_TOKENS_IX_ACCOUNTS_LEN] {
    fn from(accounts: RetrieveTokensAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.authority_role_account.clone(),
            accounts.token_mint.clone(),
            accounts.source_vault.clone(),
            accounts.destination.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RETRIEVE_TOKENS_IX_ACCOUNTS_LEN]>
for RetrieveTokensAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; RETRIEVE_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            usdon_manager_state: &arr[1],
            authority_role_account: &arr[2],
            token_mint: &arr[3],
            source_vault: &arr[4],
            destination: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const RETRIEVE_TOKENS_IX_DISCM: [u8; 8usize] = [208, 194, 68, 55, 183, 22, 93, 135];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RetrieveTokensIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RetrieveTokensIxData(pub RetrieveTokensIxArgs);
impl From<RetrieveTokensIxArgs> for RetrieveTokensIxData {
    fn from(args: RetrieveTokensIxArgs) -> Self {
        Self(args)
    }
}
impl RetrieveTokensIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RETRIEVE_TOKENS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(RetrieveTokensIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RETRIEVE_TOKENS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn retrieve_tokens_ix_with_program_id(
    program_id: Pubkey,
    keys: RetrieveTokensKeys,
    args: RetrieveTokensIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RETRIEVE_TOKENS_IX_ACCOUNTS_LEN] = keys.into();
    let data: RetrieveTokensIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn retrieve_tokens_ix(
    keys: RetrieveTokensKeys,
    args: RetrieveTokensIxArgs,
) -> std::io::Result<Instruction> {
    retrieve_tokens_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn retrieve_tokens_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RetrieveTokensAccounts<'_, '_>,
    args: RetrieveTokensIxArgs,
) -> ProgramResult {
    let keys: RetrieveTokensKeys = accounts.into();
    let ix = retrieve_tokens_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn retrieve_tokens_invoke(
    accounts: RetrieveTokensAccounts<'_, '_>,
    args: RetrieveTokensIxArgs,
) -> ProgramResult {
    retrieve_tokens_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn retrieve_tokens_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RetrieveTokensAccounts<'_, '_>,
    args: RetrieveTokensIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RetrieveTokensKeys = accounts.into();
    let ix = retrieve_tokens_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn retrieve_tokens_invoke_signed(
    accounts: RetrieveTokensAccounts<'_, '_>,
    args: RetrieveTokensIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    retrieve_tokens_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn retrieve_tokens_verify_account_keys(
    accounts: RetrieveTokensAccounts<'_, '_>,
    keys: RetrieveTokensKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.source_vault.key, keys.source_vault),
        (*accounts.destination.key, keys.destination),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn retrieve_tokens_verify_writable_privileges<'me, 'info>(
    accounts: RetrieveTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.source_vault, accounts.destination] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn retrieve_tokens_verify_signer_privileges<'me, 'info>(
    accounts: RetrieveTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn retrieve_tokens_verify_account_privileges<'me, 'info>(
    accounts: RetrieveTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    retrieve_tokens_verify_writable_privileges(accounts)?;
    retrieve_tokens_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REVOKE_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RevokeGmtokenFactoryRoleAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub role_to_revoke: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RevokeGmtokenFactoryRoleKeys {
    pub authority: Pubkey,
    pub recipient: Pubkey,
    pub authority_role_account: Pubkey,
    pub role_to_revoke: Pubkey,
    pub system_program: Pubkey,
}
impl From<RevokeGmtokenFactoryRoleAccounts<'_, '_>> for RevokeGmtokenFactoryRoleKeys {
    fn from(accounts: RevokeGmtokenFactoryRoleAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            recipient: *accounts.recipient.key,
            authority_role_account: *accounts.authority_role_account.key,
            role_to_revoke: *accounts.role_to_revoke.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RevokeGmtokenFactoryRoleKeys>
for [AccountMeta; REVOKE_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: RevokeGmtokenFactoryRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_revoke,
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
impl From<[Pubkey; REVOKE_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN]>
for RevokeGmtokenFactoryRoleKeys {
    fn from(pubkeys: [Pubkey; REVOKE_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            recipient: pubkeys[1],
            authority_role_account: pubkeys[2],
            role_to_revoke: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<RevokeGmtokenFactoryRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; REVOKE_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RevokeGmtokenFactoryRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.recipient.clone(),
            accounts.authority_role_account.clone(),
            accounts.role_to_revoke.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REVOKE_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN]>
for RevokeGmtokenFactoryRoleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REVOKE_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            recipient: &arr[1],
            authority_role_account: &arr[2],
            role_to_revoke: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const REVOKE_GMTOKEN_FACTORY_ROLE_IX_DISCM: [u8; 8usize] = [
    214, 48, 33, 96, 99, 36, 94, 198,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RevokeGmtokenFactoryRoleIxData;
impl RevokeGmtokenFactoryRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVOKE_GMTOKEN_FACTORY_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVOKE_GMTOKEN_FACTORY_ROLE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn revoke_gmtoken_factory_role_ix_with_program_id(
    program_id: Pubkey,
    keys: RevokeGmtokenFactoryRoleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REVOKE_GMTOKEN_FACTORY_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RevokeGmtokenFactoryRoleIxData.try_to_vec()?,
    })
}
pub fn revoke_gmtoken_factory_role_ix(
    keys: RevokeGmtokenFactoryRoleKeys,
) -> std::io::Result<Instruction> {
    revoke_gmtoken_factory_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn revoke_gmtoken_factory_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RevokeGmtokenFactoryRoleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RevokeGmtokenFactoryRoleKeys = accounts.into();
    let ix = revoke_gmtoken_factory_role_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn revoke_gmtoken_factory_role_invoke(
    accounts: RevokeGmtokenFactoryRoleAccounts<'_, '_>,
) -> ProgramResult {
    revoke_gmtoken_factory_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn revoke_gmtoken_factory_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RevokeGmtokenFactoryRoleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RevokeGmtokenFactoryRoleKeys = accounts.into();
    let ix = revoke_gmtoken_factory_role_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn revoke_gmtoken_factory_role_invoke_signed(
    accounts: RevokeGmtokenFactoryRoleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    revoke_gmtoken_factory_role_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn revoke_gmtoken_factory_role_verify_account_keys(
    accounts: RevokeGmtokenFactoryRoleAccounts<'_, '_>,
    keys: RevokeGmtokenFactoryRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.role_to_revoke.key, keys.role_to_revoke),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn revoke_gmtoken_factory_role_verify_writable_privileges<'me, 'info>(
    accounts: RevokeGmtokenFactoryRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.recipient, accounts.role_to_revoke] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn revoke_gmtoken_factory_role_verify_signer_privileges<'me, 'info>(
    accounts: RevokeGmtokenFactoryRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn revoke_gmtoken_factory_role_verify_account_privileges<'me, 'info>(
    accounts: RevokeGmtokenFactoryRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    revoke_gmtoken_factory_role_verify_writable_privileges(accounts)?;
    revoke_gmtoken_factory_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REVOKE_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RevokeGmtokenManagerRoleAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub role_to_revoke: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RevokeGmtokenManagerRoleKeys {
    pub authority: Pubkey,
    pub recipient: Pubkey,
    pub authority_role_account: Pubkey,
    pub role_to_revoke: Pubkey,
    pub system_program: Pubkey,
}
impl From<RevokeGmtokenManagerRoleAccounts<'_, '_>> for RevokeGmtokenManagerRoleKeys {
    fn from(accounts: RevokeGmtokenManagerRoleAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            recipient: *accounts.recipient.key,
            authority_role_account: *accounts.authority_role_account.key,
            role_to_revoke: *accounts.role_to_revoke.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RevokeGmtokenManagerRoleKeys>
for [AccountMeta; REVOKE_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: RevokeGmtokenManagerRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_revoke,
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
impl From<[Pubkey; REVOKE_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN]>
for RevokeGmtokenManagerRoleKeys {
    fn from(pubkeys: [Pubkey; REVOKE_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            recipient: pubkeys[1],
            authority_role_account: pubkeys[2],
            role_to_revoke: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<RevokeGmtokenManagerRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; REVOKE_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RevokeGmtokenManagerRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.recipient.clone(),
            accounts.authority_role_account.clone(),
            accounts.role_to_revoke.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REVOKE_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN]>
for RevokeGmtokenManagerRoleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REVOKE_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            recipient: &arr[1],
            authority_role_account: &arr[2],
            role_to_revoke: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const REVOKE_GMTOKEN_MANAGER_ROLE_IX_DISCM: [u8; 8usize] = [
    202, 63, 132, 81, 219, 242, 100, 90,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RevokeGmtokenManagerRoleIxData;
impl RevokeGmtokenManagerRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVOKE_GMTOKEN_MANAGER_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVOKE_GMTOKEN_MANAGER_ROLE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn revoke_gmtoken_manager_role_ix_with_program_id(
    program_id: Pubkey,
    keys: RevokeGmtokenManagerRoleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REVOKE_GMTOKEN_MANAGER_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RevokeGmtokenManagerRoleIxData.try_to_vec()?,
    })
}
pub fn revoke_gmtoken_manager_role_ix(
    keys: RevokeGmtokenManagerRoleKeys,
) -> std::io::Result<Instruction> {
    revoke_gmtoken_manager_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn revoke_gmtoken_manager_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RevokeGmtokenManagerRoleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RevokeGmtokenManagerRoleKeys = accounts.into();
    let ix = revoke_gmtoken_manager_role_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn revoke_gmtoken_manager_role_invoke(
    accounts: RevokeGmtokenManagerRoleAccounts<'_, '_>,
) -> ProgramResult {
    revoke_gmtoken_manager_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn revoke_gmtoken_manager_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RevokeGmtokenManagerRoleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RevokeGmtokenManagerRoleKeys = accounts.into();
    let ix = revoke_gmtoken_manager_role_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn revoke_gmtoken_manager_role_invoke_signed(
    accounts: RevokeGmtokenManagerRoleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    revoke_gmtoken_manager_role_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn revoke_gmtoken_manager_role_verify_account_keys(
    accounts: RevokeGmtokenManagerRoleAccounts<'_, '_>,
    keys: RevokeGmtokenManagerRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.role_to_revoke.key, keys.role_to_revoke),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn revoke_gmtoken_manager_role_verify_writable_privileges<'me, 'info>(
    accounts: RevokeGmtokenManagerRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.recipient, accounts.role_to_revoke] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn revoke_gmtoken_manager_role_verify_signer_privileges<'me, 'info>(
    accounts: RevokeGmtokenManagerRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn revoke_gmtoken_manager_role_verify_account_privileges<'me, 'info>(
    accounts: RevokeGmtokenManagerRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    revoke_gmtoken_manager_role_verify_writable_privileges(accounts)?;
    revoke_gmtoken_manager_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REVOKE_GMTOKEN_ROLE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct RevokeGmtokenRoleAccounts<'me, 'info> {
    pub recipient: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub role_to_revoke: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RevokeGmtokenRoleKeys {
    pub recipient: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub role_to_revoke: Pubkey,
}
impl From<RevokeGmtokenRoleAccounts<'_, '_>> for RevokeGmtokenRoleKeys {
    fn from(accounts: RevokeGmtokenRoleAccounts) -> Self {
        Self {
            recipient: *accounts.recipient.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            role_to_revoke: *accounts.role_to_revoke.key,
        }
    }
}
impl From<RevokeGmtokenRoleKeys> for [AccountMeta; REVOKE_GMTOKEN_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: RevokeGmtokenRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_revoke,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REVOKE_GMTOKEN_ROLE_IX_ACCOUNTS_LEN]> for RevokeGmtokenRoleKeys {
    fn from(pubkeys: [Pubkey; REVOKE_GMTOKEN_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            recipient: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            role_to_revoke: pubkeys[3],
        }
    }
}
impl<'info> From<RevokeGmtokenRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; REVOKE_GMTOKEN_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RevokeGmtokenRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.recipient.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.role_to_revoke.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REVOKE_GMTOKEN_ROLE_IX_ACCOUNTS_LEN]>
for RevokeGmtokenRoleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REVOKE_GMTOKEN_ROLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            recipient: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            role_to_revoke: &arr[3],
        }
    }
}
pub const REVOKE_GMTOKEN_ROLE_IX_DISCM: [u8; 8usize] = [
    108, 55, 0, 39, 40, 39, 103, 144,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RevokeGmtokenRoleIxData;
impl RevokeGmtokenRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVOKE_GMTOKEN_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVOKE_GMTOKEN_ROLE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn revoke_gmtoken_role_ix_with_program_id(
    program_id: Pubkey,
    keys: RevokeGmtokenRoleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REVOKE_GMTOKEN_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RevokeGmtokenRoleIxData.try_to_vec()?,
    })
}
pub fn revoke_gmtoken_role_ix(
    keys: RevokeGmtokenRoleKeys,
) -> std::io::Result<Instruction> {
    revoke_gmtoken_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn revoke_gmtoken_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RevokeGmtokenRoleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RevokeGmtokenRoleKeys = accounts.into();
    let ix = revoke_gmtoken_role_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn revoke_gmtoken_role_invoke(
    accounts: RevokeGmtokenRoleAccounts<'_, '_>,
) -> ProgramResult {
    revoke_gmtoken_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn revoke_gmtoken_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RevokeGmtokenRoleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RevokeGmtokenRoleKeys = accounts.into();
    let ix = revoke_gmtoken_role_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn revoke_gmtoken_role_invoke_signed(
    accounts: RevokeGmtokenRoleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    revoke_gmtoken_role_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn revoke_gmtoken_role_verify_account_keys(
    accounts: RevokeGmtokenRoleAccounts<'_, '_>,
    keys: RevokeGmtokenRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.recipient.key, keys.recipient),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.role_to_revoke.key, keys.role_to_revoke),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn revoke_gmtoken_role_verify_writable_privileges<'me, 'info>(
    accounts: RevokeGmtokenRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.recipient, accounts.role_to_revoke] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn revoke_gmtoken_role_verify_signer_privileges<'me, 'info>(
    accounts: RevokeGmtokenRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn revoke_gmtoken_role_verify_account_privileges<'me, 'info>(
    accounts: RevokeGmtokenRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    revoke_gmtoken_role_verify_writable_privileges(accounts)?;
    revoke_gmtoken_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REVOKE_ROLE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RevokeRoleAccounts<'me, 'info> {
    pub recipient: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub role_to_revoke: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub program_data: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RevokeRoleKeys {
    pub recipient: Pubkey,
    pub authority: Pubkey,
    pub role_to_revoke: Pubkey,
    pub system_program: Pubkey,
    pub program: Pubkey,
    pub program_data: Pubkey,
}
impl From<RevokeRoleAccounts<'_, '_>> for RevokeRoleKeys {
    fn from(accounts: RevokeRoleAccounts) -> Self {
        Self {
            recipient: *accounts.recipient.key,
            authority: *accounts.authority.key,
            role_to_revoke: *accounts.role_to_revoke.key,
            system_program: *accounts.system_program.key,
            program: *accounts.program.key,
            program_data: *accounts.program_data.key,
        }
    }
}
impl From<RevokeRoleKeys> for [AccountMeta; REVOKE_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: RevokeRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_revoke,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.program_data,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REVOKE_ROLE_IX_ACCOUNTS_LEN]> for RevokeRoleKeys {
    fn from(pubkeys: [Pubkey; REVOKE_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            recipient: pubkeys[0],
            authority: pubkeys[1],
            role_to_revoke: pubkeys[2],
            system_program: pubkeys[3],
            program: pubkeys[4],
            program_data: pubkeys[5],
        }
    }
}
impl<'info> From<RevokeRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; REVOKE_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RevokeRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.recipient.clone(),
            accounts.authority.clone(),
            accounts.role_to_revoke.clone(),
            accounts.system_program.clone(),
            accounts.program.clone(),
            accounts.program_data.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REVOKE_ROLE_IX_ACCOUNTS_LEN]>
for RevokeRoleAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REVOKE_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            recipient: &arr[0],
            authority: &arr[1],
            role_to_revoke: &arr[2],
            system_program: &arr[3],
            program: &arr[4],
            program_data: &arr[5],
        }
    }
}
pub const REVOKE_ROLE_IX_DISCM: [u8; 8usize] = [179, 232, 2, 180, 48, 227, 82, 7];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RevokeRoleIxArgs {
    pub _role: RoleType,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RevokeRoleIxData(pub RevokeRoleIxArgs);
impl From<RevokeRoleIxArgs> for RevokeRoleIxData {
    fn from(args: RevokeRoleIxArgs) -> Self {
        Self(args)
    }
}
impl RevokeRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVOKE_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let _role: RoleType = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(RevokeRoleIxArgs { _role }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVOKE_ROLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0._role, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn revoke_role_ix_with_program_id(
    program_id: Pubkey,
    keys: RevokeRoleKeys,
    args: RevokeRoleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REVOKE_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: RevokeRoleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn revoke_role_ix(
    keys: RevokeRoleKeys,
    args: RevokeRoleIxArgs,
) -> std::io::Result<Instruction> {
    revoke_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn revoke_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RevokeRoleAccounts<'_, '_>,
    args: RevokeRoleIxArgs,
) -> ProgramResult {
    let keys: RevokeRoleKeys = accounts.into();
    let ix = revoke_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn revoke_role_invoke(
    accounts: RevokeRoleAccounts<'_, '_>,
    args: RevokeRoleIxArgs,
) -> ProgramResult {
    revoke_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn revoke_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RevokeRoleAccounts<'_, '_>,
    args: RevokeRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RevokeRoleKeys = accounts.into();
    let ix = revoke_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn revoke_role_invoke_signed(
    accounts: RevokeRoleAccounts<'_, '_>,
    args: RevokeRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    revoke_role_invoke_signed_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args, seeds)
}
pub fn revoke_role_verify_account_keys(
    accounts: RevokeRoleAccounts<'_, '_>,
    keys: RevokeRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.recipient.key, keys.recipient),
        (*accounts.authority.key, keys.authority),
        (*accounts.role_to_revoke.key, keys.role_to_revoke),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program.key, keys.program),
        (*accounts.program_data.key, keys.program_data),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn revoke_role_verify_writable_privileges<'me, 'info>(
    accounts: RevokeRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.recipient, accounts.role_to_revoke] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn revoke_role_verify_signer_privileges<'me, 'info>(
    accounts: RevokeRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn revoke_role_verify_account_privileges<'me, 'info>(
    accounts: RevokeRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    revoke_role_verify_writable_privileges(accounts)?;
    revoke_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REVOKE_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RevokeSanityConfigurerRoleAccounts<'me, 'info> {
    pub recipient: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub role_to_revoke: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RevokeSanityConfigurerRoleKeys {
    pub recipient: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub role_to_revoke: Pubkey,
    pub system_program: Pubkey,
}
impl From<RevokeSanityConfigurerRoleAccounts<'_, '_>>
for RevokeSanityConfigurerRoleKeys {
    fn from(accounts: RevokeSanityConfigurerRoleAccounts) -> Self {
        Self {
            recipient: *accounts.recipient.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            role_to_revoke: *accounts.role_to_revoke.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RevokeSanityConfigurerRoleKeys>
for [AccountMeta; REVOKE_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: RevokeSanityConfigurerRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_revoke,
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
impl From<[Pubkey; REVOKE_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN]>
for RevokeSanityConfigurerRoleKeys {
    fn from(pubkeys: [Pubkey; REVOKE_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            recipient: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            role_to_revoke: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<RevokeSanityConfigurerRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; REVOKE_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RevokeSanityConfigurerRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.recipient.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.role_to_revoke.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REVOKE_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN]>
for RevokeSanityConfigurerRoleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REVOKE_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            recipient: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            role_to_revoke: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const REVOKE_SANITY_CONFIGURER_ROLE_IX_DISCM: [u8; 8usize] = [
    170, 122, 0, 190, 220, 165, 67, 86,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RevokeSanityConfigurerRoleIxData;
impl RevokeSanityConfigurerRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVOKE_SANITY_CONFIGURER_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVOKE_SANITY_CONFIGURER_ROLE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn revoke_sanity_configurer_role_ix_with_program_id(
    program_id: Pubkey,
    keys: RevokeSanityConfigurerRoleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REVOKE_SANITY_CONFIGURER_ROLE_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RevokeSanityConfigurerRoleIxData.try_to_vec()?,
    })
}
pub fn revoke_sanity_configurer_role_ix(
    keys: RevokeSanityConfigurerRoleKeys,
) -> std::io::Result<Instruction> {
    revoke_sanity_configurer_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn revoke_sanity_configurer_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RevokeSanityConfigurerRoleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RevokeSanityConfigurerRoleKeys = accounts.into();
    let ix = revoke_sanity_configurer_role_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn revoke_sanity_configurer_role_invoke(
    accounts: RevokeSanityConfigurerRoleAccounts<'_, '_>,
) -> ProgramResult {
    revoke_sanity_configurer_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn revoke_sanity_configurer_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RevokeSanityConfigurerRoleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RevokeSanityConfigurerRoleKeys = accounts.into();
    let ix = revoke_sanity_configurer_role_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn revoke_sanity_configurer_role_invoke_signed(
    accounts: RevokeSanityConfigurerRoleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    revoke_sanity_configurer_role_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn revoke_sanity_configurer_role_verify_account_keys(
    accounts: RevokeSanityConfigurerRoleAccounts<'_, '_>,
    keys: RevokeSanityConfigurerRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.recipient.key, keys.recipient),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.role_to_revoke.key, keys.role_to_revoke),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn revoke_sanity_configurer_role_verify_writable_privileges<'me, 'info>(
    accounts: RevokeSanityConfigurerRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.recipient, accounts.role_to_revoke] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn revoke_sanity_configurer_role_verify_signer_privileges<'me, 'info>(
    accounts: RevokeSanityConfigurerRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn revoke_sanity_configurer_role_verify_account_privileges<'me, 'info>(
    accounts: RevokeSanityConfigurerRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    revoke_sanity_configurer_role_verify_writable_privileges(accounts)?;
    revoke_sanity_configurer_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REVOKE_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RevokeSanitySetterRoleAccounts<'me, 'info> {
    pub recipient: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub role_to_revoke: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RevokeSanitySetterRoleKeys {
    pub recipient: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub role_to_revoke: Pubkey,
    pub system_program: Pubkey,
}
impl From<RevokeSanitySetterRoleAccounts<'_, '_>> for RevokeSanitySetterRoleKeys {
    fn from(accounts: RevokeSanitySetterRoleAccounts) -> Self {
        Self {
            recipient: *accounts.recipient.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            role_to_revoke: *accounts.role_to_revoke.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RevokeSanitySetterRoleKeys>
for [AccountMeta; REVOKE_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: RevokeSanitySetterRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_revoke,
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
impl From<[Pubkey; REVOKE_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN]>
for RevokeSanitySetterRoleKeys {
    fn from(pubkeys: [Pubkey; REVOKE_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            recipient: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            role_to_revoke: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<RevokeSanitySetterRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; REVOKE_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RevokeSanitySetterRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.recipient.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.role_to_revoke.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REVOKE_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN]>
for RevokeSanitySetterRoleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REVOKE_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            recipient: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            role_to_revoke: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const REVOKE_SANITY_SETTER_ROLE_IX_DISCM: [u8; 8usize] = [
    224, 165, 35, 213, 84, 53, 9, 229,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RevokeSanitySetterRoleIxData;
impl RevokeSanitySetterRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVOKE_SANITY_SETTER_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVOKE_SANITY_SETTER_ROLE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn revoke_sanity_setter_role_ix_with_program_id(
    program_id: Pubkey,
    keys: RevokeSanitySetterRoleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REVOKE_SANITY_SETTER_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RevokeSanitySetterRoleIxData.try_to_vec()?,
    })
}
pub fn revoke_sanity_setter_role_ix(
    keys: RevokeSanitySetterRoleKeys,
) -> std::io::Result<Instruction> {
    revoke_sanity_setter_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn revoke_sanity_setter_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RevokeSanitySetterRoleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RevokeSanitySetterRoleKeys = accounts.into();
    let ix = revoke_sanity_setter_role_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn revoke_sanity_setter_role_invoke(
    accounts: RevokeSanitySetterRoleAccounts<'_, '_>,
) -> ProgramResult {
    revoke_sanity_setter_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn revoke_sanity_setter_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RevokeSanitySetterRoleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RevokeSanitySetterRoleKeys = accounts.into();
    let ix = revoke_sanity_setter_role_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn revoke_sanity_setter_role_invoke_signed(
    accounts: RevokeSanitySetterRoleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    revoke_sanity_setter_role_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn revoke_sanity_setter_role_verify_account_keys(
    accounts: RevokeSanitySetterRoleAccounts<'_, '_>,
    keys: RevokeSanitySetterRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.recipient.key, keys.recipient),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.role_to_revoke.key, keys.role_to_revoke),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn revoke_sanity_setter_role_verify_writable_privileges<'me, 'info>(
    accounts: RevokeSanitySetterRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.recipient, accounts.role_to_revoke] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn revoke_sanity_setter_role_verify_signer_privileges<'me, 'info>(
    accounts: RevokeSanitySetterRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn revoke_sanity_setter_role_verify_account_privileges<'me, 'info>(
    accounts: RevokeSanitySetterRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    revoke_sanity_setter_role_verify_writable_privileges(accounts)?;
    revoke_sanity_setter_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REVOKE_USDON_ROLE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct RevokeUsdonRoleAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub role_to_revoke: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RevokeUsdonRoleKeys {
    pub authority: Pubkey,
    pub recipient: Pubkey,
    pub authority_role_account: Pubkey,
    pub role_to_revoke: Pubkey,
}
impl From<RevokeUsdonRoleAccounts<'_, '_>> for RevokeUsdonRoleKeys {
    fn from(accounts: RevokeUsdonRoleAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            recipient: *accounts.recipient.key,
            authority_role_account: *accounts.authority_role_account.key,
            role_to_revoke: *accounts.role_to_revoke.key,
        }
    }
}
impl From<RevokeUsdonRoleKeys> for [AccountMeta; REVOKE_USDON_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: RevokeUsdonRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.role_to_revoke,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REVOKE_USDON_ROLE_IX_ACCOUNTS_LEN]> for RevokeUsdonRoleKeys {
    fn from(pubkeys: [Pubkey; REVOKE_USDON_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            recipient: pubkeys[1],
            authority_role_account: pubkeys[2],
            role_to_revoke: pubkeys[3],
        }
    }
}
impl<'info> From<RevokeUsdonRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; REVOKE_USDON_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RevokeUsdonRoleAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.recipient.clone(),
            accounts.authority_role_account.clone(),
            accounts.role_to_revoke.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REVOKE_USDON_ROLE_IX_ACCOUNTS_LEN]>
for RevokeUsdonRoleAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REVOKE_USDON_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            recipient: &arr[1],
            authority_role_account: &arr[2],
            role_to_revoke: &arr[3],
        }
    }
}
pub const REVOKE_USDON_ROLE_IX_DISCM: [u8; 8usize] = [
    231, 35, 217, 203, 112, 165, 182, 69,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RevokeUsdonRoleIxData;
impl RevokeUsdonRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVOKE_USDON_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVOKE_USDON_ROLE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn revoke_usdon_role_ix_with_program_id(
    program_id: Pubkey,
    keys: RevokeUsdonRoleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REVOKE_USDON_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RevokeUsdonRoleIxData.try_to_vec()?,
    })
}
pub fn revoke_usdon_role_ix(keys: RevokeUsdonRoleKeys) -> std::io::Result<Instruction> {
    revoke_usdon_role_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys)
}
pub fn revoke_usdon_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RevokeUsdonRoleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RevokeUsdonRoleKeys = accounts.into();
    let ix = revoke_usdon_role_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn revoke_usdon_role_invoke(
    accounts: RevokeUsdonRoleAccounts<'_, '_>,
) -> ProgramResult {
    revoke_usdon_role_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts)
}
pub fn revoke_usdon_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RevokeUsdonRoleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RevokeUsdonRoleKeys = accounts.into();
    let ix = revoke_usdon_role_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn revoke_usdon_role_invoke_signed(
    accounts: RevokeUsdonRoleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    revoke_usdon_role_invoke_signed_with_program_id(ONDO_GM_PROGRAM_ID, accounts, seeds)
}
pub fn revoke_usdon_role_verify_account_keys(
    accounts: RevokeUsdonRoleAccounts<'_, '_>,
    keys: RevokeUsdonRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.role_to_revoke.key, keys.role_to_revoke),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn revoke_usdon_role_verify_writable_privileges<'me, 'info>(
    accounts: RevokeUsdonRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.recipient, accounts.role_to_revoke] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn revoke_usdon_role_verify_signer_privileges<'me, 'info>(
    accounts: RevokeUsdonRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn revoke_usdon_role_verify_account_privileges<'me, 'info>(
    accounts: RevokeUsdonRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    revoke_usdon_role_verify_writable_privileges(accounts)?;
    revoke_usdon_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_ALLOWED_DEVIATION_BPS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetAllowedDeviationBpsAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub sanity_check_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetAllowedDeviationBpsKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub mint: Pubkey,
    pub sanity_check_account: Pubkey,
}
impl From<SetAllowedDeviationBpsAccounts<'_, '_>> for SetAllowedDeviationBpsKeys {
    fn from(accounts: SetAllowedDeviationBpsAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            mint: *accounts.mint.key,
            sanity_check_account: *accounts.sanity_check_account.key,
        }
    }
}
impl From<SetAllowedDeviationBpsKeys>
for [AccountMeta; SET_ALLOWED_DEVIATION_BPS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetAllowedDeviationBpsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sanity_check_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_ALLOWED_DEVIATION_BPS_IX_ACCOUNTS_LEN]>
for SetAllowedDeviationBpsKeys {
    fn from(pubkeys: [Pubkey; SET_ALLOWED_DEVIATION_BPS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            mint: pubkeys[2],
            sanity_check_account: pubkeys[3],
        }
    }
}
impl<'info> From<SetAllowedDeviationBpsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_ALLOWED_DEVIATION_BPS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetAllowedDeviationBpsAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.mint.clone(),
            accounts.sanity_check_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_ALLOWED_DEVIATION_BPS_IX_ACCOUNTS_LEN]>
for SetAllowedDeviationBpsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_ALLOWED_DEVIATION_BPS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            mint: &arr[2],
            sanity_check_account: &arr[3],
        }
    }
}
pub const SET_ALLOWED_DEVIATION_BPS_IX_DISCM: [u8; 8usize] = [
    46, 36, 29, 85, 194, 180, 86, 124,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetAllowedDeviationBpsIxArgs {
    pub allowed_deviation_bps: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetAllowedDeviationBpsIxData(pub SetAllowedDeviationBpsIxArgs);
impl From<SetAllowedDeviationBpsIxArgs> for SetAllowedDeviationBpsIxData {
    fn from(args: SetAllowedDeviationBpsIxArgs) -> Self {
        Self(args)
    }
}
impl SetAllowedDeviationBpsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_ALLOWED_DEVIATION_BPS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let allowed_deviation_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetAllowedDeviationBpsIxArgs {
                allowed_deviation_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_ALLOWED_DEVIATION_BPS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.allowed_deviation_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_allowed_deviation_bps_ix_with_program_id(
    program_id: Pubkey,
    keys: SetAllowedDeviationBpsKeys,
    args: SetAllowedDeviationBpsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_ALLOWED_DEVIATION_BPS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetAllowedDeviationBpsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_allowed_deviation_bps_ix(
    keys: SetAllowedDeviationBpsKeys,
    args: SetAllowedDeviationBpsIxArgs,
) -> std::io::Result<Instruction> {
    set_allowed_deviation_bps_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn set_allowed_deviation_bps_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetAllowedDeviationBpsAccounts<'_, '_>,
    args: SetAllowedDeviationBpsIxArgs,
) -> ProgramResult {
    let keys: SetAllowedDeviationBpsKeys = accounts.into();
    let ix = set_allowed_deviation_bps_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_allowed_deviation_bps_invoke(
    accounts: SetAllowedDeviationBpsAccounts<'_, '_>,
    args: SetAllowedDeviationBpsIxArgs,
) -> ProgramResult {
    set_allowed_deviation_bps_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn set_allowed_deviation_bps_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetAllowedDeviationBpsAccounts<'_, '_>,
    args: SetAllowedDeviationBpsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetAllowedDeviationBpsKeys = accounts.into();
    let ix = set_allowed_deviation_bps_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_allowed_deviation_bps_invoke_signed(
    accounts: SetAllowedDeviationBpsAccounts<'_, '_>,
    args: SetAllowedDeviationBpsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_allowed_deviation_bps_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_allowed_deviation_bps_verify_account_keys(
    accounts: SetAllowedDeviationBpsAccounts<'_, '_>,
    keys: SetAllowedDeviationBpsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.sanity_check_account.key, keys.sanity_check_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_allowed_deviation_bps_verify_writable_privileges<'me, 'info>(
    accounts: SetAllowedDeviationBpsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.sanity_check_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_allowed_deviation_bps_verify_signer_privileges<'me, 'info>(
    accounts: SetAllowedDeviationBpsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_allowed_deviation_bps_verify_account_privileges<'me, 'info>(
    accounts: SetAllowedDeviationBpsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_allowed_deviation_bps_verify_writable_privileges(accounts)?;
    set_allowed_deviation_bps_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_ATTESTATION_SIGNER_SECP_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetAttestationSignerSecpAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub gmtoken_manager_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetAttestationSignerSecpKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub gmtoken_manager_state: Pubkey,
}
impl From<SetAttestationSignerSecpAccounts<'_, '_>> for SetAttestationSignerSecpKeys {
    fn from(accounts: SetAttestationSignerSecpAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            gmtoken_manager_state: *accounts.gmtoken_manager_state.key,
        }
    }
}
impl From<SetAttestationSignerSecpKeys>
for [AccountMeta; SET_ATTESTATION_SIGNER_SECP_IX_ACCOUNTS_LEN] {
    fn from(keys: SetAttestationSignerSecpKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.gmtoken_manager_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_ATTESTATION_SIGNER_SECP_IX_ACCOUNTS_LEN]>
for SetAttestationSignerSecpKeys {
    fn from(pubkeys: [Pubkey; SET_ATTESTATION_SIGNER_SECP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            gmtoken_manager_state: pubkeys[2],
        }
    }
}
impl<'info> From<SetAttestationSignerSecpAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_ATTESTATION_SIGNER_SECP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetAttestationSignerSecpAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.gmtoken_manager_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_ATTESTATION_SIGNER_SECP_IX_ACCOUNTS_LEN]>
for SetAttestationSignerSecpAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_ATTESTATION_SIGNER_SECP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            gmtoken_manager_state: &arr[2],
        }
    }
}
pub const SET_ATTESTATION_SIGNER_SECP_IX_DISCM: [u8; 8usize] = [
    121, 157, 129, 65, 181, 51, 55, 0,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetAttestationSignerSecpIxArgs {
    pub attestation_signer_secp: [u8; 20],
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetAttestationSignerSecpIxData(pub SetAttestationSignerSecpIxArgs);
impl From<SetAttestationSignerSecpIxArgs> for SetAttestationSignerSecpIxData {
    fn from(args: SetAttestationSignerSecpIxArgs) -> Self {
        Self(args)
    }
}
impl SetAttestationSignerSecpIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_ATTESTATION_SIGNER_SECP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let attestation_signer_secp: [u8; 20] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetAttestationSignerSecpIxArgs {
                attestation_signer_secp,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_ATTESTATION_SIGNER_SECP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.attestation_signer_secp, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_attestation_signer_secp_ix_with_program_id(
    program_id: Pubkey,
    keys: SetAttestationSignerSecpKeys,
    args: SetAttestationSignerSecpIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_ATTESTATION_SIGNER_SECP_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetAttestationSignerSecpIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_attestation_signer_secp_ix(
    keys: SetAttestationSignerSecpKeys,
    args: SetAttestationSignerSecpIxArgs,
) -> std::io::Result<Instruction> {
    set_attestation_signer_secp_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn set_attestation_signer_secp_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetAttestationSignerSecpAccounts<'_, '_>,
    args: SetAttestationSignerSecpIxArgs,
) -> ProgramResult {
    let keys: SetAttestationSignerSecpKeys = accounts.into();
    let ix = set_attestation_signer_secp_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_attestation_signer_secp_invoke(
    accounts: SetAttestationSignerSecpAccounts<'_, '_>,
    args: SetAttestationSignerSecpIxArgs,
) -> ProgramResult {
    set_attestation_signer_secp_invoke_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_attestation_signer_secp_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetAttestationSignerSecpAccounts<'_, '_>,
    args: SetAttestationSignerSecpIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetAttestationSignerSecpKeys = accounts.into();
    let ix = set_attestation_signer_secp_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_attestation_signer_secp_invoke_signed(
    accounts: SetAttestationSignerSecpAccounts<'_, '_>,
    args: SetAttestationSignerSecpIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_attestation_signer_secp_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_attestation_signer_secp_verify_account_keys(
    accounts: SetAttestationSignerSecpAccounts<'_, '_>,
    keys: SetAttestationSignerSecpKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.gmtoken_manager_state.key, keys.gmtoken_manager_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_attestation_signer_secp_verify_writable_privileges<'me, 'info>(
    accounts: SetAttestationSignerSecpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.gmtoken_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_attestation_signer_secp_verify_signer_privileges<'me, 'info>(
    accounts: SetAttestationSignerSecpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_attestation_signer_secp_verify_account_privileges<'me, 'info>(
    accounts: SetAttestationSignerSecpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_attestation_signer_secp_verify_writable_privileges(accounts)?;
    set_attestation_signer_secp_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_LAST_PRICE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetLastPriceAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub sanity_check_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetLastPriceKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub mint: Pubkey,
    pub sanity_check_account: Pubkey,
}
impl From<SetLastPriceAccounts<'_, '_>> for SetLastPriceKeys {
    fn from(accounts: SetLastPriceAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            mint: *accounts.mint.key,
            sanity_check_account: *accounts.sanity_check_account.key,
        }
    }
}
impl From<SetLastPriceKeys> for [AccountMeta; SET_LAST_PRICE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLastPriceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sanity_check_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_LAST_PRICE_IX_ACCOUNTS_LEN]> for SetLastPriceKeys {
    fn from(pubkeys: [Pubkey; SET_LAST_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            mint: pubkeys[2],
            sanity_check_account: pubkeys[3],
        }
    }
}
impl<'info> From<SetLastPriceAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_LAST_PRICE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetLastPriceAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.mint.clone(),
            accounts.sanity_check_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LAST_PRICE_IX_ACCOUNTS_LEN]>
for SetLastPriceAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_LAST_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            mint: &arr[2],
            sanity_check_account: &arr[3],
        }
    }
}
pub const SET_LAST_PRICE_IX_DISCM: [u8; 8usize] = [58, 56, 189, 177, 199, 145, 145, 43];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetLastPriceIxArgs {
    pub last_price: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLastPriceIxData(pub SetLastPriceIxArgs);
impl From<SetLastPriceIxArgs> for SetLastPriceIxData {
    fn from(args: SetLastPriceIxArgs) -> Self {
        Self(args)
    }
}
impl SetLastPriceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_LAST_PRICE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let last_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetLastPriceIxArgs { last_price }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_LAST_PRICE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.last_price, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_last_price_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLastPriceKeys,
    args: SetLastPriceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_LAST_PRICE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetLastPriceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_last_price_ix(
    keys: SetLastPriceKeys,
    args: SetLastPriceIxArgs,
) -> std::io::Result<Instruction> {
    set_last_price_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn set_last_price_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLastPriceAccounts<'_, '_>,
    args: SetLastPriceIxArgs,
) -> ProgramResult {
    let keys: SetLastPriceKeys = accounts.into();
    let ix = set_last_price_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_last_price_invoke(
    accounts: SetLastPriceAccounts<'_, '_>,
    args: SetLastPriceIxArgs,
) -> ProgramResult {
    set_last_price_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn set_last_price_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLastPriceAccounts<'_, '_>,
    args: SetLastPriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLastPriceKeys = accounts.into();
    let ix = set_last_price_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_last_price_invoke_signed(
    accounts: SetLastPriceAccounts<'_, '_>,
    args: SetLastPriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_last_price_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_last_price_verify_account_keys(
    accounts: SetLastPriceAccounts<'_, '_>,
    keys: SetLastPriceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.sanity_check_account.key, keys.sanity_check_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_last_price_verify_writable_privileges<'me, 'info>(
    accounts: SetLastPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.sanity_check_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_last_price_verify_signer_privileges<'me, 'info>(
    accounts: SetLastPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_last_price_verify_account_privileges<'me, 'info>(
    accounts: SetLastPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_last_price_verify_writable_privileges(accounts)?;
    set_last_price_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_MAX_TIME_DELAY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetMaxTimeDelayAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub sanity_check_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMaxTimeDelayKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub mint: Pubkey,
    pub sanity_check_account: Pubkey,
}
impl From<SetMaxTimeDelayAccounts<'_, '_>> for SetMaxTimeDelayKeys {
    fn from(accounts: SetMaxTimeDelayAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            mint: *accounts.mint.key,
            sanity_check_account: *accounts.sanity_check_account.key,
        }
    }
}
impl From<SetMaxTimeDelayKeys> for [AccountMeta; SET_MAX_TIME_DELAY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMaxTimeDelayKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sanity_check_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_MAX_TIME_DELAY_IX_ACCOUNTS_LEN]> for SetMaxTimeDelayKeys {
    fn from(pubkeys: [Pubkey; SET_MAX_TIME_DELAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            mint: pubkeys[2],
            sanity_check_account: pubkeys[3],
        }
    }
}
impl<'info> From<SetMaxTimeDelayAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_MAX_TIME_DELAY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMaxTimeDelayAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.mint.clone(),
            accounts.sanity_check_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MAX_TIME_DELAY_IX_ACCOUNTS_LEN]>
for SetMaxTimeDelayAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_MAX_TIME_DELAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            mint: &arr[2],
            sanity_check_account: &arr[3],
        }
    }
}
pub const SET_MAX_TIME_DELAY_IX_DISCM: [u8; 8usize] = [
    145, 127, 145, 218, 239, 173, 252, 9,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetMaxTimeDelayIxArgs {
    pub max_time_delay: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetMaxTimeDelayIxData(pub SetMaxTimeDelayIxArgs);
impl From<SetMaxTimeDelayIxArgs> for SetMaxTimeDelayIxData {
    fn from(args: SetMaxTimeDelayIxArgs) -> Self {
        Self(args)
    }
}
impl SetMaxTimeDelayIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_MAX_TIME_DELAY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_time_delay: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetMaxTimeDelayIxArgs {
                max_time_delay,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_MAX_TIME_DELAY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_time_delay, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_max_time_delay_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMaxTimeDelayKeys,
    args: SetMaxTimeDelayIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MAX_TIME_DELAY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetMaxTimeDelayIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_max_time_delay_ix(
    keys: SetMaxTimeDelayKeys,
    args: SetMaxTimeDelayIxArgs,
) -> std::io::Result<Instruction> {
    set_max_time_delay_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn set_max_time_delay_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMaxTimeDelayAccounts<'_, '_>,
    args: SetMaxTimeDelayIxArgs,
) -> ProgramResult {
    let keys: SetMaxTimeDelayKeys = accounts.into();
    let ix = set_max_time_delay_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_max_time_delay_invoke(
    accounts: SetMaxTimeDelayAccounts<'_, '_>,
    args: SetMaxTimeDelayIxArgs,
) -> ProgramResult {
    set_max_time_delay_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn set_max_time_delay_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMaxTimeDelayAccounts<'_, '_>,
    args: SetMaxTimeDelayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetMaxTimeDelayKeys = accounts.into();
    let ix = set_max_time_delay_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_max_time_delay_invoke_signed(
    accounts: SetMaxTimeDelayAccounts<'_, '_>,
    args: SetMaxTimeDelayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_max_time_delay_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_max_time_delay_verify_account_keys(
    accounts: SetMaxTimeDelayAccounts<'_, '_>,
    keys: SetMaxTimeDelayKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.sanity_check_account.key, keys.sanity_check_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_max_time_delay_verify_writable_privileges<'me, 'info>(
    accounts: SetMaxTimeDelayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.sanity_check_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_max_time_delay_verify_signer_privileges<'me, 'info>(
    accounts: SetMaxTimeDelayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_max_time_delay_verify_account_privileges<'me, 'info>(
    accounts: SetMaxTimeDelayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_max_time_delay_verify_writable_privileges(accounts)?;
    set_max_time_delay_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_ONDO_USER_LIMITS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SetOndoUserLimitsAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub ondo_user: &'me AccountInfo<'info>,
    pub token_limit: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetOndoUserLimitsKeys {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub authority_role_account: Pubkey,
    pub ondo_user: Pubkey,
    pub token_limit: Pubkey,
}
impl From<SetOndoUserLimitsAccounts<'_, '_>> for SetOndoUserLimitsKeys {
    fn from(accounts: SetOndoUserLimitsAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            mint: *accounts.mint.key,
            authority_role_account: *accounts.authority_role_account.key,
            ondo_user: *accounts.ondo_user.key,
            token_limit: *accounts.token_limit.key,
        }
    }
}
impl From<SetOndoUserLimitsKeys>
for [AccountMeta; SET_ONDO_USER_LIMITS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetOndoUserLimitsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ondo_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_limit,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_ONDO_USER_LIMITS_IX_ACCOUNTS_LEN]> for SetOndoUserLimitsKeys {
    fn from(pubkeys: [Pubkey; SET_ONDO_USER_LIMITS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            mint: pubkeys[1],
            authority_role_account: pubkeys[2],
            ondo_user: pubkeys[3],
            token_limit: pubkeys[4],
        }
    }
}
impl<'info> From<SetOndoUserLimitsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_ONDO_USER_LIMITS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetOndoUserLimitsAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.mint.clone(),
            accounts.authority_role_account.clone(),
            accounts.ondo_user.clone(),
            accounts.token_limit.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_ONDO_USER_LIMITS_IX_ACCOUNTS_LEN]>
for SetOndoUserLimitsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_ONDO_USER_LIMITS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            mint: &arr[1],
            authority_role_account: &arr[2],
            ondo_user: &arr[3],
            token_limit: &arr[4],
        }
    }
}
pub const SET_ONDO_USER_LIMITS_IX_DISCM: [u8; 8usize] = [
    86, 133, 101, 215, 245, 143, 28, 159,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetOndoUserLimitsIxArgs {
    pub rate_limit: u64,
    pub limit_window: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetOndoUserLimitsIxData(pub SetOndoUserLimitsIxArgs);
impl From<SetOndoUserLimitsIxArgs> for SetOndoUserLimitsIxData {
    fn from(args: SetOndoUserLimitsIxArgs) -> Self {
        Self(args)
    }
}
impl SetOndoUserLimitsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_ONDO_USER_LIMITS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let rate_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let limit_window: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetOndoUserLimitsIxArgs {
                rate_limit,
                limit_window,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_ONDO_USER_LIMITS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.rate_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.limit_window, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_ondo_user_limits_ix_with_program_id(
    program_id: Pubkey,
    keys: SetOndoUserLimitsKeys,
    args: SetOndoUserLimitsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_ONDO_USER_LIMITS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetOndoUserLimitsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_ondo_user_limits_ix(
    keys: SetOndoUserLimitsKeys,
    args: SetOndoUserLimitsIxArgs,
) -> std::io::Result<Instruction> {
    set_ondo_user_limits_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn set_ondo_user_limits_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetOndoUserLimitsAccounts<'_, '_>,
    args: SetOndoUserLimitsIxArgs,
) -> ProgramResult {
    let keys: SetOndoUserLimitsKeys = accounts.into();
    let ix = set_ondo_user_limits_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_ondo_user_limits_invoke(
    accounts: SetOndoUserLimitsAccounts<'_, '_>,
    args: SetOndoUserLimitsIxArgs,
) -> ProgramResult {
    set_ondo_user_limits_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn set_ondo_user_limits_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetOndoUserLimitsAccounts<'_, '_>,
    args: SetOndoUserLimitsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetOndoUserLimitsKeys = accounts.into();
    let ix = set_ondo_user_limits_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_ondo_user_limits_invoke_signed(
    accounts: SetOndoUserLimitsAccounts<'_, '_>,
    args: SetOndoUserLimitsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_ondo_user_limits_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_ondo_user_limits_verify_account_keys(
    accounts: SetOndoUserLimitsAccounts<'_, '_>,
    keys: SetOndoUserLimitsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.mint.key, keys.mint),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.ondo_user.key, keys.ondo_user),
        (*accounts.token_limit.key, keys.token_limit),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_ondo_user_limits_verify_writable_privileges<'me, 'info>(
    accounts: SetOndoUserLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ondo_user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_ondo_user_limits_verify_signer_privileges<'me, 'info>(
    accounts: SetOndoUserLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_ondo_user_limits_verify_account_privileges<'me, 'info>(
    accounts: SetOndoUserLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_ondo_user_limits_verify_writable_privileges(accounts)?;
    set_ondo_user_limits_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_ORACLE_PRICE_MAX_AGE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetOraclePriceMaxAgeAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetOraclePriceMaxAgeKeys {
    pub authority: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub authority_role_account: Pubkey,
}
impl From<SetOraclePriceMaxAgeAccounts<'_, '_>> for SetOraclePriceMaxAgeKeys {
    fn from(accounts: SetOraclePriceMaxAgeAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            authority_role_account: *accounts.authority_role_account.key,
        }
    }
}
impl From<SetOraclePriceMaxAgeKeys>
for [AccountMeta; SET_ORACLE_PRICE_MAX_AGE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetOraclePriceMaxAgeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_ORACLE_PRICE_MAX_AGE_IX_ACCOUNTS_LEN]>
for SetOraclePriceMaxAgeKeys {
    fn from(pubkeys: [Pubkey; SET_ORACLE_PRICE_MAX_AGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            usdon_manager_state: pubkeys[1],
            authority_role_account: pubkeys[2],
        }
    }
}
impl<'info> From<SetOraclePriceMaxAgeAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_ORACLE_PRICE_MAX_AGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetOraclePriceMaxAgeAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.authority_role_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_ORACLE_PRICE_MAX_AGE_IX_ACCOUNTS_LEN]>
for SetOraclePriceMaxAgeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_ORACLE_PRICE_MAX_AGE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            usdon_manager_state: &arr[1],
            authority_role_account: &arr[2],
        }
    }
}
pub const SET_ORACLE_PRICE_MAX_AGE_IX_DISCM: [u8; 8usize] = [
    198, 173, 23, 10, 148, 48, 248, 61,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetOraclePriceMaxAgeIxArgs {
    pub oracle_price_max_age: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetOraclePriceMaxAgeIxData(pub SetOraclePriceMaxAgeIxArgs);
impl From<SetOraclePriceMaxAgeIxArgs> for SetOraclePriceMaxAgeIxData {
    fn from(args: SetOraclePriceMaxAgeIxArgs) -> Self {
        Self(args)
    }
}
impl SetOraclePriceMaxAgeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_ORACLE_PRICE_MAX_AGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let oracle_price_max_age: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetOraclePriceMaxAgeIxArgs {
                oracle_price_max_age,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_ORACLE_PRICE_MAX_AGE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_price_max_age, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_oracle_price_max_age_ix_with_program_id(
    program_id: Pubkey,
    keys: SetOraclePriceMaxAgeKeys,
    args: SetOraclePriceMaxAgeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_ORACLE_PRICE_MAX_AGE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetOraclePriceMaxAgeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_oracle_price_max_age_ix(
    keys: SetOraclePriceMaxAgeKeys,
    args: SetOraclePriceMaxAgeIxArgs,
) -> std::io::Result<Instruction> {
    set_oracle_price_max_age_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn set_oracle_price_max_age_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetOraclePriceMaxAgeAccounts<'_, '_>,
    args: SetOraclePriceMaxAgeIxArgs,
) -> ProgramResult {
    let keys: SetOraclePriceMaxAgeKeys = accounts.into();
    let ix = set_oracle_price_max_age_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_oracle_price_max_age_invoke(
    accounts: SetOraclePriceMaxAgeAccounts<'_, '_>,
    args: SetOraclePriceMaxAgeIxArgs,
) -> ProgramResult {
    set_oracle_price_max_age_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn set_oracle_price_max_age_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetOraclePriceMaxAgeAccounts<'_, '_>,
    args: SetOraclePriceMaxAgeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetOraclePriceMaxAgeKeys = accounts.into();
    let ix = set_oracle_price_max_age_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_oracle_price_max_age_invoke_signed(
    accounts: SetOraclePriceMaxAgeAccounts<'_, '_>,
    args: SetOraclePriceMaxAgeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_oracle_price_max_age_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_oracle_price_max_age_verify_account_keys(
    accounts: SetOraclePriceMaxAgeAccounts<'_, '_>,
    keys: SetOraclePriceMaxAgeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
        (*accounts.authority_role_account.key, keys.authority_role_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_oracle_price_max_age_verify_writable_privileges<'me, 'info>(
    accounts: SetOraclePriceMaxAgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.usdon_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_oracle_price_max_age_verify_signer_privileges<'me, 'info>(
    accounts: SetOraclePriceMaxAgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_oracle_price_max_age_verify_account_privileges<'me, 'info>(
    accounts: SetOraclePriceMaxAgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_oracle_price_max_age_verify_writable_privileges(accounts)?;
    set_oracle_price_max_age_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_TOKEN_LIMIT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetTokenLimitAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_limit: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetTokenLimitKeys {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub token_limit: Pubkey,
    pub authority_role_account: Pubkey,
}
impl From<SetTokenLimitAccounts<'_, '_>> for SetTokenLimitKeys {
    fn from(accounts: SetTokenLimitAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            mint: *accounts.mint.key,
            token_limit: *accounts.token_limit.key,
            authority_role_account: *accounts.authority_role_account.key,
        }
    }
}
impl From<SetTokenLimitKeys> for [AccountMeta; SET_TOKEN_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(keys: SetTokenLimitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_limit,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_TOKEN_LIMIT_IX_ACCOUNTS_LEN]> for SetTokenLimitKeys {
    fn from(pubkeys: [Pubkey; SET_TOKEN_LIMIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            mint: pubkeys[1],
            token_limit: pubkeys[2],
            authority_role_account: pubkeys[3],
        }
    }
}
impl<'info> From<SetTokenLimitAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_TOKEN_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetTokenLimitAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.mint.clone(),
            accounts.token_limit.clone(),
            accounts.authority_role_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_TOKEN_LIMIT_IX_ACCOUNTS_LEN]>
for SetTokenLimitAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_TOKEN_LIMIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            mint: &arr[1],
            token_limit: &arr[2],
            authority_role_account: &arr[3],
        }
    }
}
pub const SET_TOKEN_LIMIT_IX_DISCM: [u8; 8usize] = [216, 80, 182, 245, 223, 87, 77, 53];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetTokenLimitIxArgs {
    pub rate_limit: Option<u64>,
    pub limit_window: Option<u64>,
    pub default_user_rate_limit: Option<u64>,
    pub default_user_limit_window: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetTokenLimitIxData(pub SetTokenLimitIxArgs);
impl From<SetTokenLimitIxArgs> for SetTokenLimitIxData {
    fn from(args: SetTokenLimitIxArgs) -> Self {
        Self(args)
    }
}
impl SetTokenLimitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_TOKEN_LIMIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let rate_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let limit_window: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let default_user_rate_limit: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let default_user_limit_window: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SetTokenLimitIxArgs {
                rate_limit,
                limit_window,
                default_user_rate_limit,
                default_user_limit_window,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_TOKEN_LIMIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.rate_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.limit_window, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.default_user_rate_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.default_user_limit_window,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_token_limit_ix_with_program_id(
    program_id: Pubkey,
    keys: SetTokenLimitKeys,
    args: SetTokenLimitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_TOKEN_LIMIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetTokenLimitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_token_limit_ix(
    keys: SetTokenLimitKeys,
    args: SetTokenLimitIxArgs,
) -> std::io::Result<Instruction> {
    set_token_limit_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn set_token_limit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetTokenLimitAccounts<'_, '_>,
    args: SetTokenLimitIxArgs,
) -> ProgramResult {
    let keys: SetTokenLimitKeys = accounts.into();
    let ix = set_token_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_token_limit_invoke(
    accounts: SetTokenLimitAccounts<'_, '_>,
    args: SetTokenLimitIxArgs,
) -> ProgramResult {
    set_token_limit_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn set_token_limit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetTokenLimitAccounts<'_, '_>,
    args: SetTokenLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetTokenLimitKeys = accounts.into();
    let ix = set_token_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_token_limit_invoke_signed(
    accounts: SetTokenLimitAccounts<'_, '_>,
    args: SetTokenLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_token_limit_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_token_limit_verify_account_keys(
    accounts: SetTokenLimitAccounts<'_, '_>,
    keys: SetTokenLimitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_limit.key, keys.token_limit),
        (*accounts.authority_role_account.key, keys.authority_role_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_token_limit_verify_writable_privileges<'me, 'info>(
    accounts: SetTokenLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.token_limit] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_token_limit_verify_signer_privileges<'me, 'info>(
    accounts: SetTokenLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_token_limit_verify_account_privileges<'me, 'info>(
    accounts: SetTokenLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_token_limit_verify_writable_privileges(accounts)?;
    set_token_limit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_USDC_PRICE_UPDATE_ADDRESS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetUsdcPriceUpdateAddressAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetUsdcPriceUpdateAddressKeys {
    pub authority: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub authority_role_account: Pubkey,
}
impl From<SetUsdcPriceUpdateAddressAccounts<'_, '_>> for SetUsdcPriceUpdateAddressKeys {
    fn from(accounts: SetUsdcPriceUpdateAddressAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            authority_role_account: *accounts.authority_role_account.key,
        }
    }
}
impl From<SetUsdcPriceUpdateAddressKeys>
for [AccountMeta; SET_USDC_PRICE_UPDATE_ADDRESS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetUsdcPriceUpdateAddressKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_USDC_PRICE_UPDATE_ADDRESS_IX_ACCOUNTS_LEN]>
for SetUsdcPriceUpdateAddressKeys {
    fn from(pubkeys: [Pubkey; SET_USDC_PRICE_UPDATE_ADDRESS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            usdon_manager_state: pubkeys[1],
            authority_role_account: pubkeys[2],
        }
    }
}
impl<'info> From<SetUsdcPriceUpdateAddressAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_USDC_PRICE_UPDATE_ADDRESS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetUsdcPriceUpdateAddressAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.authority_role_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_USDC_PRICE_UPDATE_ADDRESS_IX_ACCOUNTS_LEN]>
for SetUsdcPriceUpdateAddressAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_USDC_PRICE_UPDATE_ADDRESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            usdon_manager_state: &arr[1],
            authority_role_account: &arr[2],
        }
    }
}
pub const SET_USDC_PRICE_UPDATE_ADDRESS_IX_DISCM: [u8; 8usize] = [
    12, 142, 252, 201, 224, 148, 254, 32,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetUsdcPriceUpdateAddressIxArgs {
    pub new_price_update_address: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetUsdcPriceUpdateAddressIxData(pub SetUsdcPriceUpdateAddressIxArgs);
impl From<SetUsdcPriceUpdateAddressIxArgs> for SetUsdcPriceUpdateAddressIxData {
    fn from(args: SetUsdcPriceUpdateAddressIxArgs) -> Self {
        Self(args)
    }
}
impl SetUsdcPriceUpdateAddressIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_USDC_PRICE_UPDATE_ADDRESS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_price_update_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetUsdcPriceUpdateAddressIxArgs {
                new_price_update_address,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_USDC_PRICE_UPDATE_ADDRESS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_price_update_address, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_usdc_price_update_address_ix_with_program_id(
    program_id: Pubkey,
    keys: SetUsdcPriceUpdateAddressKeys,
    args: SetUsdcPriceUpdateAddressIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_USDC_PRICE_UPDATE_ADDRESS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetUsdcPriceUpdateAddressIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_usdc_price_update_address_ix(
    keys: SetUsdcPriceUpdateAddressKeys,
    args: SetUsdcPriceUpdateAddressIxArgs,
) -> std::io::Result<Instruction> {
    set_usdc_price_update_address_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn set_usdc_price_update_address_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetUsdcPriceUpdateAddressAccounts<'_, '_>,
    args: SetUsdcPriceUpdateAddressIxArgs,
) -> ProgramResult {
    let keys: SetUsdcPriceUpdateAddressKeys = accounts.into();
    let ix = set_usdc_price_update_address_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_usdc_price_update_address_invoke(
    accounts: SetUsdcPriceUpdateAddressAccounts<'_, '_>,
    args: SetUsdcPriceUpdateAddressIxArgs,
) -> ProgramResult {
    set_usdc_price_update_address_invoke_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_usdc_price_update_address_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetUsdcPriceUpdateAddressAccounts<'_, '_>,
    args: SetUsdcPriceUpdateAddressIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetUsdcPriceUpdateAddressKeys = accounts.into();
    let ix = set_usdc_price_update_address_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_usdc_price_update_address_invoke_signed(
    accounts: SetUsdcPriceUpdateAddressAccounts<'_, '_>,
    args: SetUsdcPriceUpdateAddressIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_usdc_price_update_address_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_usdc_price_update_address_verify_account_keys(
    accounts: SetUsdcPriceUpdateAddressAccounts<'_, '_>,
    keys: SetUsdcPriceUpdateAddressKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
        (*accounts.authority_role_account.key, keys.authority_role_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_usdc_price_update_address_verify_writable_privileges<'me, 'info>(
    accounts: SetUsdcPriceUpdateAddressAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.usdon_manager_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_usdc_price_update_address_verify_signer_privileges<'me, 'info>(
    accounts: SetUsdcPriceUpdateAddressAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_usdc_price_update_address_verify_account_privileges<'me, 'info>(
    accounts: SetUsdcPriceUpdateAddressAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_usdc_price_update_address_verify_writable_privileges(accounts)?;
    set_usdc_price_update_address_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_SCALED_UI_MULTIPLIER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateScaledUiMultiplierAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub usdon_manager_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateScaledUiMultiplierKeys {
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub mint_authority: Pubkey,
    pub mint: Pubkey,
    pub usdon_manager_state: Pubkey,
    pub token_program: Pubkey,
}
impl From<UpdateScaledUiMultiplierAccounts<'_, '_>> for UpdateScaledUiMultiplierKeys {
    fn from(accounts: UpdateScaledUiMultiplierAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            mint_authority: *accounts.mint_authority.key,
            mint: *accounts.mint.key,
            usdon_manager_state: *accounts.usdon_manager_state.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<UpdateScaledUiMultiplierKeys>
for [AccountMeta; UPDATE_SCALED_UI_MULTIPLIER_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateScaledUiMultiplierKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdon_manager_state,
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
impl From<[Pubkey; UPDATE_SCALED_UI_MULTIPLIER_IX_ACCOUNTS_LEN]>
for UpdateScaledUiMultiplierKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SCALED_UI_MULTIPLIER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            authority_role_account: pubkeys[1],
            mint_authority: pubkeys[2],
            mint: pubkeys[3],
            usdon_manager_state: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateScaledUiMultiplierAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_SCALED_UI_MULTIPLIER_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateScaledUiMultiplierAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.mint_authority.clone(),
            accounts.mint.clone(),
            accounts.usdon_manager_state.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_SCALED_UI_MULTIPLIER_IX_ACCOUNTS_LEN]>
for UpdateScaledUiMultiplierAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SCALED_UI_MULTIPLIER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            authority_role_account: &arr[1],
            mint_authority: &arr[2],
            mint: &arr[3],
            usdon_manager_state: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const UPDATE_SCALED_UI_MULTIPLIER_IX_DISCM: [u8; 8usize] = [
    96, 212, 189, 168, 220, 27, 240, 70,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateScaledUiMultiplierIxArgs {
    pub new_multiplier: f64,
    pub timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateScaledUiMultiplierIxData(pub UpdateScaledUiMultiplierIxArgs);
impl From<UpdateScaledUiMultiplierIxArgs> for UpdateScaledUiMultiplierIxData {
    fn from(args: UpdateScaledUiMultiplierIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateScaledUiMultiplierIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SCALED_UI_MULTIPLIER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_multiplier: f64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateScaledUiMultiplierIxArgs {
                new_multiplier,
                timestamp,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SCALED_UI_MULTIPLIER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_multiplier, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.timestamp, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_scaled_ui_multiplier_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateScaledUiMultiplierKeys,
    args: UpdateScaledUiMultiplierIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_SCALED_UI_MULTIPLIER_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateScaledUiMultiplierIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_scaled_ui_multiplier_ix(
    keys: UpdateScaledUiMultiplierKeys,
    args: UpdateScaledUiMultiplierIxArgs,
) -> std::io::Result<Instruction> {
    update_scaled_ui_multiplier_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn update_scaled_ui_multiplier_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateScaledUiMultiplierAccounts<'_, '_>,
    args: UpdateScaledUiMultiplierIxArgs,
) -> ProgramResult {
    let keys: UpdateScaledUiMultiplierKeys = accounts.into();
    let ix = update_scaled_ui_multiplier_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_scaled_ui_multiplier_invoke(
    accounts: UpdateScaledUiMultiplierAccounts<'_, '_>,
    args: UpdateScaledUiMultiplierIxArgs,
) -> ProgramResult {
    update_scaled_ui_multiplier_invoke_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_scaled_ui_multiplier_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateScaledUiMultiplierAccounts<'_, '_>,
    args: UpdateScaledUiMultiplierIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateScaledUiMultiplierKeys = accounts.into();
    let ix = update_scaled_ui_multiplier_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_scaled_ui_multiplier_invoke_signed(
    accounts: UpdateScaledUiMultiplierAccounts<'_, '_>,
    args: UpdateScaledUiMultiplierIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_scaled_ui_multiplier_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_scaled_ui_multiplier_verify_account_keys(
    accounts: UpdateScaledUiMultiplierAccounts<'_, '_>,
    keys: UpdateScaledUiMultiplierKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.mint.key, keys.mint),
        (*accounts.usdon_manager_state.key, keys.usdon_manager_state),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_scaled_ui_multiplier_verify_writable_privileges<'me, 'info>(
    accounts: UpdateScaledUiMultiplierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_scaled_ui_multiplier_verify_signer_privileges<'me, 'info>(
    accounts: UpdateScaledUiMultiplierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_scaled_ui_multiplier_verify_account_privileges<'me, 'info>(
    accounts: UpdateScaledUiMultiplierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_scaled_ui_multiplier_verify_writable_privileges(accounts)?;
    update_scaled_ui_multiplier_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct UpdateTokenMetadataAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub authority_role_account: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateTokenMetadataKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub authority_role_account: Pubkey,
    pub mint_authority: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateTokenMetadataAccounts<'_, '_>> for UpdateTokenMetadataKeys {
    fn from(accounts: UpdateTokenMetadataAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            authority_role_account: *accounts.authority_role_account.key,
            mint_authority: *accounts.mint_authority.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateTokenMetadataKeys>
for [AccountMeta; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateTokenMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_role_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
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
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]> for UpdateTokenMetadataKeys {
    fn from(pubkeys: [Pubkey; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            authority_role_account: pubkeys[2],
            mint_authority: pubkeys[3],
            mint: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<UpdateTokenMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateTokenMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.authority_role_account.clone(),
            accounts.mint_authority.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]>
for UpdateTokenMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            authority_role_account: &arr[2],
            mint_authority: &arr[3],
            mint: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const UPDATE_TOKEN_METADATA_IX_DISCM: [u8; 8usize] = [
    243, 6, 8, 23, 126, 181, 251, 158,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateTokenMetadataIxArgs {
    pub new_name: Option<String>,
    pub new_symbol: Option<String>,
    pub new_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTokenMetadataIxData(pub UpdateTokenMetadataIxArgs);
impl From<UpdateTokenMetadataIxArgs> for UpdateTokenMetadataIxData {
    fn from(args: UpdateTokenMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateTokenMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_TOKEN_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_name: Option<String> = crate::borsh_de_or_default(&mut reader)?;
        let new_symbol: Option<String> = crate::borsh_de_or_default(&mut reader)?;
        let new_uri: Option<String> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateTokenMetadataIxArgs {
                new_name,
                new_symbol,
                new_uri,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_TOKEN_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_uri, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_token_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateTokenMetadataKeys,
    args: UpdateTokenMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateTokenMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_token_metadata_ix(
    keys: UpdateTokenMetadataKeys,
    args: UpdateTokenMetadataIxArgs,
) -> std::io::Result<Instruction> {
    update_token_metadata_ix_with_program_id(ONDO_GM_PROGRAM_ID, keys, args)
}
pub fn update_token_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    args: UpdateTokenMetadataIxArgs,
) -> ProgramResult {
    let keys: UpdateTokenMetadataKeys = accounts.into();
    let ix = update_token_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_token_metadata_invoke(
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    args: UpdateTokenMetadataIxArgs,
) -> ProgramResult {
    update_token_metadata_invoke_with_program_id(ONDO_GM_PROGRAM_ID, accounts, args)
}
pub fn update_token_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    args: UpdateTokenMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateTokenMetadataKeys = accounts.into();
    let ix = update_token_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_token_metadata_invoke_signed(
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    args: UpdateTokenMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_token_metadata_invoke_signed_with_program_id(
        ONDO_GM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_token_metadata_verify_account_keys(
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    keys: UpdateTokenMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.authority_role_account.key, keys.authority_role_account),
        (*accounts.mint_authority.key, keys.mint_authority),
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
pub fn update_token_metadata_verify_writable_privileges<'me, 'info>(
    accounts: UpdateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_token_metadata_verify_signer_privileges<'me, 'info>(
    accounts: UpdateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_token_metadata_verify_account_privileges<'me, 'info>(
    accounts: UpdateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_token_metadata_verify_writable_privileges(accounts)?;
    update_token_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
