use std::collections::HashMap;

use csv::{ReaderBuilder, Writer};

use hausarbeit::agent::benched::Row;
use hausarbeit::game::player::Player;

pub fn read_from_console() -> (String, String, String) {
    println!("Enter the path to the input CSV file:");

    let mut input_path = String::new();
    std::io::stdin()
        .read_line(&mut input_path)
        .expect("Could not read line");

    println!();

    println!("Enter the path to the output CSV file:");

    let mut output_path = String::new();
    std::io::stdin()
        .read_line(&mut output_path)
        .expect("Could not read line");

    println!();
    println!("Enter the name of the first player:");

    let mut name_of_first_player = String::new();
    std::io::stdin()
        .read_line(&mut name_of_first_player)
        .expect(&format!("Could not read line for {}", name_of_first_player));

    (
        input_path.trim().to_string(),
        output_path.trim().to_string(),
        name_of_first_player.trim().to_string(),
    )
}

pub fn multi_process() {
    let tasks = vec![
        ("sh vs ph", "p", "SH"),
        ("sh vs mh", "p", "SH"),
        ("sh vs mcts", "p", "SH"),
        ("sh vs rand", "p", "SH"),
        ("ph vs sh", "p", "PH"),
        ("ph vs mh", "p", "PH"),
        ("ph vs mcts", "p", "PH"),
        ("ph vs rand", "p", "PH"),
        ("mh vs sh", "p", "MH"),
        ("mh vs ph", "p", "MH"),
        ("mh vs mcts", "p", "MH"),
        ("mh vs rand", "p", "MH"),
        ("mcts vs sh", "p", "MCTS"),
        ("mcts vs ph", "p", "MCTS"),
        ("mcts vs mh", "p", "MCTS"),
        ("mcts vs rand", "p", "MCTS"),
        ("rand vs sh", "p", "RAND"),
        ("rand vs ph", "p", "RAND"),
        ("rand vs mh", "p", "RAND"),
        ("rand vs mcts", "p", "RAND"),
    ];

    for (input_path, output_path, name_of_first_player) in tasks {
        process(
            input_path.to_string(),
            output_path.to_string(),
            name_of_first_player.to_string(),
        );
    }
}

pub fn process(input_path: String, output_path: String, name_of_first_player: String) {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&format!("{}.csv", &input_path))
        .expect(&format!("Could not create CSV reader for {}", input_path));

    let mut writer = Writer::from_path(&format!("{}/{}.csv", &output_path, &input_path))
        .expect(&format!("Could not create CSV writer for {}", output_path));

    let mut map: HashMap<(Player, u32), Vec<Row>> = HashMap::new();

    for result in reader.records() {
        if let Ok(res) = result {
            let row: Row = res.deserialize(None).expect("Could not deserialize row");

            let key = (row.player, row.turn_num);

            match map.get_mut(&key) {
                Some(vec) => vec.push(row),
                None => {
                    map.insert(key, vec![row]);
                }
            }
        }
    }

    let mut player_maps: Vec<HashMap<u32, Vec<Row>>> = vec![HashMap::new(), HashMap::new()];

    for ((player, turn_num), row) in map {
        if player == Player::One {
            player_maps[0].insert(turn_num, row);
        } else {
            player_maps[1].insert(turn_num, row);
        }
    }

    println!("The player maps have been created successfully.");

    let mut turn_map = HashMap::new();

    for (turn_num, row) in player_maps[0].iter() {
        let mut total = 0;
        let row_len = row.len();

        for r in row {
            total += r.duration;
        }

        let avg = total / row_len as u128;

        turn_map.insert(*turn_num, avg);
    }

    println!("The turn map for the first player has been created successfully.");

    writer
        .write_field("turn_num")
        .expect("Could not write field");
    writer
        .write_field(name_of_first_player)
        .expect("Could not write field");
    writer
        .write_record(None::<&[u8]>)
        .expect("Could not write record");

    for (turn_num, avg) in turn_map {
        writer
            .write_field(turn_num.to_string())
            .expect("Could not write field");
        writer
            .write_field(avg.to_string())
            .expect("Could not write field");
        writer
            .write_record(None::<&[u8]>)
            .expect("Could not write record");
    }

    println!("The turn map for the first player has been written to the output CSV file.");
}
