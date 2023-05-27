use std::{collections::HashMap, sync::Mutex};
use tauri::State;

use tictactoe::Node;

#[tauri::command]
pub fn make_move(node: State<Mutex<Node>>, idx: usize) -> Result<HashMap<String, i32>, String> {
    Ok(node.lock().unwrap().make_move(idx))
}

#[tauri::command]
pub fn reset(node: State<Mutex<Node>>) -> Result<Node, String> {
    Ok(node.lock().unwrap().reset())
}
