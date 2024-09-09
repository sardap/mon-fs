use std::fs::{self, File};

use mon_fs_box::pc::PC;
use structopt::StructOpt;

mod decode;
mod encode;
mod options;

#[derive(Debug)]
enum ProgramError {
    BadGuideFileGiven(String),
    BadPathGiven(String),
    BadModeGiven,
    DecoderFailure(String),
}

fn main() -> Result<(), ProgramError> {
    let options = options::Options::from_args();

    let mut pc = if options.pc_file.exists() {
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
            Ok(pc) => pc,
            Err(_) => {
                return Err(ProgramError::BadGuideFileGiven(format!(
                    "{}",
                    options.pc_file.display()
                )));
            }
        }
    } else {
        PC::new()
    };

    match options.command {
        options::Command::Encode(options_encode) => {
            if let Err(err) = encode::encode_file_to_pc(&mut pc, &options_encode) {
                return Err(err);
            }
        }
        options::Command::Decode(options_decode) => {
            println!("Parsing screenshots...");
            pc = match decode::load_pc_from_screenshots(&options_decode) {
                Ok(pc) => pc,
                Err(err) => {
                    return Err(err);
                }
            };

            if let Err(err) = decode::decode_pc_files(&pc, &options_decode) {
                return Err(err);
            }
        }
    }

    // Delete old guide file
    match fs::remove_file(&options.pc_file) {
        Ok(_) => {}
        Err(_) => {}
    }
    let file = File::create(options.pc_file).unwrap();

    serde_json::to_writer(file, &pc).unwrap();

    Ok(())
}
