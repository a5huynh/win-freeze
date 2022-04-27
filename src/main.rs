#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{LogicalSize, Size, Window};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            resize_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn resize_window(window: Window, height: f64) {
    window
        .set_size(Size::Logical(LogicalSize {
            width: 800.0,
            height,
        }))
        .unwrap();
}
