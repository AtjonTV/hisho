use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    pub name: String,
    pub environments: Vec<Environment>,
    pub containers: Vec<Container>,
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Environment {
    pub name: String,
    pub inherits: Vec<String>,
    pub values: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Container {
    pub name: String,
    pub required: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    pub name: String,
    pub environment: String,
    pub shell: Vec<String>,
    pub args: HashMap<String, String>,
    pub capture_all: bool,
}
