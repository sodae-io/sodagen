use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{format_ident, quote};
use std::{io::Write, path::Path};

use crate::{idl_format::IdlFormat, utils::open_file_create_overwrite, Args};

const DEFAULT_PROGRAM_ID_STR: &str = "TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111";

const MAX_BASE58_LEN: usize = 44;
const PUBKEY_BYTES_SIZE: usize = 32;

/// Copied from solana_program::Pubkey::from_str()
/// so that we dont have to have solana_program as a dep
fn is_valid_pubkey(s: &str) -> bool {
    if s.len() > MAX_BASE58_LEN {
        return false;
    }
    let pubkey_vec = match bs58::decode(s).into_vec() {
        Ok(v) => v,
        Err(_) => return false,
    };
    if pubkey_vec.len() != PUBKEY_BYTES_SIZE {
        return false;
    }
    true
}

pub fn write_lib(args: &Args, idl: &dyn IdlFormat) -> std::io::Result<()> {
    let user_provided_id_opt =
        args.program_id
            .as_ref()
            .and_then(|s| if is_valid_pubkey(s) { Some(s) } else { None });
    let id = user_provided_id_opt
        .map(|string| string.as_ref())
        .unwrap_or_else(|| {
            idl.program_address().unwrap_or_else(|| {
                log::warn!(
                    "program address not in IDL, setting to default: {}",
                    DEFAULT_PROGRAM_ID_STR
                );
                DEFAULT_PROGRAM_ID_STR
            })
        });

    let program_id_ident = format_ident!(
        "{}_PROGRAM_ID",
        idl.program_name().to_shouty_snake_case()
    );
    let mut contents = quote! {
        solana_pubkey::declare_id!(#id);
        pub const #program_id_ident: solana_pubkey::Pubkey = ID;
        pub type ProgramResult = Result<(), solana_program_error::ProgramError>;
        pub(crate) mod big_array_serde;

        /// Tolerant field read: returns `T::default()` when the slice is already
        /// exhausted, so payloads encoded before a trailing field was added still
        /// decode. Only defaults at a true field boundary (slice empty), so a
        /// field present-but-truncated still errors via the inner read.
        #[doc(hidden)]
        #[allow(dead_code)]
        #[inline]
        pub(crate) fn borsh_de_or_default<T: borsh::BorshDeserialize + Default>(
            reader: &mut &[u8],
        ) -> std::io::Result<T> {
            if reader.is_empty() {
                Ok(T::default())
            } else {
                borsh::BorshDeserialize::deserialize_reader(reader)
            }
        }
    };

    // Write big_array_serde.rs as a raw file (bypasses quote/syn since it uses const generics)
    write_big_array_serde_file(args)?;

    for module in idl.modules(args) {
        let module_name = module.name();
        let module_ident = Ident::new(module.name(), Span::call_site());
        contents.extend(quote! {
            pub mod #module_ident;
            pub use #module_ident::*;
        });
        let mut module_contents = module.gen_head();
        module_contents.extend(module.gen_body());
        write_src_file(args, &format!("src/{module_name}.rs"), module_contents)?;
    }

    write_src_file(args, "src/lib.rs", contents)
}

fn write_big_array_serde_file(args: &Args) -> std::io::Result<()> {
    let content = r#"#![allow(unused)]
use std::fmt;
use std::marker::PhantomData;
use serde::ser::{Serialize, Serializer, SerializeTuple};
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};

pub fn serialize<S, T, const N: usize>(array: &[T; N], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let mut seq = serializer.serialize_tuple(N)?;
    for item in array.iter() {
        seq.serialize_element(item)?;
    }
    seq.end()
}

pub fn deserialize<'de, D, T, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct ArrayVisitor<T, const N: usize>(PhantomData<T>);

    impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
    where
        T: Deserialize<'de>,
    {
        type Value = [T; N];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an array of length {}", N)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<[T; N], A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = Vec::with_capacity(N);
            for i in 0..N {
                vec.push(
                    seq.next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?,
                );
            }
            vec.try_into().map_err(|_| serde::de::Error::custom("wrong array length"))
        }
    }

    deserializer.deserialize_tuple(N, ArrayVisitor(PhantomData))
}
"#;
    let path = args.output_dir.join("src/big_array_serde.rs");
    let mut file = open_file_create_overwrite(path)?;
    file.write_all(content.as_bytes())?;
    file.flush()
}

fn write_src_file<P: AsRef<Path>>(
    args: &Args,
    src_file_path: P,
    contents: TokenStream,
) -> std::io::Result<()> {
    let sanitized_contents = sanitize_tokens(contents);

    let unpretty = syn::parse2(sanitized_contents).unwrap();
    let formatted = prettyplease::unparse(&unpretty);

    let path = args.output_dir.join(src_file_path);
    let mut file = open_file_create_overwrite(path)?;
    file.write_all(formatted.as_bytes())?;
    file.flush()
}


fn sanitize_tokens(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut result = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        match token {
            TokenTree::Group(group) => {
                let content = sanitize_tokens(group.stream());
                result.push(TokenTree::Group(proc_macro2::Group::new(group.delimiter(), content)));
            },
            TokenTree::Ident(ident) if ident == "type" => {
                // `type` is the Rust keyword when followed by an Ident (e.g. `pub type Foo = ...`).
                // In all other cases (followed by `:`, `=`, `,`, `}`, etc.), it's a
                // field/variable name and must be escaped to `r#type`.
                let next_is_ident = tokens.get(i + 1).map_or(false, |t| {
                    matches!(t, TokenTree::Ident(_))
                });
                if next_is_ident {
                    result.push(token.clone());
                } else {
                    let raw_type = quote! { r#type };
                    result.push(raw_type.into_iter().next().unwrap());
                }
            },
            _ => result.push(token.clone()),
        }
    }
    result.into_iter().collect()
}