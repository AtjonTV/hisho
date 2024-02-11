// This file 'main.rs' is part of the 'hisho' project.
//
// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use clap::{Arg, ArgAction, ArgMatches, Command};
use hisho_core::build_tool;
use hisho_core::config_models::Project;
use hisho_core::files;
use hisho_core::git;
use hisho_core::log;
use hisho_core::template::TemplateVariables;
use hisho_core::{arg_parse, command};
use ron::error::SpannedResult;
use std::process::exit;
use std::{env, fs, io};

#[tokio::main]
async fn main() -> io::Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let default_project_file = "hisho.ron";

    let clap_command = Command::new("hisho")
        .about(
            "Hisho CLI is a tool for local development with dependencies built using Hisho Core.",
        )
        .version(version)
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
                .visible_aliases(["r", "cmd"])
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
        .subcommand(
            Command::new("build")
                .visible_aliases(["b", "make"])
                .about("Run a build step")
                .arg(
                    Arg::new("build_step")
                        .help("The build_step to run")
                        .action(ArgAction::Set)
                        .num_args(1..=1),
                )
                .arg(
                    Arg::new("environment")
                        .help("The environment to use")
                        .short('e')
                        .long("env")
                        .default_value("")
                        .action(ArgAction::Set),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("explain")
                .about("Explain what would happen")
                .subcommand(
                    Command::new("run")
                        .visible_aliases(["r", "cmd"])
                        .about("Explain what the command would do")
                        .arg(
                            Arg::new("command")
                                .help("The command to explain")
                                .action(ArgAction::Set)
                                .allow_hyphen_values(true)
                                .num_args(1..),
                        )
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("build")
                        .about("Explain the build step")
                        .visible_aliases(["b", "make"])
                        .arg(
                            Arg::new("build_step")
                                .help("The build_step to explain")
                                .action(ArgAction::Set)
                                .num_args(1..=1),
                        )
                        .arg(
                            Arg::new("environment")
                                .help("The environment to use")
                                .short('e')
                                .long("env")
                                .default_value("")
                                .action(ArgAction::Set),
                        )
                        .arg_required_else_help(true),
                ),
        );
    let matches = clap_command.clone().get_matches();

    log::print(format!(
        "Hisho v{} (hisho_cli2) by Thomas Obernosterer",
        version
    ));

    let project_file_path = matches.get_one::<String>("project-file").unwrap();

    let project_file = files::resolve_path(project_file_path.clone()).unwrap_or_else(|e| {
        log::error(format!(
            "Could not find project file '{}': {:?}",
            project_file_path,
            e.to_string()
        ));
        exit(2);
    });
    let workdir = project_file.parent().unwrap_or_else(|| {
        log::error(format!(
            "Could not resolve parent directory of project file '{}'",
            project_file_path,
        ));
        exit(2);
    });

    let data_from_file = fs::read_to_string(project_file_path.clone());
    if let Err(e) = data_from_file {
        log::error(format!(
            "Could not read project file '{}': {:?}",
            project_file_path,
            e.to_string()
        ));
        exit(2);
    }
    let project_data: SpannedResult<Project> = ron::from_str(data_from_file.unwrap().as_str());
    if let Err(e) = project_data {
        log::error(format!(
            "Could not parse project file '{}': {:?}",
            project_file_path,
            e.to_string()
        ));
        exit(2);
    }
    let mut project_mut = project_data.unwrap();
    project_mut.workdir = workdir
        .to_path_buf()
        .into_os_string()
        .into_string()
        .unwrap();
    let project: Project = project_mut;

    let mut vars = TemplateVariables::new();
    vars.insert("git", git::fetch_repo_vars(workdir));

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            command_run_impl(run_matches, &project, &mut vars, false).await;
        }
        Some(("build", build_matches)) => {
            command_build_impl(build_matches, &project, &mut vars, false).await
        }
        Some(("explain", explain_matches)) => match explain_matches.subcommand() {
            Some(("run", run_matches)) => {
                command_run_impl(run_matches, &project, &mut vars, true).await;
            }
            Some(("build", build_matches)) => {
                command_build_impl(build_matches, &project, &mut vars, true).await
            }
            _ => {
                let help_suffix = build_help_suffix(&project);
                if let Some(sub_command) = clap_command.find_subcommand("explain") {
                    sub_command.clone().after_help(help_suffix).print_help()?;
                }
            }
        },
        _ => {
            let help_suffix = build_help_suffix(&project);
            clap_command.after_help(help_suffix).print_help()?;
        }
    }
    Ok(())
}

async fn command_run_impl(
    matches: &ArgMatches,
    project: &Project,
    vars: &mut TemplateVariables,
    explain_only: bool,
) {
    // Collect all the arguments for this subcommand
    let args = matches
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

            vars.insert("arg", command_options.clone());

            command::run_command5(&project, cmd, &vars, &args, explain_only).await;
            break;
        }
    }
    if !command_found {
        log::error(format!("Could not find command '{}'", command_name));
        exit(2);
    }
}

async fn command_build_impl(
    matches: &ArgMatches,
    project: &Project,
    vars: &TemplateVariables,
    explain_only: bool,
) {
    let build_name = matches.get_one::<String>("build_step").unwrap();
    let environment = matches.get_one::<String>("environment").unwrap();

    let mut build_found = false;
    for step in &project.build {
        if step.name == *build_name {
            build_found = true;

            build_tool::run_build5(&project, step, environment.as_str(), &vars, explain_only).await;
            break;
        }
    }
    if !build_found {
        log::error(format!("Could not find command '{}'", build_name));
        exit(2);
    }
}

fn build_help_suffix(project: &Project) -> String {
    let mut help_suffix = String::new();
    if !project.build.is_empty() {
        let known_builds = project
            .build
            .iter()
            .map(|cmd| format!("  {}", cmd.name.clone()))
            .collect::<Vec<String>>()
            .join("\n");
        help_suffix += &*format!(
            "Build Steps from Project '{}':\n{}\n\n",
            project.name, known_builds
        );
    }
    if !project.commands.is_empty() {
        let known_commands = project
            .commands
            .iter()
            .map(|cmd| format!("  {}", cmd.name.clone()))
            .collect::<Vec<String>>()
            .join("\n");
        help_suffix += &*format!(
            "Commands from Project '{}':\n{}\n\n",
            project.name, known_commands
        );
    }
    help_suffix
}
