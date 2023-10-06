// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use liquid::Object;
use std::collections::HashMap;

use crate::config_models::Process;

type TemplateVarMap = HashMap<String, HashMap<String, String>>;

#[derive(Debug, Clone)]
pub struct TemplateVariables(TemplateVarMap);

impl TemplateVariables {
    pub fn new() -> Self {
        TemplateVariables(HashMap::new())
    }

    pub fn insert(&mut self, key: &str, value: HashMap<String, String>) {
        self.0.insert(key.to_string(), value);
    }

    pub fn as_value(&self) -> Object {
        liquid::object!(self.0.clone())
    }
}

pub fn render_string(template: String, data: &Object) -> Option<String> {
    let tp_engine = liquid::ParserBuilder::with_stdlib().build();
    if let Ok(engine) = &tp_engine {
        let tp_template = engine.parse(template.as_str().clone());
        if let Ok(template) = tp_template {
            let tp_value = template.render(data);
            if let Ok(rendered_value) = tp_value {
                return Some(rendered_value);
            } else {
                println!("Failed to render template: {}", tp_value.err().unwrap());
            }
        } else {
            println!("Failed to parse template: {}", tp_template.err().unwrap());
        }
    } else {
        println!(
            "Failed to create template engine: {}",
            tp_engine.err().unwrap()
        );
    }
    return None;
}

pub fn render_environment(env: HashMap<String, String>) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    for (key, value) in &env {
        render_environment_value(key.clone(), value.clone(), &env, &mut result);
    }
    result
}

pub fn render_environment_value(
    key: String,
    value: String,
    lookup_map: &HashMap<String, String>,
    result_map: &mut HashMap<String, String>,
) {
    let mut vars = TemplateVariables::new();
    vars.insert("env", lookup_map.clone());
    if let Some(rendered_value) = render_string(value.clone(), &vars.as_value()) {
        result_map.insert(key.clone(), rendered_value);
    } else {
        result_map.insert(key.clone(), value.clone());
    }
}

pub fn render_process(process: &Process, args: Object) -> Option<Process> {
    let mut rendered_proc_args: Vec<String> = Vec::new();
    for arg in &process.args {
        if let Some(rendered_arg) = render_string(arg.clone(), &args) {
            rendered_proc_args.push(rendered_arg);
        } else {
            return None;
        }
    }
    return if cfg!(feature = "allow_unsafe_command_templates") {
        // TODO: If we add system environment variables, they MUST be removed here for security reasons!
        if let Some(command) = render_string(process.command.clone(), &args) {
            Some(Process {
                command,
                args: rendered_proc_args,
            })
        } else {
            None
        }
    } else {
        Some(Process {
            command: process.command.clone(),
            args: rendered_proc_args,
        })
    };
}
