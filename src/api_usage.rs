use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, FixedOffset, TimeZone, Utc};

use crate::{
    user_setup::structs::{City, Unit, UserSettings},
    ErrorMessageType,
};

mod response_types;

enum EventInfo<T: TimeZone> {
    Sunrise(DateTime<T>),
    Sunset(DateTime<T>),
}
impl<T: TimeZone> std::fmt::Display for EventInfo<T>
where
    T::Offset: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let local_time = match self {
            EventInfo::Sunrise(sunrise_time) => sunrise_time.format("Sunrise: %I:%M %p"),
            EventInfo::Sunset(sunset_time) => sunset_time.format("Sunset: %I:%M %p"),
        };
        write!(f, "{}", local_time)
    }
}

/// Returns sunrise and sunset time.
/// The first element in the returning array should be upcoming one.
/// ex) It shows sunset time before sunrise time in the afternoon.
fn convert_utc_to_local_time(
    sunrise_timestamp: i64,
    sunset_timestamp: i64,
    timezone: i32,
) -> Result<(EventInfo<FixedOffset>, EventInfo<FixedOffset>)> {
    let timezone = FixedOffset::east_opt(timezone).context("Failed to read timezone value.")?;

    let current_time = Utc::now().with_timezone(&timezone);
    let sunrise = DateTime::<Utc>::from_timestamp(sunrise_timestamp, 0)
        .context("Failed to read sunrise time.")?
        .with_timezone(&timezone);
    let sunset = DateTime::<Utc>::from_timestamp(sunset_timestamp, 0)
        .context("Failed to read sunset Time.")?
        .with_timezone(&timezone);

    // The first element should be the next upcoming event.
    if current_time < sunrise {
        Ok((EventInfo::Sunrise(sunrise), EventInfo::Sunset(sunset)))
    } else if current_time < sunset {
        Ok((EventInfo::Sunset(sunset), EventInfo::Sunrise(sunrise)))
    } else {
        Ok((EventInfo::Sunrise(sunrise), EventInfo::Sunset(sunset)))
    }
}

/// Gets the weather information from the API.
pub async fn get_weather_information() -> Result<()> {
    use crate::constants::{API_JSON_NAME, API_URL, SETTINGS_JSON_NAME};
    use crate::{
        api_usage::response_types::WeatherApiResponse, read_json_file, read_json_response,
        user_setup::structs::ApiSetting,
    };

    let api_json_data = read_json_file::<ApiSetting>(API_JSON_NAME);
    let api_key = api_json_data?.key;
    let setting_json_data = read_json_file::<UserSettings>(SETTINGS_JSON_NAME)?;

    let url = match (&setting_json_data.city, &setting_json_data.unit) {
        (Some(city), Some(unit)) => {
            let unit_str = unit.to_string();

            API_URL
                .replace("{lat_value}", &city.lat.to_string())
                .replace("{lon_value}", &city.lon.to_string())
                .replace("{api_key}", &api_key)
                .replace("{unit}", &unit_str)
        }
        _ => {
            return Err(anyhow!(
            "Failed to read the setting! Please run 'set-location' command to configure settings."
        ))
        }
    };

    let response = reqwest::get(url).await?.text().await?;
    let response_data = read_json_response::<WeatherApiResponse>(
        &response,
        ErrorMessageType::ApiResponseRead,
        "WeatherApiResponse",
    )?;

    let upcoming_event = convert_utc_to_local_time(
        response_data.sys.sunrise as i64,
        response_data.sys.sunset as i64,
        response_data.timezone,
    )?;

    // Print the weather information.
    if let (Some(city), Some(selected_unit)) = (setting_json_data.city, &setting_json_data.unit) {
        let wind_unit = match selected_unit {
            Unit::Metric => "m/s",
            Unit::Imperial => "mph",
        };

        /*
        Example Output:
        ```
        Toronto (CA)
        9.57° / Clouds (overcast clouds)
        H: 9.57°, L: 9.57°

        - Wind Speed: 4.59 m/s,
        - Humidity: 61 %,
        - Pressure: 1017 hPa
        - Sunrise: 06:22 AM
          (Sunset: 08:09 PM)
          ```
           */
        println!("{} ({})", city.name, city.country);

        println!(
            "{temp}° / {main} ({description})",
            temp = response_data.main.temp,
            main = response_data.weather[0].main,
            description = response_data.weather[0].description
        );

        println!(
            "H: {max}°, L: {min}°",
            max = response_data.main.temp_max,
            min = response_data.main.temp_min
        );
        println!(
            "\n- Wind Speed: {speed} {wind_speed_unit},",
            speed = response_data.wind.speed,
            wind_speed_unit = wind_unit
        );
        println!(
            "- Humidity: {humidity} %,",
            humidity = response_data.main.humidity
        );
        println!(
            "- Pressure: {pressure} hPa",
            pressure = response_data.main.pressure
        );
        println!("- {}", upcoming_event.0);
        println!("  ({})", upcoming_event.1);

        Ok(())
    } else {
        Err(anyhow!("There is no city matching the given query!"))
    }
}

/// Prints cities in the given slice.
fn show_cities(city_slice: &[City]) {
    println!("\nCity list:");

    for (index, city) in city_slice.iter().enumerate() {
        println!("{}) {}", index + 1, city);
    }
}

/// Prompts the user to select a city and their preferred unit.
fn city_select(cities: &[City]) -> Result<(&str, Unit)> {
    use crate::user_setup::update_user_settings;
    use std::io;

    // Select city
    let mut selected_city: String = String::new();
    println!();
    println!("Please select your city.");

    io::stdin().read_line(&mut selected_city)?;
    let selected_city: usize = selected_city.trim().parse()?;
    if selected_city > cities.len() {
        return Err(anyhow!("Invalid city index."));
    }
    let city = &cities[selected_city - 1];

    // Select unit
    let mut selected_unit: String = String::new();
    println!();
    println!("Do you use Celsius or Fahrenheit?");
    println!("1) Celsius");
    println!("2) Fahrenheit");

    io::stdin().read_line(&mut selected_unit)?;
    let selected_unit: usize = selected_unit
        .trim()
        .parse()
        .context("Failed to parse the input. The input may be invalid usize value. Please make sure it's at least 0.")?;
    if !(1..=2).contains(&selected_unit) {
        return Err(anyhow!("Invalid unit selection."));
    }
    let selected_unit = match selected_unit {
        1 => Unit::Metric,
        2 => Unit::Imperial,
        _ => return Err(anyhow!("Input out of range!")),
    };

    let user_setting = UserSettings {
        city: Some(City {
            name: city.name.clone(),
            lat: city.lat,
            lon: city.lon,
            country: city.country.clone(),
        }),
        unit: Some(selected_unit.clone()),
    };
    update_user_settings(&user_setting)?;

    Ok((city.name.as_str(), selected_unit))
}

/// Retrieves cities with the search query and saves
/// the selected city in the setting file.
pub async fn search_city(query: &String) -> Result<()> {
    use crate::constants::API_JSON_NAME;
    use crate::get_file_read_error_message;
    use crate::read_json_file;
    use crate::user_setup::structs::ApiSetting;
    use serde_json::Value;

    let api_json_data = read_json_file::<ApiSetting>(API_JSON_NAME)?;

    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=10&appid={}",
        query, api_json_data.key
    );
    let response = reqwest::get(url).await?.text().await?;
    let data: Value =
        serde_json::from_str(&response).context("The given JSON input may be invalid.")?;

    // Invalid API key error.
    if let Some(401) = data["cod"].as_i64() {
        return Err(anyhow!(get_file_read_error_message(
            ErrorMessageType::InvalidApiKey,
            None
        )));
    }

    let mut cities: Vec<City> = vec![];

    for city in data.as_array().unwrap() {
        cities.push(City {
            name: city["name"].as_str().unwrap().to_string(),
            lat: city["lat"].as_f64().unwrap(),
            lon: city["lon"].as_f64().unwrap(),
            country: city["country"].as_str().unwrap().to_string(),
        });
    }
    show_cities(&cities);

    match city_select(&cities) {
        Ok((city_name, unit_name)) => {
            println!("\n{} is now your city!", city_name);
            println!("I'll use {} for you.", unit_name);
        }
        Err(e) => {
            println!("ERROR: {}", e);
            let error_msg = city_select(&cities);

            if let Err(e) = error_msg {
                println!("ERROR: {}. Exiting the program...", e);
            }
        }
    };

    Ok(())
}
