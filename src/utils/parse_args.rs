use crate::Config;
use r_log::LogLevel;
use semver_common::{Alert, Version};

pub fn parse_args(args: Vec<String>) -> Result<(Config, Version, LogLevel), Alert> {
    let config_str = args.get(1).ok_or("No configuration supplied.")?;
    let version_str = args.get(2).ok_or("No version supplied.")?;
    let log_level_str = args.get(3).ok_or("No log level supplied.")?;
    let config: Config = serde_json::from_str(config_str)?;
    let version: Version = serde_json::from_str(version_str)?;
    let log_level = LogLevel::from_str(log_level_str).ok_or("Not a valid log level")?;
    Ok((config, version, log_level))
}
