#[cfg(not(target_arch = "wasm32"))]
use std::fs;
use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
pub fn path_to_bytes<P: AsRef<Path>>(path: P) -> Option<Vec<u8>> {
    fs::read(path).ok()
}
