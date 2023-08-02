use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_program,
    sysvar::Sysvar,
};
use spl_associated_token_account_interface::{
    create_verify_account_keys, create_verify_account_privileges,
    recover_nested_verify_account_keys, recover_nested_verify_account_privileges, CreateAccounts,
    CreateIxArgs, RecoverNestedAccounts, SplAssociatedTokenAccountError,
    SplAssociatedTokenAccountProgramIx, CREATE_IX_ACCOUNTS_LEN, RECOVER_NESTED_IX_ACCOUNTS_LEN,
};
use spl_associated_token_account_lib::resolvers::{
    create::CreateRootKeys, recover_nested::RecoverNestedRootAccounts,
};
use spl_token_2022::{
    extension::{ExtensionType, StateWithExtensions},
    state::{Account, Mint},
};

use crate::tools::account::{create_pda_account, get_account_len};

/// Specify when to create the associated token account
#[derive(PartialEq)]
enum CreateMode {
    /// Always try to create the ATA
    Always,
    /// Only try to create the ATA if non-existent
    Idempotent,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = if input.is_empty() {
        SplAssociatedTokenAccountProgramIx::Create(CreateIxArgs {})
    } else {
        SplAssociatedTokenAccountProgramIx::deserialize(&mut &input[..])?
    };

    msg!("{:?}", instruction);

    match instruction {
        SplAssociatedTokenAccountProgramIx::Create(_) => {
            process_create_associated_token_account(accounts, CreateMode::Always)
        }
        SplAssociatedTokenAccountProgramIx::CreateIdempotent(_) => {
            process_create_associated_token_account(accounts, CreateMode::Idempotent)
        }
        SplAssociatedTokenAccountProgramIx::RecoverNested(_) => process_recover_nested(accounts),
    }
}

fn process_create_associated_token_account(
    accounts: &[AccountInfo],
    create_mode: CreateMode,
) -> ProgramResult {
    let funding_account = accounts.get(0).ok_or(ProgramError::NotEnoughAccountKeys)?;
    let wallet = accounts.get(2).ok_or(ProgramError::NotEnoughAccountKeys)?;
    let mint = accounts.get(3).ok_or(ProgramError::NotEnoughAccountKeys)?;
    let token_program = accounts.get(5).ok_or(ProgramError::NotEnoughAccountKeys)?;
    let free_accs = CreateRootKeys {
        funding_account: *funding_account.key,
        wallet: *wallet.key,
        mint: *mint.key,
        token_program: *token_program.key,
    };
    let (expected_keys, ata_create_pda_args) = free_accs.resolve();
    let actual_accounts_slice: &[AccountInfo; CREATE_IX_ACCOUNTS_LEN] = accounts
        .get(..CREATE_IX_ACCOUNTS_LEN)
        .ok_or(ProgramError::NotEnoughAccountKeys)?
        .try_into()
        .unwrap();
    let create_accounts: CreateAccounts = actual_accounts_slice.into();

    if let Err((actual_pubkey, _expected_pubkey)) =
        create_verify_account_keys(&create_accounts, &expected_keys)
    {
        if actual_pubkey == *create_accounts.associated_token_account.key {
            msg!("Error: Associated address does not match seed derivation");
            return Err(ProgramError::InvalidSeeds);
        }
        return Err(ProgramError::InvalidAccountData);
    }
    create_verify_account_privileges(&create_accounts)?;

    if create_mode == CreateMode::Idempotent
        && create_accounts.associated_token_account.owner == create_accounts.token_program.key
    {
        let ata_data = create_accounts.associated_token_account.data.borrow();
        if let Ok(associated_token_account) = StateWithExtensions::<Account>::unpack(&ata_data) {
            if associated_token_account.base.owner != *create_accounts.wallet.key {
                let error = SplAssociatedTokenAccountError::InvalidOwner;
                msg!("{}", error);
                return Err(error.into());
            }
            if associated_token_account.base.mint != *create_accounts.mint.key {
                return Err(ProgramError::InvalidAccountData);
            }
            return Ok(());
        }
    }
    if *create_accounts.associated_token_account.owner != system_program::id() {
        return Err(ProgramError::IllegalOwner);
    }

    let rent = Rent::get()?;

    let account_len = get_account_len(
        create_accounts.mint,
        create_accounts.token_program,
        &[ExtensionType::ImmutableOwner],
    )?;

    create_pda_account(
        create_accounts.funding_account,
        &rent,
        account_len,
        create_accounts.token_program.key,
        create_accounts.system_program,
        create_accounts.associated_token_account,
        &ata_create_pda_args.to_signer_seeds(),
    )?;

    msg!("Initialize the associated token account");
    invoke(
        &spl_token_2022::instruction::initialize_immutable_owner(
            create_accounts.token_program.key,
            create_accounts.associated_token_account.key,
        )?,
        &[
            create_accounts.associated_token_account.clone(),
            create_accounts.token_program.clone(),
        ],
    )?;
    invoke(
        &spl_token_2022::instruction::initialize_account3(
            create_accounts.token_program.key,
            create_accounts.associated_token_account.key,
            create_accounts.mint.key,
            create_accounts.wallet.key,
        )?,
        &[
            create_accounts.associated_token_account.clone(),
            create_accounts.mint.clone(),
            create_accounts.wallet.clone(),
            create_accounts.token_program.clone(),
        ],
    )
}

pub fn process_recover_nested(accounts: &[AccountInfo]) -> ProgramResult {
    let wallet = accounts.get(2).ok_or(ProgramError::NotEnoughAccountKeys)?;
    let owner_token_account_mint = accounts.get(4).ok_or(ProgramError::NotEnoughAccountKeys)?;
    let nested_mint = accounts.get(1).ok_or(ProgramError::NotEnoughAccountKeys)?;
    let free_accs = RecoverNestedRootAccounts {
        wallet: *wallet.key,
        owner_token_account_mint,
        nested_mint,
    };
    let (expected_keys, owner_ata_create_pda_args) = free_accs.resolve()?;
    let actual_accounts_slice: &[AccountInfo; RECOVER_NESTED_IX_ACCOUNTS_LEN] = accounts
        .get(..RECOVER_NESTED_IX_ACCOUNTS_LEN)
        .ok_or(ProgramError::NotEnoughAccountKeys)?
        .try_into()
        .unwrap();
    let recover_nested_accounts: RecoverNestedAccounts = actual_accounts_slice.into();

    if let Err((actual_pubkey, _expected_pubkey)) =
        recover_nested_verify_account_keys(&recover_nested_accounts, &expected_keys)
    {
        // owner address derivation checked
        if actual_pubkey == *recover_nested_accounts.owner_associated_token_account.key {
            msg!("Error: Owner associated address does not match seed derivation");
            return Err(ProgramError::InvalidSeeds);
        }

        // nested address derivation checked
        if actual_pubkey == *recover_nested_accounts.nested.key {
            msg!("Error: Nested associated address does not match seed derivation");
            return Err(ProgramError::InvalidSeeds);
        }

        // destination address derivation checked
        if actual_pubkey == *recover_nested_accounts.wallet_associated_token_account.key {
            msg!("Error: Destination associated address does not match seed derivation");
            return Err(ProgramError::InvalidSeeds);
        }

        if actual_pubkey == *recover_nested_accounts.token_program.key {
            msg!("Incorrect token program");
            return Err(ProgramError::IllegalOwner);
        }

        return Err(ProgramError::InvalidAccountData);
    }
    recover_nested_verify_account_privileges(&recover_nested_accounts)?;

    // Account data is dropped at the end of this, so the CPI can succeed
    // without a double-borrow
    let (amount, decimals) = {
        // Check owner associated token account data
        if recover_nested_accounts.owner_associated_token_account.owner
            != recover_nested_accounts.token_program.key
        {
            msg!("Owner associated token account not owned by provided token program, recreate the owner associated token account first");
            return Err(ProgramError::IllegalOwner);
        }
        let owner_account_data = recover_nested_accounts
            .owner_associated_token_account
            .data
            .borrow();
        let owner_account = StateWithExtensions::<Account>::unpack(&owner_account_data)?;
        if owner_account.base.owner != *recover_nested_accounts.wallet.key {
            msg!("Owner associated token account not owned by provided wallet");
            return Err(SplAssociatedTokenAccountError::InvalidOwner.into());
        }

        // Check nested associated token account data
        if recover_nested_accounts.nested.owner != recover_nested_accounts.token_program.key {
            msg!("Nested associated token account not owned by provided token program");
            return Err(ProgramError::IllegalOwner);
        }
        let nested_account_data = recover_nested_accounts.nested.data.borrow();
        let nested_account = StateWithExtensions::<Account>::unpack(&nested_account_data)?;
        if nested_account.base.owner != *recover_nested_accounts.owner_associated_token_account.key
        {
            msg!("Nested associated token account not owned by provided associated token account");
            return Err(SplAssociatedTokenAccountError::InvalidOwner.into());
        }
        let amount = nested_account.base.amount;
        let nested_mint_data = recover_nested_accounts.nested_mint.data.borrow();
        let nested_mint = StateWithExtensions::<Mint>::unpack(&nested_mint_data)?;
        let decimals = nested_mint.base.decimals;
        (amount, decimals)
    };

    // Transfer everything out
    invoke_signed(
        &spl_token_2022::instruction::transfer_checked(
            recover_nested_accounts.token_program.key,
            recover_nested_accounts.nested.key,
            recover_nested_accounts.nested_mint.key,
            recover_nested_accounts.wallet_associated_token_account.key,
            recover_nested_accounts.owner_associated_token_account.key,
            &[],
            amount,
            decimals,
        )?,
        &[
            recover_nested_accounts.nested.clone(),
            recover_nested_accounts.nested_mint.clone(),
            recover_nested_accounts
                .wallet_associated_token_account
                .clone(),
            recover_nested_accounts
                .owner_associated_token_account
                .clone(),
            recover_nested_accounts.token_program.clone(),
        ],
        &[&owner_ata_create_pda_args.to_signer_seeds()],
    )?;

    // Close the nested account so it's never used again
    invoke_signed(
        &spl_token_2022::instruction::close_account(
            recover_nested_accounts.token_program.key,
            recover_nested_accounts.nested.key,
            recover_nested_accounts.wallet.key,
            recover_nested_accounts.owner_associated_token_account.key,
            &[],
        )?,
        &[
            recover_nested_accounts.nested.clone(),
            recover_nested_accounts.wallet.clone(),
            recover_nested_accounts
                .owner_associated_token_account
                .clone(),
            recover_nested_accounts.token_program.clone(),
        ],
        &[&owner_ata_create_pda_args.to_signer_seeds()],
    )
}
