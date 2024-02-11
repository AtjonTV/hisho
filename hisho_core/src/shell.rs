// This file 'shell.rs' is part of the 'hisho' project.
//
// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::io;
use std::process::ExitStatus;

use crate::config_models::Process;
use crate::log;

const MODULE_NAME: &str = "shell";

/////// DEPRECATED SECTION BEGIN ///////
#[deprecated(since = "1.2.0-dev.0", note = "Use `exec3` instead")]
pub fn exec(process: &Process, env: Option<&HashMap<String, String>>) -> io::Result<ExitStatus> {
    exec3(process, env, false)
}
/////// DEPRECATED SECTION END ///////

/// Execute a process with the given environment and return the exit status
pub fn exec3(
    process: &Process,
    env: Option<&HashMap<String, String>>,
    explain_only: bool,
) -> io::Result<ExitStatus> {
    // execute the command in /bin/sh
    let mut proc_command = std::process::Command::new(process.command.clone());
    proc_command.args(process.args.clone());

    if !process.cwd.is_empty() {
        proc_command.current_dir(process.cwd.clone());
    }

    if let Some(env) = env {
        proc_command.envs(env.clone());
    }

    let log_in_directory = if !process.cwd.is_empty() {
        format!(" in directory '{}'", process.cwd)
    } else {
        String::new()
    };

    if explain_only {
        log::explain2(
            MODULE_NAME,
            format!(
                "Command '{}' {:?} execution{}.",
                process.command, process.args, log_in_directory
            ),
        );
        return Ok(ExitStatus::default());
    }

    // Check if the command succeeded
    let proc_result = proc_command.status();
    if let Ok(output) = &proc_result {
        log::print2(
            MODULE_NAME,
            format!(
                "Command '{}' {:?} executed{}. ({})",
                process.command, process.args, log_in_directory, output
            ),
        );
    } else {
        log::error2(
            MODULE_NAME,
            format!(
                "Could not execute command '{}' {:?}{}: {:?}",
                process.command, process.args, log_in_directory, proc_result
            ),
        );
    }
    proc_result
}
