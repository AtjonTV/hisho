// This file 'log.rs' is part of the 'hisho' project.
//
// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use color_print::cformat;

/// Print to stdout with a colored prefix
pub fn print(text: String) {
    println!("{} {}", get_tag(), text);
}

/// Print in red color to stderr with a colored prefix
pub fn error(text: String) {
    eprintln!("{}", cformat!("{} <red>{}</>", get_tag(), text));
}

fn get_tag() -> String {
    cformat!(
        "<green>{}</><cyan>{}</><green>{}</> ",
        "[",
        "Hisho",
        "]"
    )
}
