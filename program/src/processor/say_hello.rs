use crate::error::AppError;
use crate::helper::util;
use crate::schema::hello::Hello;
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  program_error::ProgramError,
  program_pack::Pack,
  pubkey::Pubkey,
};

pub fn exec(
  amount: u32,
  program_id: &Pubkey,
  accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
  let accounts_iter = &mut accounts.iter();
  let account = next_account_info(accounts_iter)?;

  util::is_program(program_id, &[account])?;
  let mut data = Hello::unpack(&account.data.borrow())?;
  data.times = data.times.checked_add(amount).ok_or(AppError::Overflow)?;
  Hello::pack(data, &mut account.data.borrow_mut())?;

  Ok(())
}
