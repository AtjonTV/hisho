use owo_colors::{OwoColorize, Style};
use once_cell::unsync::Lazy;

pub fn print(text: String) {
    println!("{} {}", get_tag(), text);
}

pub fn error(text: String) {
    let default_style: Lazy<Style> = Lazy::new(|| Style::new().red());
    eprintln!("{} {}",  get_tag(), text.style(*default_style));
}

fn get_tag() -> String {
    let bracket_style: Lazy<Style> = Lazy::new(|| Style::new().green());
    let text_style: Lazy<Style> = Lazy::new(|| Style::new().cyan());
    format!("{}{}{} ", "[".style(*bracket_style), "Hisho".style(*text_style), "]".style(*bracket_style))
}
