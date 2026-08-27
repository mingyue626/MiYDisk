use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use jwalk::WalkDir;

use crate::error::ScanError;
use crate::scanner::node::{FileNode, NodeType};
use crate::scanner::progress::ScanProgress;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

type EntryInfo = (u64, u64, NodeType, Option<String>);

pub fn scan_directory(
    root: &Path,
    on_progress: &mut impl FnMut(ScanProgress),
) -> Result<FileNode, ScanError> {
    let mut entries: HashMap<PathBuf, EntryInfo> = HashMap::new();
    let mut files_count: u64 = 0;

    // skip_hidden(false)：默认会跳过 .git/.fingerprint 等隐藏目录，
    // 磁盘清理工具必须能扫到这些占空间大户，所以关掉
    for entry_result in WalkDir::new(root).skip_hidden(false) {
        match entry_result {
            Ok(entry) => {
                let path = entry.path();
                let id = next_id();

                let file_type = entry.file_type();
                let (node_type, size) = if file_type.is_symlink() {
                    (NodeType::Symlink, 0)
                } else if file_type.is_dir() {
                    on_progress(ScanProgress::EnteredDirectory {
                        path: path.to_string_lossy().to_string(),
                        files_so_far: files_count,
                    });
                    (NodeType::Directory, 0)
                } else {
                    files_count += 1;
                    let len = entry.metadata().map(|m| m.len()).unwrap_or(0);
                    (NodeType::File, len)
                };

                entries.insert(path, (id, size, node_type, None));
            }
            Err(e) => {
                let path_str = e
                    .path()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "<unknown>".to_string());
                on_progress(ScanProgress::Error {
                    path: path_str,
                    message: e.to_string(),
                });
            }
        }
    }

    let mut children_index: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    for path in entries.keys() {
        if let Some(parent) = path.parent() {
            children_index
                .entry(parent.to_path_buf())
                .or_default()
                .push(path.clone());
        }
    }

    let root_node = build_tree(root, &entries, &children_index, on_progress)?;

    on_progress(ScanProgress::Finished {
        total_files: files_count,
        total_size: root_node.total_size,
    });

    Ok(root_node)
}

fn build_tree(
    path: &Path,
    entries: &HashMap<PathBuf, EntryInfo>,
    children_index: &HashMap<PathBuf, Vec<PathBuf>>,
    on_progress: &mut impl FnMut(ScanProgress),
) -> Result<FileNode, ScanError> {
    let (id, own_size, node_type, error) = entries.get(path).cloned().ok_or_else(|| {
        ScanError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("节点丢失: {}", path.display()),
        ))
    })?;

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());
    let path_str = path.to_string_lossy().to_string();

    if node_type != NodeType::Directory {
        return Ok(FileNode {
            id,
            name,
            path: path_str,
            node_type,
            own_size,
            total_size: own_size,
            children: Vec::new(),
            hash: None,
            error,
        });
    }

    let mut children: Vec<FileNode> = Vec::new();
    let mut total: u64 = 0;

    if let Some(child_paths) = children_index.get(path) {
        let mut sorted_children = child_paths.clone();
        sorted_children.sort();

        for child_path in &sorted_children {
            let child = build_tree(child_path, entries, children_index, on_progress)?;
            total += child.total_size;
            children.push(child);
        }
    }

    children.sort_by(|a, b| b.total_size.cmp(&a.total_size));

    let node = FileNode {
        id,
        name,
        path: path_str,
        node_type: NodeType::Directory,
        own_size: 0,
        total_size: total,
        children,
        hash: None,
        error,
    };

    on_progress(ScanProgress::NodeCompleted {
        id: node.id,
        path: node.path.clone(),
        total_size: node.total_size,
    });

    Ok(node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn make_temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "miydisk_test_{}_{}",
            tag,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn scans_flat_files_and_sums_size() {
        let root = make_temp_dir("flat");
        fs::write(root.join("a.txt"), vec![0u8; 100]).unwrap();
        fs::write(root.join("b.txt"), vec![0u8; 200]).unwrap();

        let mut events = Vec::new();
        let node = scan_directory(&root, &mut |e| events.push(e)).unwrap();

        assert_eq!(node.total_size, 300);
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].total_size, 200);
        assert_eq!(node.children[1].total_size, 100);

        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn sums_nested_directories_recursively() {
        let root = make_temp_dir("nested");
        let sub = root.join("sub");
        fs::create_dir_all(&sub).unwrap();
        fs::write(root.join("top.bin"), vec![0u8; 50]).unwrap();
        fs::write(sub.join("deep.bin"), vec![0u8; 150]).unwrap();

        let node = scan_directory(&root, &mut |_| {}).unwrap();

        assert_eq!(node.total_size, 200);

        let sub_node = node
            .children
            .iter()
            .find(|c| c.name == "sub")
            .expect("应找到子目录节点");
        assert_eq!(sub_node.total_size, 150);
        assert_eq!(sub_node.own_size, 0);

        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn scans_hidden_files_too() {
        let root = make_temp_dir("hidden");
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::write(root.join(".git").join("config"), vec![0u8; 77]).unwrap();
        fs::write(root.join(".hidden_file"), vec![0u8; 33]).unwrap();

        let node = scan_directory(&root, &mut |_| {}).unwrap();

        assert_eq!(node.total_size, 110);
        assert!(node.children.iter().any(|c| c.name == ".git"));
        assert!(node.children.iter().any(|c| c.name == ".hidden_file"));

        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn emits_finished_event_with_correct_file_count() {
        let root = make_temp_dir("events");
        fs::write(root.join("x.txt"), vec![0u8; 10]).unwrap();
        fs::write(root.join("y.txt"), vec![0u8; 10]).unwrap();
        fs::create_dir_all(root.join("empty_dir")).unwrap();

        let mut finished_count = None;
        scan_directory(&root, &mut |e| {
            if let ScanProgress::Finished { total_files, .. } = e {
                finished_count = Some(total_files);
            }
        })
        .unwrap();

        assert_eq!(finished_count, Some(2));

        fs::remove_dir_all(&root).unwrap();
    }
}