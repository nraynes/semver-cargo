use semver_cargo::{cargo, parse_args};
use semver_common::Alert;
use std::env;

fn main() -> Result<(), Alert> {
    let args: Vec<String> = env::args().collect();
    let (_, version) = parse_args(args)?;
    cargo::set_version(&version)?;
    cargo::publish()?;
    Ok(())
}
