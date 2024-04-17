//! packer - take in an `Entry`, turn it into a .cram file
use crate::types::Entry;

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

/**
 * Format
 * The format starts with the header - the file name and a null byte, then the modified time as a u64, then the created time as a u64
 * Then the file size in bytes as a u64
 */

impl Serialize for Entry {
    fn serialize(&self) -> Vec<u8> {
        match self {
            Entry::File(file) => {
                let mut data = Vec::new();

                let name = file.name.as_bytes();

                // write the file name then a null byte
                data.extend(name.iter());
                data.push(0);

                // write the modified time
                data.extend(file.updated_at.to_le_bytes().iter());

                // write the created time
                data.extend(file.created_at.to_le_bytes().iter());

                let size = file.contents.len().to_le_bytes();
                data.extend(size.iter());

                // now write the file contents
                data.extend(file.contents.iter());

                data
            }
            Entry::Directory(dir) => {
                let mut vec = Vec::new();
                for entry in &dir.contents {
                    let data = entry.serialize();

                    vec.extend(data.iter());
                }

                vec
            }
        }
    }
}
