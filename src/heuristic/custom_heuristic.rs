//! # Contains the [CustomHeuristic] and [CustomMiniBoardHeuristic] struct
//! The CustomHeuristic struct represents a heuristic that uses a custom evaluation function.
//! The heuristic is used by the [MiniMaxAgent](crate::agent::minimax_agent::MiniMaxAgent) to evaluate the best move.

use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::game::board::Board;
use crate::game::game_result::GameResult;
use crate::game::player::Player;
use crate::game::ultimate_board::UltimateBoard;
use crate::heuristic::{Heuristic, MiniBoardHeuristic, MAX_VALUE, MIN_VALUE};

/// # Contains the evaluation of all legal [boards](Board) for the [CustomMiniBoardHeuristic].
///
/// The evaluation is calculated from the perspective of [Player::One].
static SMALL_BOARD_LOOKUP_TABLE: Lazy<HashMap<u32, f64>> =
    Lazy::new(|| CustomMiniBoardHeuristic.initialize());

/// A [Heuristic] that uses a custom evaluation function to evaluate the best move.
/// # Fields
/// * `player` - The [Player] for which the heuristic should evaluate the best move.
#[derive(Clone)]
pub struct CustomHeuristic {
    player: Player,
}

impl CustomHeuristic {
    pub fn new(player: Player) -> Self {
        let _ = SMALL_BOARD_LOOKUP_TABLE.get(&0).unwrap();
        CustomHeuristic { player }
    }
}

impl Heuristic for CustomHeuristic {
    fn evaluate(&self, board: UltimateBoard) -> f64 {
        let mut value = 0.;

        if board.get_game_status() == GameResult::Win(self.player) {
            return *MAX_VALUE;
        }

        if board.get_game_status() == GameResult::Win(self.player.get_opponent()) {
            return *MIN_VALUE;
        }

        // Reward having more positions set on small boards than the opponent
        for small_board in board.get_boards() {
            value += *SMALL_BOARD_LOOKUP_TABLE.get(&small_board.to_key()).unwrap()
                * (if self.player == Player::One { 1 } else { -1 }) as f64;
        }

        // Reward controlLing the center of the board
        if board.get_board_status()[4] == GameResult::Win(self.player) {
            value += 10.;
        }

        // Reward having more small boards won than the opponent
        for board_status in board.get_board_status() {
            if let GameResult::Win(winner) = board_status {
                if winner == self.player {
                    value += 10.;
                } else {
                    value -= 10.;
                }
            }
        }

        value
    }
}

/// A [MiniBoardHeuristic] that uses a custom evaluation function to evaluate [boards](Board).
pub struct CustomMiniBoardHeuristic;

impl MiniBoardHeuristic for CustomMiniBoardHeuristic {
    fn evaluate(&self, board: Board) -> f64 {
        let mut value = 0.;

        let positions_set_difference = board.get_positions_set_difference(Player::One) as f64;
        if positions_set_difference > 0. {
            value += positions_set_difference;
        }

        let partial_wins_difference = board.get_partial_wins_difference(Player::One) as f64;

        value += partial_wins_difference * 2.;

        value
    }
}
