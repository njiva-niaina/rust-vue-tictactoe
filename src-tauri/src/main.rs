// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;

use cmd::{make_move, reset};
use std::sync::Mutex;
use tictactoe::Node;

fn main() {
    let node = Mutex::new(Node::new());
    tauri::Builder::default()
        .manage(node)
        .invoke_handler(tauri::generate_handler![make_move, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
