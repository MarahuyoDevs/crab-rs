use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLI {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    Run {},
    Init {},
}

fn run() -> () {
    todo!("Create a run function to execute commands")
}

fn init() {
    todo!("Create a initializer to create a new project")
}

fn _validate_yaml() -> Result<(), String> {
    todo!("Validate the yaml file")
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Run {}) => {
            run();
        }
        Some(Commands::Init {}) => {
            init();
        }
        None => {
            println!("No command provided");
        }
    }
}
