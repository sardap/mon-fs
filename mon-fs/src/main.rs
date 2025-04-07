use clap::Parser;
use decode::load_guide_from_save;
use mon_fs_box::{
    box_mon_full::BoxMonFull,
    box_mon_lite::BoxMonLite,
    file_pc::FilePc,
    pc::{PcFull, PcLite},
};
use options::Options;
use std::{
    fs::{self, File},
    path::PathBuf,
    process::Command,
};
use which::which;

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
    #[allow(dead_code)]
    PKHexMonFSOutputError,
}

fn run_lite(pc_file_path: PathBuf, lite_command: options::LiteCommand) -> Result<(), ProgramError> {
    let mut file_pc = if pc_file_path.exists() {
        let existing = match fs::read(&pc_file_path) {
            Ok(data) => data,
            Err(_) => {
                return Err(ProgramError::BadGuideFileGiven(format!(
                    "{}",
                    pc_file_path.display()
                )));
            }
        };

        // Decode to PcLite
        match serde_json::from_slice::<PcLite>(&existing) {
            Ok(pc) => pc.into(),
            Err(_) => {
                return Err(ProgramError::BadGuideFileGiven(format!(
                    "{}",
                    pc_file_path.display()
                )));
            }
        }
    } else {
        FilePc::new()
    };

    match lite_command {
        options::LiteCommand::Encode(options_encode) => {
            if let Err(err) = encode::encode_file_to_file_pc(&mut file_pc, &options_encode) {
                return Err(err);
            }
        }
        options::LiteCommand::Decode(options_decode) => {
            println!("Parsing screenshots...");
            let pc = match decode::load_pc_from_screenshots::<BoxMonLite>(&options_decode) {
                Ok(pc) => pc,
                Err(err) => {
                    return Err(err);
                }
            };

            let file_pc = pc.into();

            if let Err(err) = decode::decode_pc_files(&file_pc, &options_decode.decode_to) {
                return Err(err);
            }
        }
    }

    // Delete old guide file
    if pc_file_path.exists() {
        fs::remove_file(&pc_file_path).unwrap();
    }

    let pc: PcLite = match file_pc.as_pc() {
        Ok(pc) => pc,
        Err(err) => {
            return Err(ProgramError::IoError(err));
        }
    };

    let file = File::create(pc_file_path).unwrap();
    serde_json::to_writer(file, &pc).unwrap();

    Ok(())
}

fn run_full(
    full_command: options::FullCommand,
    pc_file_path: PathBuf,
    pk_hex_mon_fs: PathBuf,
    save_path: PathBuf,
) -> Result<(), ProgramError> {
    let mut file_pc = if pc_file_path.exists() {
        let existing = match fs::read(&pc_file_path) {
            Ok(data) => data,
            Err(_) => {
                return Err(ProgramError::BadGuideFileGiven(format!(
                    "{}",
                    pc_file_path.display()
                )));
            }
        };

        match serde_json::from_slice::<PcFull>(&existing) {
            Ok(pc) => pc.into(),
            Err(_) => {
                return Err(ProgramError::BadGuideFileGiven(format!(
                    "{}",
                    pc_file_path.display()
                )));
            }
        }
    } else {
        FilePc::new()
    };

    match full_command {
        options::FullCommand::Encode(options_encode) => {
            if let Err(err) = encode::encode_file_to_file_pc(&mut file_pc, &options_encode) {
                return Err(err);
            }

            // Delete old guide file
            if pc_file_path.exists() {
                fs::remove_file(&pc_file_path).unwrap();
            }

            let pc: PcFull = match file_pc.as_pc() {
                Ok(pc) => pc,
                Err(err) => {
                    return Err(ProgramError::IoError(err));
                }
            };

            let file = File::create(&pc_file_path).unwrap();
            serde_json::to_writer(file, &pc).unwrap();

            println!("Encoding files into the save...");

            let output = Command::new(pk_hex_mon_fs)
                .arg("encode")
                .arg(save_path)
                .arg(pc_file_path)
                .output();

            if let Err(err) = output {
                return Err(ProgramError::IoError(err));
            }

            println!("Encoded files into the save");
        }
        options::FullCommand::Decode(options_decode) => {
            let pc =
                match load_guide_from_save::<BoxMonFull>(&pk_hex_mon_fs, &save_path, &pc_file_path)
                {
                    Ok(pc) => pc,
                    Err(err) => {
                        return Err(err);
                    }
                };

            let file_pc = pc.into();

            if let Err(err) = decode::decode_pc_files(&file_pc, &options_decode.decode_to) {
                return Err(err);
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), ProgramError> {
    let options = Options::parse();

    let pc_file_path = options.pc_file.clone();

    match options.mode {
        options::Mode::Lite { command } => run_lite(pc_file_path, command),
        options::Mode::Full {
            command,
            pk_hex_mon_fs,
            save_path,
        } => {
            let pk_hex_mon_fs = match pk_hex_mon_fs {
                Some(path) => path,
                None => match which("PKHeX.CLI.MonFS") {
                    Ok(path) => path,
                    Err(_) => return Err(ProgramError::BadPathGiven(
                        "pk_hex_mon_fs could not be found in system $PATH please provide the path with --pk-hex-mon-fs"
                            .to_string(),
                    )),
                },
            };

            if !pk_hex_mon_fs.exists() {
                return Err(ProgramError::BadPathGiven(format!(
                    "{}",
                    pk_hex_mon_fs.display()
                )));
            }

            run_full(command, pc_file_path, pk_hex_mon_fs, save_path)
        }
    }
}
