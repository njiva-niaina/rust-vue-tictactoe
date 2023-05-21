use std::{collections::HashMap, sync::Mutex};

use tauri::State;
use tictactoe::{DrawResult, Node, ResetResult};

#[tauri::command]
pub fn draw(
    morpion: State<Mutex<Node>>,
    idx: usize,
) -> Result<HashMap<String, DrawResult>, String> {
    Ok(morpion.lock().unwrap().draw(idx))
}

#[tauri::command]
pub fn reset(morpion: State<Mutex<Node>>) -> Result<HashMap<String, ResetResult>, String> {
    Ok(morpion.lock().unwrap().reset())
}
