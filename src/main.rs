use clap::{CommandFactory, Parser};
use runix::{Cli, Commands};

fn main() {
   let args = Cli::parse();

   match &args.command {
    Some(Commands::Scan(arg)) => {
        println!("{:?}", arg.file);
    },
    None => {
        Cli::command().print_help().unwrap();
    }
   }
}
