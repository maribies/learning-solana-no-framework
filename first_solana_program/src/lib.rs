use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
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
    Ok(())
}

// macro- sharable extraction of code at syntax level
// Define the type of state stored in accounts
// When init from Anchor, #[account] is already available
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Accomplishments {
    pub cadet_mastery: Vec<String>,
}
