use anyhow::{Context, Result};

use core::fmt;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ApiSetting {
    pub key: String,
}

/// Saves an API key into the API setting file.
pub fn setup_api(key: String) -> Result<()> {
    use crate::constants::API_JSON_NAME;
    use crate::get_executable_directory;
    use regex::Regex;
    use std::{fs::File, io::Write};

    let executable_dir = get_executable_directory()?;
    let regex = Regex::new(r"^[a-zA-Z0-9]+$")?;

    // Api key validation.
    if key.len() != 32 || !regex.is_match(&key) {
        println!("Please enter a valid key!");
    } else {
        let new_api_setting = ApiSetting { key };

        let api_json_string = serde_json::to_string(&new_api_setting)?;
        File::create(format!(
            "{}/weather-cli-{}.json",
            executable_dir, API_JSON_NAME
        ))?
        .write_all(api_json_string.as_bytes())?;
        println!("Successfully updated your key!");
    }

    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct City {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let output = format!(
            "{}, {} (lat: {}, lon: {})",
            self.name, self.country, self.lat, self.lon
        );
        write!(f, "{}", output)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum Unit {
    Metric,
    Imperial,
}

impl fmt::Display for Unit {
    /// Returns the unit name.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Unit::Metric => "metric",
            Unit::Imperial => "imperial",
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UserSettings {
    pub city: Option<City>,
    pub unit: Option<Unit>,
    pub display_emoji: Option<bool>,
}

/// Updates the user setting file.
pub fn update_user_settings(setting_args: &UserSettings) -> Result<()> {
    use crate::constants::SETTINGS_JSON_NAME;
    use crate::{get_executable_directory, read_json_file};
    use std::{fs::File, io::Write};

    let mut json_data = read_json_file::<UserSettings>(SETTINGS_JSON_NAME)?;

    // 1. City
    {
        let mut using_city: Option<City> = None;
        if let Some(args_city) = &setting_args.city {
            using_city = Some(City {
                name: String::from(&args_city.name),
                lat: args_city.lat,
                lon: args_city.lon,
                country: String::from(&args_city.country),
            });
        }
        json_data.city = using_city;
    }
    // 2. Unit
    if let Some(unit) = &setting_args.unit {
        json_data.unit = Some(unit.clone());
    }
    // 3. Emoji
    if let Some(display_emoji) = &setting_args.display_emoji {
        json_data.display_emoji = Some(*display_emoji);
    }

    let json_string = serde_json::to_string(&json_data)?;

    // Generate a new setting file.
    let executable_dir = get_executable_directory()?;
    File::create(format!(
        "{}/weather-cli-{}.json",
        executable_dir, SETTINGS_JSON_NAME
    ))?
    .write_all(json_string.as_bytes())
    .context("Couldn't write the JSON file.")?;

    Ok(())
}
