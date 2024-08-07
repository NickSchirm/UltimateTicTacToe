//! # Contains the [Game] struct
//! The Game struct represents a game of Ultimate Tic Tac Toe.
//! The game is played by two [agents](Agent).

use crate::agent::Agent;
use crate::game_result::GameResult;
use crate::player::Player;
use crate::ultimate_board::UltimateBoard;

/// Struct representing a game of Ultimate Tic Tac Toe
///
/// The game is played by two [agents](Agent).
pub struct Game {
    agents: Vec<Box<dyn Agent>>,
    board: UltimateBoard,
}

impl Game {
    /// Creates a new game with the provided agents.
    /// # Arguments
    /// * `agent_one` - The agent of [Player::One]
    /// * `agent_two` - The agent of [Player::Two]
    /// # Returns
    /// A new game
    pub fn new(agent_one: Box<dyn Agent>, agent_two: Box<dyn Agent>) -> Self {
        Game {
            agents: vec![agent_one, agent_two],
            board: UltimateBoard::new(),
        }
    }

    /// Plays the game until a result is reached.
    /// # Returns
    /// The result of the game
    pub fn play(&mut self) -> GameResult {
        let mut game_result = self.board.get_game_status();
        let mut active_agent = Player::One;

        while game_result == GameResult::Continue {
            let current_move = self.agents[active_agent as usize].act(self.board);

            if current_move.is_none() {
                eprintln!("Agent {:?} returned None instead of a move", active_agent);
                eprintln!("{}", self.board);
                eprintln!("{:?}", self.board);
                eprintln!("{:?}", self.board.get_possible_moves().collect::<Vec<u8>>());
                self.agents[active_agent as usize].act(self.board);
                panic!();
            }

            self.board.make_move(current_move.unwrap());

            //println!("{}", self.board);

            game_result = self.board.get_game_status();

            active_agent = active_agent.get_opponent();
        }

        game_result
    }
}
