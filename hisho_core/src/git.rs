// This file 'git.rs' is part of the 'hisho' project.
//
// Copyright 2023-2024 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use gix::date::time::format::ISO8601_STRICT;
use std::collections::HashMap;
use std::path::Path;

/// Fetch git repository metadata for a given path
///
/// If the given path is not a git repository, all the keys will be empty strings.
///
/// # Arguments
///
/// * `dir` - The path to the git repository
///
/// # Returns
///
/// A hash map containing the following keys:
/// * `commit_sha` - The long hash of the newest commit
/// * `commit_sha_short` - The short hash of the newest commit
/// * `commit_date` - The date of the newest commit
/// * `commit_author_name` - The name of the author of the newest commit
/// * `commit_author_email` - The email of the author of the newest commit
/// * `commit_committer_name` - The name of the committer of the newest commit
/// * `commit_committer_email` - The email of the committer of the newest commit
/// * `branch` - The current branch of the repository
pub fn fetch_repo_vars(dir: &Path) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    // initialize variables as empty to prevent template errors outside of git repositories
    // TODO: Should we not do this so it errors outside of a git repository?
    result.insert("commit_sha".to_string(), String::new());
    result.insert("commit_sha_short".to_string(), String::new());
    result.insert("commit_date".to_string(), String::new());
    result.insert("commit_author_name".to_string(), String::new());
    result.insert("commit_author_email".to_string(), String::new());
    result.insert("commit_committer_name".to_string(), String::new());
    result.insert("commit_committer_email".to_string(), String::new());
    result.insert("branch".to_string(), String::new());
    if let Ok(repo) = gix::discover(dir) {
        if let Ok(head) = repo.head() {
            if head.is_detached() {
                result.insert("branch".to_string(), "none".to_string());
            }
            if let Some(head_id) = head.id() {
                let long_sha = head_id.to_hex();
                result.insert("commit_sha".to_string(), long_sha.to_string());
                if let Ok(short_sha) = head_id.shorten() {
                    result.insert("commit_sha_short".to_string(), short_sha.to_string());
                    result.insert("branch".to_string(), short_sha.to_string());
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
                        if let Ok(commit_committer) = commit.committer() {
                            result.insert(
                                "commit_committer_name".to_string(),
                                commit_committer.name.to_string(),
                            );
                            result.insert(
                                "commit_committer_email".to_string(),
                                commit_committer.email.to_string(),
                            );
                        }
                    }
                }
            }
        }
        if let Ok(Some(head_ref)) = repo.head_name() {
            result.insert("branch".to_string(), head_ref.shorten().to_string());
        }
    }
    result
}
