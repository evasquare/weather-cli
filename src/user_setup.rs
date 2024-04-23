use anyhow::Result;

use self::structs::UserSettings;

pub mod structs;

/// Saves an API key into the API setting file.
pub fn setup_api(key: String) -> Result<()> {
    use crate::constants::API_JSON_NAME;
    use crate::get_executable_directory;
    use crate::user_setup::structs::ApiSetting;
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

/// Updates the user setting file.
pub fn update_user_settings(setting_args: &UserSettings) -> Result<()> {
    use crate::constants::SETTINGS_JSON_NAME;
    use crate::user_setup::structs::City;
    use crate::{get_executable_directory, read_json_file};
    use anyhow::Context;

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

    let json_string = serde_json::to_string(&json_data)?;

    // Generate a new setting file.
    let executable_dir = get_executable_directory()?;
    File::create(format!(
        "{}/weather-cli-{}.json",
        executable_dir, SETTINGS_JSON_NAME
    ))?
    .write_all(json_string.as_bytes())
    .context(format!(
        "Failed to write a JSON file: {}",
        SETTINGS_JSON_NAME
    ))?;

    Ok(())
}
