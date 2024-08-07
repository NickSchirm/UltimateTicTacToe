//! # Contains the [MiniMaxAgent] and [Number] struct
//! The MiniMaxAgent struct represents an [Agent] that uses the minimax algorithm to determine the best move.
//! The agent uses the provided [Heuristic] to evaluate the board state.
//!
//! The Number struct is used to allow for easy switching between f64 and i32.
//!
//! For more information see the [MiniMaxAgent](MiniMaxAgent) struct.

use crate::agent::Agent;
use crate::game::game_result::GameResult::Continue;
use crate::game::ultimate_board::UltimateBoard;
use crate::heuristic::Heuristic;
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[allow(rustdoc::private_intra_doc_links)]
/// An Ultimate Tic Tac Toe agent that uses the minimax algorithm to determine the best move.
/// The agent uses the provided heuristic to evaluate the board state.
///
/// <b>Optimizations</b>:
/// * [Alpha-beta pruning](https://www.chessprogramming.org/Alpha-Beta)
/// * [Transposition table](https://www.chessprogramming.org/Transposition_Table) using [Zobrist Hashing](https://www.chessprogramming.org/Zobrist_Hashing)
/// * [Quiescence search](https://www.chessprogramming.org/Quiescence_Search) to combat the [Horizon effect](https://www.chessprogramming.org/Horizon_Effect)
///
/// Note: Quiescence search depth has a large impact on the performance of the agent. The effect of Quiescence search may be small.
/// Quiescence search can be disabled by setting the [quiescence_search_depth](MiniMaxAgent::quiescence_search_depth) to 0.
pub struct MiniMaxAgent<H> {
    /// The depth minimax should search to
    depth: u32,
    /// The depth the quiescence search should search to
    quiescence_search_depth: u32,
    /// The heuristic used to evaluate the board state
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

    /// Returns the best move for the current player
    ///
    /// The minimax algorithm is used to determine the best move.
    ///
    /// This is the root call for the minimax algorithm.
    ///
    /// For more info see [`MiniMaxAgent::minimax`]
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

        let mut alpha = Number::MIN;
        let beta = Number::MAX;

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
    ///
    /// Alpha-beta pruning is used to reduce the number of nodes that need to be evaluated.
    ///
    /// A [transposition table](https://www.chessprogramming.org/Transposition_Table) is used to store the values of already evaluated nodes.
    ///
    /// Calls [MiniMaxAgent::quiescence_search] if the depth is 0.
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
        mut alpha: Number,
        mut beta: Number,
        transposition_table: &mut HashMap<u64, Number>,
    ) -> Number {
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
                alpha = Number::max(
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
                beta = Number::min(
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

    /// The [quiescence search](https://www.chessprogramming.org/Quiescence_Search) algorithm
    ///
    /// This algorithm is used to avoid the [horizon effect](https://www.chessprogramming.org/Horizon_Effect).
    ///
    /// Only continues searching if the next move can be made on any open square.
    ///
    /// If the depth is 0, the [heuristic](MiniMaxAgent::heuristic) is used to evaluate the board.
    /// the quiescence search can be disabled by setting [quiescence_search_depth](MiniMaxAgent::quiescence_search_depth) to 0.
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
        mut alpha: Number,
        mut beta: Number,
    ) -> Number {
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
                alpha = Number::max(
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
                beta = Number::min(
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

/// A number type that implements the basic arithmetic operations.
///
/// This type is used to allow for easy switching between f64 and i32.
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
pub struct Number(pub f64);

impl Number {
    pub fn get_value(&self) -> f64 {
        self.0
    }

    pub const MIN: Number = Number(f64::MIN);
    pub const MAX: Number = Number(f64::MAX);

    pub fn min(lhs: Number, rhs: Number) -> Number {
        Number(f64::min(lhs.0, rhs.0))
    }

    pub fn max(lhs: Number, rhs: Number) -> Number {
        Number(f64::max(lhs.0, rhs.0))
    }

    pub const ZERO: Number = Number(0.0);
}

impl Deref for Number {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl Add<f64> for Number {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Number(self.0 + rhs)
    }
}

impl Add<i32> for Number {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Number(self.0 + rhs as f64)
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl AddAssign<f64> for Number {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
    }
}

impl AddAssign<i32> for Number {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs as f64;
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Number(self.0 - rhs.0)
    }
}

impl Sub<f64> for Number {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Number(self.0 - rhs)
    }
}

impl Sub<i32> for Number {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Number(self.0 - rhs as f64)
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl SubAssign<f64> for Number {
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
    }
}

impl SubAssign<i32> for Number {
    fn sub_assign(&mut self, rhs: i32) {
        self.0 -= rhs as f64;
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Number(self.0 * rhs.0)
    }
}

impl Mul<f64> for Number {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Number(self.0 * rhs)
    }
}

impl Mul<i32> for Number {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Number(self.0 * rhs as f64)
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl MulAssign<f64> for Number {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl MulAssign<i32> for Number {
    fn mul_assign(&mut self, rhs: i32) {
        self.0 *= rhs as f64;
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Number(self.0 / rhs.0)
    }
}

impl Div<f64> for Number {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Number(self.0 / rhs)
    }
}

impl Div<i32> for Number {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Number(self.0 / rhs as f64)
    }
}

impl DivAssign for Number {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl DivAssign<f64> for Number {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
    }
}

impl DivAssign<i32> for Number {
    fn div_assign(&mut self, rhs: i32) {
        self.0 /= rhs as f64;
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number(value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number(value as f64)
    }
}
