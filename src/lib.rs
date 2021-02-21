mod config;

use config::{Config, DebyError};

/// Updates `changelog` and `control` files
///
/// ## Arguments
///
/// - `version` - an updated version string
/// - `changes` - changes to be included in your files
/// - `user_defined_fields` - additional dynamic fields to be included in `control` file
pub fn update(version: &str, changes: &str, user_defined_fields: &str) -> Result<(), DebyError> {
    let config = Config::new().map_err(|_| DebyError::ConfigNew)?;

    config
        .update(version, changes, user_defined_fields)
        .map_err(|_| DebyError::Update)?;

    Ok(())
}
