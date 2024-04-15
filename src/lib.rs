mod types;
mod packer;
mod unpacker;


#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::types::{Entry, Parse};
    use crate::packer::Serialize;
    use crate::unpacker::Deserialize;

    use super::*;

    #[test]
    fn it_works() {
        let a = types::Directory::open("src".to_string()).unwrap();

        let b = a.serialize();

        
        // print contents of first file
        let d = <Vec<Entry>>::deserialize(&b);

        // for each entry, print the name, modified time, created time, and size
        for entry in d {
            match entry {
                Entry::File(file) => {
                    println!("File: {} - Modified: {} - Created: {} - Size: {}", file.name, file.updated_at, file.created_at, file.size);
                }
                Entry::Directory(dir) => {
                    println!("Directory: {} - Modified: {} - Created: {}", dir.name, dir.updated_at, dir.created_at);
                }
            }
        }
    }
}