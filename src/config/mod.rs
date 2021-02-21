use serde::Deserialize;

use std::fmt;
use std::fs;
use std::path::Path;

mod changelog;
mod control;

use changelog::Changelog;
use control::Control;

#[derive(Debug)]
pub enum DebyError {
    ConfigParse,
    ConfigNew,
    Update,
    ConfigRead,
    Deserialize,
    DebianDir,
    ChangelogUpdate,
    ChangelogOpen,
    ChangelogRead,
    ChangelogWrite,
    ControlUpdate,
    ControlOpen,
    ControlWrite,
}

impl fmt::Display for DebyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DebyError::ConfigNew => {
                write!(f, "Could not create configuration from config file .debyrc")
            }
            DebyError::ConfigParse => write!(f, "Could not parse config file .debyrc"),
            DebyError::ConfigRead => write!(f, "No config file .debyrc found"),

            DebyError::Update => write!(f, "Could not update your files"),
            DebyError::Deserialize => write!(f, "Could not deserialize .debyrc config file"),
            DebyError::DebianDir => write!(
                f,
                "No directory with `debian` name found in the root of the project"
            ),

            DebyError::ChangelogUpdate => write!(f, "Could not update debian changelog file"),
            DebyError::ChangelogOpen => write!(f, "Could not open debian changelog file"),
            DebyError::ChangelogRead => write!(f, "Could not read debian changelog file"),
            DebyError::ChangelogWrite => write!(f, "Could not update debian changelog file"),

            DebyError::ControlUpdate => write!(f, "Could not update debian control file"),
            DebyError::ControlOpen => write!(f, "Could not open debian control file"),
            DebyError::ControlWrite => write!(f, "Could not update debian control file"),
        }
    }
}

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
    pub(crate) fn new() -> Result<Self, DebyError> {
        let config = Self::parse().map_err(|_| DebyError::ConfigParse)?;

        Ok(Self {
            changelog: config.changelog,
            control: config.control,
        })
    }

    fn parse() -> Result<Config, DebyError> {
        let config_data = fs::read_to_string(CONFIG_FILE).map_err(|_| DebyError::ConfigRead)?;
        let config: Config =
            serde_json::from_str(&config_data).map_err(|_| DebyError::Deserialize)?;

        Ok(config)
    }

    pub(crate) fn update(
        &self,
        version: &str,
        changes: &str,
        user_defined_fields: Vec<&str>,
    ) -> Result<(&str, &str), DebyError> {
        if !Path::new("debian").exists() {
            fs::create_dir("debian").map_err(|_| DebyError::DebianDir)?;
        }

        let changelog_msg =
            Changelog::update(&self, &version, &changes).map_err(|_| DebyError::ChangelogUpdate)?;

        let control_msg =
            Control::update(&self, user_defined_fields).map_err(|_| DebyError::ControlUpdate)?;

        let msg = (changelog_msg, control_msg);

        Ok(msg)
    }
}
