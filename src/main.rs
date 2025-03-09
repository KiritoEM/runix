use clap::{CommandFactory, Parser};
use runix::{Cli, Commands};

fn main() {
   let args = Cli::parse();

   match &args.command {
    Some(Commands::Scan(arg)) => {
        println!("{:?}", arg.file);
    },
    Some(Commands::Prompt) => {
        println!("Running prompt !!!");
    },
    None => {
        Cli::command().print_help().unwrap();
    }
   }
}
