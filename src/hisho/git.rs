// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use gix::date::time::format::ISO8601_STRICT;
use std::collections::HashMap;
use std::path::Path;

pub fn fetch_repo_vars(path: &str) -> HashMap<String, String> {
    // let path_handle = Path::new(path).parent();
    let mut result: HashMap<String, String> = HashMap::new();

    // initialize variables as empty to prevent template errors outside of git repositories
    // TODO: Should we not do this so it errors outside of a git repository?
    result.insert("commit_sha".to_string(), String::new());
    result.insert("commit_sha_short".to_string(), String::new());
    result.insert("commit_date".to_string(), String::new());
    result.insert("commit_author_name".to_string(), String::new());
    result.insert("commit_author_email".to_string(), String::new());
    // if let Some(dir) = path_handle {
    //     println!("Dir path: {}", dir.to_str().unwrap());
    if let Ok(repo) = gix::discover(Path::new(".")) {
        if let Ok(head) = repo.head() {
            if let Some(head_id) = head.id() {
                let long_sha = head_id.to_hex();
                result.insert("commit_sha".to_string(), long_sha.to_string());
                if let Ok(short_sha) = head_id.shorten() {
                    result.insert("commit_sha_short".to_string(), short_sha.to_string());
                }
                if let Ok(head_obj) = head_id.object() {
                    if let Ok(commit) = head_obj.try_into_commit() {
                        if let Ok(commit_time) = commit.time() {
                            result.insert(
                                "commit_date".to_string(),
                                commit_time.format(ISO8601_STRICT),
                            );
                        }
                        if let Ok(commit_author) = commit.author() {
                            result.insert(
                                "commit_author_name".to_string(),
                                commit_author.name.to_string(),
                            );
                            result.insert(
                                "commit_author_email".to_string(),
                                commit_author.email.to_string(),
                            );
                        }
                    }
                }
            }
        }
    }
    // }
    result
}
