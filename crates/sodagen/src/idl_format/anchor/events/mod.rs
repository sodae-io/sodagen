use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::idl_format::IdlCodegenModule;

mod event;
pub use event::*;

mod event_v1;
pub use event_v1::*;

pub struct EventsCodegenModule<'a> {
    pub events: &'a [Event],
    pub no_default_types: HashSet<String>,
    pub struct_types: HashSet<String>,
}

impl IdlCodegenModule for EventsCodegenModule<'_> {
    fn name(&self) -> &str {
        "events"
    }

    fn gen_head(&self) -> TokenStream {
        quote! {
            use solana_pubkey::Pubkey;
            #[allow(unused_imports)] use crate::*;
        }
    }

    fn gen_body(&self) -> TokenStream {
        self.events
            .iter()
            .map(|e| e.gen_with_no_default(&self.no_default_types, &self.struct_types))
            .collect()
    }
}

pub struct EventsV1CodegenModule<'a> {
    pub events: &'a [EventV1],
    pub no_default_types: HashSet<String>,
    pub struct_types: HashSet<String>,
}

impl IdlCodegenModule for EventsV1CodegenModule<'_> {
    fn name(&self) -> &str {
        "events"
    }

    fn gen_head(&self) -> TokenStream {
        quote! {
            use solana_pubkey::Pubkey;
            #[allow(unused_imports)] use crate::*;
        }
    }

    fn gen_body(&self) -> TokenStream {
        self.events
            .iter()
            .map(|e| e.gen_with_no_default(&self.no_default_types, &self.struct_types))
            .collect()
    }
}
