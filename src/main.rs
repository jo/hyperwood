use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde_json::Value;

use hyperwood::Model;

fn reader_from_file_or_stdin(filename: &Option<PathBuf>) -> Box<dyn Read> {
    match &filename {
        Some(path) => Box::new(File::open(path).unwrap()),
        None => Box::new(io::stdin()),
    }
}

#[derive(Parser)]
#[clap(name = "hef")]
#[clap(
    about = "HEF CLI",
    long_about = "With the HEF CLI, you can generate BOMs (Bill of Material) for given HEF
    file, and other HEF related tasks."
)]

pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,

    /// HEF filename. If omitted, read STDIN
    #[clap(short, long)]
    filename: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print out a Parameters
    Parameters {},

    /// Print out a Lath Variant
    Variant {},

    /// Print out a Properties
    Properties {},

    /// Print out a BOM
    Bom {},

    /// Print out the requirements length of slat
    Requirements {},
}

pub struct Client {
    args: Args,
}

impl Client {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub fn run(&self) {
        let reader = reader_from_file_or_stdin(&self.args.filename);
        let model: Model<Value, Value> = Model::from_hef(reader);

        match &self.args.command {
            Commands::Parameters {} => {
                println!(
                    "{}",
                    serde_json::to_string(&model.parameters)
                        .expect("could not serialize parameters")
                );
            }

            Commands::Variant {} => {
                println!(
                    "{}",
                    serde_json::to_string(&model.variant).expect("could not serialize variant")
                );
            }

            Commands::Properties {} => {
                println!(
                    "{}",
                    serde_json::to_string(&model.properties)
                        .expect("could not serialize properties")
                );
            }

            Commands::Bom {} => {
                print!("{}", model.to_bom());
            }

            Commands::Requirements {} => {
                println!("{}", model.length_total());
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let client = Client::new(args);
    client.run();
}
