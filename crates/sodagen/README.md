# sodagen

Solana IDL → Rust client / CPI interface generator.

Supports Shank, Anchor (legacy and v1), and Bincode IDL formats. The format is
auto-detected, and generated decoders tolerate trailing-field schema growth.

```sh
# Basic usage
sodagen <path-to-idl.json>

# With output directory
sodagen my_program.json -o ./generated/
```

See the [repository README](../../README.md) for full documentation.
