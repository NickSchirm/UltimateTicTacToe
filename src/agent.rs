use crate::ultimate_board::UltimateBoard;

/// Trait representing an agent that can play Ultimate Tic Tac Toe
pub trait Agent {
    /// The act method is called to get the agent's move. <p>
    /// The agent should return the index of the field to play on. <p>
    /// The index is the human index (0-80) over all boards. <p>
    /// If the index is out of bounds, the game will panic. <p>
    /// # Arguments
    /// * `board` - The current state of the board
    /// # Returns
    /// The index of the field to play on
    fn act(&mut self, board: UltimateBoard) -> Option<u8>;
    fn reset(&mut self);
}
