#[cfg(test)]
mod unit_test {
    #[test]
    fn test_get_executable_directory() {
        use super::super::get_executable_directory;
        assert!(get_executable_directory().is_ok());
    }

    #[test]
    fn test_get_emoji() {
        use crate::get_emoji;

        struct TestCase<'a> {
            input: &'a str,
            output: String,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                input: "01d",
                output: "☀️ ".to_string(),
            },
            TestCase {
                input: "02d",
                output: "⛅️ ".to_string(),
            },
            TestCase {
                input: "03d",
                output: "☁️ ".to_string(),
            },
            TestCase {
                input: "04d",
                output: "☁️ ".to_string(),
            },
            TestCase {
                input: "09d",
                output: "🌧️ ".to_string(),
            },
            TestCase {
                input: "10d",
                output: "🌦️ ".to_string(),
            },
            TestCase {
                input: "11d",
                output: "⛈️ ".to_string(),
            },
            TestCase {
                input: "13d",
                output: "❄️ ".to_string(),
            },
            TestCase {
                input: "50d",
                output: "🌨️ ".to_string(),
            },
            TestCase {
                input: "01n",
                output: "🌑 ".to_string(),
            },
            TestCase {
                input: "02n",
                output: "🌑☁️ ".to_string(),
            },
            TestCase {
                input: "03n",
                output: "☁️ ".to_string(),
            },
            TestCase {
                input: "04n",
                output: "☁️☁️ ".to_string(),
            },
            TestCase {
                input: "09n",
                output: "🌧️ ".to_string(),
            },
            TestCase {
                input: "10n",
                output: "☔️ ".to_string(),
            },
            TestCase {
                input: "11n",
                output: "⛈️ ".to_string(),
            },
            TestCase {
                input: "13n",
                output: "❄️ ".to_string(),
            },
            TestCase {
                input: "random_string",
                output: "".to_string(),
            },
        ];

        for test_case in test_cases {
            assert_eq!(get_emoji(test_case.input), test_case.output);
        }
    }

    #[test]
    fn test_update_setting() {
        use crate::{
            constants::SETTINGS_JSON_NAME,
            read_json_file,
            user_setup::{
                structs::{City, Unit, UserSettings},
                update_user_settings,
            },
        };

        let option_setting_args = UserSettings {
            city: Some(City {
                name: String::from("London"),
                lat: 51.5074,
                lon: 0.1278,
                country: String::from("GB"),
            }),
            unit: Some(Unit::Imperial),
            display_emoji: Some(false),
        };

        println!("{:#?}", option_setting_args);

        let result = update_user_settings(&option_setting_args);
        println!("{:#?}", result);
        assert!(result.is_ok());

        // Get JSON data from an existing setting file.
        let json_data = read_json_file::<UserSettings>(SETTINGS_JSON_NAME).unwrap();

        assert_eq!(json_data.city.unwrap().name, String::from("London"));
        assert_eq!(json_data.unit.unwrap(), Unit::Imperial);
        assert!(!json_data.display_emoji.unwrap());
    }
}
