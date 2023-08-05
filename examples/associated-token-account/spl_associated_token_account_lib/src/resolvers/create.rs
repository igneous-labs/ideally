use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountOwner};
use spl_associated_token_account_interface::{CreateIdempotentKeys, CreateKeys};

use crate::pda::{AtaCreatePdaArgs, AtaFindPdaArgs};

pub struct CreateRootAccounts<M: KeyedAccount + ReadonlyAccountOwner> {
    pub funding_account: Pubkey,
    pub wallet: Pubkey,
    pub mint: M,
}

pub struct CreateKeysTokenProgramResolved {
    pub funding_account: Pubkey,
    pub wallet: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
}

impl<M: KeyedAccount + ReadonlyAccountOwner> CreateRootAccounts<M> {
    fn resolve_token_program(&self) -> CreateKeysTokenProgramResolved {
        let mint = *self.mint.key();
        let token_program = *self.mint.owner();
        CreateKeysTokenProgramResolved {
            funding_account: self.funding_account,
            wallet: self.wallet,
            mint,
            token_program,
        }
    }

    pub fn resolve(&self) -> (CreateKeys, AtaCreatePdaArgs) {
        self.resolve_token_program().resolve()
    }

    /// plz figure out they're the same type and optimize this away compiler
    pub fn resolve_idempotent(&self) -> (CreateIdempotentKeys, AtaCreatePdaArgs) {
        self.resolve_token_program().resolve_idempotent()
    }
}

impl CreateKeysTokenProgramResolved {
    pub fn resolve(&self) -> (CreateKeys, AtaCreatePdaArgs) {
        let find_pda_args = AtaFindPdaArgs {
            wallet: self.wallet,
            mint: self.mint,
            token_program: self.token_program,
        };
        let (ata, bump) = find_pda_args.get_associated_token_address_and_bump_seed();
        (
            CreateKeys {
                funding_account: self.funding_account,
                wallet: self.wallet,
                mint: self.mint,
                token_program: self.token_program,
                system_program: system_program::ID,
                associated_token_account: ata,
            },
            AtaCreatePdaArgs {
                find: find_pda_args,
                bump: [bump],
            },
        )
    }

    /// plz figure out they're the same type and optimize this away compiler
    pub fn resolve_idempotent(&self) -> (CreateIdempotentKeys, AtaCreatePdaArgs) {
        let (
            CreateKeys {
                funding_account,
                wallet,
                mint,
                token_program,
                system_program,
                associated_token_account,
            },
            create_pda_args,
        ) = self.resolve();
        (
            CreateIdempotentKeys {
                funding_account,
                wallet,
                mint,
                token_program,
                system_program,
                associated_token_account,
            },
            create_pda_args,
        )
    }
}
