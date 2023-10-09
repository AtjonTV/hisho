// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ron::error::SpannedResult;
use std::{env, fs};

use crate::config::fetch_environment;
use crate::config_models::{Environment, Process, Project};
use crate::template::TemplateVariables;

mod build;
mod config;
mod config_models;
mod containers;
mod shell;
mod template;

#[tokio::main]
async fn main() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Service Helper v{} by Thomas Obernosterer", version);
    let default_service_file = "service.ron";

    // remove the program name from the arguments
    let mut args: Vec<String> = env::args().skip(1).collect();

    // check if the default service file exists
    let default_file_exists = fs::metadata(default_service_file).is_ok();

    // if no arguments have been given and the default service file does not exist
    if args.is_empty() && !default_file_exists {
        print_help(None, Some(default_service_file));
        return;
    }

    // parse the args
    let mut command_set: argust::ArgContext = argust::parse_args(args.iter(), None);

    // try to get file name from -f, and default to default_service_file if -f not given or empty
    let service_file = command_set
        .long_params
        .get("service:file")
        .unwrap_or(&None)
        .clone()
        .unwrap_or(default_service_file.to_string());

    let data_from_file = fs::read_to_string(service_file.as_str());
    if let Err(e) = data_from_file {
        eprintln!(
            "Service: Could not read service file '{}': {:?}",
            service_file,
            e.to_string()
        );
        return;
    }
    let project_data: SpannedResult<Project> = ron::from_str(data_from_file.unwrap().as_str());
    if let Err(e) = project_data {
        eprintln!(
            "Service: Could not parse service file '{}': {:?}",
            service_file,
            e.to_string()
        );
        return;
    }

    // remove service consumed arguments
    if !command_set.long_params.is_empty() {
        let mut idx_to_remove = vec![];
        for (_, param) in command_set.long_params.iter().enumerate() {
            if param.0.starts_with("service:") {
                idx_to_remove.push(param.0.clone());
            }
        }
        for idx in idx_to_remove.iter().rev() {
            command_set.long_params.remove(idx);
        }
    }
    if args.len() > 0 {
        let mut idx_to_remove = vec![];
        for i in 0..args.len() {
            if args[i].starts_with("--service:") {
                idx_to_remove.push(i);
            }
        }
        for i in idx_to_remove.iter().rev() {
            args.remove(*i);
        }
    }

    let project = project_data.unwrap();

    // if no arguments have been given
    if args.is_empty() {
        print_help(Some(&project), None);
        return;
    }

    // make sure required containers are running
    containers::ensure_running(&project.containers).await;

    let mut command_found = false;

    // if a command was given, try to match it to the config defined
    if let Some(command) = args.first() {
        for cmd in &project.commands {
            if cmd.name == *command {
                command_found = true;
                // try to fetch an environment
                let env =
                    fetch_environment(cmd.environment.clone().as_str(), &project.environments)
                        .unwrap_or(Environment::new_empty());

                // make sure required builds have run successfully
                if !build::ensure_build(&cmd, &project.build, &env) {
                    return;
                }

                // if there is no shell defined, do nothing and return
                if cmd.shell.is_empty() {
                    println!("Service: No shell, nothing to do. Exiting..");
                    return;
                }

                if cmd.capture_all {
                    // Construct the command to be executed
                    let given_args = args.iter().cloned().skip(1).collect::<Vec<String>>();

                    for shell_cmd in &cmd.shell {
                        let _ = shell::exec(
                            &Process::new(shell_cmd.command.clone(), given_args.clone()),
                            &env,
                        );
                    }
                } else {
                    let mut rendered_commands: Vec<Process> = Vec::new();
                    let argument_lookup = command_set
                        .long_params
                        .iter()
                        .map(|(key, value)| {
                            if value.is_some() {
                                (key.clone(), value.clone().unwrap())
                            } else {
                                (key.clone(), String::new())
                            }
                        })
                        .collect();
                    let mut vars = TemplateVariables::new();
                    vars.insert("env", env.values.clone());
                    vars.insert("arg", argument_lookup);
                    for shell_cmd in &cmd.shell {
                        if let Some(rendered_command) =
                            template::render_process(shell_cmd, vars.as_value())
                        {
                            rendered_commands.push(rendered_command);
                        }
                    }

                    for rendered_command in &rendered_commands {
                        let _ = shell::exec(rendered_command, &env);
                    }
                }
            }
        }
    }

    if !command_found {
        print_help(Some(&project), None);
        return;
    }
}

fn print_help(project: Option<&Project>, service_file: Option<&str>) {
    println!(
        "Usage: {} <command> [args]",
        env::args().take(1).collect::<Vec<String>>().join(" ")
    );
    if service_file.is_some() {
        println!(
            "Arguments:\n--service:file\tSpecify a .ron file to load, tries to load '{}' by default", service_file.unwrap()
        );
    }
    if project.is_some() {
        println!(
            "Custom Commands:\n{}",
            project.unwrap()
                .commands
                .iter()
                .map(|c| format!("- {}", c.name.clone()))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}
