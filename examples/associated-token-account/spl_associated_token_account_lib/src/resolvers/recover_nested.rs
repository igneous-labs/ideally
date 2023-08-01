use solana_program::pubkey::Pubkey;
use spl_associated_token_account_interface::RecoverNestedKeys;

use crate::pda::{AtaCreatePdaArgs, AtaFindPdaArgs};

pub struct RecoverNestedRootKeys {
    pub wallet: Pubkey,
    pub owner_token_account_mint: Pubkey,
    pub nested_mint: Pubkey,
    pub token_program: Pubkey,
}

impl RecoverNestedRootKeys {
    pub fn find_nested_ata(&self) -> (Pubkey, u8) {
        let find_pda_args = AtaFindPdaArgs {
            wallet: self.wallet,
            mint: self.nested_mint,
            token_program: self.token_program,
        };
        find_pda_args.get_associated_token_address_and_bump_seed()
    }

    /// .1 is owner_token_account signer seeds args
    pub fn transform(&self) -> (RecoverNestedKeys, AtaCreatePdaArgs) {
        let find_owner_token_account_args = AtaFindPdaArgs {
            wallet: self.wallet,
            mint: self.owner_token_account_mint,
            token_program: self.token_program,
        };
        let (owner_associated_token_account, bump) =
            find_owner_token_account_args.get_associated_token_address_and_bump_seed();
        let find_nested_token_account_args = AtaFindPdaArgs {
            wallet: owner_associated_token_account,
            mint: self.nested_mint,
            token_program: self.token_program,
        };
        let (nested, _) =
            find_nested_token_account_args.get_associated_token_address_and_bump_seed();
        let find_wallet_ata_args = AtaFindPdaArgs {
            wallet: self.wallet,
            mint: self.nested_mint,
            token_program: self.token_program,
        };
        let (wallet_associated_token_account, _) =
            find_wallet_ata_args.get_associated_token_address_and_bump_seed();
        (
            RecoverNestedKeys {
                wallet: self.wallet,
                owner_token_account_mint: self.owner_token_account_mint,
                nested_mint: self.nested_mint,
                token_program: self.token_program,
                nested,
                owner_associated_token_account,
                wallet_associated_token_account,
            },
            AtaCreatePdaArgs {
                find: find_owner_token_account_args,
                bump: [bump],
            },
        )
    }
}
