//! # Contains the [Agent] trait
//! This module contains the Agent trait, which represents an agent that can play Ultimate Tic Tac Toe.

use crate::game::ultimate_board::UltimateBoard;

/// Trait representing an agent that can play Ultimate Tic Tac Toe
pub trait Agent: Send + Sync {
    /// The act method is called to get the agent's move.
    ///
    /// The agent should return the index of the field to play on.
    /// The index is the human index (0-80) over all boards.
    ///
    /// The game will panic if None is returned or if the index is out of bounds.
    /// In case of a panic, relevant information about the state of the game will be printed to the console.
    ///
    /// It is recommended to return None if the agent cannot play or if the agent can not find a move.
    /// # Arguments
    /// * `board` - The current state of the board
    /// # Returns
    /// The index of the field to play on
    fn act(&mut self, board: UltimateBoard) -> Option<u8>;
}
