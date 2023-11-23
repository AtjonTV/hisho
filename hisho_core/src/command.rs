// This file 'command.rs' is part of the 'hisho' project.
//
// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::config_models::{Command, Environment, Process, Project};
use crate::environment::fetch_environment;
use crate::template::TemplateVariables;
use crate::{build_tool, containers, files, log, service, shell, template};

/// Run a command with all its dependencies
pub async fn run_command(
    project: &Project,
    cmd: &Command,
    default_vars: &TemplateVariables,
    argv: &Vec<String>,
) -> bool {
    let mut vars = default_vars.clone();
    let env = fetch_environment(
        cmd.environment.clone().as_str(),
        &project.environments,
        files::string_to_path(&project.workdir).as_path(),
    )
    .unwrap_or(Environment::new_empty());
    vars.insert("env", env.values);

    // make sure required containers are running
    if !containers::start_containers(&project.containers, &vars).await {
        return false;
    }

    // make sure required services are running
    if !service::are_running(&project.services).await {
        return false;
    }

    // make sure required builds have run successfully
    if !build_tool::run_steps_for_command(cmd, &project.build, &vars) {
        return false;
    }

    // if there is no shell defined, do nothing and return
    if cmd.shell.is_empty() {
        log::print("No shell, nothing to do.".to_string());
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

    for rendered_command in &rendered_commands {
        let _ = shell::exec(rendered_command, vars.get("env"));
    }

    true
}
