use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    pub name: String,
    #[serde(default)]
    pub environments: Vec<Environment>,
    #[serde(default)]
    pub containers: Vec<Container>,
    #[serde(default)]
    pub build: Vec<BuildStep>,
    #[serde(default)]
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Environment {
    pub name: String,
    #[serde(default)]
    pub inherits: Vec<String>,
    #[serde(default)]
    pub values: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Container {
    pub name: String,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    pub name: String,
    #[serde(default)]
    pub environment: String,
    #[serde(default)]
    pub shell: Vec<Process>,
    #[serde(default)]
    pub args: HashMap<String, String>,
    #[serde(default)]
    pub capture_all: bool,
    #[serde(default)]
    pub depends_on_build: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Process {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildStep {
    pub name: String,
    pub shell: Process,
    #[serde(default)]
    pub depends_on: Vec<String>,
}
