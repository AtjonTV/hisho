// This file 'template.rs' is part of the 'hisho' project.
//
// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use liquid::Object;
use std::collections::HashMap;

use crate::config_models::Process;
use crate::log;

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

    pub fn get(&self, key: &str) -> Option<&HashMap<String, String>> {
        self.0.get(key)
    }

    pub fn as_value(&self) -> Object {
        liquid::object!(self.0.clone())
    }
}

/// Try to render a string template with the given data for variables.
pub fn render_string(template: String, data: &Object) -> Option<String> {
    let tp_engine = liquid::ParserBuilder::with_stdlib().build();
    if let Ok(engine) = &tp_engine {
        let tp_template = engine.parse(template.as_str());
        if let Ok(template) = tp_template {
            let tp_value = template.render(data);
            if let Ok(rendered_value) = tp_value {
                return Some(rendered_value);
            } else {
                log::error(format!(
                    "Failed to render template: {}",
                    tp_value.err().unwrap()
                ));
            }
        } else {
            log::error(format!(
                "Failed to parse template: {}",
                tp_template.err().unwrap()
            ));
        }
    } else {
        log::error(format!(
            "Failed to create template engine: {}",
            tp_engine.err().unwrap()
        ));
    }
    None
}

/// Render each environment variable with the environment for variables
pub fn render_environment(env: HashMap<String, String>) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    for (key, value) in &env {
        render_environment_value(key.clone(), value.clone(), &env, &mut result);
    }
    result
}

/// Try to render a process with the given args for variables.
pub fn render_process(process: &Process, args: Object) -> Option<Process> {
    let argv = Vec::new();
    render_process_with_argv(process, args, &argv)
}

/// Try to expand `[[argv]]` in the process with the given argv vector.
pub fn render_process_with_argv(
    process: &Process,
    args: Object,
    argv: &Vec<String>,
) -> Option<Process> {
    let mut rendered_proc_args: Vec<String> = Vec::new();
    let mut proc_args = process.args.clone();
    expand_argv_label(&mut proc_args, argv);
    for arg in &proc_args {
        if let Some(rendered_arg) = render_string(arg.clone(), &args) {
            rendered_proc_args.push(rendered_arg);
        } else {
            return None;
        }
    }
    let workdir = render_string(process.cwd.clone(), &args).unwrap_or(process.cwd.clone());
    if cfg!(feature = "allow_unsafe_command_templates") {
        // TODO: If we add system environment variables, they MUST be removed here for security reasons!
        render_string(process.command.clone(), &args).map(|command| Process {
            command,
            args: rendered_proc_args,
            cwd: workdir,
        })
    } else {
        Some(Process {
            command: process.command.clone(),
            args: rendered_proc_args,
            cwd: workdir,
        })
    }
}

fn expand_argv_label(data: &mut Vec<String>, argv: &Vec<String>) {
    let mut positions = Vec::new();
    // find all positions that should be expanded
    for (i, string) in data.iter().enumerate() {
        if string.eq("[[argv]]") {
            positions.push(i);
        }
    }

    let mut pos_offset = 0;
    for pos in positions {
        // add the offset to the position
        let new_pos = pos + pos_offset;
        // splice the argv into the position
        let _: Vec<String> = data
            .splice(new_pos..=new_pos, argv.iter().cloned())
            .collect();
        // increment the offset by the amount of spliced items
        pos_offset += argv.len() - 1
    }
}

fn render_environment_value(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_valid_string_template() {
        let template = "Hello, {{name}}!".to_string();
        let data = liquid::object!({
            "name": "John"
        });
        assert_eq!(
            render_string(template, &data),
            Some("Hello, John!".to_string())
        );
    }

    #[test]
    fn fail_with_invalid_string_template() {
        let template = "Hello, {{e.name}}!".to_string();
        let data = liquid::object!({
            "name": "John"
        });
        assert_eq!(render_string(template, &data), None);
    }

    #[test]
    fn render_valid_environment_template() {
        let mut env = HashMap::new();
        env.insert("hello_name".to_string(), "Hello, {{env.name}}!".to_string());
        env.insert("name".to_string(), "John".to_string());
        env.insert("world".to_string(), "world".to_string());
        env.insert(
            "hello_world".to_string(),
            "Hello, {{env.world}}!".to_string(),
        );

        let rendered = render_environment(env);
        assert_eq!(
            rendered.get("hello_name"),
            Some(&"Hello, John!".to_string())
        );
        assert_eq!(
            rendered.get("hello_world"),
            Some(&"Hello, world!".to_string())
        );
    }

    #[test]
    fn render_valid_process_template() {
        let mut env = HashMap::new();
        env.insert("name".to_string(), "John".to_string());

        let process = Process {
            command: "echo".to_string(),
            args: vec!["Hello, {{env.name}}!".to_string()],
            cwd: String::new(),
        };

        let mut vars = TemplateVariables::new();
        vars.insert("env", env);

        let rendered = render_process(&process, vars.as_value());
        if let Some(rendered_process) = rendered {
            assert_eq!(rendered_process.command, "echo".to_string());
            assert_eq!(rendered_process.args[0], "Hello, John!".to_string());
        } else {
            assert!(false);
        }
    }

    #[test]
    #[cfg(feature = "allow_unsafe_command_templates")]
    fn render_valid_process_template_with_unsafe_command_templates() {
        let mut env = HashMap::new();
        env.insert("bin_dir".to_string(), "/usr/local/bin".to_string());

        let process = Process {
            command: "{{env.bin_dir}}/echo".to_string(),
            args: vec![],
            cwd: String::new(),
        };
        let rendered = render_process(
            &process,
            liquid::object!({
                "env": env
            }),
        );
        if let Some(rendered_process) = rendered {
            assert_eq!(rendered_process.command, "/usr/local/bin/echo".to_string());
        } else {
            assert!(false);
        }
    }
}
