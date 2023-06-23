use regex::Regex;
use serde_json::Value;
use std::{
    error::Error,
    fmt,
    fs::File,
    io::{self, Read, Write},
};
use weather_cli::{get_executable_directory, get_json_file};

const API_JSON_NAME: &str = "api";
const SETTINGS_JSON_NAME: &str = "settings";
const API_URL: &str = "https://api.openweathermap.org/data/2.5/weather?lat={lat_value}&lon={lon_value}&appid={api_key}&units={unit}";

pub async fn check() -> Result<(), Box<dyn Error>> {
    // Get the API key from "api.json".
    let mut api_json_file = get_json_file(API_JSON_NAME)?;
    let mut api_json_string = String::new();
    api_json_file.read_to_string(&mut api_json_string)?;
    let api_key_data: Value = serde_json::from_str(&api_json_string)?;
    let api_key = api_key_data["key"].as_str().unwrap();

    // Get properties from "setting.json".
    let mut setting_json_file = get_json_file(SETTINGS_JSON_NAME)?;
    let mut setting_json_string = String::new();
    setting_json_file.read_to_string(&mut setting_json_string)?;
    let setting_data: Value = serde_json::from_str(&setting_json_string)?;

    let city_name: Option<&str> = setting_data["city_name"].as_str();
    let lat: Option<f64> = setting_data["lat"].as_f64();
    let lon: Option<f64> = setting_data["lon"].as_f64();
    let preferred_unit: Option<i64> = setting_data["preferred_unit"].as_i64();

    match (city_name, lat, lon, preferred_unit) {
        (Some(city_name_value), Some(lat_value), Some(lon_value), Some(preferred_unit_value)) => {
            let url = match preferred_unit_value {
                1 => API_URL
                    .replace("{lat_value}", lat_value.to_string().as_str())
                    .replace("{lon_value}", lon_value.to_string().as_str())
                    .replace("{api_key}", api_key)
                    .replace("{unit}", "metric"),
                2 => API_URL
                    .replace("{lat_value}", lat_value.to_string().as_str())
                    .replace("{lon_value}", lon_value.to_string().as_str())
                    .replace("{api_key}", api_key)
                    .replace("{unit}", "imperial"),
                _ => unreachable!(),
            };

            let resp = reqwest::get(url).await?.text().await?;
            let data: Value = serde_json::from_str(&resp)?;

            let weather = (
                format!("{}", &data["weather"][0]["main"]).replace('"', ""),
                format!("{}", &data["weather"][0]["description"]).replace('"', ""),
            );
            let temp = format!("{}", &data["main"]["temp"]).replace('"', "");

            let unit_symbol = match preferred_unit_value {
                1 => "℃",
                2 => "℉",
                _ => unreachable!(),
            };

            println!("{}", city_name_value);
            println!("{}{} / {} ({})", temp, unit_symbol, weather.0, weather.1);
        }
        _ => {
            return Err("\"settings.json\" is not valid.".into());
        }
    }

    Ok(())
}

#[derive(Clone)]
struct City<'a> {
    name: &'a str,
    lat: f64,
    lon: f64,
    country: &'a str,
}

impl<'a> fmt::Display for City<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let output = format!(
            "{}, {} (lat: {}, lon: {})",
            self.name, self.country, self.lat, self.lon
        );
        write!(f, "{}", output)
    }
}

fn show_cities(city_vec: &[City]) {
    println!("\nCity list:");
    for (index, city) in city_vec.iter().enumerate() {
        println!("{}) {}", index + 1, city);
    }
}

fn city_select<'a>(city_vec: &'a [City]) -> Result<(&'a str, &'a str), Box<dyn Error>> {
    let mut selected_city: String = String::new();
    println!("\nPlease select your city.");
    io::stdin().read_line(&mut selected_city)?;
    let selected_city: usize = selected_city.trim().parse()?;

    if selected_city - 1 >= city_vec.len() {
        return Err("Invalid city index.".into());
    }

    let mut selected_unit: String = String::new();
    println!("\nDo you use Celcius or Fahrenheit?");
    println!("1) Celcius");
    println!("2) Fahrenheit");
    io::stdin().read_line(&mut selected_unit)?;
    let selected_unit: usize = selected_unit.trim().parse()?;

    if !(1..=2).contains(&selected_unit) {
        return Err("Invalid unit selection.".into());
    }

    let selected_unit_name = match selected_unit {
        1 => "Celcius",
        2 => "Fahrenheit",
        _ => unreachable!(),
    };

    let city = &city_vec[selected_city - 1];

    let mut file = get_json_file(SETTINGS_JSON_NAME)?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;

    let mut data: Value = serde_json::from_str(&json_string)?;

    data["city_name"] = city.name.into();
    data["lat"] = city.lat.into();
    data["lon"] = city.lon.into();
    data["preferred_unit"] = selected_unit.into();
    let json_string = &data.to_string();

    let executable_dir = get_executable_directory()?;
    File::create(format!("{}/{}.json", executable_dir, SETTINGS_JSON_NAME))
        .unwrap()
        .write_all(json_string.as_bytes())?;

    Ok((city.name, selected_unit_name))
}

pub async fn search_city(query: &String) -> Result<(), Box<dyn Error>> {
    let mut api_json_file = get_json_file(API_JSON_NAME)?;
    let mut api_json_string = String::new();
    api_json_file.read_to_string(&mut api_json_string)?;
    let api_key_data: Value = serde_json::from_str(&api_json_string)?;
    let api_key = api_key_data["key"].as_str().unwrap();

    let url =
        format!("http://api.openweathermap.org/geo/1.0/direct?q={query}&limit=10&appid={api_key}");
    let resp = reqwest::get(url).await?.text().await?;
    let data: Value = serde_json::from_str(&resp).unwrap();

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

pub fn api_setup(key: String) -> Result<(), Box<dyn Error>> {
    let executable_dir = get_executable_directory()?;

    let regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();

    if (key.len() < 32 && key.len() > 32) || !regex.is_match(&key) {
        println!("Please enter a valid key!");
    } else {
        let mut api_json_file = get_json_file(API_JSON_NAME)?;
        let mut api_json_string = String::new();
        api_json_file.read_to_string(&mut api_json_string)?;
        let mut api_json_data: Value = serde_json::from_str(&api_json_string)?;

        api_json_data["key"] = key.into();

        let api_json_string = &api_json_data.to_string();

        File::create(format!("{}/{}.json", executable_dir, API_JSON_NAME))
            .unwrap()
            .write_all(api_json_string.as_bytes())?;

        println!("Successfully updated your key!");
    }

    Ok(())
}
