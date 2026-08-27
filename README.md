# MiYDisk

> 诚实、轻量、快的磁盘空间可视化清理工具。不做虚假优化，只做真正有效的事。

对标 [DaisyDisk](https://daisydiskapp.com/)（Mac）/ WinDirStat（Windows），目标是在 Windows 上做出接近 DaisyDisk 体验水准的开源工具。

**当前状态：早期开发中 🚧**
目前仅有命令行版扫描器（里程碑阶段 1），Treemap 可视化界面尚未开始。如果你现在下载，只能得到一个"扫描目录并按大小排序输出"的 CLI 工具。

---

## 为什么做这个

市面上大多数"电脑管家"类清理工具充斥着"一键加速"、"内存优化"、"注册表清理"这类无实际效果甚至有风险的功能，并且后台常驻、开机自启，透明度极低。

MiYDisk 明确**不做**这些事，只做两件事做到极致：
1. 让你直观看清磁盘空间被什么占用（Treemap 可视化）
2. 让你安全地清理它（移到回收站，而非直接删除）

## 功能规划

- [x] 目录扫描 + 按大小排序（CLI，阶段 1）
- [ ] Tauri 壳 + 进度事件推送（阶段 2）
- [ ] Treemap 可视化（阶段 3）
- [ ] 文件操作：资源管理器打开 / 删除 / 移到回收站（阶段 4）
- [ ] 重复文件检测（blake3 哈希比对）（阶段 5）
- [ ] 打包分发 .exe / .msi（阶段 6）

明确不做：一键加速、内存优化、注册表清理、后台常驻监控。

## 技术栈

- **后端**：Rust（[jwalk](https://github.com/Byron/jwalk) 并行目录遍历 + [blake3](https://github.com/BLAKE3-team/BLAKE3) 哈希去重）
- **壳**：[Tauri](https://tauri.app/)
- **前端**：Svelte + [d3-hierarchy](https://github.com/d3/d3-hierarchy)（Treemap 布局）

详细产品方案和技术选型见项目内文档。

## 快速开始

> 目前只有命令行扫描器可用，图形界面尚未实现。

```bash
git clone https://github.com/<your-username>/MiYDisk.git
cd MiYDisk
cargo run --release -- <要扫描的目录路径>
```

### 开发环境要求

- Rust（stable，建议通过 [rustup](https://rustup.rs/) 安装）
- 后续接入 Tauri 后还需要 Node.js + npm/pnpm

## 项目结构

```
MiYDisk/
├── src/
│   ├── main.rs           # CLI 入口
│   ├── lib.rs             # 核心逻辑导出，供后续 Tauri 复用
│   ├── scanner/
│   │   ├── mod.rs
│   │   ├── node.rs        # FileNode 树形数据结构
│   │   ├── scanner.rs     # jwalk 并行遍历 + 建树
│   │   └── progress.rs    # 扫描进度回调
│   └── error.rs
├── Cargo.toml
└── README.md
```

## 参与贡献

欢迎 PR / Issue。提交代码前请确保通过：

```bash
cargo fmt
cargo clippy
cargo test
```

## 协议

本项目采用 [GPLv3](./LICENSE) 协议开源——这意味着任何基于本项目的衍生作品也必须保持开源，防止被闭源套壳商用。
