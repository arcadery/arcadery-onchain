use borsh::BorshDeserialize;

#[derive(BorshDeserialize)]
pub enum ArcaderyInstruction {
    InitializeGame,
}
