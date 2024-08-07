//! # Contains the [HumanAgent] struct
//! The HumanAgent struct represents an [Agent] that allows a human player to play the game.
//! The human player can input the moves via the console.
//! The board is printed to the console before each move.
//!
//! If:
//! * a move is invalid, the player is prompted to input a new move.
//! * the player has to play on a specific board, the board is highlighted by a colorful border.
//!
//! You can start a game with a human player by calling the [start_game_with_human] function.

use colored::{Colorize, CustomColor};
use itertools::Itertools;
use once_cell::sync::Lazy;

use crate::agent::minimax_agent::MiniMaxAgent;
use crate::agent::{Agent, AgentInfo};
use crate::agent::monte_carlo_tree_agent::MonteCarloTreeAgent;
use crate::game::board::BoardSymbol;
use crate::game::Game;
use crate::game::game_result::GameResult;
use crate::game::player::Player;
use crate::game::ultimate_board::UltimateBoard;
use crate::heuristic::custom_heuristic::CustomHeuristic;

static HIGHLIGHT_COLOR: Lazy<CustomColor> = Lazy::new(|| CustomColor::new(87, 46, 105));
static BACKGROUND_COLOR: Lazy<CustomColor> = Lazy::new(|| CustomColor::new(30, 31, 34));
static X_COLOR: Lazy<CustomColor> = Lazy::new(|| CustomColor::new(154, 46, 34));
static O_COLOR: Lazy<CustomColor> = Lazy::new(|| CustomColor::new(18, 128, 106));

/// An [Agent] that allows a human player to play the game.
///
/// The human player can input the moves via the console.
/// The board is printed to the console before each move.
///
/// If:
/// * a move is invalid, the player is prompted to input a new move.
/// * the player has to play on a specific board, the board is highlighted by a colorful border.
///
/// You can start a game with a human player by calling the [start_game_with_human] function.
#[derive(Default)]
pub struct HumanAgent {
    player: Player,
    turn: u32,
}

impl HumanAgent {
    fn print_board(board: UltimateBoard, highlighted_board: Option<u8>) {
        for row in 0..17 {
            let big_row = if row < 6 {
                0
            } else if row < 12 {
                1
            } else {
                2
            };

            if row == 0 || row == 4 || row == 6 || row == 10 || row == 12 || row == 16 {
                let color = HumanAgent::convert_to_color(highlighted_board, big_row);

                // Print small board border
                if (row == 0 || row == 6 || row == 12) && highlighted_board.is_none() {
                    print!(
                        "{}{}",
                        3 * big_row + 1,
                        "              ".on_custom_color(color[0])
                    );
                    print!("|");
                    print!(
                        "{}{}",
                        3 * big_row + 2,
                        "              ".on_custom_color(color[1])
                    );
                    print!("|");
                    print!(
                        "{}{}",
                        3 * big_row + 3,
                        "              ".on_custom_color(color[2])
                    );
                } else {
                    print!("{}", "               ".on_custom_color(color[0]));
                    print!("|");
                    print!("{}", "               ".on_custom_color(color[1]));
                    print!("|");
                    print!("{}", "               ".on_custom_color(color[2]));
                }
                println!()
            } else if row == 5 || row == 11 {
                // Print board divider
                println!(
                    "{}",
                    " - - - - - - - + - - - - - - - + - - - - - - - ".bold()
                );
            } else {
                let sub_row = match row {
                    1 | 7 | 13 => 0,
                    2 | 8 | 14 => 1,
                    3 | 9 | 15 => 2,
                    _ => panic!("Invalid row"),
                };
                let color = HumanAgent::convert_to_color(highlighted_board, big_row);

                // Print board row
                for i in (big_row * 3)..(big_row * 3 + 3) {
                    // Print Small board border
                    print!("{}", "  ".on_custom_color(color[(i % 3) as usize]));

                    let row = board.get_boards()[i as usize].extract_row(sub_row);

                    print!(
                        "{}",
                        row.iter()
                            .enumerate()
                            .map(|(index, item)| match item {
                                BoardSymbol::X => " X ".on_custom_color(*X_COLOR),
                                BoardSymbol::O => " O ".on_custom_color(*O_COLOR),
                                BoardSymbol::Empty => {
                                    match highlighted_board {
                                        Some(next_board_index) => {
                                            if next_board_index == i {
                                                format!(" {} ", 3 * sub_row + index as u8 + 1)
                                                    .on_custom_color(*BACKGROUND_COLOR)
                                            } else {
                                                "   ".on_custom_color(*BACKGROUND_COLOR)
                                            }
                                        }
                                        None => "   ".on_custom_color(*BACKGROUND_COLOR),
                                    }
                                }
                            })
                            .join(" ")
                    );

                    // Print Small board border
                    print!("{}", "  ".on_custom_color(color[(i % 3) as usize]));

                    if i % 3 != 2 {
                        print!("|");
                    }
                }

                println!();
            }
        }
    }

    fn convert_to_color(highlighted_board: Option<u8>, big_row: u8) -> [CustomColor; 3] {
        match highlighted_board {
            Some(index) => {
                if big_row == index / 3 {
                    if index % 3 == 0 {
                        [*HIGHLIGHT_COLOR, *BACKGROUND_COLOR, *BACKGROUND_COLOR]
                    } else if index % 3 == 1 {
                        [*BACKGROUND_COLOR, *HIGHLIGHT_COLOR, *BACKGROUND_COLOR]
                    } else {
                        [*BACKGROUND_COLOR, *BACKGROUND_COLOR, *HIGHLIGHT_COLOR]
                    }
                } else {
                    [*BACKGROUND_COLOR, *BACKGROUND_COLOR, *BACKGROUND_COLOR]
                }
            }
            None => [*BACKGROUND_COLOR, *BACKGROUND_COLOR, *BACKGROUND_COLOR],
        }
    }
}

impl Agent for HumanAgent {
    fn act(&mut self, board: UltimateBoard, player: Player, turn: u32) -> Option<u8> {
        self.player = player;
        self.turn = turn;

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        HumanAgent::print_board(board, board.get_next_board_index());

        let possible_moves = board.get_possible_moves().collect::<Vec<u8>>();

        if let Some(next_board_index) = board.get_next_board_index() {
            println!("You have to play on the highlighted board.");

            let mut res = None;

            while res.is_none() {
                println!("Input a number between 1 and 9 to play on the corresponding field.");
                println!(
                    "Only these fields are valid: {}",
                    possible_moves
                        .iter()
                        .map(|&value| (value % 9 + 1).to_string())
                        .sorted()
                        .join(", ")
                );

                let mut input = String::new();

                std::io::stdin().read_line(&mut input).unwrap();

                let input = input.trim().parse::<u8>();

                res = match input {
                    Ok(value) => {
                        if value < 10 {
                            let mapped_value = value - 1 + next_board_index * 9;

                            if possible_moves.contains(&mapped_value) {
                                Some(mapped_value)
                            } else {
                                println!("Invalid move. Please try again.");
                                None
                            }
                        } else {
                            println!("Invalid input. Please try again.");
                            None
                        }
                    }
                    Err(_) => {
                        println!("Invalid input. Please try again.");
                        None
                    }
                }
            }

            res
        } else {
            println!("You can play on any board.");

            let mut selected_board: Option<u8> = None;

            while selected_board.is_none() {
                println!("Input a number between 1 and 9 to play on the corresponding board.");
                println!(
                    "Only these boards are valid: {}",
                    board
                        .get_board_status()
                        .iter()
                        .enumerate()
                        .filter(|(_, &status)| status == GameResult::Continue)
                        .map(|(index, _)| (index + 1).to_string())
                        .sorted()
                        .join(", ")
                );

                let mut input = String::new();

                std::io::stdin().read_line(&mut input).unwrap();

                let input = input.trim().parse::<u8>();

                selected_board = match input {
                    Ok(value) => {
                        if value < 10 {
                            let mapped_value = value - 1;

                            if board.get_board_status()[mapped_value as usize]
                                == GameResult::Continue
                            {
                                Some(mapped_value)
                            } else {
                                println!("Invalid board. Please try again.");
                                None
                            }
                        } else {
                            println!("Invalid input. Please try again.");
                            None
                        }
                    }
                    Err(_) => {
                        println!("Invalid input. Please try again.");
                        None
                    }
                }
            }

            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            HumanAgent::print_board(board, selected_board);

            let mut res = None;

            while res.is_none() {
                println!("Input a number between 1 and 9 to play on the corresponding field.");
                println!(
                    "Only these fields are valid: {}",
                    possible_moves
                        .iter()
                        .filter(|&value| value >= &(selected_board.unwrap() * 9)
                            && value < &((selected_board.unwrap() + 1) * 9))
                        .map(|&value| (value % 9 + 1).to_string())
                        .sorted()
                        .join(", ")
                );

                let mut input = String::new();

                std::io::stdin().read_line(&mut input).unwrap();

                let input = input.trim().parse::<u8>();

                res = match input {
                    Ok(value) => {
                        if value < 10 {
                            let mapped_value = value - 1 + selected_board.unwrap() * 9;

                            if possible_moves.contains(&mapped_value) {
                                Some(mapped_value)
                            } else {
                                println!("Invalid move. Please try again.");
                                None
                            }
                        } else {
                            println!("Invalid input. Please try again.");
                            None
                        }
                    }
                    Err(_) => {
                        println!("Invalid input. Please try again.");
                        None
                    }
                }
            }

            res
        }
    }

    fn get_info(&self) -> AgentInfo {
        AgentInfo::new("Human".to_string(), self.player, self.turn, "".to_string())
    }
}

/// # Starts a game with a human player.
///
/// The human player can input the moves via the console.
pub fn start_game_with_human() {
    let mut game = Game::new(
        Box::new(HumanAgent::default()),
        //Box::new(MiniMaxAgent::new(8, 1, CustomHeuristic::new(Player::Two))),
        Box::new(MonteCarloTreeAgent::new(10000)),
    );
    HumanAgent::print_board(game.get_board().clone(), None);
    println!("Result: {:?}", game.play());
}
