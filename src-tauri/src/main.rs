#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::{delete_permanently, move_to_trash, reveal_in_explorer};
use miydisk::{scan_directory, ScanProgress};
use tauri::Emitter;

#[tauri::command]
fn start_scan(app: tauri::AppHandle, path: String) {
    std::thread::spawn(move || {
        let root = std::path::PathBuf::from(&path);

        let mut on_progress = |event: ScanProgress| {
            let _ = app.emit("scan-progress", &event);
        };

        match scan_directory(&root, &mut on_progress) {
            Ok(tree) => {
                let _ = app.emit("scan-complete", &tree);
            }
            Err(e) => {
                let _ = app.emit("scan-error", e.to_string());
            }
        }
    });
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            start_scan,
            reveal_in_explorer,
            move_to_trash,
            delete_permanently
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}