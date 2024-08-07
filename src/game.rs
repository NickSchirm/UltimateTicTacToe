use crate::agent::Agent;
use crate::game_result::GameResult;
use crate::ultimate_board::UltimateBoard;

pub struct Game<'a> {
    agent_one: &'a mut dyn Agent,
    agent_two: &'a mut dyn Agent,
    board: UltimateBoard,
}

impl<'a> Game<'a> {
    pub fn new(agent_one: &'a mut dyn Agent, agent_two: &'a mut dyn Agent) -> Game<'a> {
        Game {
            agent_one,
            agent_two,
            board: UltimateBoard::new(),
        }
    }

    pub fn play(&mut self) -> GameResult {
        let mut game_result = self.board.get_game_status();

        while game_result == GameResult::Continue {
            let move_one = self.agent_one.act(self.board);
            self.board.make_move(move_one);

            println!("{}", self.board);

            game_result = self.board.get_game_status();

            if game_result != GameResult::Continue {
                break;
            }

            let move_two = self.agent_two.act(self.board);
            self.board.make_move(move_two);

            println!("{}", self.board);

            game_result = self.board.get_game_status();
        }

        game_result
    }
}
