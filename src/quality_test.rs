use crate::agent::benched::BenchedAgent;
use crate::agent::minimax_agent::MiniMaxAgent;
use crate::agent::monte_carlo_tree_agent::MonteCarloTreeAgent;
use crate::agent::random_agent::RandomAgent;
use crate::agent::random_start::RandomStartAgent;
use crate::game::game_result::GameResult;
use crate::game::game_result::GameResult::{Draw, Win};
use crate::game::player::Player::{One, Two};
use crate::game::Game;
use crate::heuristic::custom_heuristic::CustomHeuristic;
use csv::Writer;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};
use std::time::Instant;

const NUM_GAMES: u32 = 100;

const DEPTH: u32 = 7;

const MIN_QUIESCENCE_SEARCH_DEPTH: u32 = 0;
const MAX_QUIESCENCE_SEARCH_DEPTH: u32 = 4;

pub fn run() {
    let writer = Arc::new(Mutex::new(
        Writer::from_path("sh qs win stats 0-4.csv").expect("Could not create CSV writer"),
    ));

    writer
        .lock()
        .unwrap()
        .write_record(&[
            "Configuration",
            "Player One Wins",
            "Player Two Wins",
            "Draws",
        ])
        .expect("Could not write record");

    for quiescence_search_depth in MIN_QUIESCENCE_SEARCH_DEPTH..=MAX_QUIESCENCE_SEARCH_DEPTH {
        let mut games = vec![];

        for _ in 0..NUM_GAMES {
            let agent1 =
                MiniMaxAgent::new(DEPTH, quiescence_search_depth, CustomHeuristic::new(One));
            let agent2 = MonteCarloTreeAgent::new(10000);

            games.push(Game::new(
                Box::new(RandomStartAgent::new(2, agent1)),
                Box::new(agent2),
            ));
        }

        let mut stats = vec![0; NUM_GAMES as usize];

        let pre_run = Instant::now();

        let results: Vec<GameResult> = games.par_iter_mut().map(|game| game.play()).collect();

        for result in results {
            match result {
                Win(One) => stats[0] += 1,
                Win(Two) => stats[1] += 1,
                Draw => stats[2] += 1,
                _ => (),
            }
        }

        let duration = pre_run.elapsed();

        writer
            .lock()
            .unwrap()
            .write_record(&[
                format!("{}+{}", DEPTH, quiescence_search_depth),
                stats[0].to_string(),
                stats[1].to_string(),
                stats[2].to_string(),
            ])
            .expect("Could not write record");

        println!(
            "Depth: {}, Quiescence Search Depth: {}, Duration: {:?}",
            DEPTH, quiescence_search_depth, duration
        );
    }
}
