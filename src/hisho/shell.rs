// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::io;
use std::process::ExitStatus;

use crate::hisho::config_models::Process;
use crate::hisho::log;

pub fn exec(process: &Process, env: Option<&HashMap<String, String>>) -> io::Result<ExitStatus> {
    // execute the command in /bin/sh
    let mut proc_command = std::process::Command::new(process.command.clone());
    proc_command.args(process.args.clone());

    if env.is_some() {
        proc_command.envs(env.unwrap().clone());
    }

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
