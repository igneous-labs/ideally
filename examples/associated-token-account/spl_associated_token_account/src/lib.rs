#![forbid(unsafe_code)]

use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};
use spl_associated_token_account_lib::pda::AtaFindPdaArgs;

mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod tools;

// declare_id!() re-exports for program-tests to run unmodified
pub fn id() -> Pubkey {
    spl_associated_token_account_interface::id()
}

pub const ID: Pubkey = spl_associated_token_account_interface::ID;

// fn sigs copied from upstream for program-tests to run unmodified

pub fn get_associated_token_address_with_program_id(
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
    token_program_id: &Pubkey,
) -> Pubkey {
    AtaFindPdaArgs {
        wallet: *wallet_address,
        token_program: *token_program_id,
        mint: *token_mint_address,
    }
    .get_associated_token_address_and_bump_seed()
    .0
}

pub fn get_associated_token_address(
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
) -> Pubkey {
    get_associated_token_address_with_program_id(
        wallet_address,
        token_mint_address,
        &spl_token::id(),
    )
}

/// Create an associated token account for the given wallet address and token mint
///
/// Accounts expected by this instruction:
///
///   0. `[writeable,signer]` Funding account (must be a system account)
///   1. `[writeable]` Associated token account address to be created
///   2. `[]` Wallet address for the new associated token account
///   3. `[]` The token mint for the new associated token account
///   4. `[]` System program
///   5. `[]` SPL Token program
///
#[deprecated(
    since = "1.0.5",
    note = "please use `instruction::create_associated_token_account` instead"
)]
pub fn create_associated_token_account(
    funding_address: &Pubkey,
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
) -> Instruction {
    let associated_account_address =
        get_associated_token_address(wallet_address, token_mint_address);

    Instruction {
        program_id: id(),
        accounts: vec![
            AccountMeta::new(*funding_address, true),
            AccountMeta::new(associated_account_address, false),
            AccountMeta::new_readonly(*wallet_address, false),
            AccountMeta::new_readonly(*token_mint_address, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: vec![],
    }
}
