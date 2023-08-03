use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
#[derive(Clone, Debug, PartialEq)]
pub enum SplAssociatedTokenAccountProgramIx {
    Create(CreateIxArgs),
    CreateIdempotent(CreateIdempotentIxArgs),
    RecoverNested(RecoverNestedIxArgs),
}
impl BorshSerialize for SplAssociatedTokenAccountProgramIx {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Self::Create(args) => {
                CREATE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::CreateIdempotent(args) => {
                CREATE_IDEMPOTENT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::RecoverNested(args) => {
                RECOVER_NESTED_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
        }
    }
}
impl SplAssociatedTokenAccountProgramIx {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        match maybe_discm {
            CREATE_IX_DISCM => Ok(Self::Create(CreateIxArgs::deserialize(buf)?)),
            CREATE_IDEMPOTENT_IX_DISCM => Ok(Self::CreateIdempotent(
                CreateIdempotentIxArgs::deserialize(buf)?,
            )),
            RECOVER_NESTED_IX_DISCM => {
                Ok(Self::RecoverNested(RecoverNestedIxArgs::deserialize(buf)?))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
}
pub const CREATE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateAccounts<'me, 'info> {
    ///Funding account (must be a system account)
    pub funding_account: &'me AccountInfo<'info>,
    ///Associated token account address to be created
    pub associated_token_account: &'me AccountInfo<'info>,
    ///Wallet address for the new associated token account
    pub wallet: &'me AccountInfo<'info>,
    ///The token mint for the new associated token account
    pub mint: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Wallet address for the new associated token account
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateKeys {
    ///Funding account (must be a system account)
    pub funding_account: Pubkey,
    ///Associated token account address to be created
    pub associated_token_account: Pubkey,
    ///Wallet address for the new associated token account
    pub wallet: Pubkey,
    ///The token mint for the new associated token account
    pub mint: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Wallet address for the new associated token account
    pub token_program: Pubkey,
}
impl From<&CreateAccounts<'_, '_>> for CreateKeys {
    fn from(accounts: &CreateAccounts) -> Self {
        Self {
            funding_account: *accounts.funding_account.key,
            associated_token_account: *accounts.associated_token_account.key,
            wallet: *accounts.wallet.key,
            mint: *accounts.mint.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&CreateKeys> for [AccountMeta; CREATE_IX_ACCOUNTS_LEN] {
    fn from(keys: &CreateKeys) -> Self {
        [
            AccountMeta::new(keys.funding_account, true),
            AccountMeta::new(keys.associated_token_account, false),
            AccountMeta::new_readonly(keys.wallet, false),
            AccountMeta::new_readonly(keys.mint, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; CREATE_IX_ACCOUNTS_LEN]> for CreateKeys {
    fn from(pubkeys: [Pubkey; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funding_account: pubkeys[0],
            associated_token_account: pubkeys[1],
            wallet: pubkeys[2],
            mint: pubkeys[3],
            system_program: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<&CreateAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: &CreateAccounts<'_, 'info>) -> Self {
        [
            accounts.funding_account.clone(),
            accounts.associated_token_account.clone(),
            accounts.wallet.clone(),
            accounts.mint.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]>
    for CreateAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funding_account: &arr[0],
            associated_token_account: &arr[1],
            wallet: &arr[2],
            mint: &arr[3],
            system_program: &arr[4],
            token_program: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateIxData(pub CreateIxArgs);
pub const CREATE_IX_DISCM: u8 = 0u8;
impl From<CreateIxArgs> for CreateIxData {
    fn from(args: CreateIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreateIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl CreateIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CREATE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateIxArgs::deserialize(buf)?))
    }
}
pub fn create_ix<K: Into<CreateKeys>, A: Into<CreateIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CreateKeys = accounts.into();
    let metas: [AccountMeta; CREATE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CreateIxArgs = args.into();
    let data: CreateIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_invoke<'info, A: Into<CreateIxArgs>>(
    accounts: &CreateAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = create_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_invoke_signed<'info, A: Into<CreateIxArgs>>(
    accounts: &CreateAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn create_verify_account_keys(
    accounts: &CreateAccounts<'_, '_>,
    keys: &CreateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.funding_account.key, &keys.funding_account),
        (
            accounts.associated_token_account.key,
            &keys.associated_token_account,
        ),
        (accounts.wallet.key, &keys.wallet),
        (accounts.mint.key, &keys.mint),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_verify_account_privileges(
    accounts: &CreateAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.funding_account, accounts.associated_token_account] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.funding_account] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateIdempotentAccounts<'me, 'info> {
    ///Funding account (must be a system account)
    pub funding_account: &'me AccountInfo<'info>,
    ///Associated token account address to be created
    pub associated_token_account: &'me AccountInfo<'info>,
    ///Wallet address for the new associated token account
    pub wallet: &'me AccountInfo<'info>,
    ///The token mint for the new associated token account
    pub mint: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Wallet address for the new associated token account
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateIdempotentKeys {
    ///Funding account (must be a system account)
    pub funding_account: Pubkey,
    ///Associated token account address to be created
    pub associated_token_account: Pubkey,
    ///Wallet address for the new associated token account
    pub wallet: Pubkey,
    ///The token mint for the new associated token account
    pub mint: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Wallet address for the new associated token account
    pub token_program: Pubkey,
}
impl From<&CreateIdempotentAccounts<'_, '_>> for CreateIdempotentKeys {
    fn from(accounts: &CreateIdempotentAccounts) -> Self {
        Self {
            funding_account: *accounts.funding_account.key,
            associated_token_account: *accounts.associated_token_account.key,
            wallet: *accounts.wallet.key,
            mint: *accounts.mint.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&CreateIdempotentKeys> for [AccountMeta; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN] {
    fn from(keys: &CreateIdempotentKeys) -> Self {
        [
            AccountMeta::new(keys.funding_account, true),
            AccountMeta::new(keys.associated_token_account, false),
            AccountMeta::new_readonly(keys.wallet, false),
            AccountMeta::new_readonly(keys.mint, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN]> for CreateIdempotentKeys {
    fn from(pubkeys: [Pubkey; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funding_account: pubkeys[0],
            associated_token_account: pubkeys[1],
            wallet: pubkeys[2],
            mint: pubkeys[3],
            system_program: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<&CreateIdempotentAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CreateIdempotentAccounts<'_, 'info>) -> Self {
        [
            accounts.funding_account.clone(),
            accounts.associated_token_account.clone(),
            accounts.wallet.clone(),
            accounts.mint.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN]>
    for CreateIdempotentAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funding_account: &arr[0],
            associated_token_account: &arr[1],
            wallet: &arr[2],
            mint: &arr[3],
            system_program: &arr[4],
            token_program: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateIdempotentIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateIdempotentIxData(pub CreateIdempotentIxArgs);
pub const CREATE_IDEMPOTENT_IX_DISCM: u8 = 1u8;
impl From<CreateIdempotentIxArgs> for CreateIdempotentIxData {
    fn from(args: CreateIdempotentIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreateIdempotentIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_IDEMPOTENT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl CreateIdempotentIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CREATE_IDEMPOTENT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_IDEMPOTENT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateIdempotentIxArgs::deserialize(buf)?))
    }
}
pub fn create_idempotent_ix<K: Into<CreateIdempotentKeys>, A: Into<CreateIdempotentIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CreateIdempotentKeys = accounts.into();
    let metas: [AccountMeta; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CreateIdempotentIxArgs = args.into();
    let data: CreateIdempotentIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_idempotent_invoke<'info, A: Into<CreateIdempotentIxArgs>>(
    accounts: &CreateIdempotentAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = create_idempotent_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_idempotent_invoke_signed<'info, A: Into<CreateIdempotentIxArgs>>(
    accounts: &CreateIdempotentAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_idempotent_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn create_idempotent_verify_account_keys(
    accounts: &CreateIdempotentAccounts<'_, '_>,
    keys: &CreateIdempotentKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.funding_account.key, &keys.funding_account),
        (
            accounts.associated_token_account.key,
            &keys.associated_token_account,
        ),
        (accounts.wallet.key, &keys.wallet),
        (accounts.mint.key, &keys.mint),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_idempotent_verify_account_privileges(
    accounts: &CreateIdempotentAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.funding_account, accounts.associated_token_account] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.funding_account] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const RECOVER_NESTED_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct RecoverNestedAccounts<'me, 'info> {
    ///Nested associated token account, must be owned by ownerAssociatedTokenAccount
    pub nested: &'me AccountInfo<'info>,
    ///Token mint for nested
    pub nested_mint: &'me AccountInfo<'info>,
    ///wallet's associated token account of nestedMint to recover the funds to, must be owned by wallet
    pub wallet_associated_token_account: &'me AccountInfo<'info>,
    ///wallet's associated token account of ownerAssociatedTokenAccountMint that owns nested
    pub owner_associated_token_account: &'me AccountInfo<'info>,
    ///Token mint for ownerAssociatedTokenAccount
    pub owner_token_account_mint: &'me AccountInfo<'info>,
    ///Wallet address for walletAssociatedTokenAccount
    pub wallet: &'me AccountInfo<'info>,
    ///SPL token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RecoverNestedKeys {
    ///Nested associated token account, must be owned by ownerAssociatedTokenAccount
    pub nested: Pubkey,
    ///Token mint for nested
    pub nested_mint: Pubkey,
    ///wallet's associated token account of nestedMint to recover the funds to, must be owned by wallet
    pub wallet_associated_token_account: Pubkey,
    ///wallet's associated token account of ownerAssociatedTokenAccountMint that owns nested
    pub owner_associated_token_account: Pubkey,
    ///Token mint for ownerAssociatedTokenAccount
    pub owner_token_account_mint: Pubkey,
    ///Wallet address for walletAssociatedTokenAccount
    pub wallet: Pubkey,
    ///SPL token program
    pub token_program: Pubkey,
}
impl From<&RecoverNestedAccounts<'_, '_>> for RecoverNestedKeys {
    fn from(accounts: &RecoverNestedAccounts) -> Self {
        Self {
            nested: *accounts.nested.key,
            nested_mint: *accounts.nested_mint.key,
            wallet_associated_token_account: *accounts.wallet_associated_token_account.key,
            owner_associated_token_account: *accounts.owner_associated_token_account.key,
            owner_token_account_mint: *accounts.owner_token_account_mint.key,
            wallet: *accounts.wallet.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&RecoverNestedKeys> for [AccountMeta; RECOVER_NESTED_IX_ACCOUNTS_LEN] {
    fn from(keys: &RecoverNestedKeys) -> Self {
        [
            AccountMeta::new(keys.nested, false),
            AccountMeta::new_readonly(keys.nested_mint, false),
            AccountMeta::new(keys.wallet_associated_token_account, false),
            AccountMeta::new_readonly(keys.owner_associated_token_account, false),
            AccountMeta::new_readonly(keys.owner_token_account_mint, false),
            AccountMeta::new(keys.wallet, true),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; RECOVER_NESTED_IX_ACCOUNTS_LEN]> for RecoverNestedKeys {
    fn from(pubkeys: [Pubkey; RECOVER_NESTED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nested: pubkeys[0],
            nested_mint: pubkeys[1],
            wallet_associated_token_account: pubkeys[2],
            owner_associated_token_account: pubkeys[3],
            owner_token_account_mint: pubkeys[4],
            wallet: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<&RecoverNestedAccounts<'_, 'info>>
    for [AccountInfo<'info>; RECOVER_NESTED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RecoverNestedAccounts<'_, 'info>) -> Self {
        [
            accounts.nested.clone(),
            accounts.nested_mint.clone(),
            accounts.wallet_associated_token_account.clone(),
            accounts.owner_associated_token_account.clone(),
            accounts.owner_token_account_mint.clone(),
            accounts.wallet.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RECOVER_NESTED_IX_ACCOUNTS_LEN]>
    for RecoverNestedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; RECOVER_NESTED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nested: &arr[0],
            nested_mint: &arr[1],
            wallet_associated_token_account: &arr[2],
            owner_associated_token_account: &arr[3],
            owner_token_account_mint: &arr[4],
            wallet: &arr[5],
            token_program: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RecoverNestedIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct RecoverNestedIxData(pub RecoverNestedIxArgs);
pub const RECOVER_NESTED_IX_DISCM: u8 = 2u8;
impl From<RecoverNestedIxArgs> for RecoverNestedIxData {
    fn from(args: RecoverNestedIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RecoverNestedIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[RECOVER_NESTED_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl RecoverNestedIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != RECOVER_NESTED_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    RECOVER_NESTED_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RecoverNestedIxArgs::deserialize(buf)?))
    }
}
pub fn recover_nested_ix<K: Into<RecoverNestedKeys>, A: Into<RecoverNestedIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RecoverNestedKeys = accounts.into();
    let metas: [AccountMeta; RECOVER_NESTED_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RecoverNestedIxArgs = args.into();
    let data: RecoverNestedIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn recover_nested_invoke<'info, A: Into<RecoverNestedIxArgs>>(
    accounts: &RecoverNestedAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = recover_nested_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; RECOVER_NESTED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn recover_nested_invoke_signed<'info, A: Into<RecoverNestedIxArgs>>(
    accounts: &RecoverNestedAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = recover_nested_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; RECOVER_NESTED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn recover_nested_verify_account_keys(
    accounts: &RecoverNestedAccounts<'_, '_>,
    keys: &RecoverNestedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.nested.key, &keys.nested),
        (accounts.nested_mint.key, &keys.nested_mint),
        (
            accounts.wallet_associated_token_account.key,
            &keys.wallet_associated_token_account,
        ),
        (
            accounts.owner_associated_token_account.key,
            &keys.owner_associated_token_account,
        ),
        (
            accounts.owner_token_account_mint.key,
            &keys.owner_token_account_mint,
        ),
        (accounts.wallet.key, &keys.wallet),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn recover_nested_verify_account_privileges(
    accounts: &RecoverNestedAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.nested,
        accounts.wallet_associated_token_account,
        accounts.wallet,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
