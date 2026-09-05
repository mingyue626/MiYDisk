#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use std::sync::Mutex;

use commands::{delete_permanently, move_to_trash, reveal_in_explorer};
use miydisk::{find_duplicates, scan_directory, DuplicateGroup, FileNode, ScanProgress};
use tauri::{Emitter, Manager, State};

struct AppState {
    last_scan: Mutex<Option<FileNode>>,
}

#[tauri::command]
fn start_scan(app: tauri::AppHandle, path: String) {
    std::thread::spawn(move || {
        let root = std::path::PathBuf::from(&path);

        let mut on_progress = |event: ScanProgress| {
            let _ = app.emit("scan-progress", &event);
        };

        match scan_directory(&root, &mut on_progress) {
            Ok(tree) => {
                let state = app.state::<AppState>();
                if let Ok(mut guard) = state.last_scan.lock() {
                    *guard = Some(tree.clone());
                }
                let _ = app.emit("scan-complete", &tree);
            }
            Err(e) => {
                let _ = app.emit("scan-error", e.to_string());
            }
        }
    });
}

#[tauri::command]
fn find_duplicate_files(state: State<AppState>) -> Result<Vec<DuplicateGroup>, String> {
    let guard = state.last_scan.lock().map_err(|e| e.to_string())?;
    let tree = guard.as_ref().ok_or("请先完成一次扫描")?;
    Ok(find_duplicates(tree))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            last_scan: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            start_scan,
            reveal_in_explorer,
            move_to_trash,
            delete_permanently,
            find_duplicate_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}