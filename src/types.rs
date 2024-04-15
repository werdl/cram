use std::{io::Read, time::SystemTimeError};

#[derive(Debug)]
pub struct File {
    pub name: String, // File name - must be padded to 100 characters with null bytes
    pub size: u64, // File size in bytes
    pub created_at: u64, // Unix timestamp of file creation
    pub updated_at: u64, // Unix timestamp of last file update
    pub contents: Vec<u8>, // File contents
}

#[derive(Debug)]
pub struct Directory {
    pub name: String, // Directory name - must be padded to 100 characters with null bytes
    pub created_at: u64, // Unix timestamp of directory creation
    pub updated_at: u64, // Unix timestamp of last directory update
    pub contents: Vec<Entry>, // Files and directories contained within this directory
}

#[derive(Debug)]
pub enum Entry {
    File(File),
    Directory(Directory),
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    TimeError(SystemTimeError),

    #[allow(dead_code)]
    OtherError(String),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<SystemTimeError> for Error {
    fn from(error: SystemTimeError) -> Self {
        Error::TimeError(error)
    }
}


pub trait Parse 
where Self: Sized {
    fn open(name: String) -> Result<Entry, Error>;
}

impl Parse for File {
    fn open(name: String) -> Result<Entry, Error> {
        // open the file at the given path
        // read the file contents

        let mut fp = std::fs::File::open(name.clone())?;

        let metadata = fp.metadata()?;

        let mut contents = Vec::new();
        
        fp.read_to_end(&mut contents)?;

        Ok(Entry::File(File {
            name,
            size: metadata.len(),
            created_at: metadata.created()?.duration_since(std::time::SystemTime::UNIX_EPOCH)?.as_secs(),
            updated_at: metadata.modified()?.duration_since(std::time::SystemTime::UNIX_EPOCH)?.as_secs(),
            contents,
        }))
    }
}

impl Parse for Directory {
    fn open(name: String) -> Result<Entry, Error> {
        // open the directory at the given path
        // read the directory contents

        let metadata = std::fs::metadata(name.clone())?;

        let mut contents = Vec::new();

        for entry in std::fs::read_dir(name.clone())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                contents.push(Directory::open(path.to_str().unwrap().to_string())?);
            } else {
                contents.push(File::open(path.to_str().unwrap().to_string())?);
            }
        }

        Ok(Entry::Directory(Directory {
            name,
            created_at: metadata.created()?.duration_since(std::time::SystemTime::UNIX_EPOCH)?.as_secs(),
            updated_at: metadata.modified()?.duration_since(std::time::SystemTime::UNIX_EPOCH)?.as_secs(),
            contents, 
        }))
    }
}
