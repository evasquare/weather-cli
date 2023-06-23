use std::{env, error::Error, fs::File, io::Write};

pub fn get_executable_directory() -> Result<String, Box<dyn Error>> {
    let executable_path = env::current_exe()?;
    let executable_directory = executable_path.parent().unwrap();

    if let Some(dir_str) = executable_directory.to_str() {
        return Ok(dir_str.to_string());
    }

    Err("Unable to get the executable directory.".into())
}

pub fn get_json_file(name: &str) -> Result<File, Box<dyn Error>> {
    let executable_dir = get_executable_directory()?;

    let file = match File::open(format!("{}/{}.json", executable_dir, name)) {
        Ok(f) => f,
        Err(_) => {
            let mut new_file = File::create(format!("{}/{}.json", executable_dir, name)).unwrap();
            new_file.write_all("{}".as_bytes()).unwrap();

            File::open(format!("{}/{}.json", executable_dir, name)).unwrap()
        }
    };

    Ok(file)
}
