#![allow(unused_imports)]

use csv::Writer;
use hausarbeit::agent::benched::BenchedAgent;
use hausarbeit::agent::minimax_agent::MiniMaxAgent;
use hausarbeit::agent::monte_carlo_tree_agent::MonteCarloTreeAgent;
use hausarbeit::agent::random_agent::RandomAgent;
use hausarbeit::agent::random_start::RandomStartAgent;
use hausarbeit::agent::Agent;
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
use hausarbeit::heuristic::monte_carlo_game_search_heuristic::MonteCarloGameSearchHeuristic;
use hausarbeit::heuristic::parameterized_heuristic::{ParameterizedHeuristic, NUM_FEATURES};
use hausarbeit::{agent, genetic_algorithm, runtime_test};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefMutIterator;
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const NUM_GAMES: u32 = 100;
const DEPTH: u32 = 8;
const QUIESCENCE_SEARCH_DEPTH: u32 = 1;

fn main() {
    //agent::human_agent::start_game_with_human();

    //agent::human_agent::human_against_human();

    rayon::ThreadPoolBuilder::new()
        .num_threads(7)
        .build_global()
        .unwrap();

    //runtime_test::run();

    //agent::monte_carlo_tree_agent::run();

    run();

    //genetic_algorithm::run();
}

fn run() {
    let mut wins = [0, 0, 0];
    let mut games = vec![];

    let writer = Arc::new(Mutex::new(
        Writer::from_path("rand vs mcts.csv").expect("Could not create CSV writer"),
    ));

    // MiniMaxAgent::new(DEPTH, QUIESCENCE_SEARCH_DEPTH, CustomHeuristic::new(One))
    // MiniMaxAgent::new(DEPTH, QUIESCENCE_SEARCH_DEPTH, ParameterizedHeuristic::withLookUpTable(One, vec![-0.9011298820760223, -0.9047473011303433, -1.9878186210206341, -0.940735228598089, 1.3140632491937836, 0.5190040302978252, 0.7128491119909083, 1.2756963483965846, 2.264309782234436, 0.14115748887705593, 1.2441779567914344, 2.0944754371556287]))
    // MiniMaxAgent::new(3, 1, MonteCarloGameSearchHeuristic::new(One, 10))
    // MonteCarloTreeAgent::new(10000)
    // RandomAgent::new()

    for _ in 0..NUM_GAMES {
        let agent1 =
            RandomStartAgent::new(2, BenchedAgent::new(writer.clone(), RandomAgent::new()));
        let agent2 = RandomStartAgent::new(
            2,
            BenchedAgent::new(writer.clone(), MonteCarloTreeAgent::new(10000)),
        );

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
