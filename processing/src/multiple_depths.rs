//! Multiple search depths processing module.
use csv::{ReaderBuilder, Writer};
use hausarbeit::agent::benched::Row;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// Processes the benchmarking results for multiple search depths.
/// 
/// Reads from '../res.csv' and groups the results by configuration and turn number.
/// 
/// The results are stored in '../res_multiple.csv'.
pub fn process() {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("../res.csv")
        .expect("Could not create CSV reader");
    let mut writer = Writer::from_path("../res_multiple.csv").expect("Could not create CSV writer");
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

    let mut turn_map = HashMap::new();

    for ((config, turn_num), row) in map {
        let mut total = 0;
        let row_len = row.len();

        for r in row {
            total += r.duration;
        }

        let avg = total / row_len as u128;

        if !turn_map.contains_key(&turn_num) {
            turn_map.insert(turn_num, HashMap::new());
        }
        turn_map.get_mut(&turn_num).unwrap().insert(config, avg);
    }

    let mut expected_length = 0;

    for (_, config_map) in turn_map.clone() {
        let actual_length = config_map.len();

        if actual_length > expected_length {
            expected_length = actual_length;
        }
    }

    let mut config_set = turn_map
        .iter()
        .map(|(_, config_map)| config_map.keys())
        .flatten()
        .collect::<HashSet<&String>>();

    writer.write_field("Turn").expect("Could not write field");
    for config in config_set.iter().sorted_by(|a, b| a.cmp(b)) {
        writer.write_field(config).expect("Could not write field");
    }
    writer
        .write_record(None::<&[u8]>)
        .expect("Could not write record");

    for (turn_num, config_map) in turn_map {
        writer
            .write_field(turn_num.to_string())
            .expect("Could not write field");
        let mut actual_length = 0;
        for (_, avg) in config_map.iter().sorted_by(|a, b| a.0.cmp(b.0)) {
            writer
                .write_field(avg.to_string())
                .expect("Could not write field");
            actual_length += 1;
        }
        for _ in 0..(expected_length - actual_length) {
            writer.write_field("").expect("Could not write field");
        }
        writer
            .write_record(None::<&[u8]>)
            .expect("Could not write record");
    }
}
