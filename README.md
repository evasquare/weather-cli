<img src="./images/icon.png" alt="project-icon" width="64">

# weather-cli
Minimalistic command-line weather program. It works with OpenWeather API.

## Setup
You can install the crate with `cargo install` command.

```bash
cargo install weather-cli
```

Once installed, create an API key on [OpenWeather](https://openweathermap.org). You need to register your key running the following command.

```bash
weather-cli api-setup --key "EXAMPLE_KEY"
```

## Commands
| command      | description                            |
| ------------ | -------------------------------------- |
| check        | Check weather information in your city |
| set-location | Search and set your city               |
| setup-api    | Setup the OpenWeather API Key          |
| about        | View information about the program     |
| help         | View the list of commands              |


## Use Examples

1. Search city
```
$ weather-cli set-location --query "San Jose"

City list:
1) San Jose, US (lat: 37.3361663, lon: -121.890591)
2) San José, CR (lat: 9.9325427, lon: -84.0795782)
3) San Jose, PH (lat: 12.0612933, lon: 121.9565754)
4) Sant Josep de sa Talaia, ES (lat: 38.9043608, lon: 1.3178098)
5) San Jose, US (lat: 40.305598, lon: -89.6028829)

Please select your city.
1

Do you use Celsius or Fahrenheit?
1) Celsius
2) Fahrenheit
2

San Jose is now your city!
I'll use imperial for you.
```

2. Check weather

```
$ weather-cli check                          
San Jose (US)
49.33° / Clear (clear sky)
H: 49.33°, L: 49.33°

- Wind Speed: 6.08 mph,
- Humidity: 92 %,
- Pressure: 1013 hPa
- Sunrise: 06:21 AM
  (Sunset: 07:49 PM)
```