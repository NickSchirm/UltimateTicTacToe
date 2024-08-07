//! Single search depth processing module.
use csv::{ReaderBuilder, Writer};
use hausarbeit::agent::benched::Row;
use serde::Serialize;
use std::collections::HashMap;

/// Processes the benchmarking results for a single search depth.
/// 
/// Reads from '../res.csv' and groups the results by configuration and turn number.
/// 
/// The results are stored in '../res_single.csv'.
pub fn process() {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("../res.csv")
        .expect("Could not create CSV reader");
    let mut writer = Writer::from_path("../res_single.csv").expect("Could not create CSV writer");
    let mut map = HashMap::new();

    for result in reader.records() {
        if let Ok(record) = result {
            let row: Row = record.deserialize(None).expect("Could not deserialize row");

            if !map.contains_key(&(row.config.clone(), row.turn_num)) {
                map.insert((row.config.clone(), row.turn_num), vec![]);
            }

            map.get_mut(&(row.config.clone(), row.turn_num))
                .unwrap()
                .push(row);
        }
    }

    for ((config, turn_num), row) in map {
        let mut total = 0;
        let mut min = u128::MAX;
        let mut max = 0;
        let mut first_quartile = 0;
        let mut second_quartile = 0;
        let mut third_quartile = 0;
        let row_len = row.len();

        for (i, r) in row.iter().enumerate() {
            total += r.duration;
            min = min.min(r.duration);
            max = max.max(r.duration);

            if i == row_len / 4usize {
                first_quartile = r.duration;
            }

            if i == row_len / 2usize {
                second_quartile = r.duration;
            }

            if i == row_len / 4usize * 3usize {
                third_quartile = r.duration;
            }
        }

        let avg = total / row_len as u128;

        writer
            .serialize(RowWithStats {
                config,
                turn_num,
                avg,
                min,
                max,
                first_quartile,
                median: second_quartile,
                third_quartile,
            })
            .expect("Could not write row");
    }
}

#[derive(Serialize)]
struct RowWithStats {
    config: String,
    turn_num: u32,
    avg: u128,
    min: u128,
    max: u128,
    first_quartile: u128,
    median: u128,
    third_quartile: u128,
}
