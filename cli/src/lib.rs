use clap::{arg, Args, CommandFactory, Parser, Subcommand};

pub mod repl;

//exportation
pub use repl::*;

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

pub fn print_help() {
    Cli::command().print_help().unwrap();
}

pub fn new() -> Cli {
    Cli::parse()
}

pub fn run_cli(cli: Cli) {
    match &cli.command {
     Some(Commands::Scan(arg)) => {
         println!("{:?}", arg.file);
     },
     Some(Commands::Prompt) => {
         run_repl(|code| println!("{}", code));
     },
     None => {
         Cli::command().print_help().unwrap();
     }
    }
}