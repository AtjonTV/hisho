// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ron::error::SpannedResult;
use std::{env, fs};

use crate::hisho::config::fetch_environment;
use crate::hisho::config_models::{Environment, Process, Project};
use crate::hisho::template::TemplateVariables;
use crate::hisho::{build, containers, git, log, shell, template};

pub async fn cli_main() {
    let version = env!("CARGO_PKG_VERSION");
    log::print(format!("Hisho v{} by Thomas Obernosterer", version));
    let default_service_file = "hisho.ron";

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
        .get("hisho:file")
        .unwrap_or(&None)
        .clone()
        .unwrap_or(default_service_file.to_string());

    let data_from_file = fs::read_to_string(service_file.as_str());
    if let Err(e) = data_from_file {
        log::error(format!(
            "Could not read service file '{}': {:?}",
            service_file,
            e.to_string()
        ));
        return;
    }
    let project_data: SpannedResult<Project> = ron::from_str(data_from_file.unwrap().as_str());
    if let Err(e) = project_data {
        log::error(format!(
            "Could not parse service file '{}': {:?}",
            service_file,
            e.to_string()
        ));
        return;
    }

    // remove service consumed arguments
    if !command_set.long_params.is_empty() {
        let mut idx_to_remove = vec![];
        for (_, param) in command_set.long_params.iter().enumerate() {
            if param.0.starts_with("hisho:") {
                idx_to_remove.push(param.0.clone());
            }
        }
        for idx in idx_to_remove.iter().rev() {
            command_set.long_params.remove(idx);
        }
    }
    if !args.is_empty() {
        let mut idx_to_remove = vec![];
        for (i, arg) in args.iter().enumerate() {
            if arg.starts_with("--hisho:") {
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

    let mut command_found = false;

    let mut vars = TemplateVariables::new();
    vars.insert("git", git::fetch_repo_vars(service_file.as_str()));

    // if a command was given, try to match it to the config defined
    if let Some(command) = args.first() {
        for cmd in &project.commands {
            if cmd.name == *command {
                command_found = true;
                // try to fetch an environment
                let env =
                    fetch_environment(cmd.environment.clone().as_str(), &project.environments)
                        .unwrap_or(Environment::new_empty());
                vars.insert("env", env.values);

                // make sure required containers are running
                if !containers::ensure_running(&project.containers, &vars).await {
                    return;
                }

                // make sure required builds have run successfully
                if !build::ensure_build(cmd, &project.build, &vars) {
                    return;
                }

                // if there is no shell defined, do nothing and return
                if cmd.shell.is_empty() {
                    log::print("No shell, nothing to do. Exiting..".to_string());
                    return;
                }

                if cmd.capture_all {
                    // Construct the command to be executed
                    let given_args = args.iter().skip(1).cloned().collect::<Vec<String>>();

                    for shell_cmd in &cmd.shell {
                        let _ = shell::exec(
                            &Process::new(shell_cmd.command.clone(), given_args.clone()),
                            vars.get("env"),
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
                    vars.insert("arg", argument_lookup);
                    for shell_cmd in &cmd.shell {
                        if let Some(rendered_command) =
                            template::render_process(shell_cmd, vars.as_value())
                        {
                            rendered_commands.push(rendered_command);
                        }
                    }

                    for rendered_command in &rendered_commands {
                        let _ = shell::exec(rendered_command, vars.get("env"));
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
    log::print(format!(
        "Usage: {} <command> [args]",
        env::args().take(1).collect::<Vec<String>>().join(" ")
    ));
    if service_file.is_some() {
        log::print(format!(
            "Arguments:\n\t  --hisho:file\tSpecify a .ron file to load, tries to load '{}' by default", service_file.unwrap()
        ));
    }
    if project.is_some() {
        log::print(format!(
            "Custom Commands:\n{}",
            project
                .unwrap()
                .commands
                .iter()
                .map(|c| format!("\t  - {}", c.name.clone()))
                .collect::<Vec<String>>()
                .join("\n")
        ));
    }
}
