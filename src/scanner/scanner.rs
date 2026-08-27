use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::error::ScanError;
use crate::scanner::node::{FileNode, NodeType};
use crate::scanner::progress::ScanProgress;

/// 全局递增 id 生成器，保证树里每个节点 id 唯一，
/// 后续增量更新事件靠这个 id 和前端节点对应。
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// 单线程递归扫描版本。先保证正确性，跑通后再换成 jwalk 并行遍历。
///
/// `on_progress` 回调用于报告扫描进度：阶段1可以传 `|p| println!("{:?}", p)`，
/// 阶段2接 Tauri 时把它换成 `|p| app_handle.emit("scan-progress", p).unwrap()` 即可。
pub fn scan_directory(
    root: &Path,
    on_progress: &mut impl FnMut(ScanProgress),
) -> Result<FileNode, ScanError> {
    let mut files_count: u64 = 0;
    let node = scan_recursive(root, on_progress, &mut files_count)?;

    on_progress(ScanProgress::Finished {
        total_files: files_count,
        total_size: node.total_size,
    });

    Ok(node)
}

fn scan_recursive(
    path: &Path,
    on_progress: &mut impl FnMut(ScanProgress),
    files_count: &mut u64,
) -> Result<FileNode, ScanError> {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());
    let path_str = path.to_string_lossy().to_string();

    let metadata = fs::symlink_metadata(path)?;

    if metadata.is_symlink() {
        let id = next_id();
        return Ok(FileNode {
            id,
            name,
            path: path_str,
            node_type: NodeType::Symlink,
            own_size: 0,
            total_size: 0,
            children: Vec::new(),
            hash: None,
            error: None,
        });
    }

    if metadata.is_file() {
        let id = next_id();
        *files_count += 1;
        let node = FileNode::new_file(id, name, path_str.clone(), metadata.len());
        on_progress(ScanProgress::NodeCompleted {
            id,
            path: path_str,
            total_size: node.total_size,
        });
        return Ok(node);
    }

    // 目录
    let id = next_id();
    let mut dir_node = FileNode::new_directory(id, name, path_str.clone());

    on_progress(ScanProgress::EnteredDirectory {
        path: path_str.clone(),
        files_so_far: *files_count,
    });

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(e) => {
            // 权限拒绝等错误不中断整体扫描，记录在节点上并跳过
            dir_node.error = Some(e.to_string());
            on_progress(ScanProgress::Error {
                path: path_str,
                message: e.to_string(),
            });
            return Ok(dir_node);
        }
    };

    let mut total: u64 = 0;
    for entry in entries.flatten() {
        match scan_recursive(&entry.path(), on_progress, files_count) {
            Ok(child) => {
                total += child.total_size;
                dir_node.children.push(child);
            }
            Err(e) => {
                on_progress(ScanProgress::Error {
                    path: entry.path().to_string_lossy().to_string(),
                    message: e.to_string(),
                });
            }
        }
    }

    dir_node.total_size = total;
    // 子节点按大小降序排列，方便命令行直接看到"谁占用最大"
    dir_node.children.sort_by(|a, b| b.total_size.cmp(&a.total_size));

    on_progress(ScanProgress::NodeCompleted {
        id,
        path: dir_node.path.clone(),
        total_size: dir_node.total_size,
    });

    Ok(dir_node)
}
