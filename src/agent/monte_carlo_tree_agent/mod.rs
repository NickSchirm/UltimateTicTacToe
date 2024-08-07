//! # Contains the [MonteCarloTreeAgent] struct
//!
//! The [MonteCarloTreeAgent] struct represents an agent that uses the Monte Carlo Tree Search algorithm to evaluate the best move.
//!
//! The agent uses a [Tree] to store the game states and the statistics of the nodes.
use crate::agent::{Agent, AgentInfo};
use crate::game::game_result::GameResult;
use crate::game::player::Player;
use crate::game::ultimate_board::UltimateBoard;
use crate::tree::{Node, Tree};
use rand::distributions::Uniform;
use rand_distr::Distribution;
use std::cell::RefCell;
use std::rc::Rc;

/// # Struct representing an agent that uses the Monte Carlo Tree Search algorithm to evaluate the best move
///
/// The agent uses a [Tree] to store the game states and the statistics of the nodes.
///
/// The agent uses the number of iterations to determine the number of simulations.
///
/// The agent uses the [Player] and the turn to determine the best move.
pub struct MonteCarloTreeAgent {
    iterations: u32,
    player: Player,
    turn: u32,
}

impl MonteCarloTreeAgent {
    /// Creates a new [MonteCarloTreeAgent]
    ///
    /// # Arguments
    /// * `iterations` - The number of iterations to determine the number of simulations
    pub fn new(iterations: u32) -> Self {
        MonteCarloTreeAgent {
            iterations,
            player: Player::default(),
            turn: 0,
        }
    }

    fn tree_root(&self, board: UltimateBoard) -> Option<u8> {
        let tree = Tree::new(Node::new(NodeInfo::new(board)));

        let root = tree.get_root().unwrap();

        for _ in 0..self.iterations {
            let _ = self.tree_search(root.clone());
        }

        let best_child = root
            .borrow()
            .get_children()
            .iter()
            .map(|child| {
                let child_stats = child.borrow().get_data().stats;
                let uct = child_stats.wins() as f64 / child_stats.total() as f64;
                (child, uct)
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(child, _)| child)
            .unwrap()
            .clone();

        best_child.clone().borrow().get_data().get_move_index()
    }

    fn tree_search(&self, root: Rc<RefCell<Node<NodeInfo>>>) -> Stats {
        let mut root_stats = Stats::default();

        if root.borrow().get_data().get_board().get_game_status() != GameResult::Continue {
            return root_stats;
        } else if root.borrow().is_leaf() {
            let board = root.borrow().get_data().get_board();
            for possible_move in board.get_possible_moves() {
                let mut board_copy = board;
                board_copy.make_move(possible_move);
                let stats = self.playout(board_copy);

                let mut node_info = NodeInfo::withMove(board_copy, possible_move);
                node_info.apply_stats(stats);

                root_stats.merge(stats);

                root.borrow_mut().append(Node::new(node_info));
            }
        } else {
            let root_visits = root.borrow().get_data().stats.total();

            let best_child = root
                .borrow()
                .get_children()
                .iter()
                .map(|child| {
                    let uct = child.borrow().get_data().uct_value(root_visits);
                    (child, uct)
                })
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(child, _)| child)
                .unwrap()
                .clone();

            let stats = self.tree_search(best_child.clone());

            root_stats.merge(stats);
        }

        root.borrow_mut().map(|data: NodeInfo| {
            let mut res = data;
            res.apply_stats(root_stats);
            res
        });

        root_stats
    }

    fn playout(&self, mut board: UltimateBoard) -> Stats {
        let mut stats = Stats::default();

        while board.get_game_status() == GameResult::Continue {
            let possible_moves: Vec<_> = board.get_possible_moves().collect();

            let next_move = possible_moves
                [Uniform::from(0..possible_moves.len()).sample(&mut rand::thread_rng())];

            board.make_move(next_move);
        }

        match board.get_game_status() {
            GameResult::Win(player) => {
                if player == self.player {
                    stats.wins += 1;
                } else {
                    stats.losses += 1;
                }
            }
            GameResult::Draw => {
                stats.draws += 1;
            }
            _ => unreachable!(),
        }

        stats
    }
}

impl Agent for MonteCarloTreeAgent {
    fn act(&mut self, board: UltimateBoard, player: Player, turn: u32) -> Option<u8> {
        self.player = player;
        self.turn = turn;

        self.tree_root(board)
    }

    fn get_info(&self) -> AgentInfo {
        AgentInfo::new(
            "MCTS".to_string(),
            self.player,
            self.turn,
            format!("max_nodes: {}", self.iterations),
        )
    }
}

/// # Struct representing the information of a node in the tree
///
/// The information contains the board, the move index, and the statistics of the node.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NodeInfo {
    board: UltimateBoard,
    move_index: Option<u8>,
    stats: Stats,
}

impl NodeInfo {
    /// Creates a new [NodeInfo]
    ///
    /// # Arguments
    /// * `board` - The board of the node
    fn new(board: UltimateBoard) -> Self {
        NodeInfo {
            board,
            move_index: None,
            stats: Stats::default(),
        }
    }

    /// Creates a new [NodeInfo] with a move index
    ///
    /// # Arguments
    /// * `board` - The board of the node
    fn withMove(board: UltimateBoard, move_index: u8) -> Self {
        NodeInfo {
            board,
            move_index: Some(move_index),
            stats: Stats::default(),
        }
    }

    /// Gets the board of the node
    pub fn get_board(&self) -> UltimateBoard {
        self.board
    }

    /// Gets the move index of the node
    pub fn get_move_index(&self) -> Option<u8> {
        self.move_index
    }

    /// Applies the statistics to the node
    ///
    /// # Arguments
    /// * `stats` - The statistics to apply
    fn apply_stats(&mut self, stats: Stats) {
        self.stats.wins += stats.wins;
        self.stats.draws += stats.draws;
        self.stats.losses += stats.losses;
    }

    /// Calculates the UCT value of the node
    ///
    /// # Arguments
    /// * `parent_visits` - The number of visits of the parent node
    ///
    /// # Returns
    /// The UCT value of the node
    fn uct_value(&self, parent_visits: u32) -> f64 {
        let wins = self.stats.wins() as f64;
        let visits = self.stats.total() as f64;

        wins / visits + ((2. * (parent_visits as f64).ln()) / wins)
    }
}

/// # Struct representing the statistics of a node in the tree
///
/// The statistics contain the number of wins, draws, and losses.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
struct Stats {
    wins: u32,
    draws: u32,
    losses: u32,
}

impl Stats {
    /// Gets the total number of games
    fn total(&self) -> u32 {
        self.wins + self.draws + self.losses
    }
    /// Gets the number of wins
    fn wins(&self) -> u32 {
        self.wins
    }
    /// Gets the number of draws
    fn draws(&self) -> u32 {
        self.draws
    }
    /// Gets the number of losses
    fn losses(&self) -> u32 {
        self.losses
    }

    /// Merges this instance with another instance, adding the statistics
    pub fn merge(&mut self, other: Stats) {
        self.wins += other.wins;
        self.draws += other.draws;
        self.losses += other.losses;
    }
}

pub fn run() {
    let mut agent = MonteCarloTreeAgent::new(1000);
    let board = UltimateBoard::new();
    let player = Player::One;
    let turn = 0;

    agent.act(board, player, turn);
}
