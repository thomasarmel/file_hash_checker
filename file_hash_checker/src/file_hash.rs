use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{Read, Seek};
use xxhash_rust::xxh3::Xxh3;

const FILE_BUFFER_SIZE: usize = 16384; // 2^14

pub struct FileHash {
    file: File,
}

impl FileHash {
    pub fn new(filename: &str) -> Result<Self, &'static str> {
        Ok(Self {
            file: File::open(filename).map_err(|_| "Could not open file")?,
        })
    }

    pub fn get_hash(&mut self) -> Result<u64, &'static str> { // TODO: use thread ?
        self.file.seek(std::io::SeekFrom::Start(0)).map_err(|_| "Cannot read file: seek() error")?;
        let mut buffer: Vec<u8> = vec![0 as u8; FILE_BUFFER_SIZE];
        let mut hasher = Box::new(Xxh3::new()); // preferable in heap according documentation
        loop {
            let bytes_read = self.file.read(&mut buffer.borrow_mut()).map_err(|_| "Could not read file")?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        Ok(hasher.digest())
    }
}