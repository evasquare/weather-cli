use crate::weather::{api_setup, check, search_city};
use clap::{Parser, Subcommand};
use std::env;

const ABOUT: &str = "# weather-cli : Weather for command-line fans!";

#[derive(Parser)]
#[command(author, version, about = ABOUT, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check weather information in your city.
    Check {},

    /// Search and set your city.
    SetLocation {
        /// A search query.
        #[arg(short, long)]
        query: String,
    },

    /// Setup the OpenWeather API Key
    ApiSetup {
        /// API key from OpenWeather.
        #[arg(short, long)]
        key: String,
    },

    /// View information about the program.
    About {},
}

pub async fn init() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Check {}) => {
            match check().await {
                Ok(()) => {}
                Err(e) => {
                    println!("ERROR: {}", e);
                    if e.to_string().contains("No such file or directory") {
                        println!("ERROR: Try checking if \"settings.json\" exists. If not, you can create one with \"set-location\" command.");
                    }
                }
            };
        }
        Some(Commands::SetLocation { query }) => {
            search_city(query).await.unwrap();
        }
        Some(Commands::ApiSetup { key }) => {
            api_setup(key.to_string()).unwrap();
        }
        Some(Commands::About {}) => {
            let name = std::env::var("CARGO_PKG_NAME").unwrap();
            let description = std::env::var("CARGO_PKG_DESCRIPTION").unwrap();
            let version = std::env::var("CARGO_PKG_VERSION").unwrap();
            let authors = std::env::var("CARGO_PKG_AUTHORS").unwrap();

            let splited_author_list: Vec<&str> = authors.split(':').collect();

            let mut authors = String::new();
            for (index, one) in splited_author_list.into_iter().enumerate() {
                if index == 0 {
                    authors += one;
                } else {
                    authors = authors + ", " + one;
                }
            }

            println!("# {} ({}):", name, version);
            println!("{}\n", description);
            println!("Developed by: {}", authors);
        }
        None => {
            println!("Please use \"weather-cli help\" command for help.");

            if let Ok(program_dir) = env::current_dir() {
                if let Some(dir_str) = program_dir.to_str() {
                    println!("Program directory: {}", dir_str);
                }
            }
        }
    }
}
