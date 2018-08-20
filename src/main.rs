use std::path::Path;

pub struct DB;

#[derive(Debug)]
pub struct DBInfo;

#[derive(Debug)]
pub struct Error;

impl DB {
    /// Open or create a data base at the specified location
    fn new<P: AsRef<Path>>(p: P) -> Result<DB, Error> {
        unimplemented!("new method for db not yet implemented");
    }

    /// High level information an statistics about
    /// the database
    fn explain(&self) -> DBInfo {
        unimplemented!("explain not yet supported for db");
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        unimplemented!("get not yet implemented");
    }

    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        unimplemented!("put not yet implemented");
    }

    fn delete(&self, key: &[u8]) -> Option<Vec<u8>> {
        unimplemented!("delete not yet implemented");
    }

    fn delete_db(self) -> Result<(), Error> {
        unimplemented!("Deletes not yet implemented.");
    }

    // TODO: Support 'transactional puts'
    //       in two forms:
    //       - explicit batch: put_batch()
    //       - implicit batch: db.start{ db.put();db.put()....};
}

fn main() {
    let db_path = "./path/to/db";
    let db = DB::new(db_path).unwrap(); // Initialise a db with default params

    println!("Db info: {:?}", db.explain()); // list basic stats about the db. Support column families.

    db.put("key".as_bytes(), "value".as_bytes());
    assert!(db.get("key".as_bytes()) == Some("value".as_bytes().into()));

    println!("Hello, world!");
}
