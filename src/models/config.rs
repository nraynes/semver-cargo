use derive_getters::Getters;
use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

#[derive(Serialize, Deserialize, Getters)]
pub struct Config {
    #[serde(default = "default_true")]
    set_version: bool,

    #[serde(default = "default_false")]
    publish: bool,

    #[serde(default = "default_false")]
    act_on_no_update: bool,
}
