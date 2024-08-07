//! # Contains the [MonteCarloGameSearchHeuristic] struct
//! The MonteCarloGameSearchHeuristic struct represents a [Heuristic] that uses Monte Carlo Tree Search to evaluate the best move.
//! The heuristic uses random games to evaluate the best move.

use crate::game_result::GameResult;
use crate::heuristic::Heuristic;
use crate::player::Player;
use crate::ultimate_board::UltimateBoard;
use rand::prelude::SliceRandom;

/// A [Heuristic] that uses Monte Carlo Tree Search to evaluate the best move
///
/// The heuristic uses random games to evaluate the best move.
/// The move with the highest win rate and lowest loss rate is chosen.
/// # Fields
/// * `player` - The [Player] for which the heuristic should evaluate the best move
/// * `num_simulations` - The number of random games to simulate from each possible move
///
/// Note:
/// * The heuristic is not deterministic.
/// * The heuristic is not guaranteed to find the best move.
/// * The heuristic is really slow compared to [CustomHeuristic](crate::custom_heuristic::CustomHeuristic) while providing worse results.
#[derive(Clone)]
pub struct MonteCarloGameSearchHeuristic {
    player: Player,
    num_simulations: u32,
}

impl MonteCarloGameSearchHeuristic {
    pub fn new(player: Player, num_simulations: u32) -> Self {
        MonteCarloGameSearchHeuristic {
            player,
            num_simulations,
        }
    }

    /// Simulates a random game starting from the given board
    ///
    /// The game is played until a result is reached.
    fn random_game(board: UltimateBoard) -> GameResult {
        let mut board = board;
        let mut game_result = board.get_game_status();

        while game_result == GameResult::Continue {
            let possible_moves = board.get_possible_moves().collect::<Vec<u8>>();
            let random_move = possible_moves.choose(&mut rand::thread_rng()).unwrap();
            board.make_move(*random_move);
            game_result = board.get_game_status();
        }

        game_result
    }
}

impl Heuristic for MonteCarloGameSearchHeuristic {
    fn evaluate(&self, board: UltimateBoard) -> i32 {
        let possible_moves = board.get_possible_moves();
        let mut results = vec![];

        for current_move in possible_moves {
            let mut wins = 0;
            let mut losses = 0;
            let mut draws = 0;

            for _ in 0..self.num_simulations {
                let board_copy = board;

                let game_result = MonteCarloGameSearchHeuristic::random_game(board_copy);

                match game_result {
                    GameResult::Win(player) => {
                        if player == self.player {
                            wins += 1;
                        } else {
                            losses += 1;
                        }
                    }
                    GameResult::Draw => {
                        draws += 1;
                    }
                    _ => {
                        panic!("Error: Game should never be in a continue state");
                    }
                }
            }

            results.push((current_move, wins, losses, draws));
        }

        let mut best_move = None;

        for (current_move, wins, losses, draws) in results {
            if best_move.is_none() {
                best_move = Some((current_move, wins, losses, draws));
            } else {
                let (_, best_wins, best_losses, _) = best_move.unwrap();

                if (wins > best_wins) || (wins == best_wins && losses < best_losses) {
                    best_move = Some((current_move, wins, losses, draws));
                }
            }
        }

        0
    }
}
