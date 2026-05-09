use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Stringified JSON containing the SemVer-Release Config.
    pub config_json: String,
    /// Stringified JSON containing the new version information from SemVer-Release.
    pub version: String,
    /// The Log Level to use for the logger.
    pub log_level: String,
    /// Whether or not the version was updated.
    pub updated: String,
    /// Flag specifying whether to do dry run publish.
    #[arg(short, long, default_value = "false")]
    pub dry_run: String,
}
