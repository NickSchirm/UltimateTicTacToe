//! # Contains code to test the runtime of the agents

use crate::agent::benched::BenchedAgent;
use crate::agent::minimax_agent::MiniMaxAgent;
use crate::agent::monte_carlo_tree_agent::MonteCarloTreeAgent;
use crate::agent::random_agent::RandomAgent;
use crate::game::player::Player::{One, Two};
use crate::game::Game;
use crate::heuristic::custom_heuristic::CustomHeuristic;
use csv::Writer;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::ops::{Range, RangeInclusive};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const NUM_GAMES: u32 = 100;

const MIN_DEPTH: u32 = 7;
const MAX_DEPTH: u32 = 7;

const MIN_QUIESCENCE_SEARCH_DEPTH: u32 = 0;
const MAX_QUIESCENCE_SEARCH_DEPTH: u32 = 4;

const RANGE: RangeInclusive<u32> = 1000..=10000;
const RANGE_STEP: usize = 1000;

pub fn run() {
    let writer = Arc::new(Mutex::new(
        Writer::from_path("sh qs 1-3.csv").expect("Could not create CSV writer"),
    ));

    for depth in MIN_DEPTH..=MAX_DEPTH {
        for quiescence_search_depth in MIN_QUIESCENCE_SEARCH_DEPTH..=MAX_QUIESCENCE_SEARCH_DEPTH {
            let mut games = vec![];

            for _ in 0..NUM_GAMES {
                let agent1 =
                    MiniMaxAgent::new(depth, quiescence_search_depth, CustomHeuristic::new(One));
                let agent2 = RandomAgent::new();

                games.push(Game::new(
                    Box::new(BenchedAgent::new(writer.clone(), agent1)),
                    Box::new(agent2),
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

    // for i in RANGE.step_by(RANGE_STEP) {
    //     let mut games = vec![];
    //
    //     for _ in 0..NUM_GAMES {
    //         let agent1 = MonteCarloTreeAgent::new(i);
    //         let agent2 = RandomAgent::new();
    //
    //         games.push(Game::new(
    //             Box::new(BenchedAgent::new(writer.clone(), agent1)),
    //             Box::new(agent2),
    //         ));
    //     }
    //
    //     let pre_run = Instant::now();
    //
    //     games.par_iter_mut().for_each(|game| {
    //         game.play();
    //     });
    //
    //     let duration = pre_run.elapsed();
    //
    //     println!("Iterations: {}, Duration: {:?}", i, duration);
    // }
}
