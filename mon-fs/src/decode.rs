use std::{fs, path::PathBuf, process::Command};

use mon_fs_box::{
    box_mon::{BoxMon, StringMonParseError, StringsMon},
    file_pc::FilePc,
    pc::{PC, PC_BOX_SIZE},
};
use serde::Deserialize;

use crate::{options::OptionsLiteDecode, ProgramError};

pub fn decode_pc_files(pc: &FilePc, decode_to: &PathBuf) -> Result<(), ProgramError> {
    let decode_path = &decode_to;

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
struct PythonDecoderOutput {
    boxes: Vec<Vec<StringsMon>>,
}

impl<T: BoxMon> TryInto<PC<T>> for PythonDecoderOutput {
    type Error = StringMonParseError;

    fn try_into(self) -> Result<PC<T>, Self::Error> {
        let mut pc = PC::new();
        for (box_index, box_mon) in self.boxes.into_iter().enumerate() {
            for (mon_index, mon) in box_mon.into_iter().enumerate() {
                let index = box_index * PC_BOX_SIZE + mon_index;
                pc.set_mon(
                    index,
                    match T::try_from(mon) {
                        Ok(mon) => mon,
                        Err(err) => return Err(err),
                    },
                );
            }
        }

        Ok(pc)
    }
}

pub fn load_pc_from_screenshots<T: BoxMon>(
    options: &OptionsLiteDecode,
) -> Result<PC<T>, ProgramError> {
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

    let output: PythonDecoderOutput = serde_json::from_str(&output).unwrap();

    let pc: PC<T> = output.try_into().unwrap();

    Ok(pc)
}

#[derive(Debug, Deserialize)]
struct PkHexDecoderOutput {
    mons: Vec<StringsMon>,
}

impl<T: BoxMon> TryInto<PC<T>> for PkHexDecoderOutput {
    type Error = StringMonParseError;

    fn try_into(self) -> Result<PC<T>, Self::Error> {
        let mut pc = PC::new();
        for (mon_index, mon) in self.mons.into_iter().enumerate() {
            pc.set_mon(
                mon_index,
                match T::try_from(mon) {
                    Ok(mon) => mon,
                    Err(err) => {
                        return Err(err);
                    }
                },
            );
        }

        Ok(pc)
    }
}

pub fn load_guide_from_save<T: BoxMon + for<'de> Deserialize<'de>>(
    pk_hex_mon_fs: &PathBuf,
    save_path: &PathBuf,
    guide_file_path: &PathBuf,
) -> Result<PC<T>, ProgramError> {
    let output: std::process::Output = Command::new(pk_hex_mon_fs)
        .arg("decode")
        .arg(save_path)
        .arg(guide_file_path)
        .output()
        .unwrap();

    let output = match fs::read_to_string(&guide_file_path) {
        Ok(data) => data,
        Err(_) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            return Err(ProgramError::PKHexMonFSOutputError);
        }
    };

    let mut pc: PC<T> = serde_json::from_str(&output).unwrap();

    pc.fill_empty_mon_slots();

    Ok(pc)
}
