use crate::cfg::models::Configuration;

use config::{Config, ConfigError, Environment, File};
use std::env;

/// Try to read the program configuration from various sources.
///
/// # Configuration sources
///
/// The following configuration sources are accepted, in that order:
/// * Through various files that may or may not be present on the filesystem:
///   * `./appconfig.yml`
///   * `~/.alexandrie/config.yml`
///   * `/etc/alexandrie/config`
///   * `/usr/local/alexandrie/config`
///   * A custom path specified in the `ALX_CONFIG` environment variable.
/// * Through environment variables, using `_` as a separator for nested values and `ALX` as a
/// prefix (for example, `ALX_DATABASE_PASSWORD`).
pub fn read_configuration() -> Result<Configuration, ConfigError> {
    let mut configuration = Config::new();

    // Check for local application stuff first.
    configuration.merge(File::with_name("appconfig").required(false))?;

    // Try to load the configuration from a few default paths
    configuration.merge(File::with_name("~/.alexandrie/config").required(false))?;
    configuration.merge(File::with_name("/etc/alexandrie/config").required(false))?;
    configuration.merge(File::with_name("/usr/local/alexandrie/config").required(false))?;

    // Check if the user sent us a configuration path through arguments too
    match env::var("ALX_CONFIG") {
        Ok(custom_path) => {
            configuration.merge(File::with_name(custom_path.as_str()))?;
        }
        Err(_) => (),
    };

    // Add environment overrides
    configuration.merge(Environment::with_prefix("ALX").separator("_"))?;

    configuration.try_into()
}
