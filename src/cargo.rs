use std::collections::HashMap;

use derive_getters::Getters;
use r_log::{LogLevel, Logger};
use semver_common::{Alert, Version, git, run_command};

use crate::Config;

#[derive(Getters)]
pub struct Cargo {
    config: Config,
    version: Version,
    logger: Logger,
    env: HashMap<String, String>,
    updated: bool,
}

impl Cargo {
    pub fn init(
        config: Config,
        version: Version,
        level: LogLevel,
        env: HashMap<String, String>,
        updated: bool,
    ) -> Result<Self, Alert> {
        if *config.publish() {
            env.get("CARGO_REGISTRY_TOKEN")
                .ok_or("Environment variable CARGO_REGISTRY_TOKEN must be set to publish crate.")?;
        }
        Ok(Self {
            config,
            version,
            logger: Logger::new(level),
            env,
            updated,
        })
    }

    pub fn install(&self) -> Result<(), Alert> {
        run_command("cargo", ["install", "cargo-edit"], Some(&self.logger))?;
        Ok(())
    }

    pub fn set_version(&self) -> Result<(), Alert> {
        run_command(
            "cargo",
            ["set-version", &self.version.short()],
            Some(&self.logger),
        )?;
        Ok(())
    }

    pub fn publish(&self) -> Result<(), Alert> {
        run_command("cargo", ["publish"], Some(&self.logger))?;
        Ok(())
    }

    pub fn release(&self) -> Result<(), Alert> {
        if *self.updated() || *self.config.act_on_no_update() {
            // Attempt to install cargo set-version
            self.install()?;

            // Update version in Cargo.toml
            if *self.config.set_version() {
                self.set_version()?;
                git::commit_all(
                    &format!(
                        "semver-cargo bump cargo version to {}",
                        self.version.short()
                    ),
                    &self.logger(),
                )?;
            }

            // Publish crate.
            if *self.config.publish() {
                self.publish()?;
            }
        }
        Ok(())
    }
}
