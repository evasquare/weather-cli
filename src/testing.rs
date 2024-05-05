#[cfg(test)]
mod unit_test {
    #[test]
    fn test_get_executable_directory() {
        use super::super::get_executable_directory;
        assert!(get_executable_directory().is_ok());
    }

    #[test]
    fn test_update_setting() {
        use crate::{
            constants::USER_SETTING_JSON_NAME,
            read_json_file,
            types::user_settings::{City, Units, UserSetting},
            user_setup::update_user_settings,
        };

        let option_setting_args = UserSetting {
            city: Some(City {
                name: String::from("London"),
                lat: 51.5074,
                lon: 0.1278,
                country: String::from("GB"),
            }),
            units: Some(Units::Imperial),
        };

        println!("{:#?}", option_setting_args);

        let result = update_user_settings(&option_setting_args);
        println!("{:#?}", result);
        assert!(result.is_ok());

        // Get JSON data from an existing setting file.
        let json_data = read_json_file::<UserSetting>(USER_SETTING_JSON_NAME).unwrap();

        assert_eq!(json_data.city.unwrap().name, String::from("London"));
        assert_eq!(json_data.units.unwrap(), Units::Imperial);
    }
}
