use semver_cargo::{Cargo, parse_args};
use semver_common::Alert;
use std::env;

fn main() -> Result<(), Alert> {
    let args: Vec<String> = env::args().collect();
    let (config, version, log_level) = parse_args(args)?;
    let cargo = Cargo::new(log_level);

    if *config.set_version() {
        cargo.set_version(&version)?;
    }
    if *config.publish() {
        cargo.publish()?;
    }
    Ok(())
}
