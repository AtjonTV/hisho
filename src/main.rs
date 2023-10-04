use std::{env, fs};
use crate::config_models::{Environment, Service};
use crate::config::fetch_environment;

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
        println!("service-helper v{}", version);
        println!("Usage: service-helper <command> [args]\n");
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

                if !cmd.depends_on_build.is_empty() {
                    println!("Service: Checking Build dependencies ..");
                    let build_steps = build::get_build_steps(&cmd.depends_on_build, &service_data.build, &env);
                    for step in build_steps {
                        println!("\tRunning build step: {}", step.0);
                        let result = shell::exec(&step.1, &env);
                        if result.is_err() {
                            println!("\tFailed to run Build Step. Exiting ..");
                            return;
                        } else {
                            if !result.unwrap().success() {
                                println!("\tBuild Step returned non-zero exit code. Exiting ..");
                                return;
                            }
                        }
                    }
                    println!();
                }

                if cmd.capture_all {
                    // Construct the command to be executed
                    let given_args = env::args().skip(2).collect::<Vec<String>>().join(" ");

                    for shell_cmd in &cmd.shell {
                        let final_command = format!("{} {}", shell_cmd, given_args);
                        let _ = shell::exec(final_command.as_str(), &env);
                    }
                } else {
                    let mut rendered_commands: Vec<String> = Vec::new();
                    let argument_lookup = command_set.long_params.iter().map(|(key, value)| {
                        if value.is_some() {
                            (key.clone(), value.clone().unwrap())
                        } else {
                            (key.clone(), String::new())
                        }
                    }).collect();
                    for shell_cmd in &cmd.shell {
                        if let Some(rendered_command) = template::render_string(shell_cmd.clone(), &argument_lookup) {
                            rendered_commands.push(rendered_command);
                        }
                    }

                    for rendered_command in rendered_commands {
                        let _ = shell::exec(rendered_command.as_str(), &env);
                    }
                }
            }
        }
    }
}
