use crate::game_result::GameResult;
use crate::heuristic::{Heuristic, MAX_VALUE, MIN_VALUE};
use crate::player::Player;
use crate::ultimate_board::UltimateBoard;

pub struct CustomHeuristic {
    player: Player,
}

impl CustomHeuristic {
    pub fn new(player: Player) -> CustomHeuristic {
        CustomHeuristic { player }
    }
}

impl Heuristic for CustomHeuristic {
    fn evaluate(&self, board: UltimateBoard) -> isize {
        let mut value = 0;

        if board.get_game_status() == GameResult::Win(self.player) {
            return MIN_VALUE;
        }

        if board.get_game_status() == GameResult::Win(self.player.get_opponent()) {
            return MAX_VALUE;
        }

        for board_status in board.get_board_status() {
            match board_status {
                GameResult::Win(player) => {
                    if player == self.player {
                        value += 100;
                    } else {
                        value -= 100;
                    }
                }
                GameResult::Draw => {
                    value += -50;
                }
                _ => {}
            }
        }

        value
    }
}
