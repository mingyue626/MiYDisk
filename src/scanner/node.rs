use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum NodeType {
    File,
    Directory,
    Symlink,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileNode {
    pub id: u64,
    pub name: String,
    /// 用 String 而非 PathBuf，避免跨平台序列化到 JSON 时的编码问题
    pub path: String,
    pub node_type: NodeType,
    /// 节点自身大小；目录恒为 0
    pub own_size: u64,
    /// 递归汇总大小；文件等于 own_size，目录随扫描进度增量更新
    pub total_size: u64,
    pub children: Vec<FileNode>,
    /// 阶段2去重检测用的 blake3 哈希，阶段1留空
    pub hash: Option<String>,
    /// 扫描该节点时遇到的错误（如权限拒绝），成功则为 None
    pub error: Option<String>,
}

impl FileNode {
    pub fn new_file(id: u64, name: String, path: String, size: u64) -> Self {
        Self {
            id,
            name,
            path,
            node_type: NodeType::File,
            own_size: size,
            total_size: size,
            children: Vec::new(),
            hash: None,
            error: None,
        }
    }

    pub fn new_directory(id: u64, name: String, path: String) -> Self {
        Self {
            id,
            name,
            path,
            node_type: NodeType::Directory,
            own_size: 0,
            total_size: 0,
            children: Vec::new(),
            hash: None,
            error: None,
        }
    }
}
