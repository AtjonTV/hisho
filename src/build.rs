// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::config_models::{BuildStep, Environment, Process};
use crate::template;

pub fn get_build_steps(wanted_steps: &Vec<String>, build_steps: &Vec<BuildStep>, env: &Environment) -> Vec<(String, Process)> {
    let all_steps = find_build_steps(wanted_steps, build_steps);
    create_shell_from_steps(&all_steps, env)
}

fn find_build_steps(wanted_steps: &Vec<String>, build_steps: &Vec<BuildStep>) -> Vec<BuildStep> {
    let mut steps: Vec<BuildStep> = Vec::new();

    for wanted_step in wanted_steps {
        for step in build_steps {
            if step.name.eq(wanted_step) {
                steps.push(step.clone());

                if !step.depends_on.is_empty() {
                    let parent_steps = find_build_steps(&step.depends_on, build_steps);
                    for parent_step in parent_steps {
                        steps.push(parent_step.clone());
                    }
                }
            }
        }
    }

    steps.reverse();
    steps
}

fn create_shell_from_steps(steps: &Vec<BuildStep>, env: &Environment) -> Vec<(String, Process)> {
    let mut shell: Vec<(String, Process)> = Vec::new();
    for step in steps {
        if let Some(shell_cmd) = create_shell_from_step(step, env) {
            shell.push((step.name.clone(), shell_cmd));
        }
    }
    shell
}

fn create_shell_from_step(step: &BuildStep, env: &Environment) -> Option<Process> {
    template::render_process(&step.shell, &env.values)
    // if let Some(rendered_command) = template::render_string(step.shell.clone(), &env.values) {
    //     Some(rendered_command)
    // } else {
    //     None
    // }
}