use std::io::Write;

use crate::box_mon::{BoxMon, StringsMon};
use crate::marker_mon::MarkerMon;
use crate::mon_field::{BitCount, FromStringInput, GameSerializer, ToGameValueError};
use crate::mon_species::BoxMonSpecies;
use crate::BoxMonBitVec;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde_derive::{Deserialize, Serialize};

const PC_BOX_SIZE: usize = 30;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PcMon {
    BoxMon(BoxMon),
    MarkerMon(MarkerMon),
}

#[derive(Debug)]
pub enum PcMonStringMonParseError {
    BoxMonParseError(crate::box_mon::StringMonParseError),
    MarkerMonParseError(crate::marker_mon::StringMonParseError),
}

impl PcMon {
    pub fn try_from_strings_mon(raw: StringsMon) -> Result<Self, PcMonStringMonParseError> {
        if BoxMonSpecies::try_from_string(&raw.species).is_none() {
            match MarkerMon::try_from_strings_mon(raw) {
                Ok(mon) => return Ok(PcMon::MarkerMon(mon)),
                Err(err) => return Err(PcMonStringMonParseError::MarkerMonParseError(err)),
            }
        } else {
            match BoxMon::try_from_strings_mon(raw) {
                Ok(mon) => return Ok(PcMon::BoxMon(mon)),
                Err(err) => return Err(PcMonStringMonParseError::BoxMonParseError(err)),
            }
        }
    }
}

// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// pub struct PCBox {
//     mons: [; ],
// }

const NUM_PC_BOXES: usize = 14;

const NUM_OF_MONS: usize = PC_BOX_SIZE * NUM_PC_BOXES;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PC {
    pub mons: Vec<Option<PcMon>>,
}

impl BitCount for PC {
    fn bit_count() -> usize {
        return BoxMon::bit_count() * NUM_OF_MONS;
    }
}

impl PC {
    pub fn new() -> PC {
        PC {
            mons: vec![None; NUM_OF_MONS],
        }
    }

    pub fn set_mon(&mut self, box_index: usize, mon_index: usize, mon: PcMon) {
        let index = box_index * PC_BOX_SIZE + mon_index;
        self.mons[index] = Some(mon);
    }

    fn get_empty_offset(&self) -> usize {
        let mut offset = 0;
        for i in 0..self.mons.len() {
            if self.mons[i].is_some() {
                offset += 1;
            } else {
                return offset;
            }
        }
        offset
    }
}

pub struct PCWriter<'a> {
    pc: &'a mut PC,
    current_offset: usize,
}

impl<'a> PCWriter<'a> {
    pub fn new(pc: &'a mut PC) -> PCWriter<'a> {
        let current_offset = pc.get_empty_offset();

        PCWriter { pc, current_offset }
    }

    pub fn write_file<T: ToString>(&mut self, filename: T, buf: &[u8]) -> std::io::Result<usize> {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(buf).unwrap();
        let compressed_bytes = e.finish().unwrap();

        let use_compression = compressed_bytes.len() < buf.len();

        let buf = if use_compression {
            &compressed_bytes
        } else {
            buf
        };

        match MarkerMon::new_starter_marker(filename.to_string(), 0, use_compression) {
            Ok(mon) => mon,
            Err(err) => match err {
                ToGameValueError::BadBitsLength => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "filename too long",
                    ))
                }
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid data",
                    ))
                }
            },
        };

        let marker_mon_offset = self.current_offset;
        self.current_offset += 1;

        match self.write_internal(buf) {
            Ok((bytes_written, padding_amount)) => {
                self.pc.mons[marker_mon_offset] = Some(PcMon::MarkerMon(
                    MarkerMon::new_starter_marker(filename, padding_amount, use_compression)
                        .unwrap(),
                ));
                Ok(bytes_written)
            }
            Err(err) => Err(err),
        }
    }

    fn write_internal(&mut self, buf: &[u8]) -> std::io::Result<(usize, u8)> {
        let bits: BoxMonBitVec = BoxMonBitVec::new_from_raw(buf);

        assert_eq!(buf.len(), bits.0.len() / 8);

        let mut offset = 0;

        loop {
            let end_offset = (offset + BoxMon::bit_count()).min(bits.0.len());

            let mut chunk = bits.chunk(offset, end_offset);
            // Add padding for the last chunk
            let padding_amount = if end_offset == bits.0.len() {
                let padding_amount = BoxMon::bit_count() - chunk.0.len();
                for _ in 0..padding_amount {
                    chunk.0.push(false);
                }
                padding_amount
            } else {
                0
            };

            let mon = match BoxMon::bits_to_game_value(&chunk) {
                Ok(mon) => mon,
                Err(_) => {
                    if offset == 0 {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid data",
                        ));
                    } else {
                        return Ok((offset / 8, padding_amount as u8));
                    }
                }
            };
            self.pc.mons[self.current_offset] = Some(PcMon::BoxMon(mon));
            self.current_offset += 1;

            let amount_left = bits.0.len() - end_offset;
            if amount_left == 0 {
                return Ok((end_offset / 8, padding_amount as u8));
            }
            offset = end_offset;
        }
    }
}

impl std::io::Write for PCWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_internal(buf)
            .map(|(bytes_written, _)| bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct PCReader<'a> {
    pc: &'a PC,
    current_offset: usize,
    current_marker_index: Option<usize>,
    pending_buffer: Vec<u8>,
}

#[derive(Debug)]
pub enum PcReaderError {
    FileNotFound,
    ToGameValueError(ToGameValueError),
}

impl<'a> PCReader<'a> {
    pub fn new(pc: &'a PC) -> PCReader<'a> {
        return PCReader {
            pc,
            current_offset: 0,
            current_marker_index: None,
            pending_buffer: Vec::new(),
        };
    }

    pub fn seek_file(&mut self, filename: &str) -> Result<(), PcReaderError> {
        for i in 0..self.pc.mons.len() {
            if let Some(mon) = self.pc.mons[i] {
                if let PcMon::MarkerMon(marker_mon) = mon {
                    if marker_mon.to_string() == filename {
                        self.current_marker_index = Some(i);
                        self.current_offset = i + 1;
                        return Ok(());
                    }
                }
            }
        }

        Err(PcReaderError::FileNotFound)
    }

    pub fn list_files(&self) -> Vec<String> {
        let mut names = Vec::new();
        for i in 0..self.pc.mons.len() {
            if let Some(mon) = self.pc.mons[i] {
                if let PcMon::MarkerMon(marker_mon) = mon {
                    names.push(marker_mon.to_string());
                }
            }
        }
        names
    }

    fn copy_pending_buffer_into(&mut self, buf: &mut [u8]) -> usize {
        let len = buf.len().min(self.pending_buffer.len());
        buf[..len].copy_from_slice(&self.pending_buffer[..len]);
        self.pending_buffer = self.pending_buffer.split_off(len);
        len
    }
}

impl std::io::Read for PCReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.pending_buffer.len() > 0 {
            return Ok(self.copy_pending_buffer_into(buf));
        }

        if self.current_marker_index.is_none() {
            return Ok(0);
        }

        let mut bits = BoxMonBitVec::default();
        loop {
            let mon = self.pc.mons[self.current_offset];
            if let Some(mon) = mon {
                match mon {
                    PcMon::BoxMon(mon) => {
                        let next_set = match mon.game_value_to_bits() {
                            Ok(val) => val,
                            Err(_) => {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    "Invalid data",
                                ))
                            }
                        };
                        bits.0.extend(next_set.0.iter());
                    }
                    PcMon::MarkerMon(_) => {
                        break;
                    }
                }
            } else {
                break;
            }

            self.current_offset += 1;
            if self.current_offset >= self.pc.mons.len() {
                break;
            }
        }

        // Remove padding zeroes
        let compression: bool;
        match self.pc.mons[self.current_marker_index.unwrap()]
            .as_ref()
            .unwrap()
        {
            PcMon::MarkerMon(mon) => {
                let padding = mon.padding_amount();
                for _ in 0..padding {
                    bits.0.pop();
                }
                compression = mon.is_compressed();
                self.current_marker_index = None;
            }
            _ => {
                panic!("Invalid state");
            }
        }

        if compression {
            use flate2::read::ZlibDecoder;

            let raw = bits.to_raw();
            let mut decoder = ZlibDecoder::new(raw.as_slice());
            let mut decoded = Vec::new();
            decoder.read_to_end(&mut decoded).unwrap();
            bits = BoxMonBitVec::new_from_raw(&decoded);
        }

        assert_eq!(bits.0.len() % 8, 0);
        self.pending_buffer = bits.to_raw();

        Ok(self.copy_pending_buffer_into(buf))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;
    use std::{fs::File, io::Read};
    use tempdir::TempDir;

    #[test]
    fn test_write_arbitrary_data_to_pc() {
        let mut pc = PC::new();
        let mut pc_writer = PCWriter::new(&mut pc);

        let message_1 = " What the fuck did you just fucking say about me, you little bitch? I'll have you know I graduated top of my class in the Navy Seals, and I've been involved in numerous secret raids on Al-Quaeda, and I have over 300 confirmed kills. I am trained in gorilla warfare and I'm the top sniper in the entire ";
        pc_writer.write_file("a.txt", message_1.as_bytes()).unwrap();

        let message_2 = "폴 키팅은 최고야.";
        pc_writer.write_file("b.txt", message_2.as_bytes()).unwrap();

        {
            let mut reader = PCReader::new(&pc);
            reader.seek_file("a.txt").unwrap();

            let mut buf = vec![0; message_1.len()];
            reader.read(&mut buf).unwrap();

            let message_decoded = String::from_utf8(buf).unwrap();

            assert_eq!(message_1, message_decoded);
        }

        {
            let mut reader = PCReader::new(&pc);
            reader.seek_file("b.txt").unwrap();

            let mut buf = vec![0; message_2.len()];
            reader.read(&mut buf).unwrap();

            let message_decoded = String::from_utf8(buf).unwrap();

            assert_eq!(message_2, message_decoded);
        }

        {
            let reader = PCReader::new(&pc);
            let files = reader.list_files();

            assert_eq!(files.len(), 2);
            assert!(files.contains(&"a.txt".to_string()));
            assert!(files.contains(&"b.txt".to_string()));
        }
    }

    #[test]

    fn test_writing_real_file_to_pc() {
        const FILE_NAME: &str = "dot.txt";
        let tmp_dir = TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join(FILE_NAME);

        {
            let mut tmp_file = File::create(&file_path).unwrap();
            writeln!(tmp_file, "OMG").unwrap();
            writeln!(tmp_file, "This is a straight up nighamre").unwrap();
            writeln!(tmp_file, "Why isn't it encoding right?").unwrap();
            writeln!(tmp_file, "Like really just fucking work").unwrap();
            writeln!(tmp_file, "I want more text so i can text compression.").unwrap();
            writeln!(tmp_file, "The last jedi was a great movie.").unwrap();
            writeln!(tmp_file, "The last jedi was a great movie.").unwrap();
            writeln!(tmp_file, "The last jedi was a great movie.").unwrap();
            writeln!(tmp_file, "The last jedi was a great movie.").unwrap();
            writeln!(tmp_file, "The last jedi was a great movie.").unwrap();
        }

        let pc_json: String;

        let input_data = std::fs::read(&file_path).unwrap();
        {
            let mut pc = PC::new();
            let mut pc_writer = PCWriter::new(&mut pc);
            pc_writer.write_file(FILE_NAME, &input_data).unwrap();

            pc_json = serde_json::to_string(&pc).unwrap();
        }

        let mut output_data = Vec::new();
        {
            let pc: PC = serde_json::from_str(&pc_json).unwrap();
            let mut reader = PCReader::new(&pc);
            reader.seek_file(FILE_NAME).unwrap();
            reader.read_to_end(&mut output_data).unwrap();
        }

        assert_eq!(input_data, output_data);

        tmp_dir.close().unwrap();
    }

    #[test]
    fn test_complex_file_encode_decode() {
        let to_encode: Vec<u8> = include_bytes!("../../test_assets/p.webp")
            .into_iter()
            .cloned()
            .collect();

        let pc_json;
        {
            let mut pc = PC::new();
            let mut pc_writer = PCWriter::new(&mut pc);
            pc_writer.write_file("p.webp", &to_encode).unwrap();

            let mut pc_reader = PCReader::new(&pc);
            pc_reader.seek_file("p.webp").unwrap();
            let mut buffer = Vec::new();
            pc_reader.read_to_end(&mut buffer).unwrap();

            assert_eq!(to_encode.len(), buffer.len());

            pc_json = serde_json::to_string(&pc).unwrap();
        }

        let mut output_data = Vec::new();
        {
            let pc: PC = serde_json::from_str(&pc_json).unwrap();
            let mut reader = PCReader::new(&pc);
            reader.seek_file("p.webp").unwrap();
            reader.read_to_end(&mut output_data).unwrap();
        }

        assert_eq!(to_encode.len(), output_data.len());

        assert_eq!(to_encode, output_data);
    }
}
