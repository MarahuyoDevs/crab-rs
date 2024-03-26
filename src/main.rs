use clap::{Parser, Subcommand};
use serde;
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::collections::{HashMap, LinkedList};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

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
    Run {
        command: String,
        #[arg(short, long, value_name = "FILE")]
        file: Option<String>,
    },
    Init {},
}

#[derive(Serialize, Deserialize, Debug)]
struct YAMLConfig {
    name: String,
    commands: HashMap<String, YAMLCommand>,
}
#[derive(Serialize, Deserialize, Debug)]
struct YAMLCommand {
    name: String,
    steps: LinkedList<YAMLStep>,
}

#[derive(Serialize, Deserialize, Debug)]
struct YAMLStep {
    name: String,
    run: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SampleYaml {
    name: String,
}

fn run(command: &String, file: &Option<String>) -> () {
    let yaml = match _validate_yaml(file) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    yaml.commands.iter().for_each(|(key, value)| {
        if key == command {
            value.steps.iter().for_each(|step| {
                println!("Running step: {}", step.name);
                sleep(Duration::from_secs(1));
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&step.run)
                    .output();

                if output.unwrap().status.success() {
                    println!("success");
                } else {
                    panic!("Failed to run command: {}", &step.run);
                }
            });
        }
    });
}

fn init() {
    todo!("Create a initializer to create a new project")
}

fn _open_file(filename: &String) -> Result<String, String> {
    let file = File::open(filename);
    match file {
        Ok(_) => {}
        Err(_) => {
            return Err("File not found".to_string());
        }
    }
    let mut content = String::new();
    let _ = file.unwrap().read_to_string(&mut content);
    Ok(content)
}

fn _validate_yaml(path: &Option<String>) -> Result<YAMLConfig, String> {
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();

    let path = match path {
        Some(p) => p.clone(),
        None => Some("crab-config.yaml").unwrap().to_string(),
    };

    let file = _open_file(&format!("{}/{}", current_dir, path).to_string());
    match file {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }
    let yaml_content: YAMLConfig = from_str(&file.unwrap().to_string()).unwrap();

    Ok(yaml_content)
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Run { file, command }) => {
            run(command, file);
        }
        Some(Commands::Init {}) => {
            init();
        }
        None => {
            println!("No command provided");
        }
    }
}
