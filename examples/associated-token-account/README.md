# associated-token-account

[spl-associated-token-account program](https://github.com/solana-labs/solana-program-library/tree/master/associated-token-account) reimplemented as an Ideally program.

## IDL

`idl.json` is a handwritten shank-style IDL

## Codegen

`spl_associated_token_account_interface` crate generated using solores v0.2.2 with cmd `solores idl.json`

## Program

In general, we tried to follow the original program structure as closely as possible, factoring out only simple account and PDA checks into the `spl_associated_token_account_library`. A more structured rewrite with all account checks completely moved to `spl_associated_token_account_library` is possible, but we did not do it, since this is a simple proof-of-concept.

## Tests

Run the tests copied from upstream with `cd program-test && cargo test-sbf`. Make sure v1.16 solana tools are used.

**MUST** manually rebuild program with `cargo build-sbf` before running `program-tests` on every modification.
