use crate::agent::{Agent, AgentInfo};
use crate::game::player::Player;
use crate::game::ultimate_board::UltimateBoard;
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::{Arc, Mutex};

pub struct BenchedAgent<A> {
    agent: A,
    writer: Arc<Mutex<Writer<File>>>,
}

impl<A: Agent> BenchedAgent<A> {
    pub fn new(writer: Arc<Mutex<Writer<File>>>, agent: A) -> BenchedAgent<A> {
        BenchedAgent { agent, writer }
    }
}

impl<A: Agent> Agent for BenchedAgent<A> {
    fn act(&mut self, board: UltimateBoard, player: Player, turn: u32) -> Option<u8> {
        let start = std::time::Instant::now();
        let result = self.agent.act(board, player, turn);
        let duration = start.elapsed();
        println!("Duration: {:?}", duration);

        let acquired_lock = self.writer.lock();

        match acquired_lock {
            Ok(mut w) => w
                .serialize(Row::from_info(self.agent.get_info(), duration.as_micros()))
                .expect("Could not write row"),
            Err(e) => e
                .into_inner()
                .serialize(Row::from_info(self.agent.get_info(), duration.as_micros()))
                .expect("Could not write row"),
        }

        result
    }

    fn get_info(&self) -> AgentInfo {
        let sub_info = self.agent.get_info();

        AgentInfo::new(
            format!("Benched({})", sub_info.name),
            sub_info.player,
            sub_info.turn_num,
            sub_info.config.clone(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Row {
    pub name: String,
    pub player: Player,
    pub turn_num: u32,
    pub config: String,
    pub duration: u128,
}

impl Row {
    fn from_info(info: AgentInfo, duration: u128) -> Row {
        Row {
            name: info.name,
            player: info.player,
            turn_num: info.turn_num,
            config: info.config,
            duration,
        }
    }
}
