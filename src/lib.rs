use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use anyhow::{anyhow, Context, Result};

pub mod api_usage;
pub mod cli;
pub mod user_setup;

mod program_info;
#[cfg(test)]
mod testing;
mod types;

pub mod constants {
    /// JSON file name for an API key.
    pub const API_JSON_NAME: &str = "api";

    /// JSON file name for user setting.
    pub const USER_SETTING_JSON_NAME: &str = "setting";

    /// ## Current weather data
    ///
    /// Access current weather data for any location on Earth!
    /// We collect and process weather data from different sources such as
    /// global and local weather models, satellites, radars and a vast network
    /// of weather stations. Data is available in JSON, XML, or HTML format.
    /// API Documentation: [https://openweathermap.org/api/current](https://openweathermap.org/api/current)
    ///
    /// - `{lat_value}`: Latitude value of the location.
    /// - `{lon_value}`: Longitude value of the location.
    /// - `{api_key}`: OpenWeatherMap API key.
    /// - `{unit}`: The desired measurement unit.
    ///   - ex. `standard`, `metric`, `imperial`
    ///   - MORE INFO: [https://openweathermap.org/weather-data](https://openweathermap.org/weather-data)
    ///
    /// ### Example Usage
    /// ```
    /// # use weather_cli::constants::WEATHER_API_URL;
    /// let url = WEATHER_API_URL
    ///     .replace("{LAT_VALUE}", "37.3361663")
    ///     .replace("{LON_VALUE}", "-121.890591")
    ///     .replace("{API_KEY}", "EXAMPLE_KEY")
    ///     .replace("{UNIT}", "imperial");
    ///
    /// assert_eq!(url, "https://api.openweathermap.org/data/2.5/weather?lat=37.3361663&lon=-121.890591&appid=EXAMPLE_KEY&units=imperial");
    /// ```
    pub const WEATHER_API_URL: &str = "https://api.openweathermap.org/data/2.5/weather?lat={LAT_VALUE}&lon={LON_VALUE}&appid={API_KEY}&units={UNIT}";

    /// ## Geocoding API
    ///
    /// Geocoding API is a simple tool that we have developed to ease
    /// the search for locations while working with geographic names and coordinates.
    /// API Documentation: [https://openweathermap.org/api/geocoding-api](https://openweathermap.org/api/geocoding-api)
    ///
    /// - `{lat_value}`: Latitude value of the location.
    /// - `{lon_value}`: Longitude value of the location.
    /// - `{api_key}`: OpenWeatherMap API key.
    /// - `{unit}`: The desired measurement unit.
    ///   - ex. `standard`, `metric`, `imperial`
    ///   - MORE INFO: [https://openweathermap.org/weather-data](https://openweathermap.org/weather-data)
    ///
    /// ### Example Usage
    /// ```
    /// # use weather_cli::constants::WEATHER_API_URL;
    /// let url = WEATHER_API_URL
    ///     .replace("{LAT_VALUE}", "37.3361663")
    ///     .replace("{LON_VALUE}", "-121.890591")
    ///     .replace("{API_KEY}", "EXAMPLE_KEY")
    ///     .replace("{UNIT}", "imperial");
    ///
    /// assert_eq!(url, "https://api.openweathermap.org/data/2.5/weather?lat=37.3361663&lon=-121.890591&appid=EXAMPLE_KEY&units=imperial");
    /// ```
    pub const GEOLOCATION_API_URL: &str =
        "http://api.openweathermap.org/geo/1.0/direct?q={QUERY}&limit=10&appid={API_KEY}";
}

/// Returns executable directory.
pub fn get_executable_directory() -> Result<String> {
    let executable_path =
        env::current_exe().context("Failed to get the executable file directory!")?;
    let executable_directory = executable_path
        .parent()
        .context("Failed to get the executable directory!")?;

    if let Some(dir_str) = executable_directory.to_str() {
        return Ok(dir_str.to_string());
    }

    Err(anyhow!("Unable to get the executable directory."))
}

/// Returns `std::fs::File` type value of a JSON file.
pub fn get_json_file(json_suffix: &str) -> Result<File> {
    let executable_dir = get_executable_directory()?;

    let file = match File::open(format!(
        "{}/{}",
        executable_dir,
        make_json_file_name(json_suffix)
    )) {
        Ok(f) => f,
        Err(_) => {
            let mut new_file = File::create(format!(
                "{}/{}",
                executable_dir,
                make_json_file_name(json_suffix)
            ))
            .context("Failed to create a json file.")?;
            new_file
                .write_all("{}".as_bytes())
                .context("Failed to create a json file.")?;

            File::open(format!(
                "{}/{}",
                executable_dir,
                make_json_file_name(json_suffix)
            ))
            .context("Failed to get the json file.")?
        }
    };

    Ok(file)
}

/// Complete a JSON file name.
/// ## Example
/// ```
/// # use weather_cli::make_json_file_name;
/// assert_eq!(make_json_file_name("api"), "weather-cli-api.json");
/// ```
pub fn make_json_file_name(suffix: &str) -> String {
    format!("weather-cli-{}.json", suffix)
}

pub enum ErrorMessageType {
    SettingRead,
    ApiResponseRead,
    InvalidApiKey,
}

fn get_file_read_error_message(error_type: ErrorMessageType, context: Option<&str>) -> String {
    match (error_type, context) {
        (ErrorMessageType::SettingRead, Some(context)) => {
            if context == "api" {
                format!(
                    "Failed to read {}. Please make sure to setup your API key.",
                    make_json_file_name(context)
                )
            } else {
                format!("Failed to read the following file: {}", context)
            }
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

/// Read a JSON file and return the string.
pub fn read_json_file<T: serde::de::DeserializeOwned>(json_name: &str) -> Result<T> {
    let mut file = get_json_file(json_name)?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;

    let api_key_data: T = serde_json::from_str(&json_string).context(
        get_file_read_error_message(ErrorMessageType::SettingRead, Some(json_name)),
    )?; // ERROR

    Ok(api_key_data)
}

/// Reads a JSON file and returns serialized data.
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

/// URL placeholder information.
///
/// ## Example Usage
/// ```no_run
/// # use weather_cli::URLPlaceholder;
/// URLPlaceholder {
///     placeholder: "{LAT_VALUE}".to_string(),
///     value: "37.3361663".to_string(),
/// };
/// ```
pub struct URLPlaceholder {
    pub placeholder: String,
    pub value: String,
}

/// Replaces URL placeholders with given values.
pub fn replace_url_placeholders(url: &str, url_placeholders: &[URLPlaceholder]) -> String {
    let mut replaced_url = String::from(url);
    for url_placeholder in url_placeholders {
        replaced_url = replaced_url.replace(&url_placeholder.placeholder, &url_placeholder.value);
    }

    replaced_url
}
