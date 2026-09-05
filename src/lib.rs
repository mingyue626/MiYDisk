pub mod dedupe;
pub mod error;
pub mod scanner;

pub use dedupe::{find_duplicates, DuplicateGroup};
pub use error::ScanError;
pub use scanner::{scan_directory, FileNode, NodeType, ScanProgress};