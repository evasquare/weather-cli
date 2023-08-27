#[cfg(test)]
mod test_weather {
    use crate::weather::api_setup;

    #[test]
    fn test_api_setup() {
        // Please note that the API key is only for testing purposes
        // and is not valid for actual API access.
        let setup_result = api_setup(String::from("abcdefghijklmnopqrstuvwxyzabcdef"));
        assert!(setup_result.is_ok());
    }
}

#[cfg(test)]
mod test_lib {
    use super::super::{get_emoji, get_executable_directory};

    #[test]
    fn test_get_executable_directory() {
        assert!(get_executable_directory().is_ok());
    }

    #[test]
    fn test_get_emoji() {
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
}
