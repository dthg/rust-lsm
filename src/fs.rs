
use std::fs::File;
//use std::io;
use std::os::unix::prelude::FileExt;

/// Handles writing of files to storage
/// Not fully thought out yet.. full contents tbd.

/// TODO: Define this in protobuf or something similar
/// Repr on disk
/// <--- id --><--- end_offset-><--- ...contents... ---><--- id+1 --->
///                   \                                 ^
///                    \_______________________________/
///
struct WalSegment {
    id: u64,
    end_offset: u64 // offset in bytes
}
#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn file_write() {
        let test_data = b"start world";
        //let dir = tempdir().unwrap();
        //let temp_path = dir.path().join("foo.log");
        let file = File::create("/tmp/foo.bar").unwrap();

        let res = file.write_at(test_data, 0);
        res.unwrap();
        file.sync_data();
    }
}
