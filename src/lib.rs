use anyhow::{anyhow, Context, Result};

use std::{
    env,
    fs::File,
    io::{Read, Write},
};

pub mod api_usage;
pub mod cmd_line;
pub mod user_setup;

/// Constants for the information of the program.
pub mod program_info {
    /// The name of the program.
    pub const PROGRAM_NAME: &str = "weather-cli";
    /// The description of the program.
    pub const PROGRAM_DESCRIPTION: &str = "Weather for command-line fans!";
    /// The authors of the program.
    pub const PROGRAM_AUTHORS: &str = "decaplanet";
    /// URL of the program in crates.io.
    pub const CRATES_IO_URL: &str = "https://crates.io/crates/weather-cli";
    /// URL of the program repository in GitHub.
    pub const REPOSITORY_URL: &str = "https://github.com/decaplanet/weather-cli";
}

/// Constants related to the API and settings.
mod constants {
    /// The name of the json file for the API key.
    pub const API_JSON_NAME: &str = "api";

    /// The name of the json file for settings.
    pub const SETTINGS_JSON_NAME: &str = "settings";

    /// The URL template for the OpenWeatherMap API.
    ///
    /// This template can be used to retrieve weather data by replacing the following placeholders:
    ///
    /// - `{lat_value}`: Latitude value of the location.
    /// - `{lon_value}`: Longitude value of the location.
    /// - `{api_key}`: Your OpenWeatherMap API key.
    /// - `{unit}`: The desired measurement unit. (ex. `metric` or `imperial`)
    ///
    /// ## Example Usage
    ///
    /// ```
    /// pub const API_URL: &str = "https://api.openweathermap.org/data/2.5/weather?lat={lat_value}&lon={lon_value}&appid={api_key}&units={unit}";
    ///
    /// let url = API_URL
    ///     .replace("{lat_value}", "37.3361663")
    ///     .replace("{lon_value}", "-121.890591")
    ///     .replace("{api_key}", "EXAMPLE_KEY")
    ///     .replace("{unit}", "imperial");
    /// ```
    pub const API_URL: &str = "https://api.openweathermap.org/data/2.5/weather?lat={lat_value}&lon={lon_value}&appid={api_key}&units={unit}";
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

pub enum ErrorMessageType {
    SettingRead,
    ApiResponseRead,
    InvalidApiKey,
}

fn get_file_read_error_message(error_type: ErrorMessageType, context: Option<&str>) -> String {
    match (error_type, context) {
        (ErrorMessageType::SettingRead, Some(context)) => {
            format!("Failed to read the following file: {}", context)
        }
        (ErrorMessageType::ApiResponseRead, Some(context)) => {
            format!("The given '{}' JSON input may be invalid.", context)
        }
        (ErrorMessageType::InvalidApiKey, None) => {
            "API Key is invalid. Please try again.".to_string()
        }
        _ => unreachable!(),
    }
}

/// Reads the given json file and returns the string.
pub fn read_json_file<T: serde::de::DeserializeOwned>(json_name: &str) -> Result<T> {
    use constants::API_JSON_NAME;

    let mut file = get_json_file(json_name)?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;

    let api_key_data: T = serde_json::from_str(&json_string).context(
        get_file_read_error_message(ErrorMessageType::SettingRead, Some(API_JSON_NAME)),
    )?;

    Ok(api_key_data)
}

/// Reads the given json file and returns the string.
pub fn read_json_response<T: serde::de::DeserializeOwned>(
    response: &str,
    error_message_type: ErrorMessageType,
    error_context: &str,
) -> Result<T> {
    use serde_json::Value;

    let api_response: Value = serde_json::from_str(response).context(
        get_file_read_error_message(ErrorMessageType::ApiResponseRead, Some(error_context)),
    )?;

    // Invalid API key error.
    if let Some(401) = api_response["cod"].as_i64() {
        return Err(anyhow!(get_file_read_error_message(
            ErrorMessageType::InvalidApiKey,
            None
        )));
    }

    let response_data: T = serde_json::from_str(response).context(get_file_read_error_message(
        error_message_type,
        Some(error_context),
    ))?;

    Ok(response_data)
}

/// Returns the emoji for the given icon id.
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

/// This module is only used for testing.
#[cfg(test)]
mod tests;
