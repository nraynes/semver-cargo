use clap::Parser;
use r_log::LogLevel;
use semver_common::Version;

#[derive(Parser)]
pub struct Args {
    /// Stringified JSON containing the SemVer-Release Config.
    pub config_json: String,
    /// Stringified JSON containing the new version information from SemVer-Release.
    pub version: Version,
    /// The Log Level to use for the logger.
    pub log_level: LogLevel,
    /// Whether or not the version was updated.
    pub updated: String,
    /// Flag specifying whether to do dry run publish.
    #[arg(short, long, default_value = "false")]
    pub dry_run: String,
}
