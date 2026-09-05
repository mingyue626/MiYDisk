use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use serde::Serialize;

use crate::scanner::node::{FileNode, NodeType};

#[derive(Debug, Serialize, Clone)]
pub struct DuplicateGroup {
    pub size: u64,
    pub hash: String,
    pub paths: Vec<String>,
}

/// 在已扫描的树上查找重复文件。只比较 NodeType::File（跳过目录和符号链接），
/// 单个文件读取失败（如权限问题）会被跳过，不会中断整体检测。
pub fn find_duplicates(root: &FileNode) -> Vec<DuplicateGroup> {
    let mut by_size: HashMap<u64, Vec<&FileNode>> = HashMap::new();
    collect_files(root, &mut by_size);

    let mut groups = Vec::new();

    for (size, candidates) in by_size {
        if candidates.len() < 2 {
            continue; // 大小唯一，不可能有重复
        }

        let mut by_hash: HashMap<String, Vec<String>> = HashMap::new();
        for node in candidates {
            match hash_file(Path::new(&node.path)) {
                Ok(hash) => by_hash.entry(hash).or_default().push(node.path.clone()),
                Err(_) => continue, // 读取失败（权限/文件被占用等）跳过，不影响其他文件
            }
        }

        for (hash, paths) in by_hash {
            if paths.len() >= 2 {
                groups.push(DuplicateGroup { size, hash, paths });
            }
        }
    }

    // 按重复占用的总空间降序排列，最值得清理的排在前面
    groups.sort_by_key(|g| std::cmp::Reverse(g.size * g.paths.len() as u64));

    groups
}

fn collect_files<'a>(node: &'a FileNode, by_size: &mut HashMap<u64, Vec<&'a FileNode>>) {
    if node.node_type == NodeType::File {
        by_size.entry(node.own_size).or_default().push(node);
        return;
    }
    for child in &node.children {
        collect_files(child, by_size);
    }
}

fn hash_file(path: &Path) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::scan_directory;
    use std::fs;

    fn make_temp_dir(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "miydisk_dedupe_test_{}_{}",
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
    fn finds_identical_content_files() {
        let root = make_temp_dir("identical");
        fs::write(root.join("a.txt"), b"hello world").unwrap();
        fs::write(root.join("b.txt"), b"hello world").unwrap(); // 内容相同 -> 重复
        fs::write(root.join("c.txt"), b"hello WORLD").unwrap(); // 大小相同、内容不同 -> 不是重复
        fs::write(root.join("d.txt"), b"short").unwrap(); // 大小唯一 -> 不参与比较

        let tree = scan_directory(&root, &mut |_| {}).unwrap();
        let groups = find_duplicates(&tree);

        assert_eq!(groups.len(), 1, "应该只找到一组重复");
        assert_eq!(groups[0].paths.len(), 2);
        let names: Vec<&str> = groups[0]
            .paths
            .iter()
            .map(|p| p.rsplit('/').next().unwrap_or(p.as_str()))
            .collect();
        assert!(names.contains(&"a.txt") || names.iter().any(|n| n.ends_with("a.txt")));

        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn no_duplicates_returns_empty() {
        let root = make_temp_dir("nodupe");
        fs::write(root.join("a.txt"), b"aaa").unwrap();
        fs::write(root.join("b.txt"), b"bbbb").unwrap();

        let tree = scan_directory(&root, &mut |_| {}).unwrap();
        let groups = find_duplicates(&tree);

        assert!(groups.is_empty());

        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn finds_duplicates_across_nested_directories() {
        let root = make_temp_dir("nested_dupe");
        let sub = root.join("sub");
        fs::create_dir_all(&sub).unwrap();
        fs::write(root.join("top.bin"), vec![7u8; 500]).unwrap();
        fs::write(sub.join("nested.bin"), vec![7u8; 500]).unwrap(); // 跨目录内容相同

        let tree = scan_directory(&root, &mut |_| {}).unwrap();
        let groups = find_duplicates(&tree);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].size, 500);
        assert_eq!(groups[0].paths.len(), 2);

        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn three_way_duplicate_grouped_together() {
        let root = make_temp_dir("triple");
        fs::write(root.join("a.txt"), b"same content").unwrap();
        fs::write(root.join("b.txt"), b"same content").unwrap();
        fs::write(root.join("c.txt"), b"same content").unwrap();

        let tree = scan_directory(&root, &mut |_| {}).unwrap();
        let groups = find_duplicates(&tree);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].paths.len(), 3);

        fs::remove_dir_all(&root).unwrap();
    }
}