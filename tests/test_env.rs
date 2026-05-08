use std::{
    env::current_dir,
    fs,
    io::{self, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

use assert_fs::{
    TempDir,
    fixture::{ChildPath, FileTouch, FileWriteStr, PathChild},
};
use derive_getters::Getters;
use semver_common::{CommitMap, Version};
use serde_json::Value;

#[derive(Getters)]
pub struct TestEnv {
    #[allow(dead_code)]
    temp: TempDir,
    repo: ChildPath,
    config_path: ChildPath,
    text_file: ChildPath,
    cargo_bin: Command,
}

impl TestEnv {
    pub fn new(config: Value) -> Self {
        // Setup mock project repo.
        let temp = assert_fs::TempDir::new().unwrap();
        Command::new("cargo")
            .arg("new")
            .arg("temp_proj")
            .current_dir(temp.path())
            .output()
            .unwrap();
        let repo = temp.child("temp_proj");

        // Setup semver-release configuration file.
        let config_path = repo.child("config.semver.json");
        config_path.touch().unwrap();
        config_path.write_str(&config.to_string()).unwrap();

        // Add a file to write to to stage changes for commits.
        let text_file = repo.child("test_file");
        text_file.touch().unwrap();

        let cargo_bin = Self::get_cargo_bin(repo.path().to_path_buf());

        Self {
            temp,
            repo,
            config_path,
            text_file,
            cargo_bin,
        }
    }

    pub fn commit(&mut self, commit: &str) {
        self.text_file.write_str(commit).unwrap();
        Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(self.repo.path())
            .output()
            .unwrap();
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit)
            .current_dir(self.repo.path())
            .output()
            .unwrap();
    }

    fn get_cargo_bin(path: PathBuf) -> Command {
        let cargo_current_cmd_path = format!(
            "{}/{}",
            current_dir().unwrap().to_str().unwrap(),
            "target/debug/semver-cargo"
        );
        let cargo_new_cmd_path = format!("{}/{}", path.to_str().unwrap(), "semver-cargo");
        Command::new("cp")
            .arg(cargo_current_cmd_path)
            .arg(&cargo_new_cmd_path)
            .output()
            .unwrap();
        Command::new(cargo_new_cmd_path)
    }

    pub fn run(&mut self, major: u32, minor: u32, patch: u32, updated: &str) {
        let version = Version::new(major, minor, patch, CommitMap::new());
        let config_contents = fs::read_to_string(self.config_path.path()).unwrap();
        let mut child = self
            .cargo_bin
            .arg(config_contents)
            .arg(serde_json::to_string(&version).unwrap())
            .arg("info")
            .arg(updated)
            .arg("-d")
            .arg("true")
            .env("CARGO_REGISTRY_TOKEN", "test")
            .current_dir(self.repo.path())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut child_stdout = child.stdout.take().unwrap();
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = child_stdout.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            io::stdout().write_all(&buffer[..bytes_read]).unwrap();
        }
        let _ = child.wait().unwrap();
    }
}
