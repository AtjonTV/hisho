use std::io;
use std::process::ExitStatus;
use crate::config_models::{Environment, Process};

pub fn exec(process: &Process, env: &Environment) -> io::Result<ExitStatus> {
    // execute the command in /bin/sh
    let mut proc_command = std::process::Command::new(process.command.clone());
    proc_command.args(process.args.clone());
    proc_command.envs(env.values.clone());

    // Check if the command succeeded
    let proc_result = proc_command.status();
    if let Ok(output) = &proc_result {
        println!("Service: Command '{} {}' executed. ({})", process.command, process.args.join(" "), output);
    } else {
        println!("Service: Could not execute command: {} {}", process.command, process.args.join(" "));
    }
    proc_result
}
