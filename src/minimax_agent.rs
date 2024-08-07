use crate::agent::Agent;
use crate::game_result::GameResult::Continue;
use crate::heuristic::Heuristic;
use crate::ultimate_board::UltimateBoard;
use std::cmp::{max, min};
use std::collections::HashMap;

pub struct MiniMaxAgent {
    depth: u32,
    heuristic: Box<dyn Heuristic>,
}

impl MiniMaxAgent {
    pub fn new(depth: u32, heuristic: Box<dyn Heuristic>) -> MiniMaxAgent {
        MiniMaxAgent { depth, heuristic }
    }

    /// Returns the best move for the current player
    /// # Arguments
    /// * `board` - The current state of the board
    /// * `depth` - The depth of the minimax algorithm
    /// # Returns
    /// The index of the field to play on
    fn get_best_move(&self, board: UltimateBoard, depth: u32) -> Option<u8> {
        let mut transposition_table = HashMap::new();

        let possible_moves = board.get_possible_moves();

        let mut best_move = None;

        let mut alpha = isize::MIN;
        let beta = isize::MAX;

        // Iterate over all possible moves
        // Maximizing
        for current_move in possible_moves {
            let mut new_board = board;

            new_board.make_move(current_move);

            let value = self.minimax(
                new_board,
                depth - 1,
                false,
                alpha,
                beta,
                &mut transposition_table,
            );

            if value > alpha {
                alpha = value;
                best_move = Some(current_move);
            }
        }

        best_move
    }

    /// The minimax algorithm
    /// # Arguments
    /// * `board` - The current state of the board
    /// * `depth` - The depth of the minimax algorithm
    /// * `maximizing` - Whether the current player is maximizing
    /// * `alpha` - The alpha value for alpha-beta pruning
    /// * `beta` - The beta value for alpha-beta pruning
    /// # Returns
    /// The value of the current state
    fn minimax(
        &self,
        board: UltimateBoard,
        depth: u32,
        maximizing: bool,
        alpha: isize,
        beta: isize,
        transposition_table: &mut HashMap<u64, isize>,
    ) -> isize {
        if depth == 0 {
            return self.heuristic.evaluate(board);
        }

        if board.get_game_status() != Continue {
            return self.heuristic.evaluate(board);
        }

        let mut possible_moves = board.get_possible_moves().peekable();

        if possible_moves.peek().is_none() {
            return self.heuristic.evaluate(board);
        }

        let mut alpha = alpha;
        let mut beta = beta;

        // Check if the board is in the transposition table
        if let Some(evaluation) = transposition_table.get(&board.get_hash()) {
            return *evaluation;
        }

        if maximizing {
            for current_move in possible_moves {
                let mut new_board = board;
                new_board.make_move(current_move);
                alpha = max(
                    alpha,
                    self.minimax(
                        new_board,
                        depth - 1,
                        false,
                        alpha,
                        beta,
                        transposition_table,
                    ),
                );

                if alpha >= beta {
                    break;
                }
            }
            transposition_table.insert(board.get_hash(), alpha);
            alpha
        } else {
            for current_move in possible_moves {
                let mut new_board = board;
                new_board.make_move(current_move);
                beta = min(
                    beta,
                    self.minimax(new_board, depth - 1, true, alpha, beta, transposition_table),
                );

                if alpha >= beta {
                    break;
                }
            }
            transposition_table.insert(board.get_hash(), beta);
            beta
        }
    }
}

impl Agent for MiniMaxAgent {
    fn act(&mut self, board: UltimateBoard) -> Option<u8> {
        self.get_best_move(board, self.depth)
    }
}
