use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, ShankInstruction)]
#[rustfmt::skip]
pub enum ArcaderyInstruction {
    #[account(name = "player", signer, desc = "The player initializing the game")]
    #[account(name = "game_pda", mut, desc="The game PDA")]
    #[account(name = "system_program", desc="The system program")]
    InitializeGame(InitializeGameArgs),
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct InitializeGameArgs {
    pub game: Pubkey,
}
