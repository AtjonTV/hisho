use std::collections::HashMap;
use crate::config_models::Environment;
use crate::template;

pub fn fetch_environment(environment: &str, environments: &Vec<Environment>) -> Option<Environment> {
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
            if let Some(parent) = fetch_environment(parent_env, environments) {
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

    for (key, value) in env.values {
        current_env.insert(key.clone(), value.clone());
    }

    let rendered_env = template::render_environment(current_env);
    return Some(Environment::new("current", Vec::new(), rendered_env));
}

impl Environment {
    pub fn new(name: &str, inherits: Vec<String>, values: HashMap<String, String>) -> Environment {
        return Environment {
            name: name.to_string(),
            inherits,
            values,
        }
    }
}
