use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct OptionsEncode {
    #[arg(long)]
    pub to_encode: PathBuf,
}

#[derive(Parser, Debug, Clone)]
pub struct OptionsLiteDecode {
    #[arg(long)]
    pub python_script_path: PathBuf,

    #[arg(long)]
    pub pc_screenshots: PathBuf,

    #[arg(long)]
    pub decode_to: PathBuf,
}

#[derive(Parser, Debug, Clone)]
pub enum LiteCommand {
    Encode(OptionsEncode),
    Decode(OptionsLiteDecode),
}

#[derive(Parser, Debug, Clone)]
pub struct OptionsFullDecode {
    #[arg(long)]
    pub decode_to: PathBuf,
}

#[derive(Parser, Debug, Clone)]
pub enum FullCommand {
    Encode(OptionsEncode),
    Decode(OptionsFullDecode),
}

#[derive(Parser, Debug, Clone)]
pub enum Mode {
    Lite {
        #[command(subcommand)]
        command: LiteCommand,
    },
    Full {
        #[arg(long)]
        pk_hex_mon_fs: Option<PathBuf>,

        #[arg(long)]
        save_path: PathBuf,

        #[command(subcommand)]
        command: FullCommand,
    },
}

#[derive(Parser, Debug, Clone)]
pub struct Options {
    #[arg(short, long, default_value = "pc.json")]
    pub pc_file: PathBuf,

    #[command(subcommand)]
    pub mode: Mode,
}
