use semver_cargo::{cargo, parse_args};
use semver_common::Alert;
use std::env;

fn main() -> Result<(), Alert> {
    let args: Vec<String> = env::args().collect();
    let (config, version) = parse_args(args)?;
    if *config.set_version() {
        cargo::set_version(&version)?;
    }
    if *config.publish() {
        cargo::publish()?;
    }
    Ok(())
}
