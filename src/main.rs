// extern crate byteorder;
// extern crate futures;
#[cfg(test)]
// extern crate tempfile;
// extern crate tokio;
// extern crate tokio_fs;
// extern crate tokio_io;
// extern crate tokio_threadpool;
// extern crate enum_primitive_derive;
// extern crate num_traits;
// extern crate tracing;

#[macro_use]
use tracing::{event, Level};

use tracing::debug;

pub mod db;
pub mod errors;
pub mod fs;

use db::DB;

use errors::Error;

pub type Key = [u8];
pub type Value = [u8];

pub enum CompactionStrategy {
    Leveled,
    // No other options supported for now
}

pub struct TableConf {
    mem_table_size: usize,
    ss_table_target_size: usize,
    compaction_strategy: CompactionStrategy,
}

/// Logical thing abstracting between mem and SS tables
pub struct Table<M: MemTable, SS: SSTable> {
    conf: TableConf,
    mem_tables: Vec<M>,
    ss_table: SS, // Level 0 sstable keep in mem as optimization -- may come to regret this?
}

impl<M: MemTable, SS: SSTable> Table<M, SS> {
    ///Get the SSTable list at the specified level
    fn get_level(&self, level: u32) -> Result<Vec<SS>, Error> {
        unimplemented!("get_level not yet implemented");
    }

    fn get(&self, k: &Key) -> Option<&Value> {
        self.ss_table.get(k)
    }

    fn put<K: AsRef<Key>, V: AsRef<Value>>(k: K, v: V) -> Result<(), Error> {
        unimplemented!("Table::put not implemented.")
    }
}

pub trait MemTable {
    fn get(&self, k: &Key) -> Option<&Vec<u8>>;
    fn put(&mut self, k: &Key, v: &Value) -> Result<(), Error>;
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
}

/// MemTable backed by a std libary hashmap
pub struct HashMemTable {
    ht: std::collections::HashMap<Vec<u8>, Vec<u8>>,
}

impl MemTable for HashMemTable {
    fn get(&self, k: &Key) -> Option<&Vec<u8>> {
        self.ht.get(k)
    }

    fn put(&mut self, k: &Key, v: &Value) -> Result<(), Error> {
        match self.ht.insert(k.to_owned(), v.to_owned()) {
            Some(_) => Ok(()),
            None => Err(Error {}),
        }
    }

    fn len(&self) -> usize {
        // TODO: this should be in bytes, calculate size of stored entries
        // Potentially incrmented each insert??
        self.ht.len()
    }

    fn capacity(&self) -> usize {
        self.ht.capacity()
    }
}

pub trait SSTable {
    fn get(&self, k: &Key) -> Option<&Value>;
    fn put(&mut self, k: &Key, v: &Value) -> Result<(), Error>;
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
}

fn main() {
    let db_path = "./path/to/db";
    let db = DB::new(db_path).unwrap(); // Initialise a db with default params
    // event!(Level::INFO, "my span");

    println!("Db info: {:?}", db.explain()); // list basic stats about the db. Support column families.

    db.put("key".as_bytes(), "value".as_bytes()).unwrap();
    assert!(db.get("key".as_bytes()) == Some("value".as_bytes().into()));

    println!("Hello, world!");
}
