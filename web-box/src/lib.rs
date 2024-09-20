use std::io::Write;

use mon_fs_box::{file_pc::FilePc, pc::PC};
use wasm_bindgen::prelude::*;
use zip::{write::SimpleFileOptions, ZipWriter};

#[wasm_bindgen]
pub fn encode_file(existing_pc: String, filename: String, to_encode: Vec<u8>) -> String {
    let mut pc: PC = serde_json::from_str(&existing_pc).unwrap();
    pc.fill_empty_mon_slots();

    let mut file_pc = if let Some(file_pc) = FilePc::new_from_pc(pc) {
        file_pc
    } else {
        FilePc::new()
    };

    file_pc.add_file_raw(&filename, to_encode).unwrap();

    let pc: PC = file_pc.into();

    serde_json::to_string(&pc).unwrap()
}

#[wasm_bindgen]
pub fn decode_file(existing_pc: String) -> Vec<u8> {
    let mut pc: PC = serde_json::from_str(&existing_pc).unwrap();

    pc.fill_empty_mon_slots();

    let file_pc = FilePc::new_from_pc(pc).unwrap();

    let mut result = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut result);
    let mut zip = ZipWriter::new(&mut cursor);

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for file in &file_pc.files {
        zip.start_file(file.name.to_string(), options).unwrap();

        zip.write_all(&file.get_data()).unwrap();
    }

    zip.finish().unwrap();

    result
}

#[cfg(test)]
mod test {
    use mon_fs_box::mon_field::ByteCount;
    use rand::Rng;

    use super::*;

    #[test]
    fn file_encode_decode_files() {
        const FILE_COUNT: usize = 8;

        let mut data_chunk = vec![0; PC::byte_count() / 10];
        for i in 0..data_chunk.len() {
            data_chunk[i] = rand::thread_rng().gen()
        }

        let file_pc = FilePc::new();

        let pc: PC = file_pc.into();

        let mut pc_json = serde_json::to_string(&pc).unwrap();

        for i in 0..FILE_COUNT {
            pc_json = encode_file(pc_json, format!("test_{}.txt", i), data_chunk.clone());

            let pc = serde_json::from_str(&pc_json).unwrap();
            let file_pc = mon_fs_box::file_pc::FilePc::new_from_pc(pc).unwrap();
            assert_eq!(file_pc.files.len(), i + 1);
        }

        let pc = serde_json::from_str(&pc_json).unwrap();
        let pc = mon_fs_box::file_pc::FilePc::new_from_pc(pc).unwrap();

        assert_eq!(pc.files.len(), FILE_COUNT);

        decode_file(pc_json);
    }

    #[test]
    #[should_panic]
    fn fail_encode_duplicated_file() {
        let data = vec![0; 10];
        let file_pc = FilePc::new();
        let pc: PC = file_pc.into();
        let mut pc_json = serde_json::to_string(&pc).unwrap();

        pc_json = encode_file(pc_json, "test.txt".to_string(), data.clone());

        encode_file(pc_json, "test.txt".to_string(), data.clone());
    }
}
