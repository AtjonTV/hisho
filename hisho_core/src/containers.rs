// This file 'containers.rs' is part of the 'hisho' project.
//
// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dockworker::container::ContainerFilters;
use dockworker::Docker;
use std::collections::HashSet;

use crate::config_models::Containers;
use crate::log;
use crate::template;
use crate::template::TemplateVariables;

#[deprecated(since = "1.1.0-dev.0", note = "Use `start_containers` instead")]
pub async fn ensure_running(containers: &Containers, vars: &TemplateVariables) -> bool {
    start_containers(containers, vars).await
}

/// Try to start the given containers if the exist and are stopped.
pub async fn start_containers(containers: &Containers, vars: &TemplateVariables) -> bool {
    if !containers.is_empty() {
        log::print("Checking Container dependencies ..".to_string());
        let docker_con = Docker::connect_with_defaults();
        if let Ok(docker) = docker_con {
            let mut required_containers: HashSet<String> = HashSet::new();
            let mut filters = ContainerFilters::new();
            for c in containers.iter() {
                if !c.name.is_empty() {
                    if let Some(name) = template::render_string(c.name.clone(), &vars.as_value()) {
                        required_containers.insert(name.clone());
                        filters.name(name.as_str());
                    } else {
                        log::error(format!("\tFailed to render container name: {}", c.name));
                        return false;
                    }
                }
            }
            let found_containers = docker
                .list_containers(Some(true), None, None, filters)
                .await;
            if let Ok(containers) = found_containers {
                // find all containers by name that are missing from required_containers list
                let mut missing_containers: HashSet<String> = required_containers.clone();
                for container in &containers {
                    for name in &container.Names {
                        missing_containers.remove(&clean_container_name(name));
                    }
                }
                if !missing_containers.is_empty() {
                    log::error(format!(
                        "\tMissing containers: {}",
                        missing_containers
                            .iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    ));
                    return false;
                }
                for container in containers {
                    log::print(format!(
                        "\tContainer {:?} is {}",
                        container.Names, container.State
                    ));
                    if container.State != "running" {
                        if let Err(e) = docker.start_container(container.Id.as_str()).await {
                            log::error(format!(
                                "\tCould not start container {:?}: {:?}",
                                container.Names, e
                            ));
                            return false;
                        } else {
                            log::print(format!("\tStarted container {:?}", container.Names));
                        }
                    }
                }
            } else {
                log::error("\tCannot find required containers".to_string());
                return false;
            }
        } else {
            log::error("Could not connect to docker daemon".to_string());
            return false;
        }
        log::print(String::new());
    }
    true
}

fn clean_container_name(name: &String) -> String {
    if name.starts_with('/') {
        name.trim_start_matches('/').to_string()
    } else {
        name.to_string()
    }
}
