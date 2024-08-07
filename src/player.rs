/// Enum representing the two players
#[derive(Clone, Copy, Debug)]
pub enum Player {
    One = 0,
    Two = 1,
}

impl Player {
    /// Get the opponent of the current player
    /// # Returns
    /// The opponent of the current player
    pub fn get_opponent(&self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }

    /// Get an iterator of all players
    pub fn iter() -> impl Iterator<Item = Player> {
        [Player::One, Player::Two].iter().copied()
    }
}
