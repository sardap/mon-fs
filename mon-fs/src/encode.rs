use mon_fs_box::file_pc::FilePc;

use crate::{options::OptionsEncode, ProgramError};

pub fn encode_file_to_file_pc(
    pc: &mut FilePc,
    options: &OptionsEncode,
) -> Result<(), ProgramError> {
    let encode_file = &options.to_encode;

    if !encode_file.exists() {
        return Err(ProgramError::BadPathGiven(format!(
            "{}",
            encode_file.display()
        )));
    }

    if encode_file.is_dir() {
        let files = std::fs::read_dir(encode_file).unwrap();

        for file in files {
            let file = file.unwrap();
            let file_path = file.path();

            if file_path.is_dir() {
                continue;
            }

            let filename_base = file.file_name().to_str().unwrap().to_string();

            if let Err(err) = pc.add_file(&filename_base, &file_path) {
                return Err(ProgramError::IoError(err));
            }
        }
    } else {
        let filename_base = encode_file.file_name().unwrap().to_str().unwrap();

        if let Err(err) = pc.add_file(filename_base, encode_file) {
            return Err(ProgramError::IoError(err));
        }
    }

    Ok(())
}
