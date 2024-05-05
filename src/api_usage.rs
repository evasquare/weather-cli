use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, FixedOffset, TimeZone, Utc};

use crate::{
    constants::GEOLOCATION_API_URL,
    types::user_settings::{City, Units, UserSetting},
    ErrorMessageType,
};

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

/// Returns a response from the given URL.
async fn get_response(url: String) -> Result<String> {
    let resp = reqwest::get(&url).await?;
    let text = resp.text().await?;
    Ok(text)
}

/// Prints weather information from the API.
pub async fn print_weather_information() -> Result<()> {
    use crate::{
        constants::{API_JSON_NAME, USER_SETTING_JSON_NAME, WEATHER_API_URL},
        read_json_file, read_json_response, replace_url_placeholders,
        types::{response_types::WeatherApiResponse, user_settings::ApiSetting},
        URLPlaceholder,
    };

    let api_json_data = read_json_file::<ApiSetting>(API_JSON_NAME)?;
    let setting_json_data = read_json_file::<UserSetting>(USER_SETTING_JSON_NAME)?;

    let url = match (&setting_json_data.city, &setting_json_data.units) {
        (Some(city), Some(unit)) => {
            let units = unit.to_string();

            replace_url_placeholders(
                WEATHER_API_URL,
                &[
                    URLPlaceholder {
                        placeholder: "{LAT_VALUE}".to_string(),
                        value: city.lat.to_string(),
                    },
                    URLPlaceholder {
                        placeholder: "{LON_VALUE}".to_string(),
                        value: city.lon.to_string(),
                    },
                    URLPlaceholder {
                        placeholder: "{API_KEY}".to_string(),
                        value: api_json_data.key,
                    },
                    URLPlaceholder {
                        placeholder: "{UNIT}".to_string(),
                        value: units,
                    },
                ],
            )
        }
        _ => {
            return Err(anyhow!(
            "Failed to read user setting! Please run 'set-location' command to configure settings."
        ))
        }
    };

    let response = get_response(url).await?;
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
    {
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

        let selected_city = setting_json_data
            .city
            .context("Failed to read city setting data.")?;
        let selected_unit = setting_json_data
            .units
            .context("Failed to read unit setting data.")?;
        let wind_unit = match selected_unit {
            Units::Standard => "m/s",
            Units::Metric => "m/s",
            Units::Imperial => "mph",
        };

        let output_messages = [
            String::new(),
            format!("{} ({})", selected_city.name, selected_city.country),
            format!(
                "{temp}° / {main} ({description})",
                temp = response_data.main.temp,
                main = response_data.weather[0].main,
                description = response_data.weather[0].description
            ),
            format!(
                "H: {max}°, L: {min}°",
                max = response_data.main.temp_max,
                min = response_data.main.temp_min
            ),
            format!(
                "\n- Wind Speed: {speed} {wind_speed_unit},",
                speed = response_data.wind.speed,
                wind_speed_unit = wind_unit
            ),
            format!(
                "- Humidity: {humidity} %,",
                humidity = response_data.main.humidity
            ),
            format!(
                "- Pressure: {pressure} hPa",
                pressure = response_data.main.pressure
            ),
            format!("- {}", upcoming_event.0),
            format!("  ({})", upcoming_event.1),
        ];

        for item in output_messages {
            println!("{}", item);
        }

        Ok(())
    }
}

/// Prints cities from a slice argument.
fn display_cities(city_slice: &[City]) {
    println!("\n* City list:");
    for (index, city) in city_slice.iter().enumerate() {
        println!("{}) {}", index + 1, city);
    }
}

/// Displays a prompt message and read user input.
fn read_user_input(messages: &[&str]) -> Result<String> {
    use std::io;

    for &message in messages {
        println!("{}", message);
    }

    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input)?;
    println!();

    if user_input.is_empty() {
        Err(anyhow!("Input is empty!"))
    } else {
        Ok(user_input)
    }
}

/// Saves user's city and unit preferences.
fn select_user_preferences(cities: &[City]) -> Result<(String, Units)> {
    use crate::user_setup::update_user_settings;

    let city: &City = {
        let user_input = read_user_input(&["Please select your city."])?;

        let parsed_input: usize = user_input.trim().parse()?;
        if parsed_input > cities.len() {
            return Err(anyhow!("Invalid city index."));
        }

        &cities[parsed_input - 1]
    };

    let units: Units = {
        let user_input = read_user_input(&[
            "* Select your preferred unit.",
            "* MORE INFO: https://openweathermap.org/weather-data",
            "1) Standard",
            "2) Metric",
            "3) Imperial",
        ])?;

        let parsed_input: usize = user_input
            .trim()
            .parse()
            .context("Failed to parse the input. Make sure it's a valid positive number.")?;

        match parsed_input {
            1 => Units::Standard,
            2 => Units::Metric,
            3 => Units::Imperial,
            _ => return Err(anyhow!("Input is out of range!")),
        }
    };

    let user_setting = UserSetting {
        city: Some(City {
            name: city.name.clone(),
            lat: city.lat,
            lon: city.lon,
            country: city.country.clone(),
        }),
        units: Some(units.clone()),
    };

    update_user_settings(&user_setting)?;

    Ok((city.name.clone(), units))
}

/// Selects a city from a list.
pub async fn search_city(query: &str) -> Result<()> {
    use serde_json::Value;

    use crate::{
        constants::API_JSON_NAME, get_file_read_error_message, read_json_file,
        replace_url_placeholders, types::user_settings::ApiSetting, URLPlaceholder,
    };

    if query.is_empty() {
        return Err(anyhow!("Query cannot be empty."));
    }

    let api_json_data = read_json_file::<ApiSetting>(API_JSON_NAME)?;

    let url = replace_url_placeholders(
        GEOLOCATION_API_URL,
        &[
            URLPlaceholder {
                placeholder: "{QUERY}".to_string(),
                value: query.to_string(),
            },
            URLPlaceholder {
                placeholder: "{API_KEY}".to_string(),
                value: api_json_data.key.clone(),
            },
        ],
    );
    let response = get_response(url).await?;
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
    display_cities(&cities);

    match select_user_preferences(&cities) {
        Ok((city_name, unit_name)) => {
            println!("{} is now your city!", city_name);
            println!("I'll use {} for you.", unit_name);
        }
        Err(e) => {
            println!("ERROR: {}", e);
        }
    };

    Ok(())
}
