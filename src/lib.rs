mod config;

use config::Config;
use std::error::Error;

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
