use std::{fs::File, io::{self, Read, Write}, path::Path};
use crate::todo_func::AppState;

const FILE_PATH: &str = "entries.json";

pub fn read_state_from_file() -> io::Result<AppState> {
    if Path::new(FILE_PATH).exists() {
        let mut file = File::open(FILE_PATH)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        Ok(AppState::default())
    }
}

pub fn save_state_to_file(state: &AppState) -> io::Result<()> {
    let json = serde_json::to_string_pretty(state)?;
    let mut file = File::create(FILE_PATH)?;
    file.write_all(json.as_bytes())
}