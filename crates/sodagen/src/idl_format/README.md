# idl_format

The main `IdlFormat` trait defines
- how to deserialize the IDL file
- how to generate rust code from the deserialized struct

Currently each IDL format is completely isolated from each other in its own folder with minimum common code shared between them. This allows for independent evolution of each format at the cost of (a lot of) duplicated code. Might refactor this in the future.

## Modules

- `shank` — Shank IDL format
- `anchor` — Legacy Anchor IDL format
- `anchor_v1` — Anchor IDL v1 format (with `metadata.spec` and explicit discriminators). Reuses instruction, typedef, and error codegen from `anchor`, with its own accounts and events codegen for explicit discriminator support.
- `bincode` — Custom bincode IDL format for older Solana programs