use std::fs::{self};

use mon_fs_box::pc::{PCWriter, PC};

use crate::{
    options::OptionsEncode,
    ProgramError,
};

pub fn encode_file_to_pc(pc: &mut PC, options: &OptionsEncode) -> Result<(), ProgramError> {
    let encode_file = &options.to_encode;

    if !encode_file.exists() {
        return Err(ProgramError::BadPathGiven(format!(
            "{}",
            encode_file.display()
        )));
    }

    let to_encode = match fs::read(&encode_file) {
        Ok(data) => data,
        Err(_) => {
            return Err(ProgramError::BadPathGiven(format!(
                "{}",
                encode_file.display()
            )));
        }
    };

    let filename_base = encode_file.file_name().unwrap().to_str().unwrap();

    let mut writer = PCWriter::new(pc);
    writer.write_file(filename_base, &to_encode).unwrap();

    Ok(())
}
