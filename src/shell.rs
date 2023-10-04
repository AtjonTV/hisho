use std::collections::HashMap;
use std::io;
use std::process::ExitStatus;
use crate::config_models::Environment;

pub fn exec(command: &str, env: &Environment) -> io::Result<ExitStatus> {
    // execute the command in /bin/sh
    let mut proc_command = std::process::Command::new("sh");
    proc_command.args(["-c", command.clone()]);
    proc_command.envs(env.values.clone());

    // Check if the command succeeded
    let proc_result = proc_command.status();
    if let Ok(output) = &proc_result {
        println!("Command '{}' executed. ({})", command, output);
    } else {
        println!("Could not execute command.");
    }
    proc_result
}
