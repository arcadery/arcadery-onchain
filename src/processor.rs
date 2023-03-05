use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{
    assert_game_key_matches_account, assert_signer, create_account_owned_by_program, get_game_pda,
    ArcaderyInstruction, InitializeGameArgs, ARCADERY_PREFIX, GAME_SIZE,
};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ArcaderyInstruction::try_from_slice(instruction_data)?;

    use ArcaderyInstruction::*;
    match instruction {
        InitializeGame(args) => initialize_game(accounts, args, program_id),
    }
}

pub fn initialize_game(
    accounts: &[AccountInfo],
    args: InitializeGameArgs,
    program_id: &Pubkey,
) -> ProgramResult {
    msg!("IX: initialize_game, passed {} accounts", accounts.len());

    let account_info_iter = &mut accounts.iter();
    let player_info = next_account_info(account_info_iter)?;
    let game_pda_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    // 2. Check that the accounts conform to the requirements
    assert_signer(player_info)?;

    let (game_pda, bump) = get_game_pda(&args.game);
    assert_game_key_matches_account(&game_pda, game_pda_info)?;

    let seeds = &[ARCADERY_PREFIX.as_bytes(), args.game.as_ref(), &[bump]];
    create_account_owned_by_program(
        player_info,
        game_pda_info,
        system_info,
        program_id,
        seeds,
        GAME_SIZE,
    )?;
    Ok(())
}

pub fn player_join(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("IX: player_join, passed {} accounts", accounts.len());
    Ok(())
}
