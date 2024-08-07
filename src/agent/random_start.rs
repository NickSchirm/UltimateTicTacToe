use crate::agent::{Agent, AgentInfo};
use crate::agent::random_agent::RandomAgent;
use crate::game::player::Player;
use crate::game::ultimate_board::UltimateBoard;

pub struct RandomStartAgent<A: Agent> {
	depth: u32,
	agent: A,
	random_agent: RandomAgent
}

impl<A: Agent> RandomStartAgent<A> {
	pub fn new(depth: u32, agent: A) -> RandomStartAgent<A> {
		RandomStartAgent {
			depth,
			agent,
			random_agent: RandomAgent::new()
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