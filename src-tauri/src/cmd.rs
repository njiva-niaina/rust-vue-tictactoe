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

#[tauri::command]
pub fn find_best_move(node: State<Mutex<Node>>, player: i32) -> Result<usize, String> {
    Ok(node.lock().unwrap().find_best_move(player))
}
