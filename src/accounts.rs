use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum GameState {
    Uninitialized,
    Ongoing,
    Finished,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct Game {
    pub player: Pubkey,
    pub state: GameState,
}

#[rustfmt::skip]
pub const GAME_SIZE: usize = 
    /* player       */ 32 + 
    /* state          */ 1;

impl Default for Game {
    fn default() -> Self {
        Self {
            player: Default::default(),
            state: GameState::Uninitialized,
        }
    }
}

impl Game {
    pub fn init(player: Pubkey) -> Game {
        Game {
            player,
            state: GameState::Ongoing,
            ..Default::default()
        }
    }
}
