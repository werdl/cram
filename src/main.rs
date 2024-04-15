mod types;
mod packer;
mod unpacker;
mod compression;

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

use std::io::Write;

use clap::{
    Subcommand,
    Parser
};

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Pack(Pack),
    Unpack(Unpack),
}

#[derive(Parser)]
struct Pack {
    input: String,
    #[clap(short, long)]
    output: Option<String>,
    #[clap(short, long)]
    compression: Option<String>,
}

#[derive(Parser)]
struct Unpack {
    input: String,
    #[clap(short, long, default_value = ".")]
    output: String,
}

enum Compression {
    Gzip,
    Lzma,
    Zstd,
    Brotli,
    Bzip2,
}

fn probable_compression(contents: &[u8]) -> Compression {
    if contents.starts_with(&[0x1f, 0x8b]) {
        Compression::Gzip
    } else if contents.starts_with(&[0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00]) {
        Compression::Lzma
    } else if contents.starts_with(&[0x28, 0xb5, 0x2f, 0xfd]) {
        Compression::Zstd
    } else if contents.starts_with(&[0x42, 0x5a, 0x68]) {
        Compression::Bzip2
    } else {
        Compression::Brotli // probably (if it isn't compressed, it is dealt with in the unbrotli function)
    }
}

use compression::{
    Compress, Decompress
};
use packer::Serialize;
use types::Parse;
use unpacker::Deserialize;

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Pack(pack) => {
            let entry = types::Entry::open(pack.input.clone()).unwrap();
            let data = entry.serialize();

            let compressed = match pack.compression {
                Some(compression) => {
                    match compression.as_str() {
                        "gzip" => data.gzip(),
                        "lzma" => data.lzma(),
                        "zstd" => data.zstd(),
                        "brotli" => data.brotli(),
                        "bzip2" => data.bzip2(),
                        _ => {
                            println!("Unknown compression type, defaulting to gzip");
                            data.gzip()
                        },
                    }
                }
                None => data,
            };

            let output = match pack.output {
                Some(output) => output,
                None => format!("{}.cram", pack.input),
            };

            std::fs::write(output, compressed).unwrap();
        }
        SubCommand::Unpack(unpack) => {
            let data = std::fs::read(unpack.input).unwrap();
            let compression = probable_compression(&data);

            let decompressed = match compression {
                Compression::Gzip => data.gunzip(),
                Compression::Lzma => data.unlzma(),
                Compression::Zstd => data.unzstd(),
                Compression::Brotli => data.unbrotli(),
                Compression::Bzip2 => data.unbzip2(),
            };

            let entries = <Vec<types::Entry>>::deserialize(&decompressed);

            for entry in entries {
                match entry {
                    types::Entry::File(file) => {
                        let path = std::path::Path::new(&unpack.output).join(&file.name);


                        // ensure the parent directory exists
                        let parent = path.parent().unwrap();

                        if !parent.exists() {
                            std::fs::create_dir_all(parent).unwrap();
                        }


                        let mut fp = std::fs::File::create(path).unwrap();
                        fp.write_all(&file.contents).unwrap();
                    }
                    types::Entry::Directory(dir) => {
                        let path = std::path::Path::new(&unpack.output).join(&dir.name);
                        std::fs::create_dir(&path).unwrap();
                    }
                }
            }
        }
    }
}