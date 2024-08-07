//! # Contains the [ParameterizedHeuristic] and [ParameterizedMiniBoardHeuristic] struct
//! The [ParameterizedHeuristic] struct represents a [Heuristic] that uses weights for the features to evaluate the best move.
//!
//! The weights may be optimized using [GeneticAlgorithm](crate::genetic_algorithm::GeneticAlgorithm).
//!
//! The features of the heuristic are described in the [ParameterizedHeuristic::values] field.
//!
//! The heuristic uses a [ParameterizedMiniBoardHeuristic] to evaluate small boards.
//!
//! The [ParameterizedMiniBoardHeuristic] struct represents a [MiniBoardHeuristic] that uses weights for the features to evaluate small boards.

use crate::game::board::Board;
use crate::game::game_result::GameResult;
use crate::game::player::Player;
use crate::game::ultimate_board::{UltimateBoard, CENTER_INDEX, CORNER_INDICES, EDGE_INDICES};
use crate::heuristic::{Heuristic, MiniBoardHeuristic, MAX_VALUE, MIN_VALUE};

/// The number of features the heuristic uses
pub const NUM_FEATURES: usize = 13;

#[allow(rustdoc::private_intra_doc_links)]
/// # Struct representing a [Heuristic] that uses weights for the features to evaluate the best move
///
/// The weights may be optimized using [GeneticAlgorithm](crate::genetic_algorithm::GeneticAlgorithm).
///
/// The features of the heuristic are described in the [ParameterizedHeuristic::values] field.
///
/// The heuristic uses a [ParameterizedMiniBoardHeuristic] to evaluate small boards.
#[derive(Clone, Debug)]
pub struct ParameterizedHeuristic {
    /// The [player](Player) for which the heuristic should evaluate the best move
    player: Player,
    /// The weights for the features
    ///
    /// The features are:
    /// 1. Number of small boards won
    /// 2. Number of small boards lost
    /// 3. Number of small boards drawn
    /// 4. Partial wins difference on a small board
    /// 5. Positions set difference on a small board
    /// 6. Whether the center of a small board is occupied by the current player
    /// 7. Number of corners of a small board occupied by the current player
    /// 8. Number of edges of a small board occupied by the current player
    /// 9. Whether the center of the entire board is won
    /// 10. Number of corners of the entire board won
    /// 11. Number of edges of the entire board won
    /// 12. Number of partial wins difference on the entire board
    /// 13. Whether the current player can freely choose a small board
    pub values: Vec<f64>,
    //small_board_lookup_table: HashMap<u32, f64>,
}

impl ParameterizedHeuristic {
    pub fn new(player: Player, values: Vec<f64>) -> Self {
        ParameterizedHeuristic {
            player,
            values: values.clone(),
            //small_board_lookup_table: ParameterizedMiniBoardHeuristic::new(values).initialize(),
        }
    }
}

impl Heuristic for ParameterizedHeuristic {
    fn evaluate(&self, board: UltimateBoard) -> f64 {
        let mut value = 0.;

        if board.get_game_status() == GameResult::Win(self.player) {
            return *MAX_VALUE;
        }

        if board.get_game_status() == GameResult::Win(self.player.get_opponent()) {
            return *MIN_VALUE;
        }

        let mini_heuristic = ParameterizedMiniBoardHeuristic::new(self.values.clone());

        for small_board in board.get_boards() {
            // value += *self
            //     .small_board_lookup_table
            //     .get(&small_board.to_key())
            //     .unwrap()
            //     * (if self.player == Player::One { 1 } else { -1 }) as f64;
            value += mini_heuristic.evaluate(small_board)
                * (if self.player == Player::One { 1 } else { -1 }) as f64;
        }

        for board_status in board.get_board_status() {
            if let GameResult::Win(winner) = board_status {
                if winner == self.player {
                    value += self.values[0];
                } else {
                    value += self.values[1];
                }
            } else {
                value += self.values[2];
            }
        }

        value += if board.get_board_status()[CENTER_INDEX] == GameResult::Win(self.player) {
            self.values[8]
        } else {
            -self.values[8]
        };

        for corner_index in CORNER_INDICES.iter() {
            value += if board.get_board_status()[*corner_index] == GameResult::Win(self.player) {
                self.values[9]
            } else {
                -self.values[9]
            };
        }

        for edge_index in EDGE_INDICES.iter() {
            value += if board.get_board_status()[*edge_index] == GameResult::Win(self.player) {
                self.values[10]
            } else {
                -self.values[10]
            };
        }

        value += board.get_partial_wins_difference(self.player) as f64 * self.values[11];

        value += if board.get_next_board_index().is_none() {
            self.values[12]
        } else {
            -self.values[12]
        };

        value
    }
    fn get_name(&self) -> String {
        "ParameterizedHeuristic".to_string()
    }
}

/// # Struct representing a [MiniBoardHeuristic] that uses weights for the features to evaluate small boards
pub struct ParameterizedMiniBoardHeuristic {
    /// The weights for the features
    ///
    /// For the features, see [ParameterizedHeuristic::values]
    values: Vec<f64>,
}

impl ParameterizedMiniBoardHeuristic {
    pub fn new(values: Vec<f64>) -> Self {
        ParameterizedMiniBoardHeuristic { values }
    }
}

impl MiniBoardHeuristic for ParameterizedMiniBoardHeuristic {
    fn evaluate(&self, board: Board) -> f64 {
        let mut value = 0.;

        let positions_set_difference = board.get_positions_set_difference(Player::One) as f64;
        if positions_set_difference > 0. {
            value += positions_set_difference * self.values[3];
        }

        value += board.get_partial_wins_difference(Player::One) as f64 * self.values[4];

        value += board.center_occupied(Player::One) as f64 * self.values[5];

        value += board.get_corners_difference(Player::One) as f64 * self.values[6];

        value += board.get_edges_difference(Player::One) as f64 * self.values[7];

        value
    }
}
