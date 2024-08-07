//! # Contains the [Heuristic] and [MiniBoardHeuristic] trait
//! The Heuristic trait represents a heuristic that evaluates a [UltimateBoard] state.
//! The heuristic should return a value that represents how good the board state is.
//!
//! The MiniBoardHeuristic trait represents a heuristic that evaluates a [Board] state.
//! The heuristic should return a value that represents how good the board state is.

use crate::bitboard::BitBoard;
use crate::board::{Board, LegalBoardIterator};
use crate::ultimate_board::UltimateBoard;
use std::collections::HashMap;

/// The minimum value a heuristic can return
pub const MIN_VALUE: i32 = i32::MIN + 1;
/// The maximum value a heuristic can return
pub const MAX_VALUE: i32 = i32::MAX - 1;

/// # Trait representing a heuristic
///
/// A heuristic is a function that evaluates a [UltimateBoard] state.
/// The heuristic should return a value that represents how good the board state is for a specified player.
/// The maximum and minimum values are defined by the constants [MIN_VALUE] and [MAX_VALUE].
/// The heuristic should return a value between these two values.
/// <p>Implementations may provide customization options.</p>
pub trait Heuristic: Clone + Send + Sync {
    /// Evaluate the given board state
    ///
    /// The heuristic should return a value that represents how good the board state is.
    /// # Arguments
    /// * `board` - The board state to evaluate
    /// # Returns
    /// The value of the board state, always between [MIN_VALUE] and [MAX_VALUE]
    fn evaluate(&self, board: UltimateBoard) -> i32;
}

/// The number of possible legal small board states
pub const NUM_SMALL_BOARD_STATES: usize = usize::pow(3, 9);

/// # Trait representing a heuristic for small boards
///
/// A MiniBoardHeuristic is a heuristic that evaluates a [Board] state.
/// The heuristic should return a value that represents how good the board state is.
/// The maximum and minimum values are defined by the constants [MIN_VALUE] and [MAX_VALUE].
///
/// <div class="warning">
///
/// The return value of [MiniBoardHeuristic::evaluate] must be calculated from the perspective of [Player::One](crate::player::Player::One).
///
/// </div>
///
/// Implementations may provide customization options.
pub trait MiniBoardHeuristic: Send + Sync {
    /// Evaluate the given small board state
    ///
    /// The result of this function is saved in the cache and is available in [Heuristic::evaluate].
    ///
    /// <div class="warning">
    ///
    /// The return value must be calculated from the perspective of [Player::One](crate::player::Player::One).
    ///
    /// </div>
    ///
    /// # Arguments
    /// * `board` - The small board state to evaluate
    /// # Returns
    /// The value of the small board state, always between [MIN_VALUE] and [MAX_VALUE]
    fn evaluate(&self, board: Board) -> i32;

    /// Initialize the cache for the heuristic
    ///
    /// The cache is a lookup table that contains the evaluation of all possible small board states.
    /// The cache is used to speed up the evaluation of the heuristic.
    /// # Returns
    /// The cache for the heuristic
    fn initialize(&self) -> HashMap<u32, i32> {
        let mut cache = HashMap::with_capacity(NUM_SMALL_BOARD_STATES);

        for (first, second) in LegalBoardIterator::default() {
            let board = Board::from_bitboards([BitBoard::new(first), BitBoard::new(second)], 0);

            let index = first as u32 | (second as u32) << 9;

            cache.insert(index, self.evaluate(board));
        }

        cache
    }
}
