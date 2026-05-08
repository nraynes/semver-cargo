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
    dry_run: bool,
}

impl Cargo {
    /// Initialize the plugin with the data from the cli arguments.
    pub fn init(
        config: Config,
        version: Version,
        level: LogLevel,
        env: HashMap<String, String>,
        updated: bool,
        dry_run: bool,
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
            dry_run,
        })
    }

    /// Install cargo-edit to use for bumping the version in Cargo.toml.
    pub fn install(&self) -> Result<(), Alert> {
        run_command("cargo", ["install", "cargo-edit"], Some(&self.logger))?;
        Ok(())
    }

    /// Run cargo set-version to bump the version in Cargo.toml.
    pub fn set_version(&self) -> Result<(), Alert> {
        run_command(
            "cargo",
            ["set-version", &self.version.short()],
            Some(&self.logger),
        )?;
        Ok(())
    }

    /// Run cargo publish to publish the crate to crates.io. Can do a dry run for testing.
    pub fn publish(&self, dry_run: bool) -> Result<(), Alert> {
        match dry_run {
            true => run_command("cargo", ["publish", "--dry-run"], Some(&self.logger))?,
            false => run_command("cargo", ["publish"], Some(&self.logger))?,
        };
        Ok(())
    }

    /// Run the full release process for cargo.
    pub fn release(&self) -> Result<(), Alert> {
        println!("{}", serde_json::to_string(&self.config)?);
        if self.updated || *self.config.act_on_no_update() {
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
                    self.logger(),
                )?;
            }

            // Publish crate.
            if *self.config.publish() {
                self.publish(self.dry_run)?;
            }
        }
        Ok(())
    }
}
