use std::path::Path;

use errors::Error;
pub struct DB;

#[derive(Debug)]
pub struct DBInfo;


impl DB {
    /// Open or creat a data base at the specified location
    pub fn new<P: AsRef<Path>>(p: P) -> Result<DB, Error> {
        unimplemented!("new method for db not yet implemented");
    }

    /// High level information an statistics about
    /// the database
    pub fn explain(&self) -> DBInfo {
        unimplemented!("explain not yet supported for db");
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
        unimplemented!("Deletes not yet implemented.");
    }

    // TODO: Support 'transactional puts'
    //       in two forms:
    //       - explicit batch: put_batch()
    //       - implicit batch: db.start{ db.put();db.put()....};
    // TODO: Support iteration over k,v pairs
    //       (potentially with key filters??)
}
