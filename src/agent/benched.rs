//! # Contains the [BenchedAgent] struct
//! 
//! The [BenchedAgent] struct is used to benchmark agents.
use crate::agent::{Agent, AgentInfo};
use crate::game::player::Player;
use crate::game::ultimate_board::UltimateBoard;
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::{Arc, Mutex};

/// # Struct representing an agent that is used to benchmark other agents
/// 
/// The agent wraps another agent and calls the act method of the wrapped agent.
/// 
/// The agent contains a writer to write the benchmark results to a CSV file.
/// 
/// The agent writes a row to the CSV file for each act call.
pub struct BenchedAgent<A> {
    agent: A,
    writer: Arc<Mutex<Writer<File>>>,
}

impl<A: Agent> BenchedAgent<A> {
    /// Creates a new [BenchedAgent] wrapping the given agent and using the given writer
    /// 
    /// # Arguments
    /// * `writer` - The writer to write the benchmark results to
    /// * `agent` - The agent to benchmark
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

/// # Struct representing a row in the CSV file
/// 
/// The row contains the name of the agent, the player, the turn number, the configuration, and the duration of the act call.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Row {
    pub name: String,
    pub player: Player,
    pub turn_num: u32,
    pub config: String,
    pub duration: u128,
}

impl Row {
    /// Creates a new [Row] from the given [AgentInfo] and duration
    /// 
    /// # Arguments
    /// * `info` - The information of the agent
    /// * `duration` - The duration of the act call
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
