use std::collections::HashSet;

use serde::Deserialize;
use toml::{map::Map, Value};

use crate::write_cargotoml::{
    DependencyValue, FeaturesDependencyValue, BORSH_CRATE, BYTEMUCK_CRATE,
    NUM_DERIVE_CRATE, NUM_TRAITS_CRATE, SERDE_CRATE, SOLANA_ACCOUNT_INFO_CRATE,
    SOLANA_CPI_CRATE, SOLANA_INSTRUCTION_CRATE, SOLANA_PROGRAM_ERROR_CRATE,
    SOLANA_PUBKEY_CRATE, THISERROR_CRATE,
};

use crate::idl_format::{IdlCodegenModule, IdlFormat};

use super::{
    accounts::{AccountsV1CodegenModule, NamedAccountV1},
    errors::{ErrorEnumVariant, ErrorsCodegenModule},
    events::{EventType, EventV1, EventsV1CodegenModule},
    instructions::{IxAccount, IxAccountEntry, IxCodegenModule, InnerAccountStruct, NamedInstruction},
    typedefs::{compute_no_default_types, NamedType, TypedefsCodegenModule, TypedefField, TypedefType},
};

// --- V1 IDL deserialization types ---

#[derive(Deserialize)]
struct RawAnchorIdlV1 {
    address: String,
    metadata: V1Metadata,
    #[serde(default)]
    instructions: Vec<V1NamedInstruction>,
    #[serde(default)]
    accounts: Vec<V1AccountRef>,
    #[serde(default)]
    events: Vec<V1EventRef>,
    #[serde(default)]
    errors: Vec<ErrorEnumVariant>,
    #[serde(default)]
    types: Vec<NamedType>,
}

/// The converted, ready-to-use V1 IDL struct
pub struct AnchorIdlV1 {
    pub address: String,
    pub metadata: V1Metadata,
    pub errors: Vec<ErrorEnumVariant>,
    converted_instructions: Vec<NamedInstruction>,
    converted_accounts: Vec<NamedAccountV1>,
    converted_events: Vec<EventV1>,
    filtered_types: Vec<NamedType>,
}

#[derive(Deserialize)]
pub struct V1Metadata {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub spec: Option<String>,
}

fn default_version() -> String {
    "0.0.0".to_string()
}

#[derive(Deserialize)]
pub struct V1AccountRef {
    pub name: String,
    pub discriminator: Vec<u8>,
}

#[derive(Deserialize)]
pub struct V1EventRef {
    pub name: String,
    pub discriminator: Vec<u8>,
}

#[derive(Deserialize)]
pub struct V1NamedInstruction {
    pub name: String,
    #[serde(default)]
    pub discriminator: Vec<u8>,
    #[serde(default)]
    pub accounts: Vec<V1IxAccountEntry>,
    #[serde(default)]
    pub args: Vec<TypedefField>,
}

// V1 accounts don't use the nested struct pattern like old format,
// but we handle it for completeness via custom deserialization
// that mirrors the IxAccountEntry approach
impl<'de> Deserialize<'de> for V1IxAccountEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let obj = value
            .as_object()
            .ok_or_else(|| serde::de::Error::custom("expected an object"))?;

        if obj.contains_key("accounts") {
            let name = obj
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| serde::de::Error::missing_field("name"))?
                .to_owned();
            let accounts: Vec<V1IxAccountEntry> =
                serde_json::from_value(obj.get("accounts").cloned().unwrap_or_default())
                    .map_err(serde::de::Error::custom)?;
            Ok(V1IxAccountEntry::Struct(Box::new(V1InnerAccountStruct {
                name,
                accounts,
            })))
        } else {
            let account: V1IxAccount =
                serde_json::from_value(value).map_err(serde::de::Error::custom)?;
            Ok(V1IxAccountEntry::Account(account))
        }
    }
}

pub enum V1IxAccountEntry {
    Account(V1IxAccount),
    Struct(Box<V1InnerAccountStruct>),
}

pub struct V1InnerAccountStruct {
    pub name: String,
    pub accounts: Vec<V1IxAccountEntry>,
}

#[derive(Deserialize)]
pub struct V1IxAccount {
    pub name: String,
    #[serde(default)]
    pub writable: bool,
    #[serde(default)]
    pub signer: bool,
}

/// Try to deserialize as V1 format. Returns None if not V1.
pub fn try_deserialize_v1(file: &std::fs::File) -> Option<AnchorIdlV1> {
    let raw: RawAnchorIdlV1 = match serde_json::from_reader(file) {
        Ok(r) => r,
        Err(e) => {
            log::debug!("anchor_v1 deserialization failed: {:?}", e);
            return None;
        }
    };
    // V1 format must have metadata.spec
    if raw.metadata.spec.is_none() {
        return None;
    }
    Some(convert_raw(raw))
}

fn convert_raw(raw: RawAnchorIdlV1) -> AnchorIdlV1 {
    let account_names: HashSet<&str> = raw.accounts.iter().map(|a| a.name.as_str()).collect();
    let event_names: HashSet<&str> = raw.events.iter().map(|e| e.name.as_str()).collect();

    // Convert instructions
    let converted_instructions: Vec<NamedInstruction> = raw
        .instructions
        .iter()
        .map(|ix| {
            let discriminator = if ix.discriminator.is_empty() {
                None
            } else {
                Some(ix.discriminator.clone())
            };
            let accounts = if ix.accounts.is_empty() {
                None
            } else {
                Some(
                    ix.accounts
                        .iter()
                        .map(convert_v1_ix_account_entry)
                        .collect(),
                )
            };
            let args = if ix.args.is_empty() {
                None
            } else {
                Some(ix.args.clone())
            };
            NamedInstruction {
                name: ix.name.clone(),
                discriminator,
                accounts,
                args,
            }
        })
        .collect();

    // Match account refs with type definitions (clone the matching types)
    let converted_accounts: Vec<NamedAccountV1> = raw
        .accounts
        .iter()
        .filter_map(|acc_ref| {
            let discm = <[u8; 8]>::try_from(acc_ref.discriminator.as_slice()).ok()?;
            let named_type = raw.types.iter().find(|t| t.name == acc_ref.name)?;
            Some(NamedAccountV1 {
                named_type: named_type.clone(),
                discriminator: discm,
            })
        })
        .collect();

    // Match event refs with type definitions
    let converted_events: Vec<EventV1> = raw
        .events
        .iter()
        .filter_map(|evt_ref| {
            let discm = <[u8; 8]>::try_from(evt_ref.discriminator.as_slice()).ok()?;
            let named_type = raw.types.iter().find(|t| t.name == evt_ref.name)?;
            let event_type = event_type_from_named_type(named_type)?;
            Some(EventV1 {
                event_type,
                discriminator: discm,
            })
        })
        .collect();

    // Filter types: exclude account and event types (they get their own modules)
    let filtered_types: Vec<NamedType> = raw
        .types
        .into_iter()
        .filter(|t| {
            !account_names.contains(t.name.as_str()) && !event_names.contains(t.name.as_str())
        })
        .collect();

    AnchorIdlV1 {
        address: raw.address,
        metadata: raw.metadata,
        errors: raw.errors,
        converted_instructions,
        converted_accounts,
        converted_events,
        filtered_types,
    }
}

fn convert_v1_ix_account_entry(entry: &V1IxAccountEntry) -> IxAccountEntry {
    match entry {
        V1IxAccountEntry::Account(a) => IxAccountEntry::Account(IxAccount {
            name: a.name.clone(),
            is_mut: a.writable,
            is_signer: a.signer,
        }),
        V1IxAccountEntry::Struct(s) => IxAccountEntry::Struct(Box::new(
            InnerAccountStruct {
                name: s.name.clone(),
                accounts: s.accounts.iter().map(convert_v1_ix_account_entry).collect(),
            },
        )),
    }
}

/// Extract an EventType from a NamedType (must be a struct)
fn event_type_from_named_type(nt: &NamedType) -> Option<EventType> {
    match &nt.r#type {
        TypedefType::r#struct(s) => Some(EventType {
            name: nt.name.clone(),
            fields: s.fields.clone(),
        }),
        _ => None,
    }
}

impl IdlFormat for AnchorIdlV1 {
    fn program_name(&self) -> &str {
        &self.metadata.name
    }

    fn program_version(&self) -> &str {
        &self.metadata.version
    }

    fn program_address(&self) -> Option<&str> {
        Some(&self.address)
    }

    fn is_correct_idl_format(&self) -> bool {
        self.metadata.spec.is_some()
    }

    fn modules<'me>(&'me self, args: &'me crate::Args) -> Vec<Box<dyn IdlCodegenModule + 'me>> {
        // Collect all types (accounts + typedefs) to compute the no-default set
        let all_types: Vec<NamedType> = self.converted_accounts.iter()
            .map(|a| a.named_type.clone())
            .chain(self.filtered_types.iter().cloned())
            .collect();
        let no_default = compute_no_default_types(&all_types);
        let struct_types: std::collections::HashSet<String> = all_types
            .iter()
            .filter(|t| matches!(t.r#type, TypedefType::r#struct(_)))
            .map(|t| t.name.clone())
            .collect();

        let mut res: Vec<Box<dyn IdlCodegenModule + 'me>> = Vec::new();

        if !self.converted_accounts.is_empty() {
            res.push(Box::new(AccountsV1CodegenModule {
                cli_args: args,
                named_accounts: &self.converted_accounts,
                no_default_types: no_default.clone(),
                struct_types: struct_types.clone(),
            }));
        }

        if !self.filtered_types.is_empty() {
            res.push(Box::new(TypedefsCodegenModule {
                cli_args: args,
                named_types: &self.filtered_types,
                no_default_types: no_default.clone(),
                struct_types: struct_types.clone(),
            }));
        }

        if !self.converted_instructions.is_empty() {
            res.push(Box::new(IxCodegenModule {
                program_name: self.program_name(),
                instructions: &self.converted_instructions,
                no_default_types: no_default.clone(),
                struct_types: struct_types.clone(),
            }));
        }

        if !self.errors.is_empty() {
            res.push(Box::new(ErrorsCodegenModule {
                program_name: self.program_name(),
                variants: &self.errors,
            }));
        }

        if !self.converted_events.is_empty() {
            res.push(Box::new(EventsV1CodegenModule {
                events: &self.converted_events,
                no_default_types: no_default.clone(),
                struct_types: struct_types.clone(),
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
        if !self.errors.is_empty() {
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
