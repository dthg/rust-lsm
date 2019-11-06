use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};

use crate::errors::Error;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/// User tokio aio to do file writing
/// Will mean that this is Linux only for now but fine for _now_

/// Handles writing of files to storage
/// Not fully thought out yet.. full contents tbd.

/// TODO: Define this in protobuf or something similar
/// Repr on disk:
///
/// ```text
/// <--- id --><--- end_offset-><--- ...contents... ---><--- id+1 --->
///                   \                                 ^
///                    \_______________________________/
///
/// ```
/// TODO: Support log compaction
/// TODO: Have tests cover restore from log, going from WAL -> (At least an in memory table)
/// TODO: Support sharding wal by time to avoid having one massive file

/// Wal files will look like:
/// - 0001.log
/// - 0002.log
/// ...
/// - 000N.log
/// Where N is the sequence number

/// NOTE: Should Segments just be references to a MemMapped file?
///       and thus just take references to them, ie segments will be tied to the
///       underlying file buffer of the WalFile?
struct WalFile {
    id: u64,
    path: PathBuf,
}

impl WalFile {
    const FILE_HEADER: u64 = 0x72757374796c736d; // rustylsm in ascii
    const FILE_PREXIX: &'static str = "WAL";

    fn read_file(_path: PathBuf) -> WalFile {
        unimplemented!("WalFile Read not yet implemented.");
    }

    /// # Safety:
    ///    ID is assumed to be unique. This write will check that the file
    ///    with `id` has not yet been written to, however it does not check past
    ///    that point. Weird spooky not fun things can happen in that case.
    fn new_file<P: AsRef<Path>>(id: u64, wal_path: P) -> Result<WalFile, Error> {
        // TODO: make this accept any thing that is path like
        // TODO: This should be result type
        // TODO: Make this take in base wal dir and do the rest from there
        Ok(WalFile {
            id: id,
            path: wal_path.as_ref().to_path_buf(),
        })
    }

    // TODO: Make this async
    fn write_segment(&mut self, segment: WalSegment) -> Result<(), Error> {
        // TODO: Should this do some batching internally?
        self.write_segments(vec![segment])
    }

    /// Bulk write of segments to wal file.
    /// Still not sure if I want to support his usecase. 🤔
    fn write_segments(&self, _segments: Vec<WalSegment>) -> Result<(), Error> {
        unimplemented!("write to wal file not yet supported");
    }

    /// Load segments from wal file
    fn load_segments(&self) -> Result<Vec<WalSegment>, Error> {
        unimplemented!("wal file egmeng read not implemented yet");
    }
}

/// Indicates the type of the payload stored in a WalSegment.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
enum SegmentType {
    /// First segment in a multi segment record.
    Start = 0,
    /// Intermediate segment of a multi segment record
    Continuation = 1,
    /// Last segment in a multi segment record
    End = 2,
    /// A segment containing a full record.
    Full = 3,
}

impl SegmentType {
    /// Construct a segment type from a u8
    /// TODO: make less crappy
    fn from_u8(data: u8) -> Option<SegmentType> {
        match data {
            0 => Some(SegmentType::Start),
            1 => Some(SegmentType::Continuation),
            2 => Some(SegmentType::End),
            3 => Some(SegmentType::Full),
            _ => None,
        }
    }
}

/// Operations Tracked by the Write Ahead Log
/// TODO: Delete, only support writes, tombstones and versions
///       will handle Update/Delete cases

/// TODO: Determine the block size of these segments
/// TODO: Decide if segments should be checksummed
/// For the first past will probably just copy RockDB
/// Length of the payload will be inferred from the record on disk
/// Currently assumption is that segments will look like:
/// Initially will assume that payload must be < blocksize - header
/// And that there will be one block per payload.
/// This will be reworked as this will cause extreme storage fragmentation
/// ```text
///       +-----+-------------+--+----+----------+------+-- ... ----+
/// File  | r0  |        r1   |P | r2 |    r3    |  r4  |           |
///       +-----+-------------+--+----+----------+------+-- ... ----+
///       <--- kBlockSize ------>|<-- kBlockSize ------>|
/// ```
#[derive(Debug, PartialEq)]
struct WalSegment {
    segment_type: SegmentType,
    payload: Vec<u8>,
    padding: Vec<u8>,
}

impl WalSegment {
    // TODO: Make this configurable
    const MAX_BLOCK_SIZE: usize = 32_000;

    /// Compute the on-disk representation of the WalSegment
    pub fn disk_repr(&self) -> Result<Vec<u8>, Error> {
        // Use bytes crate for this
        // <-- Op: u8 --><-- Length: u16 --><-- Payload --><-- Padding -->
        let mut wtr = vec![];
        wtr.write_u8(self.segment_type as u8).unwrap();
        wtr.write_u16::<LittleEndian>(self.payload.len() as u16)
            .unwrap();
        wtr.write_all(&self.payload).unwrap();
        wtr.write_all(&self.padding).unwrap();

        Ok(wtr)
    }

    /// TODO: Implement proper error handling
    fn from_disk(data: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(data);
        let segment_type = SegmentType::from_u8(cursor.read_u8().unwrap()).unwrap();
        let payload_len = cursor.read_u16::<LittleEndian>().unwrap();
        let payload = data[3..(3 + payload_len as usize)].to_vec();
        // num bytes remaining in block
        // TODO: Implement more than one record per block
        // let remaining_len = WalSegment::MAX_BLOCK_SIZE - (payload_len + 3) as usize;
        let padding = data[3 + payload_len as usize..].to_vec();
        assert!(
            3 + payload.len() + padding.len() <= WalSegment::MAX_BLOCK_SIZE,
            "Block size must be smaller than MAX_BLOCK_SIZE"
        );
        //let payload_len = data[
        Ok(WalSegment {
            segment_type,
            payload,
            padding,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::prelude::*;
    use std::mem;
    use std::os::unix::prelude::FileExt;

    use byteorder::WriteBytesExt;
    use tempfile::tempdir;
    use tokio_fs::File as TokioFile;
    use tokio_io::io;
    use tokio_threadpool::Builder;

    use futures::future::poll_fn;
    use futures::sync::oneshot;
    use futures::{Future, Sink, Stream};

    #[test]
    fn file_write() {
        let test_data = b"start world";
        let dir = tempdir().unwrap();
        let temp_path = dir.path().join("foo.log");
        let file = File::create(temp_path).unwrap();

        let res = file.write_at(test_data, 0);
        res.unwrap();
        file.sync_data().unwrap();
    }

    #[test]
    fn wal_segment_from_disk_repr() {
        let seg_type = SegmentType::Full;
        let seg_int = seg_type as u8;
        let payload = b"hello world foo bar baz bla bla bla bla bla".to_vec();
        let payload_len = payload.len() as u16;

        let mut wrt = vec![];
        wrt.write_u8(seg_int).unwrap();
        wrt.write_u16::<LittleEndian>(payload_len).unwrap();
        wrt.append(&mut payload.clone());

        let padding_len = WalSegment::MAX_BLOCK_SIZE - (3 + payload_len) as usize;
        let padding = vec![0 as u8; padding_len];
        wrt.append(&mut padding.clone());

        let seg = WalSegment::from_disk(&wrt).unwrap();
        println!("segment size is {:?}", mem::size_of::<WalSegment>());
        assert_eq!(seg.segment_type, SegmentType::Full);
        assert_eq!(seg.padding, padding);
        assert_eq!(seg.payload, payload);
    }

    #[test]
    fn wal_segment_to_disk_repr() {
        let padding_len = WalSegment::MAX_BLOCK_SIZE - (3 + 6) as usize;
        let wal = WalSegment {
            segment_type: SegmentType::Full,
            payload: b"WALL-E".to_vec(),
            padding: vec![0; padding_len],
        };

        let disk_repr = wal.disk_repr().unwrap();
        let str_repr = String::from_utf8(disk_repr.clone()).unwrap();
        assert_eq!(disk_repr.len(), WalSegment::MAX_BLOCK_SIZE);

        let mut expected: Vec<u8> = vec![];
        expected.write_u8(SegmentType::Full as u8).unwrap();
        expected
            .write_u16::<LittleEndian>(wal.payload.len() as u16)
            .unwrap();
        expected.write_all(&wal.payload).unwrap();

        assert_eq!(&disk_repr[0..(3 + wal.payload.len())], &expected as &[u8]);

        let expected_padding: &[u8] = &vec![0u8; padding_len];
        assert_eq!(
            &disk_repr[(3 + wal.payload.len())..disk_repr.len()],
            expected_padding
        );
    }

    #[test]
    fn fs_write() {
        let pool = Builder::new().pool_size(1).build();
        let (tx, rx) = oneshot::channel();

        let dir = tempdir().unwrap();
        let path = dir.path().join("wal.log");
        let contents = b"hello world";
        println!("path is {:?}", path);

        pool.spawn({
            let path = path.clone();
            let contents = contents.clone();
            TokioFile::create(path)
                .and_then(|file| file.metadata())
                .inspect(|&(_, ref metadata)| assert!(metadata.is_file()))
                .and_then(move |(file, _)| io::write_all(file, contents))
                .and_then(|(mut file, _)| poll_fn(move || file.poll_sync_all()))
                .then(|res| {
                    let _ = res.unwrap();
                    tx.send(()).unwrap();
                    Ok(())
                })
        });

        rx.wait().unwrap();

        let mut file = File::open(&path).unwrap();
        let mut dst = vec![];
        file.read_to_end(&mut dst).unwrap();
        assert_eq!(dst, contents);

        println!("At the end....");
    }
}
