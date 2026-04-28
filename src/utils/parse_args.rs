use crate::Config;
use semver_common::{Alert, Version};

pub fn parse_args(args: Vec<String>) -> Result<(Config, Version), Alert> {
    let config_str = args.get(1).ok_or("No configuration supplied.")?;
    let version_str = args.get(1).ok_or("No version supplied.")?;
    let config: Config = serde_json::from_str(config_str)?;
    let version: Version = serde_json::from_str(version_str)?;
    Ok((config, version))
}
