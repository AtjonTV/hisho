// This file 'build_tool.rs' is part of the 'hisho' project.
//
// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use crate::config_models::{BuildStep, BuildSteps, Command, Environment, Process, Project};
use crate::environment::fetch_environment;
use crate::shell;
use crate::template;
use crate::template::TemplateVariables;
use crate::{containers, files, log};

/////// DEPRECATED SECTION BEGIN ///////

#[deprecated(since = "1.1.0-dev.0", note = "Use `run_builds` instead")]
pub fn ensure_build(cmd: &Command, build_steps: &BuildSteps, vars: &TemplateVariables) -> bool {
    run_steps_for_command(cmd, build_steps, vars)
}

#[deprecated(since = "1.1.0-dev.0", note = "Use `run_steps` instead")]
pub fn ensure_steps_are_build(
    steps: &Vec<String>,
    build_steps: &BuildSteps,
    vars: &TemplateVariables,
) -> bool {
    run_steps(steps, build_steps, vars)
}

/////// DEPRECATED SECTION END  ///////

pub async fn run_build(
    project: &Project,
    step: &BuildStep,
    environment: &str,
    default_vars: &TemplateVariables,
) -> bool {
    let mut vars = default_vars.clone();
    let env = fetch_environment(
        environment,
        &project.environments,
        files::string_to_path(&project.workdir).as_path(),
    )
    .unwrap_or(Environment::new_empty());
    vars.insert("env", env.values);

    // make sure required containers are running
    if !containers::start_containers(&project.containers, &vars).await {
        return false;
    }

    let steps: Vec<String> = vec![step.name.clone()];

    // make sure required builds have run successfully
    if !run_steps(&steps, &project.build, &vars) {
        return false;
    }

    true
}

/// Ensure that all build steps have been run successfully
///
/// 1. First all build steps that are required for the given command are collected from the given vector
/// of build steps.
/// 2. Then all of the found build steps are executed in sequence with the command outputs being printed
/// to the standard output and standard error, as if the commands where executed manually.
/// 3. Only if all the build steps executed with exist status 0, true is returned, otherwise false.
///
/// # Arguments
///
/// * `cmd` - The Command for which the build steps should be executed
/// * `build_steps` - The list of build steps to consider. Only the steps required by the command are executed
/// * `vars` - Variables for the template engine and for the execution environment variables
///
/// # Returns
///
/// * `true` if all existing build steps for cmd executed successfully
/// * `false` otherwise
///
pub fn run_steps_for_command(
    cmd: &Command,
    build_steps: &BuildSteps,
    vars: &TemplateVariables,
) -> bool {
    run_steps(&cmd.depends_on_build, build_steps, vars)
}

/// Ensure that all build steps have been run successfully
///
/// 1. First all build steps that are required
/// 2. Then all of the found build steps are executed in sequence with the command outputs being printed
/// to the standard output and standard error, as if the commands where executed manually.
/// 3. Only if all the build steps executed with exist status 0, true is returned, otherwise false.
///
/// # Arguments
///
/// * `steps` - The list of build steps by name that should be executed
/// * `build_steps` - The list of build steps to consider.
/// * `vars` - Variables for the template engine and for the execution environment variables
///
/// # Returns
///
/// * `true` if all existing build steps for cmd executed successfully
/// * `false` otherwise
pub fn run_steps(steps: &Vec<String>, build_steps: &BuildSteps, vars: &TemplateVariables) -> bool {
    if !steps.is_empty() {
        log::print("Checking Build dependencies ..".to_string());

        let build_steps = get_build_steps(steps, build_steps, vars);
        for (step_name, shell) in build_steps {
            for proc in shell {
                log::print(format!("\tRunning build step: {}", step_name));
                let result = shell::exec(&proc, vars.get("env"));
                if result.is_err() {
                    log::print("\tFailed to run Build Step!".to_string());
                    return false;
                } else if !result.unwrap().success() {
                    log::error("\tBuild Step returned non-zero exit code!".to_string());
                    return false;
                }
            }
        }
        log::print(String::new());
    }
    true
}

/// Resolve a list of globs into a list of file paths
///
/// The list of globs is resolved using the globs crate.
///
/// # Arguments
///
/// * `globs` - The list of globs to resolve
pub fn resolve_files_from_globs(globs: &Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    for glob_str in globs {
        let matches = glob::glob(glob_str);
        if let Ok(paths) = matches {
            for path in paths.flatten() {
                if let Some(path_str) = path.as_path().to_str() {
                    results.push(path_str.to_string());
                }
            }
        }
    }
    results
}

fn get_build_steps(
    wanted_steps: &Vec<String>,
    build_steps: &BuildSteps,
    vars: &TemplateVariables,
) -> Vec<(String, Vec<Process>)> {
    let all_steps = find_build_steps(wanted_steps, build_steps);
    create_shell_from_steps(&all_steps, vars)
}

fn find_build_steps(wanted_steps: &Vec<String>, build_steps: &BuildSteps) -> BuildSteps {
    let mut steps: BuildSteps = Vec::new();

    for wanted_step in wanted_steps {
        for step in build_steps {
            if step.name.eq(wanted_step) {
                // Skip if the step is already in steps
                if steps.contains(step) {
                    continue;
                }
                steps.push(step.clone());

                // Create a new build_steps, without the current step
                let new_build_steps = build_steps
                    .iter()
                    .filter(|s| !s.name.eq(wanted_step))
                    .cloned()
                    .collect::<BuildSteps>();

                if !step.depends_on.is_empty() {
                    let parent_steps = find_build_steps(&step.depends_on, &new_build_steps);
                    for parent_step in parent_steps {
                        // Skip if the step is already in steps
                        if steps.contains(&parent_step) {
                            continue;
                        }
                        steps.push(parent_step.clone());
                    }
                }
            }
        }
    }

    steps.reverse();
    steps
}

fn create_shell_from_steps(
    steps: &BuildSteps,
    vars: &TemplateVariables,
) -> Vec<(String, Vec<Process>)> {
    let mut shell: Vec<(String, Vec<Process>)> = Vec::new();
    for step in steps {
        let procs = create_shell_from_step(step, vars);
        shell.push((step.name.clone(), procs));
    }
    shell
}

fn create_shell_from_step(step: &BuildStep, vars: &TemplateVariables) -> Vec<Process> {
    let mut template_vars = vars.clone();
    if !step.input_files.is_empty() {
        template_vars.insert("build", create_build_vars(step));
    }
    step.shell
        .iter()
        .map(|proc| template::render_process(proc, template_vars.as_value()))
        .filter(|opt_proc| opt_proc.is_some())
        .map(|opt_proc| opt_proc.unwrap())
        .collect::<Vec<Process>>()
}

fn create_build_vars(step: &BuildStep) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    result.insert(
        "input_files".to_string(),
        resolve_files_from_globs(&step.input_files).join(" "),
    );
    result.insert("name".to_string(), step.name.clone());
    result
}
