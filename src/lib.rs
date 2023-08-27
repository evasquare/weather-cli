use anyhow::{anyhow, Context, Result};
use std::{env, fs::File, io::Write};
pub mod cmd_line;
pub mod weather;

/// Constants for the information of the program.
pub mod program_info {
    /// The name of the program.
    pub const PROGRAM_NAME: &str = "weather-cli";
    /// The description of the program.
    pub const PROGRAM_DESCRIPTION: &str = "Weather for command-line fans!";
    /// The authors of the program.
    pub const PROGRAM_AUTHORS: &str = "decaplanet";
}

/// Returns the running executable directory.
pub fn get_executable_directory() -> Result<String> {
    let executable_path =
        env::current_exe().context("Couldn't get the executable file directory!")?;
    let executable_directory = executable_path
        .parent()
        .context("Couldn't get the executable directory!")?;

    if let Some(dir_str) = executable_directory.to_str() {
        return Ok(dir_str.to_string());
    }

    Err(anyhow!("Unable to get the executable directory."))
}

/// Formats the given file name with the executable directory.
pub fn get_json_file(name: &str) -> Result<File> {
    let executable_dir = get_executable_directory()?;

    let file = match File::open(format!("{}/weather-cli-{}.json", executable_dir, name)) {
        Ok(f) => f,
        Err(_) => {
            let mut new_file =
                File::create(format!("{}/weather-cli-{}.json", executable_dir, name))
                    .context("Couldn't create a json file.")?;
            new_file
                .write_all("{}".as_bytes())
                .context("Couldn't create a json file.")?;

            File::open(format!("{}/weather-cli-{}.json", executable_dir, name))
                .context("Couldn't get the json file.")?
        }
    };

    Ok(file)
}

pub fn get_emoji(icon_id: &str) -> String {
    let return_value = match icon_id {
        "01d" => "☀️",
        "02d" => "⛅️",
        "03d" => "☁️",
        "04d" => "☁️",
        "09d" => "🌧️",
        "10d" => "🌦️",
        "11d" => "⛈️",
        "13d" => "❄️",
        "50d" => "🌨️",
        "01n" => "🌑",
        "02n" => "🌑☁️",
        "03n" => "☁️",
        "04n" => "☁️☁️",
        "09n" => "🌧️",
        "10n" => "☔️",
        "11n" => "⛈️",
        "13n" => "❄️",
        _ => "",
    };

    if !return_value.is_empty() {
        format!("{} ", return_value)
    } else {
        return_value.to_string()
    }
}
