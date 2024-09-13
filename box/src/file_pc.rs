use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use serde::{Deserialize, Serialize};

use crate::pc::PC;

#[derive(Debug, Serialize, Deserialize)]
pub struct PcFile {
    pub name: String,
    pub attributes: u8,
    pub data: Vec<u8>,
}

pub const COMPRESSION_LEVEL: Compression = Compression::best();

impl PcFile {
    pub fn new(name: &str, data: &[u8]) -> PcFile {
        let mut e = ZlibEncoder::new(Vec::new(), COMPRESSION_LEVEL);
        e.write_all(&data).unwrap();
        let compressed_data = e.finish().unwrap();

        let compressed = compressed_data.len() < data.len();

        let data = if compressed {
            compressed_data
        } else {
            data.to_vec()
        };

        PcFile {
            name: name.to_string(),
            attributes: if compressed { 0x01 } else { 0x00 },
            data,
        }
    }

    pub fn is_compressed(&self) -> bool {
        self.attributes & 0x01 == 0x01
    }

    pub fn write_to_folder(&self, folder: &PathBuf) {
        // This is shit but I don't care
        let data = if self.is_compressed() {
            let mut d = ZlibDecoder::new(&self.data[..]);
            let mut data = Vec::new();
            d.read_to_end(&mut data).unwrap();
            data
        } else {
            self.data.clone()
        };

        let file_path = folder.clone().join(&self.name);
        let mut file = if file_path.exists() {
            std::fs::File::open(file_path).unwrap()
        } else {
            std::fs::File::create(file_path).unwrap()
        };
        file.write_all(&data).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePc {
    pub files: Vec<PcFile>,
}

impl FilePc {
    pub fn new() -> Self {
        FilePc { files: Vec::new() }
    }

    pub fn new_from_pc(mut pc: PC) -> Option<Self> {
        let mut buf = Vec::new();
        if pc.read_to_end(&mut buf).is_err() {
            return None;
        }
        match bincode::deserialize(&buf) {
            Ok(file_pc) => Some(file_pc),
            Err(_) => None,
        }
    }

    pub fn add_file(&mut self, name: &str, source: &PathBuf) -> Result<(), io::Error> {
        let data = match File::open(source) {
            Ok(mut file) => {
                let mut buf = Vec::new();
                if let Err(err) = file.read_to_end(&mut buf) {
                    return Err(err);
                }
                buf
            }
            Err(err) => return Err(err),
        };

        self.add_file_raw(name, &data);

        Ok(())
    }

    pub fn add_file_raw(&mut self, name: &str, data: &[u8]) {
        self.files.push(PcFile::new(name, data));
    }

    pub fn write_to_folder(&self, folder: &PathBuf) {
        if !folder.exists() {
            std::fs::create_dir_all(folder).unwrap();
        }

        if !folder.is_dir() {
            panic!("Fuck");
        }

        for file in &self.files {
            file.write_to_folder(folder);
        }
    }

    pub fn as_pc(&self) -> Result<PC, std::io::Error> {
        let encoded = bincode::serialize(&self).unwrap();

        let mut pc = PC::new();
        match pc.write_all(&encoded) {
            Ok(_) => Ok(pc),
            Err(err) => Err(err),
        }
    }
}

impl Into<PC> for FilePc {
    fn into(self) -> PC {
        self.as_pc().unwrap()
    }
}

impl From<PC> for FilePc {
    fn from(value: PC) -> Self {
        Self::new_from_pc(value).unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::{fs::File, path::PathBuf};

    use crate::{mon_field::ByteCount, pc::PC};
    use tempdir::TempDir;

    use super::FilePc;

    #[test]
    fn test_copy_file_into_pc() {
        let out_dir = TempDir::new("test_copy_file_into_pc_out").unwrap();

        let mut total_size = 0;
        let pic_file_path = out_dir.path().join("ricky.webp");
        {
            let mut tmp_file = File::create(&pic_file_path).unwrap();
            let buffer = include_bytes!("../../test_assets/ricky.webp");
            total_size += buffer.len();
            let mut pic = std::io::Cursor::new(buffer);
            std::io::copy(&mut pic, &mut tmp_file).unwrap();
        }
        let song_file_path = out_dir.path().join("song.opus");
        {
            let mut tmp_file = File::create(&song_file_path).unwrap();
            let buffer = include_bytes!("../../test_assets/song.opus");
            total_size += buffer.len();
            let mut song = std::io::Cursor::new(buffer);
            std::io::copy(&mut song, &mut tmp_file).unwrap();
        }

        assert!(PC::byte_count() >= total_size);

        let mut file_pc = FilePc::new();
        file_pc.add_file("ricky.webp", &pic_file_path).unwrap();
        file_pc.add_file("song.opus", &song_file_path).unwrap();

        let pc: PC = file_pc.into();

        let file_pc: FilePc = pc.into();

        let out_dir = TempDir::new("test_copy_file_into_pc_out").unwrap();
        {
            let path: PathBuf = out_dir.path().into();
            file_pc.write_to_folder(&path);

            let ricky = std::fs::read(path.join("ricky.webp")).unwrap();
            assert_eq!(
                ricky.len(),
                include_bytes!("../../test_assets/ricky.webp").len()
            );

            let song = std::fs::read(path.join("song.opus")).unwrap();
            assert_eq!(
                song.len(),
                include_bytes!("../../test_assets/song.opus").len()
            );
        }
    }
}
