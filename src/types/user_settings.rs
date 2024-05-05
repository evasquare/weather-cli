use core::fmt;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ApiSetting {
    pub key: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UserSetting {
    pub city: Option<City>,
    pub units: Option<Units>,
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
pub enum Units {
    Standard,
    Metric,
    Imperial,
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Units::Standard => "standard",
            Units::Metric => "metric",
            Units::Imperial => "imperial",
        })
    }
}
