// This file 'arg_parse.rs' is part of the 'hisho' project.
//
// Copyright 2023-2024 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

/// Try to parse a list of string into a map
///
/// This is a simple option parser for command line arguments.
/// Only dash and double-dash prefixed keys are supported.
/// If a key is followed by a value, that value will be used.
/// If a key is followed by another key, the value of the first key will be empty.
///
/// Disclaimer: Authored by ChatGPT and modified by Thomas Obernosterer.
pub fn parse(args: Vec<String>) -> HashMap<String, String> {
    let mut parsed_args: HashMap<String, String> = HashMap::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];

        if arg.starts_with("--") {
            let parts: Vec<&str> = arg.splitn(2, |c| c == '=').collect();
            let (key, value) = if parts.len() > 1 {
                (parts[0][2..].to_string(), parts[1].to_string())
            } else if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                let next_arg = args[i + 1].to_string();
                i += 1;
                (arg.strip_prefix("--").unwrap().to_string(), next_arg)
            } else {
                (arg.strip_prefix("--").unwrap().to_string(), "".to_string())
            };
            parsed_args.insert(key, value);
        } else if arg.starts_with('-') {
            let (key, value) = if arg.contains('=') {
                let parts: Vec<&str> = arg.splitn(2, '=').collect();
                (arg[1..2].to_string(), parts[1].to_string())
            } else if arg.len() > 2 {
                (arg[1..2].to_string(), arg[3..].to_string())
            } else if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                let next_arg = args[i + 1].to_string();
                i += 1;
                (arg[1..2].to_string(), next_arg)
            } else {
                (arg[1..2].to_string(), "".to_string())
            };
            parsed_args.insert(key, value);
        }

        i += 1;
    }

    parsed_args
}

#[cfg(test)]
mod test {
    #[test]
    fn parsing_of_options_and_flags() {
        let args = vec![
            "--key1=value1".to_string(),
            "-k=value2".to_string(),
            "--key2".to_string(),
            "value3".to_string(),
            "-a".to_string(),
            "-b=value4".to_string(),
        ];

        let parsed = super::parse(args);

        assert_eq!(parsed.len(), 5);
        assert_eq!(parsed.get("key1"), Some(&"value1".to_string()));
        assert_eq!(parsed.get("k"), Some(&"value2".to_string()));
        assert_eq!(parsed.get("key2"), Some(&"value3".to_string()));
        assert_eq!(parsed.get("a"), Some(&"".to_string()));
        assert_eq!(parsed.get("b"), Some(&"value4".to_string()));
    }
}
