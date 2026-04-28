use semver_cargo::{Cargo, parse_args};
use semver_common::{Alert, git};
use std::env;

fn main() -> Result<(), Alert> {
    let args: Vec<String> = env::args().collect();
    let (config, version, log_level) = parse_args(args)?;
    let cargo = Cargo::new(log_level);

    // Attempt to install cargo via rustup
    cargo.install()?;

    // Update version in Cargo.toml
    if *config.set_version() {
        cargo.set_version(&version)?;
        git::commit_all(
            &format!("semver-cargo bump cargo version to {}", version.short()),
            &cargo.logger(),
        )?;
    }

    // Publish crate.
    if *config.publish() {
        cargo.publish()?;
    }
    Ok(())
}
