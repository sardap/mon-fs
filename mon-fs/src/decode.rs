use std::process::Command;

use mon_fs_box::{
    box_mon::{BoxMon, StringMonParseError, StringsMon},
    file_pc::FilePc,
    pc::PC,
};
use serde::Deserialize;

use crate::{options::OptionsDecode, ProgramError};

pub fn decode_pc_files(pc: &FilePc, options: &OptionsDecode) -> Result<(), ProgramError> {
    let decode_path = &options.decode_to;

    if !decode_path.exists() {
        return Err(ProgramError::BadPathGiven(format!(
            "{}",
            decode_path.display()
        )));
    }

    pc.write_to_folder(decode_path);

    println!("Decoded PC files to: {}", decode_path.display());

    Ok(())
}

#[derive(Debug, Deserialize)]
struct DecoderOutput {
    boxes: Vec<Vec<StringsMon>>,
}

impl TryInto<PC> for DecoderOutput {
    type Error = StringMonParseError;

    fn try_into(self) -> Result<PC, Self::Error> {
        let mut pc = PC::new();
        for (box_index, box_mon) in self.boxes.into_iter().enumerate() {
            for (mon_index, mon) in box_mon.into_iter().enumerate() {
                pc.set_mon(
                    box_index,
                    mon_index,
                    match BoxMon::try_from_strings_mon(mon) {
                        Ok(mon) => mon,
                        Err(err) => return Err(err),
                    },
                );
            }
        }

        Ok(pc)
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

    let pc: PC = output.try_into().unwrap();

    Ok(pc)
}
