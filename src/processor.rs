use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::ArcaderyInstruction;

pub fn process(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ArcaderyInstruction::try_from_slice(instruction_data)?;

    use ArcaderyInstruction::*;
    match instruction {
        InitializeGame => initialize_game(accounts),
    }
}

pub fn initialize_game(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("IX: initialize_game, passed {} accounts", accounts.len());
    Ok(())
}

pub fn player_join(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("IX: player_join, passed {} accounts", accounts.len());
    Ok(())
}
