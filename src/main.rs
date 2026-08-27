use std::env;
use std::path::PathBuf;
use std::process;

use miydisk::{scan_directory, FileNode, ScanProgress};

fn main() {
    let args: Vec<String> = env::args().collect();
    let target = match args.get(1) {
        Some(p) => PathBuf::from(p),
        None => {
            eprintln!("用法: miydisk <要扫描的目录路径>");
            process::exit(1);
        }
    };

    if !target.exists() {
        eprintln!("错误：路径不存在 - {}", target.display());
        process::exit(1);
    }

    println!("开始扫描：{}\n", target.display());

    let mut on_progress = |p: ScanProgress| match p {
        ScanProgress::EnteredDirectory { path, files_so_far } => {
            println!("[扫描中] {}（已扫描文件数：{}）", path, files_so_far);
        }
        ScanProgress::Error { path, message } => {
            eprintln!("[跳过] {} - {}", path, message);
        }
        ScanProgress::Finished {
            total_files,
            total_size,
        } => {
            println!(
                "\n扫描完成：共 {} 个文件，总大小 {}",
                total_files,
                human_readable(total_size)
            );
        }
        // NodeCompleted 事件量太大，命令行阶段不逐条打印，
        // 阶段2接 Tauri 时这里会换成 emit 到前端
        ScanProgress::NodeCompleted { .. } => {}
    };

    match scan_directory(&target, &mut on_progress) {
        Ok(root) => {
            println!("\n===== Top 10 占用排行 =====");
            print_top_n(&root, 10);
        }
        Err(e) => {
            eprintln!("扫描失败：{}", e);
            process::exit(1);
        }
    }
}

fn print_top_n(root: &FileNode, n: usize) {
    let mut all: Vec<&FileNode> = Vec::new();
    collect_all(root, &mut all);
    all.sort_by_key(|a| std::cmp::Reverse(a.total_size));

    for (i, node) in all.iter().take(n).enumerate() {
        println!(
            "{:>2}. {:>10}  {}",
            i + 1,
            human_readable(node.total_size),
            node.path
        );
    }
}

fn collect_all<'a>(node: &'a FileNode, out: &mut Vec<&'a FileNode>) {
    out.push(node);
    for child in &node.children {
        collect_all(child, out);
    }
}

fn human_readable(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    format!("{:.2} {}", size, UNITS[unit_idx])
}
