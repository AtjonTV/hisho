// This file 'service.rs' is part of the 'hisho' project.
//
// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::net::{Shutdown, TcpStream};

use crate::config_models::{Service, ServiceProtocol, Services};
use crate::log;

const MODULE_NAME: &str = "service";

/////// DEPRECATED SECTION BEGIN ///////
#[deprecated(since = "1.2.0-dev.0", note = "Use `are_running2` instead")]
pub async fn are_running(services: &Services) -> bool {
    are_running2(services, false).await
}
/////// DEPRECATED SECTION END ///////

/// Check that all services are running
pub async fn are_running2(services: &Services, explain_only: bool) -> bool {
    if !services.is_empty() {
        log::print2(MODULE_NAME, "Checking Services ...".to_string());
        for service in services {
            if !is_running(service).await {
                // log::error2(
                //     MODULE_NAME,
                //     format!("Service '{}' is not running.", service.name),
                // );
                return explain_only;
            } else {
                log::print2(
                    MODULE_NAME,
                    format!("Service '{}' is running.", service.name),
                );
            }
        }
    }
    true
}

async fn is_running(service: &Service) -> bool {
    return match service.protocol {
        ServiceProtocol::HTTP => match reqwest::get(service.uri.as_str()).await {
            Ok(response) => response.status().is_success(),
            Err(e) => {
                log::error2(
                    MODULE_NAME,
                    format!(
                        "Service '{}' is not reachable: {}",
                        service.name,
                        get_reason(&e)
                    ),
                );
                false
            }
        },
        ServiceProtocol::TCP => match TcpStream::connect(service.uri.as_str()) {
            Ok(stream) => {
                let _ = stream.shutdown(Shutdown::Both);
                true
            }
            Err(e) => {
                log::error2(
                    MODULE_NAME,
                    format!("Service '{}' is not reachable: {}", service.name, e),
                );
                false
            }
        },
    };
}

fn get_reason(error: &reqwest::Error) -> String {
    if error.is_connect() {
        return "connection refused".to_string();
    }
    if error.is_timeout() {
        return "connection timeout".to_string();
    }
    if error.is_request() {
        return "request failed".to_string();
    }

    error.to_string().clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config_models::{Service, ServiceProtocol};

    #[tokio::test]
    async fn cloudflare_is_running() {
        let test_service = Service {
            name: "cloudflare".to_string(),
            protocol: ServiceProtocol::HTTP,
            uri: "https://cloudflare.com".to_string(),
        };
        assert!(is_running(&test_service).await);
    }

    #[tokio::test]
    async fn cloudflare_tcp_ping() {
        let test_service = Service {
            name: "cloudflare tcp".to_string(),
            protocol: ServiceProtocol::TCP,
            uri: "cloudflare.com:80".to_string(),
        };
        assert!(is_running(&test_service).await);
    }

    #[tokio::test]
    async fn ip_172_32_137_254_port_31330_is_offline() {
        let test_service = Service {
            name: "172.32.137.254:31330".to_string(),
            protocol: ServiceProtocol::HTTP,
            uri: "http://172.32.137.254:31330/status".to_string(),
        };
        assert_eq!(is_running(&test_service).await, false);
    }

    #[tokio::test]
    async fn ip_172_32_137_254_port_31330_wont_tcp_ping() {
        let test_service = Service {
            name: "172.32.137.254:31330".to_string(),
            protocol: ServiceProtocol::TCP,
            uri: "172.32.137.254:31330".to_string(),
        };
        assert_eq!(is_running(&test_service).await, false);
    }
}
