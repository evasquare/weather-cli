# weather-cli
Use simple commands to check the weather in your city. Easily search your city and select it.

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

Do you use Celcius or Fahrenheit?
1) Celcius
2) Fahrenheit
2

San Jose is now your city!
We'll use Fahrenheit for you.
```

2. Weather Check

```
$ weather-cli check                          
San Jose
57.69 / Clouds (overcast clouds)
```