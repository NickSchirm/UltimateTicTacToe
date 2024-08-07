use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefMutIterator;

use hausarbeit::game::Game;
use hausarbeit::game_result::GameResult;
use hausarbeit::minimax_agent::MiniMaxAgent;
use hausarbeit::monte_carlo_game_search_heuristic::MonteCarloGameSearchHeuristic;
use hausarbeit::player::Player::One;
use hausarbeit::random_agent::RandomAgent;

const NUM_GAMES: u32 = 10;
const DEPTH: u32 = 4;
const QUIESCENCE_SEARCH_DEPTH: u32 = 1;

fn main() {
    //hausarbeit::human_agent::start_game_with_human();

    rayon::ThreadPoolBuilder::new()
        .num_threads(7)
        .build_global()
        .unwrap();

    let mut wins = [0, 0, 0];
    let mut games = vec![];

    for _ in 0..NUM_GAMES {
        let agent1 = MiniMaxAgent::new(
            DEPTH,
            QUIESCENCE_SEARCH_DEPTH,
            MonteCarloGameSearchHeuristic::new(One, 10),
        );
        let agent2 = RandomAgent::new();

        games.push(Game::new(Box::new(agent1), Box::new(agent2)));
    }

    let counter = AtomicUsize::new(0);

    let pre_run = Instant::now();

    games
        .par_iter_mut()
        .map(|game| {
            let result = game.play();

            counter.fetch_add(1, Ordering::Relaxed);
            println!(
                "{:.2}% of games finished",
                counter.load(Ordering::Relaxed) as f64 / NUM_GAMES as f64 * 100.
            );
            println!("{:?}", result);

            result
        })
        .collect::<Vec<GameResult>>()
        .iter()
        .for_each(|result| match result {
            GameResult::Win(player) => wins[*player as usize] += 1,
            GameResult::Draw => wins[2] += 1,
            _ => {}
        });

    println!("\nResults:");
    println!(
        "Player 1 won {:.2}% of the time",
        (wins[0] as f64 / NUM_GAMES as f64) * 100.
    );
    println!(
        "Player 2 won {:.2}% of the time",
        (wins[1] as f64 / NUM_GAMES as f64) * 100.
    );
    println!(
        "Draws: {:.2}% of the time",
        (wins[2] as f64 / NUM_GAMES as f64) * 100.
    );
    println!("Time taken: {:?} seconds", pre_run.elapsed().as_secs_f64());
}
