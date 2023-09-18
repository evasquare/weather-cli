use crate::{
    constants::{API_JSON_NAME, API_URL, SETTINGS_JSON_NAME},
    read_json_file,
    user_setup::{update_setting, ApiSetting, City, Unit, UserSetting},
};
use anyhow::{anyhow, Context, Result};

mod response_types;

/// Checks the weather information from the API.
pub async fn check() -> Result<()> {
    use crate::api_usage::response_types::WeatherApiResponse;
    use crate::get_emoji;

    // Read API Key
    let api_json_string = read_json_file(API_JSON_NAME).context(format!(
        "Failed to read the following file: {}",
        API_JSON_NAME
    ))?;
    let api_key_data: ApiSetting = serde_json::from_str(&api_json_string).context(format!(
        "Failed to read the following file: {}",
        API_JSON_NAME
    ))?;
    let api_key = api_key_data.key;

    // Get properties from the setting file.
    let setting_json_string = read_json_file(SETTINGS_JSON_NAME)?;
    let setting_json_data: UserSetting = serde_json::from_str(&setting_json_string).context(
        format!("Failed to read the following file: {}", SETTINGS_JSON_NAME),
    )?;

    let url = match (&setting_json_data.city, &setting_json_data.unit) {
        (Some(city), Some(Unit::Metric)) => API_URL
            .replace("{lat_value}", city.lat.to_string().as_str())
            .replace("{lon_value}", city.lon.to_string().as_str())
            .replace("{api_key}", api_key)
            .replace("{unit}", "metric"),
        (Some(city), Some(Unit::Imperial)) => API_URL
            .replace("{lat_value}", city.lat.to_string().as_str())
            .replace("{lon_value}", city.lon.to_string().as_str())
            .replace("{api_key}", api_key)
            .replace("{unit}", "imperial"),
        _ => return Err(anyhow!("Failed to read the setting! Please run 'set-location' command to set your city and preferred unit.")),
    };

    let resp = reqwest::get(url).await?.text().await?;
    let response_data: WeatherApiResponse = serde_json::from_str(&resp)
        .context("The given 'WeatherApiResponse' JSON input may be invalid.")?;

    let (main, description, icon_id) = (
        response_data.weather[0].main.replace('"', ""),
        response_data.weather[0].description.replace('"', ""),
        response_data.weather[0].icon.replace('"', ""),
    );

    let temp = format!("{}", response_data.main.temp).replace('"', "");
    let unit_symbol = match setting_json_data.unit {
        Some(Unit::Metric) => "℃",
        Some(Unit::Imperial) => "℉",
        _ => unreachable!(),
    };

    let emoji = match setting_json_data.display_emoji {
        Some(true) => get_emoji(icon_id.as_str()),
        Some(false) => String::new(),
        _ => unreachable!(),
    };

    if let Some(city) = setting_json_data.city {
        println!("{} ({})", city.name, city.country);
        println!(
            "{}{} / {}{} ({})",
            temp, unit_symbol, emoji, main, description
        );
        Ok(())
    } else {
        Err(anyhow!("Couldn't find city!"))
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
fn city_select(city_vec: &[City]) -> Result<(&str, Unit)> {
    use std::io;

    // Select city
    let mut selected_city: String = String::new();
    println!("\nPlease select your city.");

    io::stdin().read_line(&mut selected_city)?;
    let selected_city: usize = selected_city.trim().parse()?;
    if selected_city > city_vec.len() {
        return Err(anyhow!("Invalid city index."));
    }
    let city = &city_vec[selected_city - 1];

    // Select unit
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
    let selected_unit = match selected_unit {
        1 => Unit::Metric,
        2 => Unit::Imperial,
        _ => return Err(anyhow!("Input out of range!")),
    };

    // Select emoji option
    let mut emoji_option: String = String::new();
    println!("\nDo you want to display emoji? (y/n)");

    io::stdin().read_line(&mut emoji_option)?;
    let emoji_option: &str = emoji_option.trim();
    if emoji_option != "y" && emoji_option != "Y" && emoji_option != "n" && emoji_option != "N" {
        return Err(anyhow!("Invalid selection."));
    }
    let emoji_option = match emoji_option {
        "y" | "Y" => true,
        "n" | "N" => false,
        _ => return Err(anyhow!("Invalid selection.")),
    };

    // Update the setting.
    let user_setting = UserSetting {
        city: Some(City {
            name: city.name.clone(),
            lat: city.lat,
            lon: city.lon,
            country: city.country.clone(),
        }),
        unit: Some(selected_unit.clone()),
        display_emoji: Some(emoji_option),
    };
    update_setting(&user_setting)?;
    Ok((city.name.as_str(), selected_unit))
}

/// Retrieves cities with the search query and saves the selected city.
pub async fn search_city(query: &String) -> Result<()> {
    use serde_json::Value;

    let api_json_string = read_json_file(API_JSON_NAME)?;
    let api_key_data: ApiSetting = serde_json::from_str(&api_json_string)
        .context("The given 'ApiSetting' JSON input may be invalid.")?;

    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=10&appid={}",
        query, api_key_data.key
    );
    let response = reqwest::get(url).await?.text().await?;
    let data: Value =
        serde_json::from_str(&response).context("The given JSON input may be invalid.")?;

    let mut city_vec: Vec<City> = vec![];

    for city in data.as_array().unwrap() {
        city_vec.push(City {
            name: city["name"].as_str().unwrap().to_string(),
            lat: city["lat"].as_f64().unwrap(),
            lon: city["lon"].as_f64().unwrap(),
            country: city["country"].as_str().unwrap().to_string(),
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
