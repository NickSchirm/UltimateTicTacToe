use crate::ultimate_board::UltimateBoard;

pub const MIN_VALUE: isize = isize::MIN + 1;
pub const MAX_VALUE: isize = isize::MAX - 1;

/// Trait representing a heuristic <p>
/// A heuristic is a function that evaluates a board state
pub trait Heuristic {
    /// Evaluate the given board state <p>
    /// The heuristic should return a value that represents how good the board state is for the player One
    /// # Arguments
    /// * `board` - The board state to evaluate
    /// # Returns
    /// The value of the board state
    fn evaluate(&self, board: UltimateBoard) -> isize;
}
