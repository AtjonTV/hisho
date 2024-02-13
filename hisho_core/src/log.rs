// This file 'log.rs' is part of the 'hisho' project.
//
// Copyright 2023-2024 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use color_print::cformat;

/// Print to stdout with a colored prefix
pub fn print(text: String) {
    println!("{}", text);
}

pub fn print2(module: &str, text: String) {
    println!("{} {}", get_mod_tag(module), text);
}

/// Print in red color to stderr with a colored prefix
pub fn error(text: String) {
    eprintln!("{}", cformat!("<red>{}</>", text));
}

pub fn error2(module: &str, text: String) {
    eprintln!("{}", cformat!("{} <red>{}</>", get_mod_tag(module), text));
}

pub fn explain(text: String) {
    println!("{} {}", get_explain_tag(), text);
}
pub fn explain2(module: &str, text: String) {
    println!("{} {} {}", get_mod_tag(module), get_explain_tag(), text);
}

fn get_mod_tag(module: &str) -> String {
    cformat!("<cyan>{:<10}</> ", module.to_uppercase(),)
}

fn get_explain_tag() -> String {
    cformat!("<red>{}</> ", "EXPLAIN")
}
