//! fn sigs copied from upstream for program-tests to run without modification

use solana_program::{instruction::Instruction, pubkey::Pubkey};
use spl_associated_token_account_interface::{
    create_idempotent_ix, create_ix, CreateIdempotentIxArgs, CreateIxArgs,
};
use spl_associated_token_account_lib::resolvers::create::CreateRootKeys;

pub fn create_associated_token_account(
    funding_address: &Pubkey,
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
    token_program_id: &Pubkey,
) -> Instruction {
    let root_keys = CreateRootKeys {
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
    let root_keys = CreateRootKeys {
        funding_account: *funding_address,
        wallet: *wallet_address,
        mint: *token_mint_address,
        token_program: *token_program_id,
    };
    create_idempotent_ix(root_keys.resolve_idempotent().0, CreateIdempotentIxArgs {}).unwrap()
}
