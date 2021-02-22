use serde::Deserialize;

use std::error::Error;
use std::fs;
use std::path::Path;

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
    pub(crate) fn new() -> Result<Self, std::io::Error> {
        let config = Self::parse()?;

        Ok(Self {
            changelog: config.changelog,
            control: config.control,
        })
    }

    fn parse() -> Result<Config, std::io::Error> {
        let config_data = fs::read_to_string(CONFIG_FILE)?;
        let config: Config = serde_json::from_str(&config_data)?;

        Ok(config)
    }

    pub(crate) fn update(
        &self,
        version: &str,
        changes: &str,
        user_defined_fields: Vec<&str>,
    ) -> Result<(&str, &str), Box<dyn Error>> {
        if !Path::new("debian").exists() {
            fs::create_dir("debian")?;
        }

        let changelog_msg = Changelog::update(&self, &version, &changes)?;
        let control_msg = Control::update(&self, user_defined_fields)?;
        let msg = (changelog_msg, control_msg);

        Ok(msg)
    }

    pub(crate) fn update_control(
        &self,
        user_defined_fields: Vec<&str>,
    ) -> Result<&str, Box<dyn Error>> {
        if !Path::new("debian").exists() {
            fs::create_dir("debian")?;
        }

        let msg = Control::update(&self, user_defined_fields)?;

        Ok(msg)
    }

    pub(crate) fn update_changelog(
        &self,
        version: &str,
        changes: &str,
    ) -> Result<&str, Box<dyn Error>> {
        if !Path::new("debian").exists() {
            fs::create_dir("debian")?;
        }

        let msg = Changelog::update(&self, &version, &changes)?;

        Ok(msg)
    }
}
