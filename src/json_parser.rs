/*
TODO: Continue working on saving multiple notes/states. My current idea is:
TODO:       StateList stores the title and content of multiple states in a hashmap.
TODO:       When you open the sideback, a list of buttons, which content is a state title for each will be shown.
TODO:       Clicking one will look at the statelist and deserialize the string value which houses the content of the AppState
TODO:       Then the right side will automatically display the content of the AppState.
TODO:       Only StateList will be saved on persistent storage during the save function, and read during the new function.
*/

use std::{fs::File, io::{self, Read, Write}, path::Path};
use serde_json;
use crate::todo_func::{AppState, StateList, Theme};

const STATE_LIST_KEY: &str = "state_list";
const CONFIG_PATH: &str = "config.json";

pub fn read_state_list(cc: &eframe::CreationContext<'_>) -> io::Result<StateList> {
    if let Some(storage) = cc.storage{
        if let Some(list) = storage.get_string(STATE_LIST_KEY){
            return serde_json::from_str(&list)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
        }
    } 
    Ok(StateList::default())
}

pub fn save_state_list(state_list: &StateList, storage: &mut dyn eframe::Storage) -> io::Result<()> {
    match serde_json::to_string_pretty(state_list) {
        Ok(json) => {
            storage.set_string(STATE_LIST_KEY, json);
            Ok(())
        },
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
    }
}

pub fn read_theme() -> io::Result<Theme> {
    if Path::new(CONFIG_PATH).exists() {
        let mut file = File::open(CONFIG_PATH)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        Ok(Theme::default())
    }
}

pub fn save_theme(theme: &Theme) -> io::Result<()> {
    let json = serde_json::to_string_pretty(theme)?;
    let mut file = File::create(CONFIG_PATH)?;
    file.write_all(json.as_bytes())
}

pub fn state_to_json_string(state: &AppState) -> String {
    serde_json::to_string_pretty(state).unwrap_or_default()
}

pub fn json_string_to_state(json: &Option<&String>) -> io::Result<AppState> {
    if let Some(text) = json {
        return serde_json::from_str(text)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
    }
    Ok(AppState::default())
}
