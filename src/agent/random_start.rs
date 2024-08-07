//! # Contains the [RandomStartAgent] struct
//!
//! The RandomStartAgent struct represents an [Agent] that uses a random agent for the first `depth` turns, then switches to another agent.
//!
//! For more information see the [RandomStartAgent] struct.
use crate::agent::random_agent::RandomAgent;
use crate::agent::{Agent, AgentInfo};
use crate::game::player::Player;
use crate::game::ultimate_board::UltimateBoard;

/// An agent that uses a random agent for the first `depth` turns, then switches to another agent.
///
/// # Example
/// ```
/// use hausarbeit::agent::monte_carlo_tree_agent::MonteCarloTreeAgent;
/// use hausarbeit::agent::random_start::RandomStartAgent;
///
/// let agent = RandomStartAgent::new(5, MonteCarloTreeAgent::new(1000));
/// ```
/// The agent will use a random agent for the first 5 turns, then switch to a MonteCarloTreeAgent.
pub struct RandomStartAgent<A: Agent> {
    depth: u32,
    agent: A,
    random_agent: RandomAgent,
}

impl<A: Agent> RandomStartAgent<A> {
    pub fn new(depth: u32, agent: A) -> RandomStartAgent<A> {
        RandomStartAgent {
            depth,
            agent,
            random_agent: RandomAgent::new(),
        }
    }
}

impl<A: Agent> Agent for RandomStartAgent<A> {
    fn act(&mut self, board: UltimateBoard, player: Player, turn: u32) -> Option<u8> {
        if self.depth > turn {
            return self.random_agent.act(board, player, turn);
        }
        self.agent.act(board, player, turn)
    }

    fn get_info(&self) -> AgentInfo {
        self.agent.get_info()
    }
}
