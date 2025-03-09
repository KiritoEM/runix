use clap::{arg, Args,  Parser, Subcommand};

#[derive(Parser)]
#[command(author = "KiritoEM", version ="0.0.1", about="\nRunix is a simple compilater")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about="Scan a file")]
    Scan(Scan),

    #[command(about="Prompt your codes")]
    Prompt
}

#[derive(Args, Debug)]
pub struct Scan {
    #[arg(short='f', long="file", help="The path of file to scan")]
    pub file: std::path::PathBuf
}
