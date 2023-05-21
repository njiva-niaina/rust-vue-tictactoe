// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;

use crate::cmd::{draw, reset};
use std::sync::Mutex;
use tictactoe::Node;

fn main() {
    let morpion = Mutex::new(Node::new());
    tauri::Builder::default()
        .manage(morpion)
        .invoke_handler(tauri::generate_handler![draw, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
