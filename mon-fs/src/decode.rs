use std::{
    fs::File,
    io::{self},
    process::Command,
};

use mon_fs_box::{
    box_mon::StringsMon,
    pc::{PCReader, PcMon, PC},
};
use serde::Deserialize;

use crate::{options::OptionsDecode, ProgramError};

pub fn decode_pc_files(pc: &PC, options: &OptionsDecode) -> Result<(), ProgramError> {
    let decode_path = &options.decode_to;

    if !decode_path.exists() {
        return Err(ProgramError::BadPathGiven(format!(
            "{}",
            decode_path.display()
        )));
    }

    let reader = PCReader::new(pc);
    let files = reader.list_files();
    println!("Files to decode: {:?}", files);

    for file in files {
        println!("Decoding file: {}", file);
        let mut reader = PCReader::new(pc);
        reader.seek_file(&file).unwrap();

        let mut file = File::create(format!("{}/{}", decode_path.display(), file)).unwrap();
        io::copy(&mut reader, &mut file).unwrap();
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct DecoderOutput {
    boxes: Vec<Vec<StringsMon>>,
}

impl Into<PC> for DecoderOutput {
    fn into(self) -> PC {
        let mut pc = PC::new();
        for box_index in 0..self.boxes.len() {
            for mon_index in 0..self.boxes[box_index].len() {
                let mon = StringsMon {
                    name: self.boxes[box_index][mon_index].name.clone(),
                    species: self.boxes[box_index][mon_index].species.clone(),
                    gender: self.boxes[box_index][mon_index].gender.clone(),
                    item: self.boxes[box_index][mon_index].item.clone(),
                };
                pc.set_mon(
                    box_index,
                    mon_index,
                    PcMon::try_from_strings_mon(mon).unwrap(),
                );
            }
        }

        pc
    }
}

pub fn load_pc_from_screenshots(options: &OptionsDecode) -> Result<PC, ProgramError> {
    let pc_screenshots = std::fs::canonicalize(&options.pc_screenshots).unwrap();

    if !pc_screenshots.exists() || !pc_screenshots.is_dir() {
        return Err(ProgramError::BadPathGiven(format!(
            "{}",
            pc_screenshots.display()
        )));
    }

    let output: std::process::Output = Command::new("poetry")
        .arg("run")
        .arg("decoder")
        .current_dir(options.python_script_path.as_os_str())
        .env(
            "PC_DEC_SCREENSHOT_FOLDER",
            pc_screenshots.display().to_string(),
        )
        .output()
        .unwrap();

    if !output.status.success() {
        return Err(ProgramError::DecoderFailure(format!(
            "{}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));

    let output = String::from_utf8_lossy(&output.stdout).to_string();

    let output: DecoderOutput = serde_json::from_str(&output).unwrap();

    let pc: PC = output.into();

    Ok(pc)
}
