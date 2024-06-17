// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;
mod pilot;
mod serial;

use std::thread;
use std::time::Duration;

use chrono::Utc;
use tauri::Manager;

use self::file::*;
use self::pilot::*;
use self::serial::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            thread::spawn(move || {
                loop {
                    app_handle.emit_all(
                        "display-battery",
                        BatteryBench {
                            id: 0,
                            port: "COM 4".to_string(),
                            temperature: 2020,
                            battery_temperature: 2013,
                            electronic_load_temperature: 2054,
                            voltage: 534,
                            current: 324,
                            state: BatteryBenchState::Standby,
                            status: CompletionStatus::InProgress,
                            start_date: Utc::now(),
                            end_date: Utc::now(),
                        }
                    ).unwrap();
                    thread::sleep(Duration::from_secs(1)); // Adjust the interval as needed
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
