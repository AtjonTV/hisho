// This file 'config_models.rs' is part of the 'hisho' project.
//
// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    #[serde(default)]
    pub environments: Environments,
    #[serde(default)]
    pub containers: Containers,
    #[serde(default)]
    pub build: BuildSteps,
    #[serde(default)]
    pub services: Services,
    #[serde(default)]
    pub commands: Commands,

    // this is a runtime variable
    #[serde(skip)]
    pub workdir: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Environment {
    pub name: String,
    #[serde(default)]
    pub system: Vec<String>,
    #[serde(default)]
    pub inherits: Vec<String>,
    #[serde(default)]
    pub values: HashMap<String, String>,
    #[serde(default)]
    pub sources: Vec<String>,
}
pub type Environments = Vec<Environment>;

impl Environment {
    pub fn new_empty() -> Environment {
        Environment {
            name: "empty".to_string(),
            system: Vec::new(),
            inherits: Vec::new(),
            values: HashMap::new(),
            sources: Vec::new(),
        }
    }
    pub fn new(name: &str, inherits: Vec<String>, values: HashMap<String, String>) -> Environment {
        Environment {
            name: name.to_string(),
            system: Vec::new(),
            inherits,
            values,
            sources: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Container {
    pub name: String,
}
pub type Containers = Vec<Container>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    pub name: String,
    #[serde(default)]
    pub environment: String,
    #[serde(default)]
    pub shell: Vec<Process>,
    #[serde(default)]
    #[deprecated(note = "this field is no longer unused")]
    pub args: HashMap<String, String>,
    #[serde(default)]
    pub depends_on_build: Vec<String>,
}
pub type Commands = Vec<Command>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Process {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub cwd: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildStep {
    pub name: String,
    pub shell: Vec<Process>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub input_files: Vec<String>,
}
pub type BuildSteps = Vec<BuildStep>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Service {
    pub name: String,
    pub protocol: ServiceProtocol,
    pub uri: String,
}
pub type Services = Vec<Service>;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ServiceProtocol {
    HTTP,
    TCP,
}

impl PartialEq for BuildStep {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
