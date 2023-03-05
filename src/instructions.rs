use borsh::BorshDeserialize;
use shank::ShankInstruction;

#[derive(BorshDeserialize, ShankInstruction)]
#[rustfmt::skip]
pub enum ArcaderyInstruction {
    #[account(name = "player", signer, desc = "The player initializing the game")]
    #[account(name = "game_pda", mut, desc="The game PDA")]
    #[account(name = "system_program", desc="The system program")]
    InitializeGame,
}
