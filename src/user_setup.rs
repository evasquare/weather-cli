use anyhow::{anyhow, Context, Result};

use crate::{constants::API_JSON_NAME, make_json_file_name, types::user_settings::UserSetting};

/// Sets up an API key.
pub fn setup_api(api_key_input: String) -> Result<()> {
    use std::{fs::File, io::Write};

    use regex::Regex;

    use crate::{get_executable_directory, types::user_settings::ApiSetting};

    let executable_dir = get_executable_directory()?;
    let regex = Regex::new(r"^[a-zA-Z0-9]+$")?;

    if api_key_input.len() != 32 || !regex.is_match(&api_key_input) {
        return Err(anyhow!("Please enter a valid key!"));
    } else {
        let new_api_setting = ApiSetting { key: api_key_input };

        let api_json_string = serde_json::to_string(&new_api_setting)?;
        File::create(format!(
            "{}/{}",
            executable_dir,
            make_json_file_name(API_JSON_NAME)
        ))?
        .write_all(api_json_string.as_bytes())?;

        println!("Successfully updated your key data!");
    }

    Ok(())
}

/// Update user setting.
pub fn update_user_settings(setting_args: &UserSetting) -> Result<()> {
    use std::{fs::File, io::Write};

    use crate::{
        constants::USER_SETTING_JSON_NAME,
        types::user_settings::City,
        {get_executable_directory, read_json_file},
    };

    let mut json_data = read_json_file::<UserSetting>(USER_SETTING_JSON_NAME)?; // ERROR

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
    json_data.units = setting_args.units.clone();

    let json_string = serde_json::to_string(&json_data)?;

    // Generate a new setting file.
    let executable_dir = get_executable_directory()?;
    File::create(format!(
        "{}/{}",
        executable_dir,
        make_json_file_name(USER_SETTING_JSON_NAME)
    ))?
    .write_all(json_string.as_bytes())
    .context(format!(
        "Failed to write a JSON file: {}",
        USER_SETTING_JSON_NAME
    ))?;

    Ok(())
}
