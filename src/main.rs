use hausarbeit::custom_heuristic::CustomHeuristic;
use hausarbeit::game::Game;
use hausarbeit::game_result::GameResult;
use hausarbeit::minimax_agent::MiniMaxAgent;
use hausarbeit::player::Player::{One, Two};
use hausarbeit::random_agent::RandomAgent;

const NUM_GAMES: u32 = 100;
const DEPTH: u32 = 8;

fn main() {
    let mut wins = [0, 0, 0];

    for num_game in 0..NUM_GAMES {
        if num_game % 2 == 0 {
            let mut heuristic = CustomHeuristic::new(One);
            let mut agent1 = MiniMaxAgent::new(DEPTH, &mut heuristic);
            let mut agent2 = RandomAgent::new();

            let mut game = Game::new(&mut agent1, &mut agent2);
            let result = game.play();
            match result {
                GameResult::Win(player) => wins[player as usize] += 1,
                GameResult::Draw => wins[2] += 1,
                _ => {}
            }
        } else {
            let mut heuristic = CustomHeuristic::new(Two);
            let mut agent1 = RandomAgent::new();
            let mut agent2 = MiniMaxAgent::new(DEPTH, &mut heuristic);

            let mut game = Game::new(&mut agent2, &mut agent1);
            let result = game.play();
            match result {
                GameResult::Win(player) => wins[((player as usize) + 1) % 2] += 1,
                GameResult::Draw => wins[2] += 1,
                _ => {}
            }
        }

        println!("Game {} finished", num_game);
    }

    println!("\nResults:");
    println!(
        "Player 1 won {}% of the time",
        (wins[0] as f64 / NUM_GAMES as f64) * 100.
    );
    println!(
        "Player 2 won {}% of the time",
        (wins[1] as f64 / NUM_GAMES as f64) * 100.
    );
    println!(
        "Draws: {}% of the time",
        (wins[2] as f64 / NUM_GAMES as f64) * 100.
    );
}
