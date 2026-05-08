use clap::Parser;
use semver_cargo::{Args, Cargo, Config};
use semver_common::Alert;
use std::{collections::HashMap, env};

fn main() -> Result<(), Alert> {
    let args: Args = Args::parse();
    let environment_vars: HashMap<String, String> = env::vars().collect();
    let config: Config = serde_json::from_str(&args.config_json)?;
    let cargo = Cargo::init(
        config,
        args.version,
        args.log_level,
        environment_vars,
        args.updated.parse()?,
        args.dry_run.parse()?,
    )?;
    cargo.release()?;
    Ok(())
}
