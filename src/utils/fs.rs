//! Filesystem for cross-platform loading and uploading assets

use std::{
    fs::OpenOptions,
    io::Write,
    sync::{Arc, LazyLock, Mutex, MutexGuard},
};

pub struct Filesystem {}

pub static FILESYSTEM: LazyLock<Arc<Mutex<Filesystem>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(Filesystem::init())));

impl Filesystem {
    pub(crate) fn init() -> Self {
        Self {}
    }
    pub fn get<'a>() -> MutexGuard<'a, Filesystem> {
        FILESYSTEM.lock().unwrap()
    }
    pub fn read(&self, path: &str) -> Result<Vec<u8>, std::io::Error> {
        std::fs::read(path)
    }

    pub fn read_to_string(&self, path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
    pub fn write_all(&self, path: &str, bytes: Vec<u8>) -> Result<(), std::io::Error> {
        std::fs::write(path, bytes)
    }
    pub fn write_into_end(&self, path: &str, bytes: Vec<u8>) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new().write(true).open(path)?;
        file.write(&bytes)?;
        Ok(())
    }
}
