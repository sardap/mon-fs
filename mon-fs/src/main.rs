use std::fs::{self, File};

use mon_fs_box::{file_pc::FilePc, pc::PC};
use structopt::StructOpt;

mod decode;
mod encode;
mod options;

#[derive(Debug)]
enum ProgramError {
    #[allow(dead_code)]
    BadGuideFileGiven(String),
    #[allow(dead_code)]
    BadPathGiven(String),
    #[allow(dead_code)]
    BadModeGiven,
    #[allow(dead_code)]
    DecoderFailure(String),
    #[allow(dead_code)]
    IoError(std::io::Error),
}

fn main() -> Result<(), ProgramError> {
    let options = options::Options::from_args();

    let mut file_pc = if options.pc_file.exists() {
        let existing = match fs::read(&options.pc_file) {
            Ok(data) => data,
            Err(_) => {
                return Err(ProgramError::BadGuideFileGiven(format!(
                    "{}",
                    options.pc_file.display()
                )));
            }
        };

        // Decode to PC
        match serde_json::from_slice::<PC>(&existing) {
            Ok(pc) => pc.into(),
            Err(_) => {
                return Err(ProgramError::BadGuideFileGiven(format!(
                    "{}",
                    options.pc_file.display()
                )));
            }
        }
    } else {
        FilePc::new()
    };

    match options.command {
        options::Command::Encode(options_encode) => {
            if let Err(err) = encode::encode_file_to_file_pc(&mut file_pc, &options_encode) {
                return Err(err);
            }
        }
        options::Command::Decode(options_decode) => {
            println!("Parsing screenshots...");
            let pc = match decode::load_pc_from_screenshots(&options_decode) {
                Ok(pc) => pc,
                Err(err) => {
                    return Err(err);
                }
            };

            let file_pc = pc.into();

            if let Err(err) = decode::decode_pc_files(&file_pc, &options_decode) {
                return Err(err);
            }
        }
    }

    // Delete old guide file
    if options.pc_file.exists() {
        fs::remove_file(&options.pc_file).unwrap();
    }

    let pc: PC = match file_pc.as_pc() {
        Ok(pc) => pc,
        Err(err) => {
            return Err(ProgramError::IoError(err));
        }
    };
    let file = File::create(options.pc_file).unwrap();
    serde_json::to_writer(file, &pc).unwrap();

    Ok(())
}
