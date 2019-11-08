use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::Error;
pub struct DB {
    db_path: PathBuf,
}

#[derive(Debug)]
pub struct DBInfo;

impl DB {
    /// Open or create a database at the specified location
    pub fn new<P: AsRef<Path>>(p: P) -> Result<DB, Error> {
        if !p.as_ref().exists() {
            println!("Created db at path {:?}", p.as_ref());
            fs::create_dir_all(&p).unwrap()
        }

        Ok(DB {
            db_path: p.as_ref().to_path_buf(),
        })
    }

    /// High level information an statistics about
    /// the database
    pub fn explain(&self) -> DBInfo {
        println!("explain not yet supported for db");
        DBInfo {}
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        unimplemented!("get not yet implemented");
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        unimplemented!("put not yet implemented");
    }

    pub fn delete(&self, key: &[u8]) -> Option<Vec<u8>> {
        unimplemented!("delete not yet implemented");
    }

    pub fn delete_db(self) -> Result<(), Error> {
        fs::remove_dir_all(self.db_path).unwrap();
        Ok(())
    }

    // TODO: Support 'transactional puts'
    //       in two forms:
    //       - explicit batch: put_batch()
    //       - implicit batch: db.start{ db.put();db.put()....};
    // TODO: Support iteration over k,v pairs
    //       (potentially with key filters??)
}
