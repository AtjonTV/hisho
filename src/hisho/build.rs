// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::hisho::config_models::{BuildStep, BuildSteps, Command, Process};
use crate::hisho::template::TemplateVariables;
use crate::hisho::{log, shell, template};
use std::collections::HashMap;

pub fn ensure_build(cmd: &Command, build_steps: &BuildSteps, vars: &TemplateVariables) -> bool {
    if !cmd.depends_on_build.is_empty() {
        log::print("Checking Build dependencies ..".to_string());

        let build_steps = get_build_steps(&cmd.depends_on_build, build_steps, &vars);
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

pub fn get_build_steps(
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

fn resolve_files_from_globs(globs: &Vec<String>) -> Vec<String> {
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
