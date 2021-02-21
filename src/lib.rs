mod config;

use config::Config;
use std::error::Error;

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
    user_defined_fields: &str,
) -> Result<(), Box<dyn Error>> {
    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };

    config.update(version, changes, user_defined_fields)?;

    Ok(())
}
