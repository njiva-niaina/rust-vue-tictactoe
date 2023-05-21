// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;

use crate::cmd::{draw, reset};
use std::sync::Mutex;
use tictactoe::Node;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let morpion = Mutex::new(Node::new());
    tauri::Builder::default()
        .manage(morpion)
        .invoke_handler(tauri::generate_handler![greet, draw, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
