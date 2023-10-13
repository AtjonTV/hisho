// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use once_cell::unsync::Lazy;
use owo_colors::{OwoColorize, Style};

pub fn print(text: String) {
    println!("{} {}", get_tag(), text);
}

pub fn error(text: String) {
    let default_style: Lazy<Style> = Lazy::new(|| Style::new().red());
    eprintln!("{} {}", get_tag(), text.style(*default_style));
}

fn get_tag() -> String {
    let bracket_style: Lazy<Style> = Lazy::new(|| Style::new().green());
    let text_style: Lazy<Style> = Lazy::new(|| Style::new().cyan());
    format!(
        "{}{}{} ",
        "[".style(*bracket_style),
        "Hisho".style(*text_style),
        "]".style(*bracket_style)
    )
}
