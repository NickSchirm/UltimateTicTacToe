// use criterion::{criterion_group, criterion_main, BatchSize, Criterion, PlotConfiguration};
// use criterion::AxisScale::Logarithmic;
// use hausarbeit::agent::minimax_agent::MiniMaxAgent;
// use hausarbeit::game::player::Player::{One, Two};
// use hausarbeit::game::Game;
// use hausarbeit::heuristic::custom_heuristic::CustomHeuristic;

// fn test(c: &mut Criterion) {
//     let mut group = c.benchmark_group("game_custom_vs_custom");
//     group.plot_config(PlotConfiguration::default().summary_scale(Logarithmic));
//
//     for depth in 1..=6 {
//         group.bench_with_input(criterion::BenchmarkId::new("depth", depth), &depth, |b, &depth| {
//             b.iter_batched(
//                 || {
//                     Game::new(
//                         Box::new(MiniMaxAgent::new(depth, 1, CustomHeuristic::new(One))),
//                         Box::new(MiniMaxAgent::new(depth, 1, CustomHeuristic::new(Two))),
//                     )
//                 },
//                 |mut game| game.play(),
//                 BatchSize::LargeInput,
//             )
//         });
//     }
//
//     group.finish();
// }
//
// criterion_group!(benches, test);
// criterion_main!(benches);
