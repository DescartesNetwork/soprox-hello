use crate::instruction::AppInstruction;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

pub mod say_hello;

pub struct Processor {}

impl Processor {
  pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction = AppInstruction::unpack(instruction_data)?;
    match instruction {
      AppInstruction::SayHello { amount: u32 } => {
        msg!("Calling SayHello function");
        say_hello::exec(amount, program_id, accounts)?;
        Ok(())
      }
    }
  }
}
