use crate::board::{Board, BoardSymbol};
use crate::game_result::GameResult;
use crate::game_result::GameResult::Continue;
use crate::player::Player;
use std::fmt;
use std::fmt::Display;

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

/// Struct representing the ultimate board <p>
/// The ultimate board is a 3x3 board of 3x3 boards.
/// # Fields
/// * `board` - The 3x3 board of 3x3 boards
/// * `next_board` - The index of the next board to play on
/// * `board_status` - The status of each board
/// * `game_status` - The status of the game
#[derive(Copy, Clone, Debug)]
pub struct UltimateBoard {
    boards: [Board; 9],
    next_board_index: Option<u8>,
    board_status: [GameResult; 9],
    game_status: GameResult,
    current_player: Player,
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
    pub fn new() -> UltimateBoard {
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
        }
    }

    /// Checks if the game has been won
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

    pub fn get_game_status(&self) -> GameResult {
        self.game_status
    }

    pub fn get_board_status(&self) -> [GameResult; 9] {
        self.board_status
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
                    .flat_map(|board| board.get_possible_moves()),
            ),
        }
    }

    /// Make a move on the ultimate board <p>
    /// # Arguments
    /// * `index` - The index of the field to play on
    pub fn make_move(&mut self, index: u8) -> bool {
        // No further moves can be made if the game is over
        if self.game_status != Continue {
            eprintln!("Game is over");
            return false;
        }

        // The board index is the index of the board the move is made on
        let board_index = index / 9;

        // The next board index must be the same as the board index if it is not None
        if let Some(next_board_index) = self.next_board_index {
            if next_board_index != board_index {
                eprintln!("Invalid move");
                return false;
            }
        }

        let board = &mut self.boards[board_index as usize];

        // The field index is the index of the field on the board
        let field_index = index % 9;

        board.set(field_index, self.current_player);

        // Update the status of the board
        self.board_status[board_index as usize] = board.check_if_won();

        // Update the status of the game
        self.check_if_won();

        // Update the current player
        self.current_player = self.current_player.get_opponent();

        // Update the next_board_index
        // If the board is can't be continued, the next board index is None
        self.next_board_index = match self.board_status[field_index as usize] {
            Continue => Some(field_index),
            _ => None,
        };

        true
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

/// Enum representing the possible iterators for the board <p>
/// The Enum can either contain the possible moves for a single board or for multiple boards.
pub enum BoardIterator<I, J> {
    SingleBoard(I),
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
