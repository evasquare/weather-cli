use crate::{get_emoji, get_executable_directory, get_json_file};
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use serde_json::Value;
use std::{
    fmt,
    fs::File,
    io::{self, Read, Write},
};

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

use self::constants::{API_JSON_NAME, API_URL, SETTINGS_JSON_NAME};

/// Checks the weather information from the API.
pub async fn check() -> Result<()> {
    // Get the API key from "api.json".
    let mut api_json_file = get_json_file(API_JSON_NAME)?;

    let mut api_json_string = String::new();
    api_json_file.read_to_string(&mut api_json_string)?;
    let api_key_data: Value =
        serde_json::from_str(&api_json_string).context("The given JSON input may be invalid.")?;
    let api_key = api_key_data["key"]
        .as_str()
        .context("Couldn't get the API key! Make sure you set your key.")?;

    // Get properties from the setting file.
    let mut setting_json_file = get_json_file(SETTINGS_JSON_NAME)?;
    let mut setting_json_string = String::new();
    setting_json_file.read_to_string(&mut setting_json_string)?;
    let setting_data: Value = serde_json::from_str(&setting_json_string)
        .context("The given JSON input may be invalid.")?;

    let city_name = setting_data["city_name"]
        .as_str()
        .context("Couldn't get the city name. Make sure you set your city.")?;
    let lat = setting_data["lat"]
        .as_f64()
        .context("The \"lat\" option may be invalid f64 value.")?;
    let lon = setting_data["lon"]
        .as_f64()
        .context("The \"lon\" option may be invalid f64 value.")?;
    let preferred_unit = setting_data["preferred_unit"]
        .as_i64()
        .context("The \"preferred_unit\" option may be invalid i64 value.")?;

    let url = match preferred_unit {
        1 => API_URL
            .replace("{lat_value}", lat.to_string().as_str())
            .replace("{lon_value}", lon.to_string().as_str())
            .replace("{api_key}", api_key)
            .replace("{unit}", "metric"),
        2 => API_URL
            .replace("{lat_value}", lat.to_string().as_str())
            .replace("{lon_value}", lon.to_string().as_str())
            .replace("{api_key}", api_key)
            .replace("{unit}", "imperial"),
        _ => unreachable!(),
    };

    let resp = reqwest::get(url).await?.text().await?;
    let data: Value =
        serde_json::from_str(&resp).context("The given JSON input may be invalid.")?;

    let (main, description, icon_id) = (
        format!("{}", &data["weather"][0]["main"]).replace('"', ""),
        format!("{}", &data["weather"][0]["description"]).replace('"', ""),
        format!("{}", &data["weather"][0]["icon"]).replace('"', ""),
    );
    let temp = format!("{}", &data["main"]["temp"]).replace('"', "");
    let unit_symbol = match preferred_unit {
        1 => "℃",
        2 => "℉",
        _ => unreachable!(),
    };
    let emoji = get_emoji(icon_id.as_str());

    println!("{}", city_name);
    println!(
        "{}{} / {}{} ({})",
        temp, unit_symbol, emoji, main, description
    );

    Ok(())
}

/// Represents a city with its name, latitude, longitude, and country.
#[derive(Clone)]
struct City<'a> {
    /// The name of the city.
    name: &'a str,
    /// The latitude coordinate of the city.
    lat: f64,
    /// The longitude coordinate of the city.
    lon: f64,
    /// The country where the city is located.
    country: &'a str,
}

/// Formats properties for the City struct for printing.
///
/// ## Example Usage
///
/// ```
/// struct City<'a> {
///     name: &'a str,
///     lat: f64,
///     lon: f64,
///     country: &'a str,
/// };
///
/// let city = City {
///     name: "San Jose",
///     lat: 37.3361663,
///     lon: -121.890591,
///     country: "US",
/// };
/// ```
impl<'a> fmt::Display for City<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let output = format!(
            "{}, {} (lat: {}, lon: {})",
            self.name, self.country, self.lat, self.lon
        );
        write!(f, "{}", output)
    }
}

/// Prints each cities in the given slice.
fn show_cities(city_slice: &[City]) {
    println!("\nCity list:");
    for (index, city) in city_slice.iter().enumerate() {
        println!("{}) {}", index + 1, city);
    }
}

/// Prompts the user to select a city and preferred unit.
fn city_select<'a>(city_vec: &'a [City]) -> Result<(&'a str, &'a str)> {
    let mut selected_city: String = String::new();
    println!("\nPlease select your city.");
    io::stdin().read_line(&mut selected_city)?;
    let selected_city: usize = selected_city.trim().parse()?;

    if selected_city > city_vec.len() {
        return Err(anyhow!("Invalid city index."));
    }

    let mut selected_unit: String = String::new();
    println!("\nDo you use Celsius or Fahrenheit?");
    println!("1) Celsius");
    println!("2) Fahrenheit");
    io::stdin().read_line(&mut selected_unit)?;
    let selected_unit: usize = selected_unit
        .trim()
        .parse()
        .context("Couldn't parse the input. It may be invalid usize value.")?;

    if !(1..=2).contains(&selected_unit) {
        return Err(anyhow!("Invalid unit selection."));
    }

    let selected_unit_name = match selected_unit {
        1 => "Celsius",
        2 => "Fahrenheit",
        _ => return Err(anyhow!("Input out of range!")),
    };

    let city = &city_vec[selected_city - 1];

    let mut file = get_json_file(SETTINGS_JSON_NAME)?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;

    let mut data: Value =
        serde_json::from_str(&json_string).context("The given JSON input may be invalid.")?;

    data["city_name"] = city.name.into();
    data["lat"] = city.lat.into();
    data["lon"] = city.lon.into();
    data["preferred_unit"] = selected_unit.into();
    let json_string = &data.to_string();

    let executable_dir = get_executable_directory()?;
    File::create(format!(
        "{}/weather-cli-{}.json",
        executable_dir, SETTINGS_JSON_NAME
    ))
    .unwrap()
    .write_all(json_string.as_bytes())
    .context("Couldn't write JSON file.")?;

    Ok((city.name, selected_unit_name))
}

/// Retrieves cities with the search query and saves the selected city.
pub async fn search_city(query: &String) -> Result<()> {
    let mut api_json_file = get_json_file(API_JSON_NAME)?;
    let mut api_json_string = String::new();
    api_json_file.read_to_string(&mut api_json_string)?;
    let api_key_data: Value =
        serde_json::from_str(&api_json_string).context("The given JSON input may be invalid.")?;
    let api_key = api_key_data["key"]
        .as_str()
        .context("Couldn't get the API key! Make sure you set your key.")?;

    let url =
        format!("http://api.openweathermap.org/geo/1.0/direct?q={query}&limit=10&appid={api_key}");
    let resp = reqwest::get(url).await?.text().await?;
    let data: Value =
        serde_json::from_str(&resp).context("The given JSON input may be invalid.")?;

    let mut city_vec: Vec<City> = vec![];

    for city in data.as_array().unwrap() {
        city_vec.push(City {
            name: city["name"].as_str().unwrap(),
            lat: city["lat"].as_f64().unwrap(),
            lon: city["lon"].as_f64().unwrap(),
            country: city["country"].as_str().unwrap(),
        });
    }
    show_cities(&city_vec);

    match city_select(&city_vec) {
        Ok((city_name, unit_name)) => {
            println!("\n{} is now your city!", city_name);
            println!("We'll use {} for you.", unit_name);
        }
        Err(e) => {
            println!("ERROR: {}", e);
            let error_msg = city_select(&city_vec);

            if let Err(e) = error_msg {
                println!("ERROR: {}. Exit the program...", e);
            }
        }
    };

    Ok(())
}

/// Saves the given API key for later usage.
pub fn api_setup(key: String) -> Result<()> {
    let executable_dir = get_executable_directory()?;

    let regex = Regex::new(r"^[a-zA-Z0-9]+$")?;

    if key.len() != 32 || !regex.is_match(&key) {
        println!("Please enter a valid key!");
    } else {
        let mut api_json_file = get_json_file(API_JSON_NAME)?;
        let mut api_json_string = String::new();
        api_json_file.read_to_string(&mut api_json_string)?;
        let mut api_json_data: Value = serde_json::from_str(&api_json_string)
            .context("The given JSON input may be invalid.")?;

        api_json_data["key"] = key.into();

        let api_json_string = &api_json_data.to_string();

        File::create(format!(
            "{}/weather-cli-{}.json",
            executable_dir, API_JSON_NAME
        ))?
        .write_all(api_json_string.as_bytes())?;

        println!("Successfully updated your key!");
    }

    Ok(())
}
