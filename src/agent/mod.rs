//! # Contains the [Agent] trait and implementations
//! 
//! The [Agent] trait represents an agent that can play Ultimate Tic Tac Toe.
//! 
//! The trait has a single method [act](Agent::act) that is called to get the agent's move.
//! 
//! The agent should return the index of the field to play on.
//! 
//! The index is the human index (0-80).
pub mod benched;
pub mod human_agent;
pub mod minimax_agent;
pub mod random_agent;
pub mod monte_carlo_tree_agent;

use crate::game::player::Player;
use crate::game::ultimate_board::UltimateBoard;

/// Trait representing an agent that can play Ultimate Tic Tac Toe
pub trait Agent: Send + Sync {
    /// The act method is called to get the agent's move.
    ///
    /// The agent should return the index of the field to play on.
    /// The index is the human index (0-80) over all boards.
    ///
    /// The game will panic if None is returned or if the index is out of bounds.
    /// In case of a panic, relevant information about the state of the game will be printed to the console.
    ///
    /// It is recommended to return None if the agent cannot play or if the agent can not find a move.
    /// # Arguments
    /// * `board` - The current state of the board
    /// # Returns
    /// The index of the field to play on
    fn act(&mut self, board: UltimateBoard, player: Player, turn: u32) -> Option<u8>;
    fn get_info(&self) -> AgentInfo;
}

/// # Struct representing the information of an agent
/// 
/// The information contains the name of the agent, the player, the turn number, and the configuration of the agent.
#[derive(Clone, Debug)]
pub struct AgentInfo {
    name: String,
    player: Player,
    turn_num: u32,
    config: String,
}

impl AgentInfo {
    /// Creates a new [AgentInfo]
    /// 
    /// # Arguments
    /// * `name` - The name of the agent
    /// * `player` - The player of the agent
    /// * `turn_num` - The turn number of the agent
    /// * `config` - The configuration of the agent
    /// 
    /// # Returns
    /// The new [AgentInfo]
    pub fn new(name: String, player: Player, turn_num: u32, config: String) -> AgentInfo {
        AgentInfo {
            name,
            player,
            turn_num,
            config,
        }
    }
}
