use regex::Regex;
use serde_json::Value;
use std::{
    env,
    error::Error,
    fmt,
    fs::File,
    io::{self, Read, Write},
};

pub async fn check() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("API_KEY").unwrap();

    let mut file = File::open("settings.json")?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;
    let data: Value = serde_json::from_str(&json_string)?;

    let city_name: Option<&str> = data["city_name"].as_str();
    let lat: Option<f64> = data["lat"].as_f64();
    let lon: Option<f64> = data["lon"].as_f64();

    match (city_name, lat, lon) {
        (Some(city_name_value), Some(lat_value), Some(lon_value)) => {
            let url = format!(
                "https://api.openweathermap.org/data/2.5/weather?lat={lat_value}&lon={lon_value}&appid={api_key}&units=imperial"
            );
            let resp = reqwest::get(url).await?.text().await?;
            let data: Value = serde_json::from_str(&resp)?;

            let weather = (
                format!("{}", &data["weather"][0]["main"]).replace('"', ""),
                format!("{}", &data["weather"][0]["description"]).replace('"', ""),
            );

            let temp = format!("{}", &data["main"]["temp"]).replace('"', "");

            println!("{}", city_name_value);
            println!("{} / {} ({})", temp, weather.0, weather.1);
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
    println!("City list:");
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

    let mut file = match File::open("settings.json") {
        Ok(f) => f,
        Err(_) => {
            let mut new_file = File::create("settings.json")?;
            new_file.write_all("{}".as_bytes())?;

            File::open("settings.json")?
        }
    };

    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;

    let mut data: Value = serde_json::from_str(&json_string)?;

    data["city_name"] = city.name.into();
    data["lat"] = city.lat.into();
    data["lon"] = city.lon.into();
    data["preferred_unit"] = selected_unit.into();

    let json_string = &data.to_string();
    File::create("settings.json")
        .unwrap()
        .write_all(json_string.as_bytes())?;

    Ok((city.name, selected_unit_name))
}

pub async fn search_city(query: &String) -> Result<(), Box<dyn Error>> {
    let api_key = env::var("API_KEY").unwrap();
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
            println!("{} is now your city!", city_name);
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
    let regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();

    if (key.len() < 32 && key.len() > 32) || !regex.is_match(&key) {
        println!("Please enter a valid key!");
    } else {
        let data = format!("API_KEY={}", key);
        let mut env = File::create(".env")?;
        env.write_all(data.as_bytes())?;
        println!("Successfully updated your key!");
    }

    Ok(())
}
