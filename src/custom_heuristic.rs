//! # Module containing the [CustomHeuristic] struct
//! The CustomHeuristic struct represents a heuristic that uses a custom evaluation function.
//! The heuristic is used by the [MiniMaxAgent](crate::minimax_agent::MiniMaxAgent) to evaluate the best move.

use crate::game_result::GameResult;
use crate::heuristic::{Heuristic, MAX_VALUE, MIN_VALUE};
use crate::player::Player;
use crate::ultimate_board::UltimateBoard;

/// A [Heuristic] that uses a custom evaluation function to evaluate the best move
/// # Fields
/// * `player` - The [Player] for which the heuristic should evaluate the best move
pub struct CustomHeuristic {
    player: Player,
}

impl CustomHeuristic {
    pub fn new(player: Player) -> CustomHeuristic {
        CustomHeuristic { player }
    }
}

impl Heuristic for CustomHeuristic {
    fn evaluate(&self, board: UltimateBoard) -> isize {
        let mut value = 0;

        if board.get_game_status() == GameResult::Win(self.player) {
            return MAX_VALUE;
        }

        if board.get_game_status() == GameResult::Win(self.player.get_opponent()) {
            return MIN_VALUE;
        }

        // Reward having more positions set on small boards than the opponent
        for board in board.get_boards() {
            let positions_set_difference = board.get_positions_set_difference(self.player) as isize;
            if positions_set_difference > 0 {
                value += positions_set_difference;
            }
        }

        // Reward controlLing the center of the board
        if board.get_board_status()[4] == GameResult::Win(self.player) {
            value += 10;
        }

        // Reward having more small boards won than the opponent
        for board_status in board.get_board_status() {
            if let GameResult::Win(winner) = board_status {
                if winner == self.player {
                    value += 10;
                } else {
                    value -= 10;
                }
            }
        }

        value
    }
}
