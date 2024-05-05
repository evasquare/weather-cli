<img src="./images/icon.png" alt="project-icon" width="64">

# weather-cli
Minimalistic command-line weather program. It works with OpenWeather API.

## Setup
You can install the crate with `cargo install` command.

```bash
cargo install weather-cli
```

Once installed, create an API key on [OpenWeather](https://openweathermap.org).
You need to register your key running the following command.

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
$ weather-cli set-location --query "Toronto"

* City list:
1) Old Toronto, CA (lat: 43.6534817, lon: -79.3839347)
2) Toronto, CA (lat: 43.6534817, lon: -79.3839347)
3) Toronto, US (lat: 41.9048584, lon: -90.8640346)
4) Toronto, US (lat: 37.7989253, lon: -95.9491562)
5) Toronto, CA (lat: 46.4524682, lon: -63.3799629)
Please select your city.
2

* Select your preferred unit.
* MORE INFO: https://openweathermap.org/weather-data
1) Standard
2) Metric
3) Imperial
2

Toronto is now your city!
I'll use metric for you.
```

2. Check weather

```
$ weather-cli check

Toronto (CA)
11.34° / Mist (mist)
H: 13.06°, L: 9.89°

- Wind Speed: 3.6 m/s,
- Humidity: 93 %,
- Pressure: 1014 hPa
- Sunset: 08:24 PM
  (Sunrise: 06:03 AM)
```