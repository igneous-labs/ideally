//! fn sigs copied from upstream for program-tests to run without modification

use solana_program::{instruction::Instruction, pubkey::Pubkey};
use spl_associated_token_account_interface::{
    create_idempotent_ix, create_ix, recover_nested_ix, CreateIdempotentIxArgs, CreateIxArgs,
    RecoverNestedIxArgs,
};
use spl_associated_token_account_lib::resolvers::{
    create::CreateKeysTokenProgramResolved, recover_nested::RecoverNestedRootKeys,
};

pub fn create_associated_token_account(
    funding_address: &Pubkey,
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
    token_program_id: &Pubkey,
) -> Instruction {
    let root_keys = CreateKeysTokenProgramResolved {
        funding_account: *funding_address,
        wallet: *wallet_address,
        mint: *token_mint_address,
        token_program: *token_program_id,
    };
    create_ix(root_keys.resolve().0, CreateIxArgs {}).unwrap()
}

pub fn create_associated_token_account_idempotent(
    funding_address: &Pubkey,
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
    token_program_id: &Pubkey,
) -> Instruction {
    let root_keys = CreateKeysTokenProgramResolved {
        funding_account: *funding_address,
        wallet: *wallet_address,
        mint: *token_mint_address,
        token_program: *token_program_id,
    };
    create_idempotent_ix(root_keys.resolve_idempotent().0, CreateIdempotentIxArgs {}).unwrap()
}

pub fn recover_nested(
    wallet_address: &Pubkey,
    owner_token_mint_address: &Pubkey,
    nested_token_mint_address: &Pubkey,
    token_program_id: &Pubkey,
) -> Instruction {
    let root_keys = RecoverNestedRootKeys {
        wallet: *wallet_address,
        owner_token_account_mint: *owner_token_mint_address,
        nested_mint: *nested_token_mint_address,
        token_program: *token_program_id,
    };
    recover_nested_ix(root_keys.resolve().0, RecoverNestedIxArgs {}).unwrap()
}
