use crate::agent::Agent;
use crate::game_result::GameResult::Continue;
use crate::heuristic::Heuristic;
use crate::ultimate_board::UltimateBoard;
use std::cmp::{max, min};
use std::collections::HashMap;

/// An Ultimate Tic Tac Toe agent that uses the minimax algorithm to determine the best move.
/// The agent uses the provided heuristic to evaluate the board state.
///
/// <b>Optimizations</b>:
/// * [Alpha-beta pruning](https://www.chessprogramming.org/Alpha-Beta)
/// * [Transposition table](https://www.chessprogramming.org/Transposition_Table) using [Zobrist Hashing](https://www.chessprogramming.org/Zobrist_Hashing)
/// * [Quiescence search](https://www.chessprogramming.org/Quiescence_Search) to combat the [Horizon effect](https://www.chessprogramming.org/Horizon_Effect)
/// <p>
/// Note: Quiescence search depth has a large impact on the performance of the agent. The effect of Quiescence search may be small.
/// Quiescence search can be disabled by setting the depth to 0.
pub struct MiniMaxAgent<H> {
    depth: u32,
    quiescence_search_depth: u32,
    heuristic: H,
}

impl<H: Heuristic> MiniMaxAgent<H> {
    pub fn new(depth: u32, quiescence_search_depth: u32, heuristic: H) -> MiniMaxAgent<H> {
        MiniMaxAgent {
            depth,
            quiescence_search_depth,
            heuristic,
        }
    }

    /// Returns the best move for the current player <p>
    /// Uses the minimax algorithm to determine the best move <p>
    /// Is the root call for the minimax algorithm <p>
    /// For more info see [`MiniMaxAgent::minimax`] <p>
    /// # Arguments
    /// * `board` - The current state of the board
    /// * `depth` - The depth of the minimax algorithm
    /// # Returns
    /// The index of the field to play on
    fn get_best_move(&self, board: UltimateBoard, depth: u32) -> Option<u8> {
        // https://www.chessprogramming.org/Transposition_Table
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

    /// The minimax algorithm <p>
    /// Uses alpha-beta pruning to reduce the number of nodes that need to be evaluated <p>
    /// Uses a [transposition table](https://www.chessprogramming.org/Transposition_Table) to store the values of already evaluated nodes <p>
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
        mut alpha: isize,
        mut beta: isize,
        transposition_table: &mut HashMap<u64, isize>,
    ) -> isize {
        if depth == 0 {
            return self.quiescence_search(
                board,
                self.quiescence_search_depth,
                maximizing,
                alpha,
                beta,
            );
        }

        if board.get_game_status() != Continue {
            return self.heuristic.evaluate(board);
        }

        let mut possible_moves = board.get_possible_moves().peekable();

        if possible_moves.peek().is_none() {
            return self.heuristic.evaluate(board);
        }

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

    /// The [quiescence search](https://www.chessprogramming.org/Quiescence_Search) algorithm <p>
    /// This algorithm is used to avoid the [horizon effect](https://www.chessprogramming.org/Horizon_Effect). <p>
    /// Only continues searching if the next move can be made on any open square. <p>
    /// # Arguments
    /// * `board` - The current state of the board
    /// * `depth` - The depth of the quiescence search algorithm
    /// * `maximizing` - Whether the current player is maximizing
    /// * `alpha` - The alpha value for alpha-beta pruning
    /// * `beta` - The beta value for alpha-beta pruning
    /// # Returns
    /// The value of the current state
    fn quiescence_search(
        &self,
        board: UltimateBoard,
        depth: u32,
        maximizing: bool,
        mut alpha: isize,
        mut beta: isize,
    ) -> isize {
        if depth == 0 {
            return self.heuristic.evaluate(board);
        }

        if board.get_game_status() != Continue {
            return self.heuristic.evaluate(board);
        }

        if board.get_next_board_index().is_some() {
            return self.heuristic.evaluate(board);
        }

        let mut possible_moves = board.get_possible_moves().peekable();

        if possible_moves.peek().is_none() {
            return self.heuristic.evaluate(board);
        }

        if maximizing {
            for current_move in possible_moves {
                let mut new_board = board;
                new_board.make_move(current_move);
                alpha = max(
                    alpha,
                    self.quiescence_search(new_board, depth - 1, false, alpha, beta),
                );

                if alpha >= beta {
                    break;
                }
            }
            alpha
        } else {
            for current_move in possible_moves {
                let mut new_board = board;
                new_board.make_move(current_move);
                beta = min(
                    beta,
                    self.quiescence_search(new_board, depth - 1, true, alpha, beta),
                );

                if alpha >= beta {
                    break;
                }
            }
            beta
        }
    }
}

impl<H: Heuristic> Agent for MiniMaxAgent<H> {
    fn act(&mut self, board: UltimateBoard) -> Option<u8> {
        self.get_best_move(board, self.depth)
    }
}
