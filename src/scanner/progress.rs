/// 扫描进度事件。阶段1先用 println! 消费，阶段2接入 Tauri 时
/// 直接把消费者换成 `app_handle.emit(...)` 即可，扫描逻辑不用改。
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum ScanProgress {
    /// 发现一个新目录，附带当前已扫描的文件计数
    EnteredDirectory { path: String, files_so_far: u64 },
    /// 某个节点扫描完成（含递归汇总大小）
    NodeCompleted {
        id: u64,
        path: String,
        total_size: u64,
    },
    /// 整体扫描完成
    Finished { total_files: u64, total_size: u64 },
    /// 某个节点扫描出错（如权限拒绝），不中断整体扫描
    Error { path: String, message: String },
}
