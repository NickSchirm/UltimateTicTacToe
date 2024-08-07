//! # Contains the [Board] and [LegalBoardIterator] struct
//! The Board struct represents the board of the Tic Tac Toe game.
//! The board is represented as 2 [BitBoard] structs.
//! Each BitBoard represents the state of the board for one player.
//!
//! Nine boards are used in the [UltimateBoard](crate::ultimate_board::UltimateBoard) struct to represent the state of the game.
//!
//! The nine squares of the board are represented internally as follows:
//! ```text
//! 0 | 1 | 2
//! --+---+--
//! 7 | 8 | 3
//! --+---+--
//! 6 | 5 | 4
//! ```
//!
//! The human-readable representation is:
//! ```text
//! 0 | 1 | 2
//! --+---+--
//! 3 | 4 | 5
//! --+---+--
//! 6 | 7 | 8
//! ```
//!
//! The [LegalBoardIterator] struct is an iterator over all possible legal boards.

use crate::bitboard::BitBoard;
use crate::game_result::GameResult;
use crate::game_result::GameResult::Continue;
use crate::player::Player;
use std::fmt::Display;

/// All win positions for the board encoded in the internal representation
const WIN_POSITIONS: [u16; 8] = [
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

/// All partial win positions for the board encoded in the internal representation
const PARTIAL_WIN_POSITIONS: [u16; 24] = [
    // Row 1
    0b011,
    0b101,
    0b110,
    // Row 2
    0b110000000,
    0b10001000,
    0b110000000,
    // Row 3
    0b1100000,
    0b110000,
    0b1010000,
    // Column 1
    0b10000001,
    0b1000001,
    0b11000000,
    // Column 2
    0b100000010,
    0b100100000,
    0b100010,
    // Column 3
    0b1100,
    0b11000,
    0b10100,
    //Diagonal 1
    0b100010000,
    0b10001,
    0b100000001,
    //Diagonal 2
    0b101000000,
    0b1000001,
    0b100000001,
];

/// Rows of the board in the internal representation
const ROWS: [[u8; 3]; 3] = [[0, 1, 2], [7, 8, 3], [6, 5, 4]];

/// Implementation of a 3x3 board for Tic Tac Toe
#[derive(Copy, Clone, Debug)]
#[allow(rustdoc::invalid_html_tags)]
pub struct Board {
    /// This represents a 3*3 board. Each char represents the state for each player.
    ///
    /// Internal representation:
    ///  ```text
    /// 0 | 1 | 2
    /// --+---+--
    /// 7 | 8 | 3
    /// --+---+--
    /// 6 | 5 | 4
    /// ```
    ///  Human-readable representation:
    ///  ```text
    /// 0 | 1 | 2
    /// --+---+--
    /// 3 | 4 | 5
    /// --+---+--
    /// 6 | 7 | 8
    /// ```
    board: [BitBoard; 2],
    /// The unique id of the board
    ///
    /// The id is used to offset the move ids for each board
    unique_id: u8,
}

impl Board {
    pub fn new(unique_id: u8) -> Self {
        Board {
            board: [BitBoard::EMPTY, BitBoard::EMPTY],
            unique_id,
        }
    }

    pub fn from_bitboards(bitboards: [BitBoard; 2], unique_id: u8) -> Self {
        Board {
            board: bitboards,
            unique_id,
        }
    }

    /// Get the possible moves for the board
    /// # Returns
    /// An iterator of the possible moves
    pub fn get_possible_moves(&self) -> impl Iterator<Item = u8> {
        let empty_squares = !(self.board[0] | self.board[1]);
        let id = self.unique_id;
        empty_squares
            .into_iter()
            .map(move |i| Self::from_bit_to_human(i) + 9 * id)
    }

    /// # <b> FOR INTERNAL USE ONLY!</b>
    ///
    /// Set the bit at the given index to the given player
    /// # Arguments
    /// * `index` - The index of the board
    /// * `player` - The player to set the bit to
    #[allow(dead_code)]
    pub(crate) fn set_internal(&mut self, index: u8, player: Player) {
        if index > 8 {
            panic!("Index out of bounds");
        }

        self.board[player as usize] |= BitBoard::new(1 << index);
    }

    /// Set the bit at the given index to the given player
    ///
    /// The index is the human index (0-8)
    /// # Arguments
    /// * `index` - The index of the board
    /// * `player` - The player to set the bit to
    pub fn set(&mut self, index: u8, player: Player) {
        if index > 8 {
            panic!("Index out of bounds");
        }

        let translated_index = Self::from_human_to_bit(index);

        self.board[player as usize] |= BitBoard::new(1 << translated_index);
    }

    /// Set the unique id of the board.
    /// # Arguments
    /// * `unique_id` - The unique id to set
    pub fn set_unique_id(&mut self, unique_id: u8) {
        self.unique_id = unique_id;
    }

    /// Get the key of the board
    /// # Returns
    /// The key of the board
    pub fn to_key(&self) -> u32 {
        let first: u32 = self.board[0].into();
        let second: u32 = self.board[1].into();
        first | (second << 9)
    }

    /// Check if the game has been won
    /// # Returns
    /// The result of the game
    pub fn check_if_won(&self) -> GameResult {
        // Check if the game has been won by a player
        for i in WIN_POSITIONS.iter() {
            for player in Player::iter() {
                // If the result of the bitwise AND is the same as the input, the player has won
                if (BitBoard::new(*i) & self.board[player as usize]) == BitBoard::new(*i) {
                    return GameResult::from(player);
                }
            }
        }

        // Check if the game has been drawn
        if self.board[0] | self.board[1] == BitBoard::new(0b111111111) {
            return GameResult::Draw;
        }

        Continue
    }

    /// Get the positions set difference between the two players
    /// # Arguments
    /// * `player` - The player to get the difference for
    /// # Returns
    /// The difference between the two players
    pub fn get_positions_set_difference(&self, player: Player) -> i8 {
        let mut diff = 0;

        for _ in self.board[player as usize].into_iter() {
            diff += 1;
        }

        for _ in self.board[(player as usize + 1) % 2].into_iter() {
            diff -= 1;
        }

        diff
    }

    /// Get the partial wins difference between the two players
    /// # Arguments
    /// * `player` - The player to get the difference for
    /// # Returns
    /// The difference between the two players
    pub fn get_partial_wins_difference(&self, player: Player) -> i8 {
        let mut diff = 0;

        for i in PARTIAL_WIN_POSITIONS.iter() {
            let bit = BitBoard::new(*i);

            let player_bit = self.board[player as usize] & bit;
            let opponent_bit = self.board[(player as usize + 1) % 2] & bit;

            if player_bit == bit && opponent_bit == BitBoard::EMPTY {
                diff += 1;
            } else if opponent_bit == bit && player_bit == BitBoard::EMPTY {
                diff -= 1;
            }
        }

        diff
    }

    /// Check if the center square is occupied by a player
    /// # Arguments
    /// * `player` - The player to check for
    /// # Returns
    /// 1 if the center is occupied by the player, -1 if it is occupied by the opponent, 0 otherwise
    pub fn center_occupied(&self, player: Player) -> i8 {
        let center = BitBoard::new(0b100000000);
        let player_center = center & self.board[player as usize];
        let opponent_center = center & self.board[player.get_opponent() as usize];

        if player_center != BitBoard::EMPTY {
            return 1;
        } else if opponent_center != BitBoard::EMPTY {
            return -1;
        }

        0
    }

    /// Get the corners difference between the two players
    /// # Arguments
    /// * `player` - The player to get the difference for
    /// # Returns
    /// The difference between the two players
    pub fn get_corners_difference(&self, player: Player) -> i8 {
        let mut diff = 0;

        let corners = BitBoard::new(0b1010101);

        let player_corners = corners & self.board[player as usize];
        let opponent_corners = corners & self.board[player.get_opponent() as usize];

        diff += player_corners.into_iter().count() as i8;

        diff -= opponent_corners.into_iter().count() as i8;

        diff
    }

    /// Get the edges difference between the two players
    /// # Arguments
    /// * `player` - The player to get the difference for
    /// # Returns
    /// The difference between the two players
    pub fn get_edges_difference(&self, player: Player) -> i8 {
        let mut diff = 0;

        let edges = BitBoard::new(0b10101010);

        let player_edges = edges & self.board[player as usize];
        let opponent_edges = edges & self.board[player.get_opponent() as usize];

        diff += player_edges.into_iter().count() as i8;

        diff -= opponent_edges.into_iter().count() as i8;

        diff
    }

    /// Translates the human index to the index in the internal representation
    pub fn from_human_to_bit(index: u8) -> u8 {
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

    /// Translates the index in the internal representation to the human index
    /// # Arguments
    /// * `index` - The index to translate
    /// # Returns
    /// The translated index
    pub fn from_bit_to_human(index: u8) -> u8 {
        match index {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 5,
            4 => 8,
            5 => 7,
            6 => 6,
            7 => 3,
            8 => 4,
            _ => panic!("Index out of bounds"),
        }
    }

    /// Extracts a row from the board
    /// # Arguments
    /// * `row` - The row to extract
    /// # Returns
    /// The extracted row
    pub fn extract_row(&self, row: u8) -> Vec<BoardSymbol> {
        let mut res = vec![];
        for i in ROWS[row as usize].iter() {
            let bit = 1 << i;

            if self.board[0] & BitBoard::new(bit) != BitBoard::EMPTY {
                res.push(BoardSymbol::X);
            } else if self.board[1] & BitBoard::new(bit) != BitBoard::EMPTY {
                res.push(BoardSymbol::O);
            } else {
                res.push(BoardSymbol::Empty);
            }
        }
        res
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for a in ROWS.iter() {
            for i in a.iter() {
                let bit = 1 << i;

                if self.board[0] & BitBoard::new(bit) != BitBoard::EMPTY {
                    f.write_str("X ")?;
                } else if self.board[1] & BitBoard::new(bit) != BitBoard::EMPTY {
                    f.write_str("O ")?;
                } else {
                    f.write_str("  ")?;
                }
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

/// # Enum representing the symbols on the board
#[derive(Copy, Clone, Debug)]
pub enum BoardSymbol {
    /// [Player::One]
    X = 1,
    /// [Player::Two]
    O = 2,
    /// Empty square
    Empty = 0,
}

impl From<Player> for BoardSymbol {
    fn from(player: Player) -> Self {
        match player {
            Player::One => BoardSymbol::X,
            Player::Two => BoardSymbol::O,
        }
    }
}

/// Iterator over all possible legal boards
///
/// A legal board is a board where no square is set for both players
#[derive(Default)]
pub struct LegalBoardIterator {
    index: u32,
}

impl Iterator for LegalBoardIterator {
    type Item = (u16, u16);

    fn next(&mut self) -> Option<Self::Item> {
        // Iterate over all possible, legal and illegal, boards
        while self.index < u32::pow(2, 18) {
            let first = self.index as u16 & 0b111111111;
            let second = (self.index >> 9) as u16;

            self.index += 1;

            if first & second == 0 {
                return Some((first, second));
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set() {
        let mut board = Board::new(0);
        board.set(0, Player::One);
        assert_eq!(board.board[0], BitBoard::new(1));
        board.set(1, Player::Two);
        assert_eq!(board.board[1], BitBoard::new(2));
    }

    #[test]
    fn test_legal_board_iterator() {
        let iter = LegalBoardIterator::default();
        let count = iter.count();
        assert_eq!(count, usize::pow(3, 9));
    }
}
