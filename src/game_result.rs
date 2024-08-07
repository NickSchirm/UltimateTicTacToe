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
