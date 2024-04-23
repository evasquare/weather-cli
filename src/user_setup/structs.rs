use core::fmt;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UserSettings {
    pub city: Option<City>,
    pub unit: Option<Unit>,
    pub display_emoji: Option<bool>,
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
pub struct ApiSetting {
    pub key: String,
}
