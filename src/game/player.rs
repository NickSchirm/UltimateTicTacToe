//! # Contains the [Player] enum
//! The Player enum represents the two players of the game.
//! The enum can be used to determine the opponent of a player.

use serde::{Deserialize, Serialize};

/// Enum representing the two players
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
pub enum Player {
    #[default]
    One = 0,
    Two = 1,
}

impl Player {
    /// Get the opponent of the current player
    /// # Returns
    /// The opponent of the current player
    pub fn get_opponent(&self) -> Self {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }

    /// Get an iterator of all players
    pub fn iter() -> impl Iterator<Item = Self> {
        [Player::One, Player::Two].iter().copied()
    }
}
