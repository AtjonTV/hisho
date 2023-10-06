// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::fs;

use crate::config_models::{Environment, Environments, Process};
use crate::template;

pub fn fetch_environment(environment: &str, environments: &Environments) -> Option<Environment> {
    let mut found_env: Option<Environment> = None;
    for env in environments {
        if env.name == environment {
            found_env = Some(env.clone());
        }
    }

    if found_env.is_none() {
        return None;
    }

    let mut current_env: HashMap<String, String> = HashMap::new();
    let env = found_env.unwrap();

    if env.inherits.len() != 0 {
        let mut parent_envs: Vec<Environment> = Vec::new();
        for parent_env in &env.inherits {
            if let Some(mut parent) = fetch_environment(parent_env, environments) {
                load_env_from_file(&parent.sources, &mut parent.values);
                parent_envs.push(parent);
            }
        }
        parent_envs.reverse();
        for parent_env in parent_envs {
            for (key, value) in parent_env.values {
                current_env.insert(key.clone(), value.clone());
            }
        }
    }

    load_env_from_file(&env.sources, &mut current_env);
    for (key, value) in env.values {
        current_env.insert(key.clone(), value.clone());
    }

    let rendered_env = template::render_environment(current_env);
    return Some(Environment::new("current", Vec::new(), rendered_env));
}

fn load_env_from_file(sources: &Vec<String>, out_env: &mut HashMap<String, String>) {
    if !sources.is_empty() {
        for path in sources {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(btree) = dotenv_parser::parse_dotenv(data.as_str()) {
                    for (k,v) in btree {
                        out_env.insert(k,v);
                    }
                }
            }
        }
    }
}

impl Environment {
    pub fn new_empty() -> Environment {
        return Environment {
            name: "empty".to_string(),
            inherits: Vec::new(),
            values: HashMap::new(),
            sources: Vec::new(),
        };
    }
    pub fn new(name: &str, inherits: Vec<String>, values: HashMap<String, String>) -> Environment {
        return Environment {
            name: name.to_string(),
            inherits,
            values,
            sources: Vec::new(),
        };
    }
}

impl Process {
    pub fn new(command: String, args: Vec<String>) -> Process {
        return Process { command, args };
    }
}
