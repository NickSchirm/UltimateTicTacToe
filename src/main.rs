use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefMutIterator;

use hausarbeit::custom_heuristic::CustomHeuristic;
use hausarbeit::game::Game;
use hausarbeit::game_result::GameResult;
use hausarbeit::genetic_algorithm::gene::Gene;
use hausarbeit::genetic_algorithm::mutations::normal_distribution_mutation::NormalDistributionMutation;
use hausarbeit::genetic_algorithm::mutations::shift_mutation::ShiftMutation;
use hausarbeit::genetic_algorithm::recombinations::one_point_crossover::OnePointCrossover;
use hausarbeit::genetic_algorithm::selections::roulette_wheel_selection::RouletteWheelSelection;
use hausarbeit::genetic_algorithm::GeneticAlgorithm;
use hausarbeit::minimax_agent::MiniMaxAgent;
use hausarbeit::parameterized_heuristic::{ParameterizedHeuristic, NUM_FEATURES};
use hausarbeit::player::Player::{One, Two};

const NUM_GAMES: u32 = 100;
const DEPTH: u32 = 4;
const QUIESCENCE_SEARCH_DEPTH: u32 = 3;

fn main() {
    //hausarbeit::human_agent::start_game_with_human();

    rayon::ThreadPoolBuilder::new()
        .num_threads(7)
        .build_global()
        .unwrap();

    let mut genes = vec![];

    for _ in 0..10 {
        genes.push(Gene::new(NUM_FEATURES));
    }

    let mut genetic_algorithm = GeneticAlgorithm::new(
        100,
        genes,
        Box::new(RouletteWheelSelection {}),
        Box::new(NormalDistributionMutation::new(0.1)),
        Box::new(OnePointCrossover {}),
        2,
        1,
    );

    genetic_algorithm.run();
    //
    // let mut wins = [0, 0, 0];
    // let mut games = vec![];
    //
    // for _ in 0..NUM_GAMES {
    //     let agent1 = MiniMaxAgent::new(DEPTH, QUIESCENCE_SEARCH_DEPTH, CustomHeuristic::new(One));
    //     let agent2 = MiniMaxAgent::new(
    //         DEPTH,
    //         QUIESCENCE_SEARCH_DEPTH,
    //         ParameterizedHeuristic::new(Two, Gene::new(NUM_FEATURES).get_values()),
    //     );
    //
    //     games.push(Game::new(Box::new(agent1), Box::new(agent2)));
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
