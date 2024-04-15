//! unpacker - unpack a .cram file into an `Entry`

use crate::types::{Directory, Entry, File};

pub trait Deserialize 
where Self: Sized {
    fn deserialize(data: &[u8]) -> Self;
}

impl Deserialize for Vec<Entry> {
    fn deserialize(data: &[u8]) -> Vec<Entry> {
        let mut entries = Vec::new();

        let mut i = 0;

        while i < data.len() {
            let name = String::from_utf8(data[i..].iter().take_while(|&&c| c != 0).cloned().collect()).unwrap();
            i += name.len() + 1;

            let updated_at = u64::from_le_bytes(data[i..i+8].try_into().unwrap());

            i += 8;

            let created_at = u64::from_le_bytes(data[i..i+8].try_into().unwrap());

            i += 8;

            let size = u64::from_le_bytes(data[i..i+8].try_into().unwrap()) as usize;
            i += 8;

            let contents = data[i..i+size].to_vec();
            i += size;

            entries.push(Entry::File(File {
                name,
                size: size as u64,
                created_at,
                updated_at,
                contents,
            }));
        }

        entries
    }
}