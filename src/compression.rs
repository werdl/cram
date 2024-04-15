use std::io::{Read, Write};

pub trait Compress {
    fn gzip(&self) -> Vec<u8>;
    fn lzma(&self) -> Vec<u8>;
    fn zstd(&self) -> Vec<u8>;
    fn brotli(&self) -> Vec<u8>;
    fn bzip2(&self) -> Vec<u8>;
}

impl Compress for Vec<u8> {
    fn gzip(&self) -> Vec<u8> {
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(self).unwrap();
        encoder.finish().unwrap()
    }

    fn lzma(&self) -> Vec<u8> {
        let mut encoder = xz2::write::XzEncoder::new(Vec::new(), 6);
        encoder.write_all(self).unwrap();
        encoder.finish().unwrap()
    }

    fn zstd(&self) -> Vec<u8> {
        zstd::stream::encode_all(self.as_slice(), 3).unwrap()
    }

    fn brotli(&self) -> Vec<u8> {
        let mut encoder = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
        encoder.write_all(self).unwrap();
        encoder.flush().unwrap();
        encoder.into_inner()
    }

    fn bzip2(&self) -> Vec<u8> {
        let mut encoder = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::new(9));
        encoder.write_all(self).unwrap();
        encoder.finish().unwrap()
    }
}

pub trait Decompress {
    fn gunzip(&self) -> Vec<u8>;
    fn unlzma(&self) -> Vec<u8>;
    fn unzstd(&self) -> Vec<u8>;
    fn unbrotli(&self) -> Vec<u8>;
    fn unbzip2(&self) -> Vec<u8>;
}

impl Decompress for Vec<u8> {
    fn gunzip(&self) -> Vec<u8> {
        let mut decoder = flate2::read::GzDecoder::new(self.as_slice());
        let mut data = Vec::new();
        decoder.read_to_end(&mut data).unwrap();
        data
    }

    fn unlzma(&self) -> Vec<u8> {
        let mut decoder = xz2::read::XzDecoder::new(self.as_slice());
        let mut data = Vec::new();
        decoder.read_to_end(&mut data).unwrap();
        data
    }

    fn unzstd(&self) -> Vec<u8> {
        zstd::stream::decode_all(self.as_slice()).unwrap()
    }

    fn unbrotli(&self) -> Vec<u8> {
        let mut decoder = brotli::Decompressor::new(self.as_slice(), 4096);

        let mut data = Vec::new();
        
        match decoder.read_to_end(&mut data) {
            Ok(_) => {},
            Err(_) => {
                // brotli is the default: thus if we don't decompress, we assume it's not compressed - return the original data
                data = self.clone();
            }
        }

        data
    }

    fn unbzip2(&self) -> Vec<u8> {
        let mut decoder = bzip2::read::BzDecoder::new(self.as_slice());
        let mut data = Vec::new();
        decoder.read_to_end(&mut data).unwrap();
        data
    }
}