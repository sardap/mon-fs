use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "encode-mon-fs")]
pub struct OptionsEncode {
    #[structopt(short, long, parse(from_os_str))]
    pub to_encode: PathBuf,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "decode-mon-fs")]
pub struct OptionsDecode {
    #[structopt(short, long, parse(from_os_str))]
    pub python_script_path: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    pub pc_screenshots: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    pub decode_to: PathBuf,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Encode(OptionsEncode),
    Decode(OptionsDecode),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "mon-fs")]
pub struct Options {
    #[structopt(short, long, parse(from_os_str), default_value = "pc.json")]
    pub pc_file: PathBuf,

    #[structopt(subcommand)]
    pub command: Command,
}
