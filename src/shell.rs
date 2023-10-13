// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;
use std::process::ExitStatus;

use crate::config_models::{Environment, Process};
use crate::log;

pub fn exec(process: &Process, env: &Environment) -> io::Result<ExitStatus> {
    // execute the command in /bin/sh
    let mut proc_command = std::process::Command::new(process.command.clone());
    proc_command.args(process.args.clone());
    proc_command.envs(env.values.clone());

    // Check if the command succeeded
    let proc_result = proc_command.status();
    if let Ok(output) = &proc_result {
        log::print(format!(
            "Command '{} {}' executed. ({})",
            process.command,
            process.args.join(" "),
            output
        ));
    } else {
        log::error(format!(
            "Could not execute command: {} {}",
            process.command,
            process.args.join(" ")
        ));
    }
    proc_result
}
