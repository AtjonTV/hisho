// This file 'main.rs' is part of the 'hisho' project.
//
// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ron::error::SpannedResult;
use std::process::exit;
use std::{env, fs};

use hisho_core::build_tool;
use hisho_core::config_models::{Environment, Process, Project};
use hisho_core::containers;
use hisho_core::environment::fetch_environment;
use hisho_core::files;
use hisho_core::git;
use hisho_core::log;
use hisho_core::shell;
use hisho_core::template;
use hisho_core::template::TemplateVariables;

#[tokio::main]
async fn main() {
    let version = env!("CARGO_PKG_VERSION");
    log::print(format!(
        "Hisho v{} (hisho_cli) by Thomas Obernosterer",
        version
    ));
    log::error("WARNING: hisho_cli is deprecated. Please use hisho_cli2.".to_string());
    let default_service_file = "hisho.ron";

    // remove the program name from the arguments
    let mut args: Vec<String> = env::args().skip(1).collect();

    // check if the default service file exists
    let default_file_exists = fs::metadata(default_service_file).is_ok();

    // if no arguments have been given and the default service file does not exist
    if args.is_empty() && !default_file_exists {
        print_help(None, Some(default_service_file));
        exit(1);
    }

    // parse the args
    let mut command_set: argust::ArgContext = argust::parse_args(args.iter(), None);

    // try to get file name from -f, and default to default_service_file if -f not given or empty
    let service_file_path = command_set
        .long_params
        .get("hisho:file")
        .unwrap_or(&None)
        .clone()
        .unwrap_or(default_service_file.to_string());

    let service_file = files::resolve_path(service_file_path.clone()).unwrap_or_else(|e| {
        log::error(format!(
            "Could not find service file '{}': {:?}",
            service_file_path,
            e.to_string()
        ));
        exit(2);
    });

    let workdir = service_file.parent().unwrap_or_else(|| {
        log::error(format!(
            "Could not resolve parent directory of service file '{}'",
            service_file_path,
        ));
        exit(2);
    });

    let data_from_file = fs::read_to_string(service_file.clone());
    if let Err(e) = data_from_file {
        log::error(format!(
            "Could not read service file '{}': {:?}",
            service_file.to_str().unwrap(),
            e.to_string()
        ));
        exit(2);
    }
    let project_data: SpannedResult<Project> = ron::from_str(data_from_file.unwrap().as_str());
    if let Err(e) = project_data {
        log::error(format!(
            "Could not parse service file '{}': {:?}",
            service_file.to_str().unwrap(),
            e.to_string()
        ));
        exit(2);
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
        exit(1);
    }

    let mut command_found = false;

    let mut vars = TemplateVariables::new();
    vars.insert("git", git::fetch_repo_vars(&workdir));

    // if a command was given, try to match it to the config defined
    if let Some(command) = args.first() {
        for cmd in &project.commands {
            if cmd.name == *command {
                command_found = true;
                // try to fetch an environment
                let env = fetch_environment(
                    cmd.environment.clone().as_str(),
                    &project.environments,
                    workdir,
                )
                .unwrap_or(Environment::new_empty());
                vars.insert("env", env.values);

                // make sure required containers are running
                if !containers::ensure_running(&project.containers, &vars).await {
                    return;
                }

                // make sure required builds have run successfully
                if !build_tool::ensure_build(cmd, &project.build, &vars) {
                    return;
                }

                // if there is no shell defined, do nothing and return
                if cmd.shell.is_empty() {
                    log::print("No shell, nothing to do. Exiting..".to_string());
                    return;
                }

                // collect all args removing the command name
                let given_args = args.iter().skip(1).cloned().collect::<Vec<String>>();

                let mut rendered_commands: Vec<Process> = Vec::new();
                // collect the --long-param=value pairs
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
                        template::render_process_with_argv(shell_cmd, vars.as_value(), &given_args)
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

    if !command_found {
        print_help(Some(&project), None);
        exit(1);
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
