use std::{env, fs};
use std::io::Read;
use toml;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub rdrives: Vec<String>,
    pub mountdirs: Vec<String>,
}


pub fn read_config() -> Result<Config, toml::de::Error> {
    let home_dir = env::var("HOME").expect("Failed to get HOME environment variable");
    let file_path = format!("{}/.config/rcloner/config.toml", home_dir);

    let mut file = fs::File::open(file_path).expect("Unable to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read config file");
    toml::from_str(&contents)
}
