use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{KeyedAccount, ReadonlyAccount};
use spl_associated_token_account_interface::RecoverNestedKeys;

use crate::pda::{AtaCreatePdaArgs, AtaFindPdaArgs};

pub struct RecoverNestedRootAccounts<A: ReadonlyAccount + KeyedAccount> {
    pub wallet: Pubkey,
    pub owner_token_account_mint: A,
    pub nested_mint: A,
}

impl<A: ReadonlyAccount + KeyedAccount> RecoverNestedRootAccounts<A> {
    /// Determins the spl-token program ID to use from the program owners of
    /// owner_token_account_mint and nested_mint
    /// Returns ProgramError::IllegalOwner if the 2 dont match
    pub fn det_token_program(&self) -> Result<Pubkey, ProgramError> {
        let owner_token_program = self.owner_token_account_mint.owner();
        let nested_token_program = self.nested_mint.owner();
        if owner_token_program != nested_token_program {
            return Err(ProgramError::IllegalOwner);
        }
        Ok(*owner_token_program)
    }

    /// .1 is owner_token_account signer seeds args
    pub fn resolve(&self) -> Result<(RecoverNestedKeys, AtaCreatePdaArgs), ProgramError> {
        let token_program = self.det_token_program()?;
        let find_owner_token_account_args = AtaFindPdaArgs {
            wallet: self.wallet,
            mint: *self.owner_token_account_mint.key(),
            token_program,
        };
        let (owner_associated_token_account, bump) =
            find_owner_token_account_args.get_associated_token_address_and_bump_seed();
        let find_nested_token_account_args = AtaFindPdaArgs {
            wallet: owner_associated_token_account,
            mint: *self.nested_mint.key(),
            token_program,
        };
        let (nested, _) =
            find_nested_token_account_args.get_associated_token_address_and_bump_seed();
        let find_wallet_ata_args = AtaFindPdaArgs {
            wallet: self.wallet,
            mint: *self.nested_mint.key(),
            token_program,
        };
        let (wallet_associated_token_account, _) =
            find_wallet_ata_args.get_associated_token_address_and_bump_seed();
        Ok((
            RecoverNestedKeys {
                wallet: self.wallet,
                owner_token_account_mint: *self.owner_token_account_mint.key(),
                nested_mint: *self.nested_mint.key(),
                token_program,
                nested,
                owner_associated_token_account,
                wallet_associated_token_account,
            },
            AtaCreatePdaArgs {
                find: find_owner_token_account_args,
                bump: [bump],
            },
        ))
    }
}
