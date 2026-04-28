use semver_cargo::{Cargo, parse_args};
use semver_common::Alert;
use std::{collections::HashMap, env};

fn main() -> Result<(), Alert> {
    let args: Vec<String> = env::args().collect();
    let environment_vars: HashMap<String, String> = env::vars().collect();
    let (config, version, log_level) = parse_args(args)?;
    let cargo = Cargo::init(config, version, log_level, environment_vars)?;
    cargo.release()?;
    Ok(())
}
