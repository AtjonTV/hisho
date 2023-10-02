use std::collections::HashMap;
use std::io;
use std::process::Output;
use crate::config_models::Environment;

pub fn exec(command: &str, env: &Option<Environment>) -> io::Result<Output> {
    // execute the command in /bin/sh
    let mut proc_command = std::process::Command::new("sh");
    proc_command.args(["-c", command.clone()]);
    proc_command.envs(if let Some(e) = &env { e.values.clone() } else { HashMap::new() });

    // Check if the command succeeded
    let proc_result = proc_command.output();
    if let Ok(output) = &proc_result {
        if let Ok(text) = String::from_utf8(output.stdout.clone()) {
            println!("{}", text);
        }
        if let Ok(text) = String::from_utf8(output.stderr.clone()) {
            if !text.is_empty() {
                println!("{}", text);
            }
        }
        println!("Command '{}' executed. ({})", command, output.status);
    } else {
        println!("Could not execute command.");
    }
    proc_result
}
