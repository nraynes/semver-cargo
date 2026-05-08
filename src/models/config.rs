use derive_getters::Getters;
use semver_common::Alert;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::str::FromStr;

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

#[derive(Serialize, Deserialize, Getters, Clone)]
pub struct Config {
    /// A flag specifying whether to set the version in the Cargo.toml file.
    #[serde(default = "default_true")]
    set_version: bool,

    /// A flag specifying whether to publish the crate to crates.io.
    #[serde(default = "default_false")]
    publish: bool,

    /// A flag specifying whether to run this plugin even if the version was no updated.
    #[serde(default = "default_false")]
    act_on_no_update: bool,
}

impl FromStr for Config {
    type Err = Alert;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let config: Config = serde_json::from_str(s)?;
        Ok(config)
    }
}

impl<T> From<T> for Config
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self::from_str(&value.to_string()).unwrap()
    }
}
