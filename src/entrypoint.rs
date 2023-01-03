use solana_program::{
    account_info::{next_account_info, AccountInfo}, msg,
    entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
    program_error::ProgramError
};
use thiserror::Error;


#[derive(Error, Debug, Copy, Clone)]
pub enum AccountCloserError {
    #[error("Amount Overflow")]
    AmountOverflow,
}

impl From<AccountCloserError> for ProgramError {
    fn from(e: AccountCloserError) -> Self {
        ProgramError::Custom(e as u32)
    }
}


entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let data_account = next_account_info(accounts_iter)?;
    let transfer_account = next_account_info(accounts_iter)?;

    msg!("Closing data account {}", data_account.key);

    **transfer_account.try_borrow_mut_lamports()? = transfer_account
        .lamports()
        .checked_add(data_account.lamports())  // data_account.lamports()
        .ok_or(AccountCloserError::AmountOverflow)?;
    **data_account.try_borrow_mut_lamports()? = 0;
    *data_account.try_borrow_mut_data()? = &mut [];

    Ok(())
}
