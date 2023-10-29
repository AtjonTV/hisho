// This file 'main.rs' is part of the 'hisho' project.
//
// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use clap::{Arg, ArgAction, Command};
use hisho_core::arg_parse;
use hisho_core::build_tool;
use hisho_core::config_models::{Environment, Process, Project};
use hisho_core::containers;
use hisho_core::environment::fetch_environment;
use hisho_core::files;
use hisho_core::git;
use hisho_core::log;
use hisho_core::shell;
use hisho_core::template;
use hisho_core::template::TemplateVariables;
use ron::error::SpannedResult;
use std::process::exit;
use std::{env, fs};

#[tokio::main]
async fn main() {
    let version = env!("CARGO_PKG_VERSION");
    log::print(format!(
        "Hisho v{} (hisho_cli2) by Thomas Obernosterer",
        version
    ));
    let default_project_file = "hisho.ron";

    let matches = Command::new("hisho")
        .about(
            "Hisho CLI is a tool for local development with dependencies built using Hisho Core.",
        )
        .version(version)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Thomas Obernosterer")
        .arg(
            Arg::new("project-file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .default_value(default_project_file)
                .help("Path to the Hisho project file"),
        )
        .subcommand(
            Command::new("run")
                .alias("r")
                .about("Run a command")
                .arg(
                    Arg::new("command")
                        .help("The command to run")
                        .action(ArgAction::Set)
                        .allow_hyphen_values(true)
                        .num_args(1..),
                )
                .arg_required_else_help(true),
        )
        .get_matches();

    let project_file_path = matches.get_one::<String>("project-file").unwrap();

    let project_file = files::resolve_path(project_file_path.clone()).unwrap_or_else(|e| {
        log::error(format!(
            "Could not find service file '{}': {:?}",
            project_file_path,
            e.to_string()
        ));
        exit(2);
    });
    let workdir = project_file.parent().unwrap_or_else(|| {
        log::error(format!(
            "Could not resolve parent directory of service file '{}'",
            project_file_path,
        ));
        exit(2);
    });

    let data_from_file = fs::read_to_string(project_file_path.clone());
    if let Err(e) = data_from_file {
        log::error(format!(
            "Could not read service file '{}': {:?}",
            project_file_path,
            e.to_string()
        ));
        exit(2);
    }
    let project_data: SpannedResult<Project> = ron::from_str(data_from_file.unwrap().as_str());
    if let Err(e) = project_data {
        log::error(format!(
            "Could not parse service file '{}': {:?}",
            project_file_path,
            e.to_string()
        ));
        exit(2);
    }
    let project = project_data.unwrap();

    let mut vars = TemplateVariables::new();
    vars.insert("git", git::fetch_repo_vars(workdir));

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            // Collect all the arguments for this subcommand
            let args = run_matches
                .get_many::<String>("command")
                .unwrap()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            // take the first argument as command name
            let command_name = args.first().unwrap();

            // parse options from the arguments for arg template variables
            let command_options = arg_parse::parse(args.clone());

            let mut command_found = false;
            for cmd in &project.commands {
                if cmd.name == *command_name {
                    command_found = true;
                    // try to fetch an environment
                    let env = fetch_environment(
                        cmd.environment.clone().as_str(),
                        &project.environments,
                        workdir,
                    )
                    .unwrap_or(Environment::new_empty());
                    vars.insert("env", env.values);

                    // make sure required containers are running
                    if !containers::ensure_running(&project.containers, &vars).await {
                        return;
                    }

                    // make sure required builds have run successfully
                    if !build_tool::ensure_build(cmd, &project.build, &vars) {
                        return;
                    }

                    // if there is no shell defined, do nothing and return
                    if cmd.shell.is_empty() {
                        log::print("No shell, nothing to do. Exiting..".to_string());
                        return;
                    }

                    let mut rendered_commands: Vec<Process> = Vec::new();
                    vars.insert("arg", command_options.clone());
                    for shell_cmd in &cmd.shell {
                        if let Some(rendered_command) =
                            template::render_process_with_argv(shell_cmd, vars.as_value(), &args)
                        {
                            rendered_commands.push(rendered_command);
                        }
                    }

                    for rendered_command in &rendered_commands {
                        let _ = shell::exec(rendered_command, vars.get("env"));
                    }
                    break;
                }
            }
            if !command_found {
                log::error(format!("Could not find command '{}'", command_name));
                exit(2);
            }
        }
        _ => unreachable!(),
    }
}
