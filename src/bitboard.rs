use crate::game_result::GameResult;
use crate::game_result::GameResult::Continue;
use crate::player::Player;

/// All win positions for the board encoded in the internal representation
static WIN_POSITIONS: [u16; 8] = [
    // Rows
    0b111,
    0b110001000,
    0b1110000,
    // Columns
    0b11000001,
    0b100100010,
    0b11100,
    // Diagonals
    0b100010001,
    0b101000100,
];

#[derive(Copy, Clone, Debug)]
pub struct Bitboard {
    /// This represents a 3*3 board. Each char represents the state for each player.
    /// <p>
    /// Internal representation: <p>
    ///  0 | 1 | 2 <p>
    ///  --------- <p>
    ///  7 | 8 | 3 <p>
    ///  --------- <p>
    ///  6 | 5 | 4 <p>
    /// <p>
    ///  Human-readable representation: <p>
    ///  0 | 1 | 2 <p>
    ///  --------- <p>
    ///  3 | 4 | 5 <p>
    /// --------- <p>
    /// 6 | 7 | 8
    board: [u16; 2],
    /// The unique id of the board <p>
    /// This is used to offset the move ids for each board
    unique_id: u16,
}

impl Bitboard {
    pub fn new(unique_id: u16) -> Bitboard {
        Bitboard {
            board: [0, 0],
            unique_id,
        }
    }

    /// Returns the possible moves for the current board as an iterator. <p>
    /// The unique id * 9 is used to offset the moves for each board.
    /// # Returns
    /// An iterator of possible moves
    pub fn get_possible_moves(&self) -> impl Iterator<Item = u16> {
        let combined = self.board[0] | self.board[1];
        let id = self.unique_id;
        (0..9)
            .map(move |i| i + 9 * id)
            .filter(move |i| (combined & (1 << i) as u16) == 0)
    }

    /// # <b> FOR INTERNAL USE ONLY!</b> <p>
    /// Set the bit at the given index to the given player <p>
    /// The human is the internal representation of the board
    /// # Arguments
    /// * `index` - The index of the board
    /// * `player` - The player to set the bit to
    pub(crate) fn set_internal(&mut self, index: u8, player: Player) {
        if index > 8 {
            panic!("Index out of bounds");
        }

        self.board[player as usize] |= 1 << index;
    }

    /// Set the bit at the given index to the given player <p>
    /// The index is the human index (0-8)
    /// # Arguments
    /// * `index` - The index of the board
    /// * `player` - The player to set the bit to
    pub fn set(&mut self, index: u8, player: Player) {
        if index > 8 {
            panic!("Index out of bounds");
        }

        let translated_index = Self::from_human_to_bit(index);

        self.board[player as usize] |= 1 << translated_index;
    }

    /// Check if the game has been won
    /// # Returns
    /// The result of the game
    pub fn check_if_won(&self) -> GameResult {
        for i in WIN_POSITIONS.iter() {
            for player in Player::iter() {
                // If the result of the bitwise AND is the same as the input, the player has won
                if (i & self.board[player as usize]) == *i {
                    return GameResult::from(player);
                }
            }
        }

        if self.board[0] | self.board[1] == 0b111111111 {
            return GameResult::Draw;
        }

        Continue
    }

    /// Translates the human index to the index in the internal representation
    fn from_human_to_bit(index: u8) -> u8 {
        match index {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 7,
            4 => 8,
            5 => 3,
            6 => 6,
            7 => 5,
            8 => 4,
            _ => panic!("Index out of bounds"),
        }
    }
}
