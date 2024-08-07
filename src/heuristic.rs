//! # Module containing the [Heuristic] trait
//! The Heuristic trait represents a heuristic that evaluates a board state.
//! The heuristic should return a value that represents how good the board state is.

use crate::ultimate_board::UltimateBoard;

pub const MIN_VALUE: isize = isize::MIN + 1;
pub const MAX_VALUE: isize = isize::MAX - 1;

/// # Trait representing a heuristic
///
/// A heuristic is a function that evaluates a board state.
/// The heuristic should return a value that represents how good the board state is.
/// The maximum and minimum values are defined by the constants [MIN_VALUE] and [MAX_VALUE].
/// The heuristic should return a value between these two values.
/// <p>Implementations may provide customization options.</p>
pub trait Heuristic: Send + Sync {
    /// Evaluate the given board state
    ///
    /// The heuristic should return a value that represents how good the board state is.
    /// # Arguments
    /// * `board` - The board state to evaluate
    /// # Returns
    /// The value of the board state, always between [MIN_VALUE] and [MAX_VALUE]
    fn evaluate(&self, board: UltimateBoard) -> isize;
}
