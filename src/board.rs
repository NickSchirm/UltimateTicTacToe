use std::fmt::Display;
use crate::bitboard::BitBoard;
use crate::game_result::GameResult;
use crate::game_result::GameResult::Continue;
use crate::player::Player;

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

const ROWS: [[u8; 3]; 3] = [
    [0, 1, 2],
    [7, 8, 3],
    [6, 5, 4],
];

#[derive(Copy, Clone, Debug)]
pub struct Board {
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
    board: [BitBoard; 2],
    /// The unique id of the board <p>
    /// This is used to offset the move ids for each board
    unique_id: u8,
}

impl Board {
    pub fn new(unique_id: u8) -> Board {
        Board {
            board: [BitBoard::EMPTY, BitBoard::EMPTY],
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

        self.board[player as usize] |= BitBoard::new(1 << index);
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

        self.board[player as usize] |= BitBoard::new(1 << translated_index);
    }

    pub fn set_unique_id(&mut self, unique_id: u8) {
        self.unique_id = unique_id;
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

#[derive(Copy, Clone, Debug)]
pub enum BoardSymbol {
    X = 1,
    O = 2,
    Empty = 0,
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
}
