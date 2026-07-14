use std::fs;
use std::path::Path;

use anyhow::Result;

pub fn read_tail(path: &Path, max_bytes: usize) -> Result<String> {
    let metadata = fs::metadata(path)?;
    let file_size = metadata.len() as usize;
    let start = if file_size > max_bytes {
        file_size - max_bytes
    } else {
        0
    };
    let bytes = fs::read(path)?;
    let tail = &bytes[start..];
    Ok(String::from_utf8_lossy(tail).to_string())
}
