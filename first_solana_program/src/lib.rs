use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation- all calls go through here.
// function: declare new ones with 'fn'
fn process_instruction(
    program_id: &Pubkey, // The public key of the currently executing program
    accounts: &[AccountInfo], // An ordered slice of the accounts referenced by the instruction and represented as an AccountInfo structures... The account to work with
    instruction_data: &[u8], // The general purpose byte array from the instruction's instruction data being processed
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to add accomplishment to
    // the `?` is a rust operator that "hides some of the boilerplate of propagating errors up the call stack"
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("the account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut accomplishments_account = Accomplishments::try_from_slice(&account.data.borrow())?;
    // FIXME: initializing? will need to add logic to update here later?
    // probably need to check if cadet_mastery is already in existance?
    // initalization only. don't want to set this if account already has accomplishments!
    let mut accomplishments = Vec::new();
    accomplishments.push("Solana".to_string());
    accomplishments_account.cadet_mastery = accomplishments;
    accomplishments_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Accomplishments {:?}!", accomplishments_account.cadet_mastery);

    Ok(())
}

// macro- sharable extraction of code at syntax level
// Define the type of state stored in accounts
// When init from Anchor, #[account] is already available
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Accomplishments {
    pub cadet_mastery: Vec<String>,
}
