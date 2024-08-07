#![allow(unused_imports)]

use csv::Writer;
use hausarbeit::agent::benched::BenchedAgent;
use hausarbeit::agent::minimax_agent::MiniMaxAgent;
use hausarbeit::game::game_result::GameResult;
use hausarbeit::game::player::Player::{One, Two};
use hausarbeit::game::Game;
use hausarbeit::genetic_algorithm::fitness::full_ordering_fitness::FullOrderingFitness;
use hausarbeit::genetic_algorithm::gene::Gene;
use hausarbeit::genetic_algorithm::mutation::normal_distribution_mutation::NormalDistributionMutation;
use hausarbeit::genetic_algorithm::mutation::shift_mutation::ShiftMutation;
use hausarbeit::genetic_algorithm::recombination::one_point_crossover::OnePointCrossover;
use hausarbeit::genetic_algorithm::selection::roulette_wheel_selection::RouletteWheelSelection;
use hausarbeit::genetic_algorithm::GeneticAlgorithm;
use hausarbeit::heuristic::custom_heuristic::CustomHeuristic;
use hausarbeit::heuristic::parameterized_heuristic::{ParameterizedHeuristic, NUM_FEATURES};
use hausarbeit::runtime_test;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefMutIterator;
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const NUM_GAMES: u32 = 100;
const DEPTH: u32 = 7;
const QUIESCENCE_SEARCH_DEPTH: u32 = 3;

fn main() {
    //hausarbeit::human_agent::start_game_with_human();

    rayon::ThreadPoolBuilder::new()
        .num_threads(7)
        .build_global()
        .unwrap();

    runtime_test::run();

    // let mut genes = vec![];
    //
    // for _ in 0..10 {
    //     genes.push(Gene::new(NUM_FEATURES));
    // }
    //
    // let mut genetic_algorithm = GeneticAlgorithm::new(
    //     10,
    //     genes,
    //     Box::new(FullOrderingFitness::new(4, 1)),
    //     Box::new(RouletteWheelSelection {}),
    //     Box::new(NormalDistributionMutation::new(0.1)),
    //     Box::new(OnePointCrossover {}),
    // );
    //
    // genetic_algorithm.run();

    // let mut wins = [0, 0, 0];
    // let mut games = vec![];
    //
    // let writer = Arc::new(Mutex::new(Writer::from_path("res.csv").expect("Could not create CSV writer")));
    //
    // for _ in 0..NUM_GAMES {
    //     let agent1 = MiniMaxAgent::new(DEPTH, QUIESCENCE_SEARCH_DEPTH, CustomHeuristic::new(One));
    //     let agent2 = MiniMaxAgent::new(DEPTH, QUIESCENCE_SEARCH_DEPTH, CustomHeuristic::new(Two));
    //
    //     games.push(Game::new(Box::new(BenchedAgent::new(writer.clone(), agent1)), Box::new(BenchedAgent::new(writer.clone(), agent2))));
    // }
    //
    // let counter = AtomicUsize::new(0);
    //
    // let pre_run = Instant::now();
    //
    // games
    //     .par_iter_mut()
    //     .map(|game| {
    //         let result = game.play();
    //
    //         counter.fetch_add(1, Ordering::Relaxed);
    //         println!(
    //             "{:.2}% of games finished",
    //             counter.load(Ordering::Relaxed) as f64 / NUM_GAMES as f64 * 100.
    //         );
    //         println!("{:?}", result);
    //
    //         result
    //     })
    //     .collect::<Vec<GameResult>>()
    //     .iter()
    //     .for_each(|result| match result {
    //         GameResult::Win(player) => wins[*player as usize] += 1,
    //         GameResult::Draw => wins[2] += 1,
    //         _ => {}
    //     });
    //
    // println!("\nResults:");
    // println!(
    //     "Player 1 won {:.2}% of the time",
    //     (wins[0] as f64 / NUM_GAMES as f64) * 100.
    // );
    // println!(
    //     "Player 2 won {:.2}% of the time",
    //     (wins[1] as f64 / NUM_GAMES as f64) * 100.
    // );
    // println!(
    //     "Draws: {:.2}% of the time",
    //     (wins[2] as f64 / NUM_GAMES as f64) * 100.
    // );
    // println!("Time taken: {:?} seconds", pre_run.elapsed().as_secs_f64());
}
