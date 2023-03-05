use solana_program::{
    account_info::AccountInfo,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{id, ArcaderyError, ARCADERY_PREFIX};

pub fn get_game_pda(game: &Pubkey) -> (Pubkey, u8) {
    let program_id = id();
    let seeds = [ARCADERY_PREFIX.as_bytes(), game.as_ref()];
    let (pda, bump) = Pubkey::find_program_address(&seeds, &program_id);
    (pda, bump)
}

#[inline(always)]
pub fn transfer_lamports<'a>(
    payer_info: &AccountInfo<'a>,
    to_account_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    lamports: u64,
) -> Result<(), ProgramError> {
    msg!("transfer_lamports() transfer {} lamports", lamports);
    if payer_info.lamports() < lamports {
        msg!("Payer has only {} lamports", payer_info.lamports());
        return Err(ArcaderyError::InsufficientFunds.into());
    }
    invoke(
        &system_instruction::transfer(payer_info.key, to_account_info.key, lamports),
        &[
            payer_info.clone(),
            to_account_info.clone(),
            system_program_info.clone(),
        ],
    )
}

#[inline(always)]
pub fn create_account_owned_by_program<'a>(
    payer_info: &AccountInfo<'a>,
    new_account_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    program_id: &Pubkey,
    signer_seeds: &[&[u8]],
    size: usize,
) -> Result<(), ProgramError> {
    let rent = Rent::get()?;
    let required_lamports = rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account_info.lamports());

    if required_lamports > 0 {
        transfer_lamports(
            payer_info,
            new_account_info,
            system_program_info,
            required_lamports,
        )?;
    }

    msg!("create_account() allocate space");
    invoke_signed(
        &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
        &[new_account_info.clone(), system_program_info.clone()],
        &[signer_seeds],
    )?;

    msg!("create_account() assign to owning program");
    invoke_signed(
        &system_instruction::assign(new_account_info.key, program_id),
        &[new_account_info.clone(), system_program_info.clone()],
        &[signer_seeds],
    )?;

    Ok(())
}
