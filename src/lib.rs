use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Db {
    data: BTreeMap<Vec<u8>, Vec<u8>>,
}

const DEFAULT_PATH: &str = "default.lsm";

pub struct Config {
    path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: PathBuf::from(DEFAULT_PATH),
        }
    }
}

/// Configuration object for the db
impl Config {
    /// Return a new default `Config`
    pub fn new() -> Config {
        Config::default()
    }

    /// Set the path of the database
    pub fn path<P: AsRef<Path>>(mut self, path: P) -> Config {
        self.path = path.as_ref().to_path_buf();
        self
    }

    fn db_path(&self) -> PathBuf {
        let mut path = self.path.clone();
        path.push("db");
        path
    }

    /// open db file, creating if required
    fn open_file(&self) -> Result<File> {
        let path = self.db_path();
        unimplemented!("")
    }

    /// Open (and create if required) a `Db` associated with this config
    pub fn open(&self) -> Result<Db> {
        unimplemented!("")
    }
}

impl Db {
    pub fn open<P: AsRef<Path>>(path: P) -> Db {
        Db {
            data: BTreeMap::new(),
        }
    }

    pub fn insert<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        self.data
            .insert(key.as_ref().to_vec(), value.as_ref().to_vec());
        Ok(())
    }

    pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<&Vec<u8>>> {
        Ok(self.data.get(key.as_ref()).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_into_db() {
        let mut db = Db::open("foo");
        db.insert(b"hello", b"world");
        assert_eq!(db.get(b"hello").unwrap().unwrap(), b"world");
    }

    #[test]
    fn get_nonexistant() {
        let mut db = Db::open("foo");
        assert_eq!(db.get(b"hello").unwrap(), None);
    }
}
