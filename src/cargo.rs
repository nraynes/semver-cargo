use r_log::{LogLevel, Logger};
use semver_common::{Alert, Version, run_command};

pub struct Cargo {
    logger: Logger,
}

impl Cargo {
    pub fn new(level: LogLevel) -> Self {
        Self {
            logger: Logger::new(level),
        }
    }

    pub fn set_version(&self, version: &Version) -> Result<(), Alert> {
        run_command(
            "cargo",
            ["set-version", &version.short()],
            Some(&self.logger),
        )?;
        Ok(())
    }

    pub fn publish(&self) -> Result<(), Alert> {
        run_command("cargo", ["publish"], Some(&self.logger))?;
        Ok(())
    }
}
