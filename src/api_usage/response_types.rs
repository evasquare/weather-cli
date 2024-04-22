// API Documentation:
// https://openweathermap.org/current

#[derive(serde::Deserialize)]
pub struct WeatherApiResponse {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: u32,
    pub wind: Wind,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub clouds: Clouds,
    pub dt: u32,
    pub sys: Sys,
    pub timezone: i32,
    pub cod: u32,
    ///  Please note that built-in geocoder functionality has been deprecated. (https://openweathermap.org/current#builtin)
    #[allow(dead_code)]
    id: u32,
    ///  Please note that built-in geocoder functionality has been deprecated. (https://openweathermap.org/current#builtin)
    #[allow(dead_code)]
    name: String,
}

#[derive(serde::Deserialize)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(serde::Deserialize)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(serde::Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: u32,
    pub humidity: u32,
    pub temp_min: f64,
    pub temp_max: f64,
    pub sea_level: Option<u32>,
    pub grnd_level: Option<u32>,
}

#[derive(serde::Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: u32,
    pub gust: Option<f64>,
}

#[derive(serde::Deserialize)]
pub struct Rain {
    #[serde(rename = "1h")]
    pub one_h: Option<f64>,
    #[serde(rename = "3h")]
    pub three_h: Option<f64>,
}

#[derive(serde::Deserialize)]
pub struct Snow {
    #[serde(rename = "1h")]
    pub one_h: Option<f64>,
    #[serde(rename = "3h")]
    pub three_h: Option<f64>,
}

#[derive(serde::Deserialize)]
pub struct Clouds {
    pub all: u32,
}

#[derive(serde::Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_: Option<u32>,
    pub id: Option<u32>,
    pub message: Option<String>,
    pub country: String,
    pub sunrise: u32,
    pub sunset: u32,
}
