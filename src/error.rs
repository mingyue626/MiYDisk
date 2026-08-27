use std::fmt;

#[derive(Debug)]
pub enum ScanError {
    Io(std::io::Error),
    PermissionDenied(String),
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::Io(e) => write!(f, "IO error: {}", e),
            ScanError::PermissionDenied(path) => {
                write!(f, "Permission denied: {}", path)
            }
        }
    }
}

impl std::error::Error for ScanError {}

impl From<std::io::Error> for ScanError {
    fn from(e: std::io::Error) -> Self {
        ScanError::Io(e)
    }
}
