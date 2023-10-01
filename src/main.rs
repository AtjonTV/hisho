use std::collections::HashMap;
use std::{env, fs};
use crate::config_models::{Service};
use crate::config::fetch_environment;

mod config_models;
mod config;

fn main() {
    let version = "0.1.0";
    let data_from_file = fs::read_to_string("service.ron").unwrap_or_else(|e| {
        panic!("Could not read service ron file: {:?}", e);
    });
    let service_data: Service = ron::from_str(data_from_file.as_str()).unwrap_or_else(|e| {
        panic!("Could not parse service ron file: {:?}", e);
    });
    // println!("{:?}", service_data);

    // remove the program name from the arguments
    let args: Vec<String> = env::args().skip(1).collect();

    // if no arguments have been given
    if args.is_empty() {
        println!("service-helper v{}", version);
        println!("Usage: service-helper <command> [args]\n");
        println!("Custom Commands:\n{}", service_data.commands.iter().map(|c| format!("- {}", c.name.clone())).collect::<Vec<String>>().join("\n"));
        return;
    }

    // parse the args
    let command_set: argust::ArgContext = argust::parse_args(args.iter(), None);

    // if a command was given, try to match it to the config defined
    if let Some(command) = command_set.args.first() {
        for cmd in &service_data.commands {
            if cmd.name == *command {
                // try to fetch an environment
                let env = fetch_environment(cmd.environment.clone().as_str(), &service_data.environments);

                // Construct the command to be executed
                let cmd_args = if cmd.capture_all {
                    env::args().skip(2).collect::<Vec<String>>().join(" ")
                } else { String::new() };

                // execute the command in /bin/sh
                let mut proc_command = std::process::Command::new("sh");
                proc_command.args(["-c", cmd_args.as_str().clone()]);
                proc_command.envs(if let Some(e) = env { e.values.clone() } else { HashMap::new() });

                // Check if the command succeeded
                let proc_result = proc_command.output();
                if let Ok(output) = proc_result {
                    println!("Command returned: {}", output.status);
                    println!("Command returned: {:?}", String::from_utf8(output.stdout));
                    println!("Command returned: {:?}", String::from_utf8(output.stderr));
                } else {
                    println!("Could not execute command.");
                }
            }
        }
    }
}
