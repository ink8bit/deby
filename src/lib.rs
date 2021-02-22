mod config;

use config::Config;
use std::fmt;

#[derive(Debug)]
pub enum DebyError {
    ConfigNew,
    Update,
    ChangelogUpdate,
    ControlUpdate,
}

impl fmt::Display for DebyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DebyError::ConfigNew => {
                write!(f, "Could not create configuration from config file .debyrc")
            }
            DebyError::Update => write!(f, "Could not update your files"),
            DebyError::ChangelogUpdate => write!(f, "Could not update debian changelog file"),
            DebyError::ControlUpdate => write!(f, "Could not update debian control file"),
        }
    }
}

/// Updates `changelog` and `control` files
///
/// ## Arguments
///
/// - `version` - an updated version string
/// - `changes` - changes to be included in your files
/// - `user_defined_fields` - additional dynamic fields to be included in `control` file
pub fn update(
    version: &str,
    changes: &str,
    user_defined_fields: Vec<&str>,
) -> Result<(String, String), DebyError> {
    let config = Config::new().map_err(|_| DebyError::ConfigNew)?;

    let msg = config
        .update(version, changes, user_defined_fields)
        .map_err(|_| DebyError::Update)?;

    let (changelog_msg, control_msg) = msg;

    Ok((changelog_msg.to_string(), control_msg.to_string()))
}

/// Updates debian control file
///
/// ## Arguments
///
/// - `user_defined_fields` - dynamic fields to be included in binary section of control file
pub fn update_control_file(user_defined_fields: Vec<&str>) -> Result<String, DebyError> {
    let config = Config::new().map_err(|_| DebyError::ConfigNew)?;

    let msg = config
        .update_control(user_defined_fields)
        .map_err(|_| DebyError::ControlUpdate)?;

    Ok(msg.to_string())
}

/// Updates debian changelog file
///
/// ## Arguments
///
/// - `version` - version string to be included in changelog file
/// - `changes` - changes to be included in changelog file
pub fn update_changelog_file(version: &str, changes: &str) -> Result<String, DebyError> {
    let config = Config::new().map_err(|_| DebyError::ConfigNew)?;

    let msg = config
        .update_changelog(&version, &changes)
        .map_err(|_| DebyError::ChangelogUpdate)?;

    Ok(msg.to_string())
}
