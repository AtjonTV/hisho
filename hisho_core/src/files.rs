// This file 'files.rs' is part of the 'hisho' project.
//
// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::PathBuf;
use std::{fs, io};

/// Try to resolve a path using home resolution and canonicalization.
///
/// If `~` is present anywhere in the input path, we try to resolve it to the current users home directory.
/// For this resolution we use the `get_home_dir()` function
pub fn resolve_path(path: String) -> Result<PathBuf, io::Error> {
    let mut path_str = String::from(path.clone());
    if let Some(home_dir) = get_home_dir() {
        path_str = path_str.replace("~", home_dir.as_str());
    }
    let path_buf_res = fs::canonicalize(path_str);
    if let Ok(path_buf) = path_buf_res {
        Ok(path_buf)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not find service file",
        ))
    }
}

/// Try to resolve the home directory of the current user.
///
/// We try to read the environment variable depending on the complication platform:
/// * For Windows we try to read the `USERPROFILE` environment variable.
/// * For Unix we try to read the `HOME` environment variable.
pub fn get_home_dir() -> Option<String> {
    if cfg!(windows) {
        if let Ok(home_dir) = std::env::var("USERPROFILE") {
            return Some(home_dir);
        }
    } else if cfg!(unix) {
        if let Ok(home_dir) = std::env::var("HOME") {
            return Some(home_dir);
        }
    }

    None
}
