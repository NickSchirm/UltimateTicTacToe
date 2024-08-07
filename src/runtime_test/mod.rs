use crate::agent::benched::BenchedAgent;
use crate::agent::minimax_agent::MiniMaxAgent;
use crate::game::player::Player::{One, Two};
use crate::game::Game;
use crate::heuristic::custom_heuristic::CustomHeuristic;
use csv::Writer;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};
use std::time::Instant;

const NUM_GAMES: u32 = 100;

const MIN_DEPTH: u32 = 1;
const MAX_DEPTH: u32 = 8;

const MIN_QUIESCENCE_SEARCH_DEPTH: u32 = 1;
const MAX_QUIESCENCE_SEARCH_DEPTH: u32 = 1;

pub fn run() {
    let writer = Arc::new(Mutex::new(
        Writer::from_path("res.csv").expect("Could not create CSV writer"),
    ));

    for depth in MIN_DEPTH..=MAX_DEPTH {
        for quiescence_search_depth in MIN_QUIESCENCE_SEARCH_DEPTH..=MAX_QUIESCENCE_SEARCH_DEPTH {
            let mut games = vec![];

            for _ in 0..NUM_GAMES {
                let agent1 =
                    MiniMaxAgent::new(depth, quiescence_search_depth, CustomHeuristic::new(One));
                let agent2 =
                    MiniMaxAgent::new(depth, quiescence_search_depth, CustomHeuristic::new(Two));

                games.push(Game::new(
                    Box::new(BenchedAgent::new(writer.clone(), agent1)),
                    Box::new(BenchedAgent::new(writer.clone(), agent2)),
                ));
            }

            let pre_run = Instant::now();

            games.par_iter_mut().for_each(|game| {
                game.play();
            });

            let duration = pre_run.elapsed();

            println!(
                "Depth: {}, Quiescence Search Depth: {}, Duration: {:?}",
                depth, quiescence_search_depth, duration
            );
        }
    }
}
