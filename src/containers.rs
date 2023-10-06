// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dockworker::container::ContainerFilters;
use dockworker::Docker;

use crate::config_models::Containers;

pub async fn ensure_running(containers: &Containers) {
    if !containers.is_empty() {
        println!("Service: Checking Container dependencies ..");
        let docker_con = Docker::connect_with_defaults();
        if let Ok(docker) = docker_con {
            let mut filters = ContainerFilters::new();
            containers.iter().for_each(|c| {
                if c.required {
                    filters.name(c.name.as_str().clone());
                }
            });
            let found_containers = docker.list_containers(Some(true), None, None, filters).await;
            if let Ok(containers) = found_containers {
                for container in containers {
                    println!("\tContainer {:?} is {}", container.Names, container.State);
                    if container.State != "running" {
                        if let Err(e) = docker.start_container(container.Id.as_str()).await {
                            println!("\tCould not start container {:?}: {:?}", container.Names, e);
                        } else {
                            println!("\tStarted container {:?}", container.Names);
                        }
                    }
                }
            }
        }
        println!();
    }
}
