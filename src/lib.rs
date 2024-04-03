use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_program::program::invoke;

// Define the program entrypoint function
entrypoint!(process_instruction);

// Define the entry point function for the program
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Get the accounts passed to the program
    let accounts_iter = &mut accounts.iter();
    let from_account = next_account_info(accounts_iter)?;
    let to_account = next_account_info(accounts_iter)?;

    // Parse the instruction data to get the amount of SOL to withdraw
    let amount_to_withdraw = instruction_data.get(0..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    // Check if the caller has sufficient balance to withdraw the specified amount
    if from_account.lamports() < amount_to_withdraw {
        return Err(ProgramError::InsufficientFunds);
    }

    // Transfer the specified amount of SOL from the caller's account to the target account
    invoke(
        &solana_program::system_instruction::transfer(
            from_account.key,
            to_account.key,
            amount_to_withdraw,
        ),
        &[from_account.clone(), to_account.clone()],
    )?;

    msg!("{} SOL withdrawn from caller's account", amount_to_withdraw);

    Ok(())
}