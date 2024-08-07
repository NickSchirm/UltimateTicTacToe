//! # Module containing the [GameResult] enum
//! The GameResult enum represents the possible results of a game of Ultimate Tic Tac Toe.
//! The enum can be used to determine the winner of a game, if it's a draw or if the game is still ongoing.

use crate::player::Player;

/// Enum representing the possible game results
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameResult {
    Win(Player),
    Draw,
    Continue,
}

impl From<Player> for GameResult {
    /// Transforms a player into a game result
    /// # Arguments
    /// * `player` - The player to transform
    /// # Returns
    /// The resulting game result
    fn from(player: Player) -> Self {
        GameResult::Win(player)
    }
}
