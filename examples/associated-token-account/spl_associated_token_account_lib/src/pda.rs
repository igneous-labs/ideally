use solana_program::pubkey::Pubkey;

pub struct AtaFindPdaArgs {
    pub wallet: Pubkey,
    pub token_program: Pubkey,
    pub mint: Pubkey,
}

impl AtaFindPdaArgs {
    // For more complex PDAs where some seeds do not impl
    // AsRef<[u8]> like Pubkey, probably need to use a tuple + macro
    // or smth to make it more ergonomic
    pub fn to_seeds(&self) -> [&[u8]; 3] {
        [
            self.wallet.as_ref(),
            self.token_program.as_ref(),
            self.mint.as_ref(),
        ]
    }

    pub fn get_associated_token_address_and_bump_seed(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &self.to_seeds(),
            &spl_associated_token_account_interface::ID,
        )
    }
}

pub struct AtaCreatePdaArgs {
    pub find: AtaFindPdaArgs,
    pub bump: [u8; 1],
}

impl AtaCreatePdaArgs {
    pub fn to_signer_seeds(&self) -> [&[u8]; 4] {
        let [a, b, c] = self.find.to_seeds();
        [a, b, c, &self.bump]
    }
}
