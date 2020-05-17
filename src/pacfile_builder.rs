use crate::proto::index as proto;
use crate::{Error, Result};

use protobuf::Message;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub enum DataSource {
    File(File),
    Path(String),
    Vec(Vec<u8>),
    // TODO: add pacfile entry
}

impl From<File> for DataSource {
    fn from(file: File) -> DataSource {
        DataSource::File(file)
    }
}

impl From<String> for DataSource {
    fn from(file: String) -> DataSource {
        DataSource::Path(file)
    }
}

impl From<Vec<u8>> for DataSource {
    fn from(file: Vec<u8>) -> DataSource {
        DataSource::Vec(file)
    }
}

impl Read for DataSource {
    fn read(&mut self, buffer: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        match self {
            DataSource::File(file) => file.read(buffer),
            DataSource::Path(path) => {
                *self = DataSource::File(File::open(path)?);
                self.read(buffer)
            }
            DataSource::Vec(vec) => (&vec[..]).read(buffer),
        }
    }
}

struct FileEntry {
    name: String,
    data: DataSource,
}

#[derive(Default)]
pub struct PacfileBuilder {
    entries: Vec<FileEntry>,
}

impl PacfileBuilder {
    pub fn new() -> PacfileBuilder {
        PacfileBuilder {
            entries: Vec::new(),
        }
    }

    pub fn add<D: Into<DataSource>>(&mut self, name: String, data: D) {
        self.entries.push(FileEntry {
            name,
            data: data.into(),
        });
    }

    pub fn sort(&mut self) {
        self.entries.sort_by(|a, b| a.name.cmp(&b.name));
    }

    pub fn build<P: AsRef<Path>>(&mut self, index_file: P) -> Result<()> {
        let index_file = index_file.as_ref();
        let data_file = index_file.with_extension(crate::format::DATA_FILE_EXTENSION);
        let mut index_file = File::create(index_file)?;
        let mut data_file = File::create(data_file)?;
        index_file.write_all(&crate::format::MAGIC_NUMBER)?;

        let mut package_index = proto::PackageIndex::new();
        let mut current_offset: u64 = 0;

        for entry in self.entries.iter_mut() {
            let mut entry_proto = proto::File::new();
            entry_proto.set_name(entry.name.clone());
            entry_proto.set_offset(current_offset);
            let len = std::io::copy(&mut entry.data, &mut data_file)?;
            entry_proto.set_length(len);
            current_offset += len;
            package_index.mut_files().push(entry_proto);
        }

        match package_index.write_to_writer(&mut index_file) {
            Ok(_) => Ok(()),
            Err(err) => {
                if let protobuf::ProtobufError::IoError(err) = err {
                    Err(Error::IoError(err))
                } else {
                    Err(Error::Unknown)
                }
            }
        }
    }
}
