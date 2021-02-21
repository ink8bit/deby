mod config;

use config::{Config, DebyError};

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
