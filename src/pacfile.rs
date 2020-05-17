use crate::proto::index as proto;
use crate::{Error, Result};

use protobuf;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

struct FileEntry {
    offset: u64,
    length: u64,
}

pub struct Pacfile {
    entries: HashMap<String, FileEntry>,
    data_file: File,
}

impl Pacfile {
    fn read_index_proto(read: &mut dyn Read) -> Result<proto::PackageIndex> {
        match protobuf::parse_from_reader(read) {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::InvalidIndexFile),
        }
    }

    pub fn from_file<P: AsRef<Path>>(file: P) -> Result<Pacfile> {
        let file: &Path = file.as_ref();
        let mut index = File::open(file)?;
        let data_file = file.with_extension("p2d");
        let data_file = File::open(data_file)?;

        let mut magic_number: [u8; 4] = [0; 4];
        index.read_exact(&mut magic_number)?;
        if magic_number != crate::format::MAGIC_NUMBER {
            return Err(Error::InvalidIndexFile);
        }

        let mut index = Pacfile::read_index_proto(&mut index)?;

        let mut entries = HashMap::new();
        for mut file in index.take_files().into_iter() {
            entries.insert(
                file.take_name(),
                FileEntry {
                    offset: file.get_offset(),
                    length: file.get_length(),
                },
            );
        }

        Ok(Pacfile { entries, data_file })
    }

    pub fn read(&mut self, file: &str) -> Result<Vec<u8>> {
        let entry = self.entries.get(file);
        match entry {
            Some(entry) => {
                let _ = self.data_file.seek(SeekFrom::Start(entry.offset))?;
                let mut buf = vec![0; entry.length as usize];
                self.data_file.read_exact(&mut buf)?;
                Ok(buf)
            }
            None => Err(Error::NotFound),
        }
    }
}
