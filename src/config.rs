use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub version: String,
    pub description: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum Step {
    Move {
        description: String,
        src: String,
        dest: String,
    },
    UpdateJson {
        description: String,
        file: String,
        field: String,
        value: String,
    },
    Command {
        description: String,
        command: String,
    },
}
