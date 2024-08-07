//! # Contains the [UltimateBoard] struct
//! The UltimateBoard struct represents the board of the [Ultimate Tic Tac Toe game](https://en.wikipedia.org/wiki/Ultimate_tic-tac-toe).
//! The board is represented as a 3x3 array of [Board] structs.
//!
//! The UltimateBoard also contains the status of the game and the status of each board as well as the next board to play on.
//!
//! The board contains a [Zobrist hash](https://www.chessprogramming.org/Zobrist_Hashing) used to store the evaluation of a [Heuristic](crate::heuristic::Heuristic) in a [Transposition table](https://www.chessprogramming.org/Transposition_Table).

use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::game::board::{Board, BoardSymbol};
use crate::game::game_result::GameResult;
use crate::game::game_result::GameResult::Continue;
use crate::game::player::Player;

/// All possible win positions for the ultimate board
const WIN_POSITIONS: [[u8; 3]; 8] = [
    // Rows
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    // Columns
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    // Diagonals
    [0, 4, 8],
    [2, 4, 6],
];

/// All possible partial win positions for the ultimate board
const PARTIAL_WIN_POSITIONS: [[u8; 2]; 24] = [
    // ROW 1
    [0, 1],
    [0, 2],
    [1, 2],
    // ROW 2
    [3, 4],
    [3, 5],
    [4, 5],
    // ROW 3
    [6, 7],
    [6, 8],
    [7, 8],
    // COLUMN 1
    [0, 3],
    [0, 6],
    [3, 6],
    // COLUMN 2
    [1, 4],
    [1, 7],
    [4, 7],
    // COLUMN 3
    [2, 5],
    [2, 8],
    [5, 8],
    // DIAGONAL 1
    [0, 4],
    [0, 8],
    [4, 8],
    // DIAGONAL 2
    [2, 4],
    [2, 6],
    [4, 6],
];

/// Number of squares in Ultimate Tic Tac Toe
const NUM_POSITIONS: usize = 9 * 9;

/// Number of Zobrist values needed for the ultimate board
const NUM_ZOBRIST_VALUES: usize = NUM_POSITIONS * 2 + 9;

/// Values used for [Zobrist hashing](https://www.chessprogramming.org/Zobrist_Hashing)
///
/// The values are generated using a [ChaCha20Rng] PRNG and are lazily initialized.
static ZOBRIST_VALUES: Lazy<[u64; NUM_ZOBRIST_VALUES]> = Lazy::new(|| {
    let mut values = [0; NUM_ZOBRIST_VALUES];

    // PRNG for generating the zobrist values
    let mut prng = ChaCha20Rng::from_seed([0; 32]);

    for value in values.iter_mut().take(NUM_ZOBRIST_VALUES) {
        *value = prng.next_u64();
    }

    values
});

#[allow(rustdoc::private_intra_doc_links)]
/// Offset of the `next_board_index` hashes in [Zobrist values](ZOBRIST_VALUES)
pub const ZOBRIST_VALUES_NEXT_BOARD_INDEX_OFFSET: usize = NUM_POSITIONS * 2;

/// The indices of the corners of a [UltimateBoard]
pub const CORNER_INDICES: [usize; 4] = [0, 2, 6, 8];

/// The indices of the edges of a [UltimateBoard]
pub const EDGE_INDICES: [usize; 4] = [1, 3, 5, 7];

/// The index of the center of a [UltimateBoard]
pub const CENTER_INDEX: usize = 4;

/// Struct representing the ultimate board
///
/// The ultimate board is a 3x3 board of 3x3 boards.
/// # Fields
/// * `board` - The 3x3 board of 3x3 boards
/// * `next_board` - The index of the next board to play on
/// * `board_status` - The status of each board
/// * `game_status` - The status of the game
#[derive(Copy, Clone, Debug)]
pub struct UltimateBoard {
    boards: [Board; 9],
    board_status: [GameResult; 9],
    next_board_index: Option<u8>,
    game_status: GameResult,
    current_player: Player,
    hash: u64,
}

impl Default for UltimateBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl UltimateBoard {
    /// Create a new ultimate board using the default values
    /// # Returns
    /// A new ultimate board
    pub fn new() -> Self {
        let mut boards = [Board::new(0); 9];

        for (i, board) in boards.iter_mut().enumerate() {
            board.set_unique_id(i as u8)
        }

        UltimateBoard {
            boards,
            next_board_index: None,
            board_status: [Continue; 9],
            game_status: Continue,
            current_player: Player::One,
            hash: 0,
        }
    }

    /// Checks if the game has been won
    ///
    /// The field `self.game_status` is updated with the result of the game.
    fn check_if_won(&mut self) {
        // Check if the game has been won by a player
        for a in WIN_POSITIONS.iter() {
            for player in Player::iter() {
                if a.iter()
                    .all(|&i| self.board_status[i as usize] == GameResult::Win(player))
                {
                    self.game_status = GameResult::Win(player);
                    return;
                }
            }
        }

        // Check if the game has been drawn
        if self
            .board_status
            .iter()
            .all(|&status| status != GameResult::Continue)
        {
            self.game_status = GameResult::Draw;
            return;
        }

        self.game_status = GameResult::Continue;
    }

    /// Get the status of the game as a [GameResult]
    /// # Returns
    /// The status of the game
    pub fn get_game_status(&self) -> GameResult {
        self.game_status
    }

    /// Get the status of the boards that make up the ultimate board as [GameResults](GameResult)
    /// # Returns
    /// The status of the boards
    pub fn get_board_status(&self) -> [GameResult; 9] {
        self.board_status
    }

    /// Get the boards that make up the ultimate board
    /// # Returns
    /// The boards
    pub fn get_boards(&self) -> [Board; 9] {
        self.boards
    }

    /// Get the current player
    /// # Returns
    /// The current player
    pub fn get_current_player(&self) -> Player {
        self.current_player
    }

    /// Get the Zobrist hash of the board
    /// # Returns
    /// The Zobrist hash of the board
    pub fn get_hash(&self) -> u64 {
        self.hash
    }

    /// Get the index of the next board to play on. If the next player can play on any board, the index is None
    /// # Returns
    /// The index of the next board to play on
    pub fn get_next_board_index(&self) -> Option<u8> {
        self.next_board_index
    }

    /// Get the partial win difference for a player
    /// # Arguments
    /// * `player` - The player to get the partial win difference for
    /// # Returns
    /// The partial win difference for the player
    pub fn get_partial_wins_difference(&self, player: Player) -> i8 {
        let mut diff = 0;

        for partial_win in PARTIAL_WIN_POSITIONS.iter() {
            let mut player_count = 0;
            let mut opponent_count = 0;

            for &index in partial_win.iter() {
                match self.board_status[index as usize] {
                    GameResult::Win(p) if p == player => player_count += 1,
                    GameResult::Win(_) => opponent_count += 1,
                    _ => {}
                }
            }

            if player_count > 0 && opponent_count == 0 {
                diff += 1;
            } else if opponent_count > 0 && player_count == 0 {
                diff -= 1;
            }
        }

        diff
    }

    /// Get the possible moves for the ultimate board
    /// # Returns
    /// An iterator of the possible moves
    pub fn get_possible_moves(&self) -> impl Iterator<Item = u8> {
        match self.next_board_index {
            Some(index) => {
                BoardIterator::SingleBoard(self.boards[index as usize].get_possible_moves())
            }
            None => BoardIterator::MultiBoard(
                self.boards
                    .into_iter()
                    .zip(self.board_status.clone().into_iter())
                    .filter(|(_, status)| *status == Continue)
                    .flat_map(|(board, _)| board.get_possible_moves()),
            ),
        }
    }

    /// Make a move on the ultimate board
    /// # Arguments
    /// * `index` - The index of the field to play on
    pub fn make_move(&mut self, index: u8) {
        // No further moves can be made if the game is over
        if self.game_status != Continue {
            panic!("Game is over");
        }

        // The board index is the index of the board the move is made on
        let board_index = index / 9;

        // The next board index must be the same as the board index if it is not None
        if let Some(next_board_index) = self.next_board_index {
            if next_board_index != board_index {
                panic!("Invalid move");
            }
        }

        let board = &mut self.boards[board_index as usize];

        // The field index is the index of the field on the board
        let field_index = index % 9;

        board.set(field_index, self.current_player);
        // Apply the zobrist hash for the specific square and player
        self.hash ^= ZOBRIST_VALUES[(index * 2 + self.current_player as u8) as usize];

        // Update the status of the board
        self.board_status[board_index as usize] = board.check_if_won();

        // Update the status of the game
        self.check_if_won();

        // Update the current player
        self.current_player = self.current_player.get_opponent();

        if let Some(next_board_index) = self.next_board_index {
            // Remove the zobrist hash for the previously set next_board_index
            self.hash ^=
                ZOBRIST_VALUES[next_board_index as usize + ZOBRIST_VALUES_NEXT_BOARD_INDEX_OFFSET];
        }

        // Update the next_board_index
        // If the board is can't be continued, the next board index is None
        self.next_board_index = match self.board_status[field_index as usize] {
            Continue => {
                // Apply the zobrist hash for the current next board index
                self.hash ^=
                    ZOBRIST_VALUES[field_index as usize + ZOBRIST_VALUES_NEXT_BOARD_INDEX_OFFSET];
                Some(field_index)
            }
            _ => None,
        };
    }
}

impl Display for UltimateBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for sub_row in 0..9 {
            for i in (sub_row - (sub_row % 3))..(sub_row - (sub_row % 3) + 3) {
                let row = &self.boards[i as usize].extract_row(sub_row % 3);

                for item in row.iter().take(3) {
                    f.write_str(match item {
                        BoardSymbol::X => "X ",
                        BoardSymbol::O => "O ",
                        BoardSymbol::Empty => "  ",
                    })?;
                }

                if i % 3 != 2 {
                    f.write_str("| ")?;
                }
            }

            if sub_row == 2 || sub_row == 5 {
                f.write_str("\n- - - + - - - + - - - \n")?;
            } else {
                f.write_str("\n")?;
            }
        }

        f.write_fmt(format_args!("Game status: {:?}\n", self.game_status))?;
        f.write_fmt(format_args!("Board status: {:?}\n", self.board_status))?;
        f.write_fmt(format_args!(
            "Next board index: {:?}\n",
            self.next_board_index
        ))?;
        f.write_fmt(format_args!("Current player: {:?}\n", self.current_player))?;

        Ok(())
    }
}

impl PartialEq<Self> for UltimateBoard {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for UltimateBoard {}

/// Enum representing the possible iterators for the board
///
/// The Enum can either contain the possible moves for a single board or for multiple boards.
pub enum BoardIterator<I, J> {
    /// The possible moves for a single board
    SingleBoard(I),
    /// The possible moves for multiple boards
    MultiBoard(J),
}

impl<I, J> Iterator for BoardIterator<I, J>
where
    I: Iterator<Item = u8>,
    J: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            BoardIterator::SingleBoard(iter) => iter.next(),
            BoardIterator::MultiBoard(iter) => iter.next(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iterator() {
        let mut board = UltimateBoard::new();

        assert_eq!(board.get_hash(), 0);

        board.make_move(0);

        assert_eq!(board.get_hash(), ZOBRIST_VALUES[0] ^ ZOBRIST_VALUES[162]);

        board.make_move(1);

        assert_eq!(
            board.get_hash(),
            ZOBRIST_VALUES[0] ^ ZOBRIST_VALUES[3] ^ ZOBRIST_VALUES[163]
        );
    }
}
