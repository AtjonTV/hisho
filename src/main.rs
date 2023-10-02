use std::{env, fs};
use crate::config_models::{Service};
use crate::config::fetch_environment;

mod config_models;
mod config;
mod shell;

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

                if cmd.capture_all {
                    // Construct the command to be executed
                    let given_args = env::args().skip(2).collect::<Vec<String>>().join(" ");

                    for shell_cmd in &cmd.shell {
                        let final_command = format!("{} {}", shell_cmd, given_args);
                        let _ = shell::exec(final_command.as_str(), &env);
                    }
                }
            }
        }
    }
}
