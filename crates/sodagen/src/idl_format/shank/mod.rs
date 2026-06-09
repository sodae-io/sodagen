use serde::Deserialize;
use toml::{map::Map, Value};

use crate::write_cargotoml::{
    DependencyValue, FeaturesDependencyValue, BORSH_CRATE, BYTEMUCK_CRATE,
    NUM_DERIVE_CRATE, NUM_TRAITS_CRATE, SERDE_CRATE, SOLANA_ACCOUNT_INFO_CRATE,
    SOLANA_CPI_CRATE, SOLANA_INSTRUCTION_CRATE, SOLANA_PROGRAM_ERROR_CRATE,
    SOLANA_PUBKEY_CRATE, THISERROR_CRATE,
};

use super::{IdlCodegenModule, IdlFormat};

use self::{
    accounts::AccountsCodegenModule,
    errors::{ErrorEnumVariant, ErrorsCodegenModule},
    instructions::{IxCodegenModule, NamedInstruction},
    typedefs::{compute_no_default_types, NamedType, TypedefType, TypedefsCodegenModule},
};

pub mod accounts;
pub mod errors;
pub mod instructions;
pub mod typedefs;

#[derive(Deserialize)]
pub struct ShankIdl {
    pub name: String,
    pub version: String,
    pub metadata: Metadata,
    pub accounts: Option<Vec<NamedType>>,
    pub types: Option<Vec<NamedType>>,
    pub instructions: Option<Vec<NamedInstruction>>,
    pub errors: Option<Vec<ErrorEnumVariant>>,
}

#[derive(Deserialize)]
pub struct Metadata {
    pub address: String,
    pub origin: String,
}

impl IdlFormat for ShankIdl {
    fn program_name(&self) -> &str {
        &self.name
    }

    fn program_version(&self) -> &str {
        &self.version
    }

    fn program_address(&self) -> Option<&str> {
        Some(&self.metadata.address)
    }

    fn is_correct_idl_format(&self) -> bool {
        self.metadata.origin == "shank"
    }

    fn modules<'me>(&'me self, args: &'me crate::Args) -> Vec<Box<dyn IdlCodegenModule + 'me>> {
        // Compute no-default + struct-type sets over ALL shank types (accounts +
        // typedefs) so tolerant deserialization and nested-struct dispatch are
        // correct across module boundaries.
        let all_types: Vec<&NamedType> = self
            .accounts
            .as_deref()
            .unwrap_or_default()
            .iter()
            .chain(self.r#types.as_deref().unwrap_or_default().iter())
            .collect();
        let no_default = compute_no_default_types(&all_types);
        let struct_types: std::collections::HashSet<String> = all_types
            .iter()
            .filter(|t| matches!(t.r#type, TypedefType::r#struct(_)))
            .map(|t| t.name.clone())
            .collect();

        let mut res: Vec<Box<dyn IdlCodegenModule + 'me>> = Vec::new();
        if let Some(v) = &self.accounts {
            res.push(Box::new(AccountsCodegenModule {
                cli_args: args,
                named_types: v,
                no_default_types: no_default.clone(),
                struct_types: struct_types.clone(),
            }));
        }
        if let Some(v) = &self.r#types {
            res.push(Box::new(TypedefsCodegenModule {
                cli_args: args,
                named_types: v,
                no_default_types: no_default.clone(),
                struct_types: struct_types.clone(),
            }));
        }
        if let Some(v) = &self.instructions {
            res.push(Box::new(IxCodegenModule {
                program_name: self.program_name(),
                instructions: v,
                no_default_types: no_default.clone(),
                struct_types: struct_types.clone(),
            }));
        }
        if let Some(v) = &self.errors {
            res.push(Box::new(ErrorsCodegenModule {
                program_name: self.program_name(),
                variants: v,
            }));
        }
        res
    }

    fn dependencies(&self, args: &crate::Args) -> Map<String, Value> {
        let mut map = Map::new();
        map.insert(
            BORSH_CRATE.into(),
            FeaturesDependencyValue {
                dependency: DependencyValue(&args.borsh_vers),
                features: vec!["derive".into()],
            }
            .into(),
        );
        if !args.zero_copy.is_empty() {
            map.insert(
                BYTEMUCK_CRATE.into(),
                FeaturesDependencyValue {
                    dependency: DependencyValue(&args.bytemuck_vers),
                    features: vec!["derive".into()],
                }
                .into(),
            );
        }
        map.insert(
            SOLANA_PUBKEY_CRATE.into(),
            FeaturesDependencyValue {
                dependency: DependencyValue(&args.solana_vers),
                features: vec!["borsh".into(), "serde".into()],
            }
            .into(),
        );
        map.insert(
            SOLANA_PROGRAM_ERROR_CRATE.into(),
            FeaturesDependencyValue {
                dependency: DependencyValue(&args.solana_vers),
                features: vec!["borsh".into()],
            }
            .into(),
        );
        map.insert(
            SOLANA_INSTRUCTION_CRATE.into(),
            DependencyValue(&args.solana_vers).into(),
        );
        map.insert(
            SOLANA_CPI_CRATE.into(),
            DependencyValue(&args.solana_vers).into(),
        );
        map.insert(
            SOLANA_ACCOUNT_INFO_CRATE.into(),
            DependencyValue(&args.solana_vers).into(),
        );
        map.insert(
            SERDE_CRATE.into(),
            FeaturesDependencyValue {
                dependency: DependencyValue(&args.serde_vers),
                features: vec!["derive".into()],
            }
            .into(),
        );
        if self.errors.is_some() {
            map.insert(
                THISERROR_CRATE.into(),
                DependencyValue(&args.thiserror_vers).into(),
            );
            map.insert(
                NUM_DERIVE_CRATE.into(),
                DependencyValue(&args.num_derive_vers).into(),
            );
            map.insert(
                NUM_TRAITS_CRATE.into(),
                DependencyValue(&args.num_traits_vers).into(),
            );
        }
        map
    }
}
