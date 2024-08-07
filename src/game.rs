use crate::agent::Agent;
use crate::game_result::GameResult;
use crate::player::Player;
use crate::ultimate_board::UltimateBoard;

pub struct Game<'a> {
    agents: Vec<&'a mut dyn Agent>,
    board: UltimateBoard,
}

impl<'a> Game<'a> {
    pub fn new(agent_one: &'a mut dyn Agent, agent_two: &'a mut dyn Agent) -> Game<'a> {
        Game {
            agents: vec![agent_one, agent_two],
            board: UltimateBoard::new(),
        }
    }

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
