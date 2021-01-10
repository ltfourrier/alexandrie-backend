use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LogConfiguration {
    pub level: String,
}

#[derive(Deserialize)]
pub struct ServerConfiguration {
    pub listen_address: String,
    pub listen_port: u32,
}

#[derive(Deserialize)]
pub struct DatabaseConfiguration {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub max_connections: u32,
}

#[derive(Deserialize)]
pub struct Configuration {
    pub log: LogConfiguration,
    pub server: ServerConfiguration,
    pub database: DatabaseConfiguration,
}

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
///   * A custom value specified in the program arguments as `-c /path/to/config` or
///     `--configuration-file /path/to/config`.
/// * Through environment variables, prefixed with `ALEX` (for example, `ALEX_DATABASE_PASSWORD`).
pub fn read_configuration() -> Result<Configuration, ConfigError> {
    let mut configuration = Config::new();

    // Check for local application stuff first.
    configuration.merge(File::with_name("appconfig").required(false))?;

    // Try to load the configuration from a few default paths
    configuration.merge(File::with_name("~/.alexandrie/config").required(false))?;
    configuration.merge(File::with_name("/etc/alexandrie/config").required(false))?;
    configuration.merge(File::with_name("/usr/local/alexandrie/config").required(false))?;

    // Check if the user sent us a configuration path through arguments too
    match retrieve_path_from_args() {
        Some(custom_path) => {
            configuration.merge(File::with_name(custom_path.as_str()));
        }
        None => (),
    };

    // Add environment overrides
    configuration.merge(Environment::with_prefix("ALEX"))?;

    configuration.try_into()
}

/// Tries to retrieve a custom configuration file path from program arguments.
///
/// This method will look for an argument called `-c` or `--configuration-file` and will return the
/// value of the next argument, if it exists.
fn retrieve_path_from_args() -> Option<String> {
    let args = env::args();

    let mut next_arg = false;
    for arg in args {
        if next_arg {
            return Some(arg);
        }

        next_arg = match arg.as_str() {
            "-c" => true,
            "--configuration_file" => true,
            _ => false,
        }
    }
    return None;
}
