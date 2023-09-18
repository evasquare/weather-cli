# weather-cli
Use simple commands to check the weather in your city. Easily search your city and select it.


## Setup
You can install the crate with `cargo install` command.
```bash
cargo install weather-cli
```
Once installed, create an API key on [OpenWeather](https://openweathermap.org). You can register your key using the `weather-cli api-setup` command.

```bash
weather-cli api-setup --key "EXAMPLE_KEY"
```


## Commands

| command      | description                            |
| ------------ | -------------------------------------- |
| check        | Check weather information in your city |
| set-location | Search and set your city               |
| api-setup    | Setup the OpenWeather API Key          |
| about        | View information about the program     |
| help         | View the list of commands              |


## Use Examples

1. City Search
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

Do you want to display emoji? (y/n)
n

San Jose is now your city!
We'll use Imperial for you.
```

2. Weather Check

```
$ weather-cli check                          
San Jose (US)
58.62℉ / Clouds (broken clouds)
```