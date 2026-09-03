// 阶段4：文件操作。这几个命令都是直接操作系统文件系统的破坏性/半破坏性操作，
// 前端在调用前已经做好了用户确认（永久删除必须确认，移到回收站因为可恢复不强制确认）。
use std::path::Path;

/// 在系统文件管理器中打开并选中该文件/文件夹。
#[tauri::command]
pub fn reveal_in_explorer(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        let parent = Path::new(&path).parent().unwrap_or_else(|| Path::new("/"));
        std::process::Command::new("xdg-open")
            .arg(parent)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 移到系统回收站（可恢复），不需要用户二次确认。
#[tauri::command]
pub fn move_to_trash(path: String) -> Result<(), String> {
    trash::delete(&path).map_err(|e| e.to_string())
}

/// 永久删除，跳过回收站，不可恢复。前端必须先弹确认对话框。
#[tauri::command]
pub fn delete_permanently(path: String) -> Result<(), String> {
    let p = Path::new(&path);
    let metadata = std::fs::symlink_metadata(p).map_err(|e| e.to_string())?;
    if metadata.is_dir() {
        std::fs::remove_dir_all(p).map_err(|e| e.to_string())
    } else {
        std::fs::remove_file(p).map_err(|e| e.to_string())
    }
}