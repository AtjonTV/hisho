// This file 'command.rs' is part of the 'hisho' project.
//
// Copyright 2023-2024 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::config_models::{Command, Environment, Process, Project};
use crate::environment::fetch_environment;
use crate::template::TemplateVariables;
use crate::{build_tool, containers, files, log, service, shell, template};

const MODULE_NAME: &str = "command";

/////// DEPRECATED SECTION BEGIN ///////
/// Run a command with all its dependencies
#[deprecated(since = "1.2.0-dev.0", note = "Use `run_command5` instead")]
pub async fn run_command(
    project: &Project,
    cmd: &Command,
    default_vars: &TemplateVariables,
    argv: &Vec<String>,
) -> bool {
    run_command5(project, cmd, default_vars, argv, false).await
}
/////// DEPRECATED SECTION END ///////

/// Run a command with all its dependencies
///
/// The function is named "run_command5",
/// due to its arity of 5 and rust not allowing argument overloading.
pub async fn run_command5(
    project: &Project,
    cmd: &Command,
    default_vars: &TemplateVariables,
    argv: &Vec<String>,
    explain_only: bool,
) -> bool {
    let mut vars = default_vars.clone();
    let env = fetch_environment(
        cmd.environment.clone().as_str(),
        &project.environments,
        files::string_to_path(&project.workdir).as_path(),
    )
    .unwrap_or(Environment::new_empty());
    vars.insert("env", env.values);

    if explain_only {
        log::print2(MODULE_NAME, format!("Environment: {}", cmd.environment));
    }

    // make sure required containers are running
    if !containers::start_containers3(&project.containers, &vars, explain_only).await {
        return false;
    }

    // make sure required services are running
    if !service::are_running2(&project.services, explain_only).await {
        return false;
    }

    // make sure required builds have run successfully
    if !build_tool::run_steps_for_command4(cmd, &project.build, &vars, explain_only) {
        return false;
    }

    // if there is no shell defined, do nothing and return
    if cmd.shell.is_empty() {
        log::print2(MODULE_NAME, "No shell, nothing to do.".to_string());
        return true;
    }

    let mut rendered_commands: Vec<Process> = Vec::new();
    for shell_cmd in &cmd.shell {
        if let Some(rendered_command) =
            template::render_process_with_argv(shell_cmd, vars.as_value(), argv)
        {
            rendered_commands.push(rendered_command);
        }
    }

    log::print2(MODULE_NAME, "Executing shell".to_string());
    for rendered_command in &rendered_commands {
        let _ = shell::exec3(rendered_command, vars.get("env"), explain_only);
    }

    true
}
