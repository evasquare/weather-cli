use clap::Parser;

use crate::{
    api_usage::{check, search_city},
    get_executable_directory,
    user_setup::setup_api,
};

const ABOUT: &str = "# weather-cli : Weather for command-line fans!";

#[derive(clap::Parser)]
#[command(author, version, about = ABOUT, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Check weather information in your city
    Check {},

    /// Search and set your city
    SetLocation {
        /// A search query.
        #[arg(short, long)]
        query: String,
    },

    /// Setup the OpenWeather API Key
    /// (https://openweathermap.org)
    SetupApi {
        /// API key from OpenWeather.
        #[arg(short, long)]
        key: String,
    },

    /// View information about the program
    About {},
}

/// Initialize the command line interface.
pub async fn init() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Check {}) => {
            match check().await {
                Ok(()) => {}
                Err(e) => {
                    println!("ERROR: {}", e);
                }
            };
        }
        Some(Commands::SetLocation { query }) => {
            search_city(query).await.unwrap_or_else(|e| {
                println!("ERROR: {}", e);
            });
        }
        Some(Commands::SetupApi { key }) => {
            setup_api(key.to_string()).unwrap_or_else(|e| {
                println!("ERROR: {}", e);
            });
        }
        Some(Commands::About {}) => {
            use crate::program_info::{
                CRATES_IO_URL, PROGRAM_AUTHORS, PROGRAM_DESCRIPTION, PROGRAM_NAME, REPOSITORY_URL,
            };

            let splitted_author_list: Vec<&str> = PROGRAM_AUTHORS.split(',').collect();

            let mut authors = String::new();
            for (index, one) in splitted_author_list.into_iter().enumerate() {
                if index == 0 {
                    authors += one.trim();
                } else {
                    authors = authors + ", " + one.trim();
                }
            }

            println!("# {}", PROGRAM_NAME);
            println!("{}\n", PROGRAM_DESCRIPTION);
            println!("Developed by: {}", authors);
            println!("- Crates.io: {}", CRATES_IO_URL);
            println!("- Github: {}", REPOSITORY_URL);
        }
        None => {
            println!("Please use \"weather-cli help\" command for help.");

            let executable_directory = get_executable_directory().unwrap();
            println!("- Program Executable Directory: {}", executable_directory);
        }
    }
}
