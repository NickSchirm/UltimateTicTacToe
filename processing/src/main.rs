//! This module contains data processing functions for the benchmarking results.
//!
//! The results are stored in CSV files and can be processed to generate statistics.
mod multiple_depths;
mod single_depth;
mod turnier;

fn main() {
    println!("Select the processing mode:");
    println!("1. Single Depth");
    println!("2. Multiple Depths");
    println!("3. Turnier");
    println!("4. Turnier All");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    let num = input.trim().parse::<u32>().expect("Could not parse input");

    match num {
        1 => single_depth::process(),
        2 => multiple_depths::process(),
        3 => {
            let (input_path, output_path, name_of_first_player) = turnier::read_from_console();
            turnier::process(input_path, output_path, name_of_first_player);
        }
        4 => turnier::multi_process(),
        _ => println!("Invalid input"),
    }
}
