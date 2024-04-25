use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    pub long_name: String,
    pub short_name: String,
    pub cmd: String,
}

impl Process {
    pub(crate) fn new(long_name: &str, short_name: &str, cmd: &str) -> Self {
        Process {
            long_name: long_name.to_string(),
            short_name: short_name.to_string(),
            cmd: cmd.to_string(),
        }
    }
}

pub(crate) struct ProcessLoader {}

impl ProcessLoader {
    pub(crate) fn load() -> Result<Vec<Process>, Box<dyn std::error::Error>> {
        let mut file = File::open(crate::config::CMD_LOADER_PATH)
            .map_err(|_| format!("{} not found", crate::config::CMD_LOADER_PATH))?;
        let mut raw_data = String::new();
        file.read_to_string(&mut raw_data)
            .map_err(|_| "Failed to load cmd.json")?;
        let processes_vector: Vec<Process> =
            serde_json::from_str(&raw_data).map_err(|_| "Failed to process cmd.json")?;
        Ok(processes_vector)
    }
}