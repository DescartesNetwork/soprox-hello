use crate::error::AppError;
use crate::interfaces::{xsplata::XSPLATA, xsplt::XSPLT, xsystem::XSystem};
use solana_program::{
  account_info::AccountInfo,
  clock::Clock,
  entrypoint::ProgramResult,
  program_error::ProgramError,
  program_pack::{IsInitialized, Pack},
  pubkey::{Pubkey, PubkeyError},
  sysvar::Sysvar,
};
use spl_token::state::Account;

pub fn checked_initialize_splt_account<'a>(
  funding_acc: &AccountInfo<'a>,
  target_acc: &AccountInfo<'a>,
  target_owner: &AccountInfo<'a>,
  mint_acc: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  splt_program: &AccountInfo<'a>,
  sysvar_rent_acc: &AccountInfo<'a>,
  splata_program: &AccountInfo<'a>,
) -> ProgramResult {
  // Check account
  let mut rented: bool = true;
  if !XSystem::check_account(target_acc)? {
    rented = false;
  } else {
    let target_data = Account::unpack_unchecked(&target_acc.data.borrow())?;
    if !target_data.is_initialized() {
      rented = false;
    }
  }
  // Initialize account
  if !rented {
    XSPLATA::initialize_account(
      funding_acc,
      target_acc,
      target_owner,
      mint_acc,
      system_program,
      splt_program,
      sysvar_rent_acc,
      splata_program,
      &[],
    )?;
  }
  // Ready
  Ok(())
}

pub fn checked_transfer_splt<'a>(
  amount: u64,
  funding_acc: &AccountInfo<'a>,
  src_acc: &AccountInfo<'a>,
  src_owner: &AccountInfo<'a>,
  dst_acc: &AccountInfo<'a>,
  dst_owner: &AccountInfo<'a>,
  mint_acc: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  splt_program: &AccountInfo<'a>,
  sysvar_rent_acc: &AccountInfo<'a>,
  splata_program: &AccountInfo<'a>,
  seed: &[&[&[u8]]],
) -> ProgramResult {
  // Initialize account
  checked_initialize_splt_account(
    funding_acc,
    dst_acc,
    dst_owner,
    mint_acc,
    system_program,
    splt_program,
    sysvar_rent_acc,
    splata_program,
  )?;
  // Transfer
  XSPLT::transfer(amount, src_acc, dst_acc, src_owner, splt_program, seed)?;
  Ok(())
}

pub fn is_program(program_id: &Pubkey, accounts: &[&AccountInfo]) -> ProgramResult {
  for acc in &mut accounts.iter() {
    if acc.owner != program_id {
      return Err(AppError::IncorrectProgramId.into());
    }
  }
  Ok(())
}

pub fn is_signer(accounts: &[&AccountInfo]) -> ProgramResult {
  for acc in &mut accounts.iter() {
    if !acc.is_signer {
      return Err(AppError::InvalidOwner.into());
    }
  }
  Ok(())
}

pub fn safe_seed(
  seed_acc: &AccountInfo,
  expected_acc: &AccountInfo,
  program_id: &Pubkey,
) -> Result<[u8; 32], PubkeyError> {
  let seed: [u8; 32] = seed_acc.key.to_bytes();
  let key = Pubkey::create_program_address(&[&seed], program_id)?;
  if key != *expected_acc.key {
    return Err(PubkeyError::InvalidSeeds);
  }
  Ok(seed)
}

pub fn current_timestamp() -> Result<i64, ProgramError> {
  let clock = Clock::get()?;
  Ok(clock.unix_timestamp)
}
