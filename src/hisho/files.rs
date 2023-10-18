use std::path::PathBuf;
use std::{fs, io};

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

fn get_home_dir() -> Option<String> {
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
