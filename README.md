# sodagen

Solana IDL → Rust client / CPI interface generator.

`sodagen` turns a Solana program IDL into a self-contained Rust crate with typed
instruction builders, account/event/typedef structs, error enums, and CPI
helpers. It auto-detects the IDL format and generates **resilient** decoders that
tolerate schema evolution (programs that add fields over time).

Features tolerant deserialization, allocation-free decode error paths, and a
curated set of public DEX/AMM IDLs.

## Highlights

- **Auto-detected formats:** [Shank](https://github.com/metaplex-foundation/shank),
  [Anchor](https://github.com/coral-xyz/anchor) (legacy and v1), and Bincode
  (system/stake-style programs). Detection order: Shank → Bincode → Anchor V1 → Anchor (legacy).
- **Resilient decoding:** missing *trailing* fields decode to `Default` instead of
  erroring, so historical data encoded before a program added a field still
  decodes. Works at the top level **and through directly-nested structs**, for
  both Anchor and Shank output.
- **Allocation-free hot path:** discriminator mismatches return a non-allocating
  error (no `format!`), so decoders/routers that probe many program types pay
  zero allocations per miss.

## Repository layout

```
.
├── Cargo.toml            # workspace
├── crates/
│   └── sodagen/          # the generator (binary + library)
├── idls/                 # input IDLs, grouped by protocol
├── interfaces/           # generated *_interface crates (output)
└── examples/             # sample IDLs + golden generated crates
```

## Install

```sh
# From this repository
cargo install --path crates/sodagen

# Or build just the binary
cargo build --release --bin sodagen   # binary at target/release/sodagen
```

## Usage

```
sodagen <IDL_PATH> [OPTIONS]
```

**Arguments**
- `<IDL_PATH>` — path to the IDL JSON file

**Options**
- `-o, --output-dir <DIR>` — directory to output the generated crate to (default: `./`)
- `--output-crate-name <NAME>` — output crate name (default: `<program_name>_interface`)
- `-p, --program-id <PUBKEY>` — program ID to use when the IDL embeds none (default: from IDL, else a placeholder)
- `-z, --zero-copy <TYPE>` — typedef/account to additionally derive `bytemuck::Pod` for (repeatable)
- `--solana-vers <VER>` — solana micro-crate dependency version (default: `^3.0`)
- `-b, --borsh-vers <VER>` — borsh dependency version (default: `^1.5`)
- `--thiserror-vers <VER>` — thiserror dependency version (default: `^2.0`)
- `--serde-vers <VER>` — serde dependency version
- `--bytemuck-vers <VER>` — bytemuck dependency version (default: `^1.16`)

**Examples**

```sh
# Generate from an IDL into an output directory
sodagen my_program.json -o ./interfaces/

# Custom crate name + program ID (for IDLs without an embedded address)
sodagen my_program.json --output-crate-name my_sdk -p <PROGRAM_PUBKEY>
```

## Generated crate structure

Each `*_interface` crate contains:

- `lib.rs` — module declarations, re-exports, `declare_id!`, program-ID constant
- `instructions.rs` — instruction builders, account structs, `invoke` helpers
- `accounts.rs` — account types with discriminator-checked (de)serialization
- `typedefs.rs` — shared struct/enum types
- `events.rs` — event types
- `errors.rs` — program error enum

All types derive `serde::Serialize`/`Deserialize`. Accounts, events, instruction
args, and typedef structs use field-by-field (de)serialization rather than the
borsh derive, which is what enables tolerant decoding.

## Features

### Tolerant deserialization

The generated `deserialize(&mut &[u8])` methods read field-by-field over a slice
cursor. A field whose type implements `Default` is read only while bytes remain;
once the buffer is exhausted, that field and all following ones default. This
makes old payloads (encoded before trailing fields were added) decode cleanly.
Defaulting only happens at a true field boundary, so a field that is *present but
truncated* still errors — corruption is not silently masked. Fields stored inside
`Vec`/`Option`/arrays remain strict (mid-collection truncation is unrecoverable).

### Allocation-free mismatch errors

Discriminator checks return `io::ErrorKind::InvalidData` without formatting a
message, so probing many program types for a match costs no allocations per miss.

### Zero-copy / bytemuck

Pass `-z <type-or-account-name>` (repeatable) to additionally derive
`Pod + Zeroable + Copy` for the named generated types.

### `*_with_program_id()`

Instruction helpers that take the program ID as an argument are also exported,
so the same interface can target a program deployed at a different address:

- `*_ix_with_program_id()`
- `*_invoke_with_program_id()`
- `*_invoke_signed_with_program_id()`

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or the
[MIT license](LICENSE-MIT), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
