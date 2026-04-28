use semver_common::{Alert, Version, run_command};

pub fn set_version(version: &Version) -> Result<(), Alert> {
    run_command("cargo", ["set-version", &version.short()])?;
    Ok(())
}

pub fn publish() -> Result<(), Alert> {
    run_command("cargo", ["publish"])?;
    Ok(())
}
