use serde::Deserialize;

use std::path::Path;
use std::{error::Error, fs};

mod changelog;
mod control;

use changelog::Changelog;
use control::Control;

#[derive(Deserialize, Debug)]
struct Maintainer {
    name: String,
    email: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    #[serde(default = "Changelog::default")]
    changelog: Changelog,
    #[serde(default = "Control::default")]
    control: Control,
}

const CONFIG_FILE: &str = ".debyrc";

impl Config {
    pub(crate) fn new() -> Result<Self, Box<dyn Error>> {
        let config = Self::parse()?;
        Ok(Self {
            changelog: config.changelog,
            control: config.control,
        })
    }

    fn parse() -> Result<Config, Box<dyn Error>> {
        let config_data = match fs::read_to_string(CONFIG_FILE) {
            Ok(data) => data,
            Err(_) => {
                panic!(
                    "No config file provided. Create {} file in your project root.",
                    CONFIG_FILE
                )
            }
        };
        let config: Config = serde_json::from_str(&config_data)?;
        Ok(config)
    }

    pub(crate) fn update(
        &self,
        version: &str,
        changes: &str,
        user_defined_fields: &str,
    ) -> Result<(), Box<dyn Error>> {
        if !Path::new("debian").exists() {
            fs::create_dir("debian")?;
        }

        match Changelog::update(&self, &version, &changes) {
            Ok(msg) => println!("{}", msg),
            Err(e) => panic!("{}", e),
        };

        match Control::update(&self, &version, &user_defined_fields) {
            Ok(msg) => println!("{}", msg),
            Err(e) => panic!("{}", e),
        };

        Ok(())
    }
}
