// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{env, fs};

use crate::config::fetch_environment;
use crate::config_models::{Environment, Process, Service};

mod config_models;
mod config;
mod shell;
mod template;
mod containers;
mod build;

#[tokio::main]
async fn main() {
    let version = "0.1.0";
    println!("Service Helper v{} by Thomas Obernosterer", version);
    let data_from_file = fs::read_to_string("service.ron").unwrap_or_else(|e| {
        panic!("Could not read service ron file: {:?}", e);
    });
    let service_data: Service = ron::from_str(data_from_file.as_str()).unwrap_or_else(|e| {
        panic!("Could not parse service ron file: {:?}", e);
    });

    // remove the program name from the arguments
    let args: Vec<String> = env::args().skip(1).collect();

    // if no arguments have been given
    if args.is_empty() {
        println!("Usage: {} <command> [args]\n", env::args().take(1).collect::<Vec<String>>().join(" "));
        println!("Custom Commands:\n{}", service_data.commands.iter().map(|c| format!("- {}", c.name.clone())).collect::<Vec<String>>().join("\n"));
        return;
    }

    // make sure required containers are running
    containers::ensure_running(&service_data.containers).await;

    // parse the args
    let command_set: argust::ArgContext = argust::parse_args(args.iter(), None);

    // if a command was given, try to match it to the config defined
    if let Some(command) = command_set.args.first() {
        for cmd in &service_data.commands {
            if cmd.name == *command {
                // try to fetch an environment
                let env = fetch_environment(cmd.environment.clone().as_str(), &service_data.environments).unwrap_or(Environment::new_empty());

                // make sure required builds have run successfully
                if !build::ensure_build(&cmd, &service_data.build, &env) {
                    return;
                }

                if cmd.capture_all {
                    // Construct the command to be executed
                    let given_args = env::args().skip(2).collect::<Vec<String>>();

                    for shell_cmd in &cmd.shell {
                        let _ = shell::exec(&Process::new(shell_cmd.command.clone(), given_args.clone()), &env);
                    }
                } else {
                    let mut rendered_commands: Vec<Process> = Vec::new();
                    let argument_lookup = command_set.long_params.iter().map(|(key, value)| {
                        if value.is_some() {
                            (key.clone(), value.clone().unwrap())
                        } else {
                            (key.clone(), String::new())
                        }
                    }).collect();
                    for shell_cmd in &cmd.shell {
                        if let Some(rendered_command) = template::render_process(shell_cmd, &argument_lookup) {
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
}
