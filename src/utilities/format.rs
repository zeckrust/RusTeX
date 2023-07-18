use super::def_syntax::*;
use crate::components::document::*;
use std::io::{Write, Error};
use regex::*;


const BOLD_REGEX: &str = r"\*{2}(?P<text>[^\*]+?)\*{2}";
const ITALIC_REGEX: &str = r"_(?P<text>[^_]+?)_";
const COLOR_REGEX: &str = r"#(?P<color>[^{} ]+)\{(?P<text>[^{}]+)\}";

pub fn into_braces(string: &String) -> String {
    let mut str_with_braces: String = String::from("{");
    str_with_braces = format!("{}{}", str_with_braces, string);
    format!("{}{}", str_with_braces, "}")
}

pub fn into_brackets(string: &String) -> String {
    let mut str_with_brackets: String = String::from("[");
    str_with_brackets = format!("{}{}", str_with_brackets, string);
    format!("{}{}", str_with_brackets, "]")
}

fn into_bold(string: &String) -> String {
    format!("{}{}", DEF_BOLD, into_braces(string))
}

fn into_italic(string: &String) -> String {
    format!("{}{}", DEF_ITALIC, into_braces(string) )
}

fn into_color(string: &String, color: &String) -> String {
    let formatted_string = format!("{}{}{}", DEF_COLOR, into_braces(color), into_braces(string));
    into_braces(&formatted_string)
}

pub fn indent_line(doc: &Document, num_tabs: &usize) -> Result<(), Error> {
    let tabs_str: String = "\t".repeat(*num_tabs);
    write!(doc.get_file(), "{}", tabs_str)
}

pub fn write_indented_line(doc: &Document, num_tabs: &usize, text: &str) -> Result<(), Error> {
    indent_line(&doc, num_tabs)?;
    writeln!(doc.get_file(), "{}", text)
}

enum FormatType {
    Bold,
    Italic,
    Color
}

impl FormatType {
    fn handle_formatting(&self, text: &String, color: Option<&String>) -> String {
        match self {
            FormatType::Bold => into_bold(text),
            FormatType::Italic => into_italic(text),
            FormatType::Color => into_color(text, color.unwrap())
        }
    }
}

pub fn format_text(text: String) -> String {
    handle_color(handle_italic(handle_bold(text)))
}

fn handle_bold(text: String) -> String {
    let bold_regex: Result<Regex, regex::Error> = Regex::new(BOLD_REGEX);

    match bold_regex {
        Ok(regex) => replace_matches(text, regex, FormatType::Bold),

        Err(error) => {
            println!("Bold regex error: {}", error);
            text
        }
    }
}

fn handle_italic(text: String) -> String {
    let italic_regex: Result<Regex, regex::Error> = Regex::new(ITALIC_REGEX);

    match italic_regex {
        Ok(regex) => replace_matches(text, regex, FormatType::Italic),

        Err(error) => {
            println!("Italic regex error: {}", error);
            text
        }
    }
}

fn handle_color(text: String) -> String {
    let color_regex: Result<Regex, regex::Error> = Regex::new(COLOR_REGEX);

    match color_regex {
        Ok(regex) => replace_matches(text, regex, FormatType::Color),

        Err(error) => {
            println!("Color regex error: {}", error);
            text
        }
    }
}

fn replace_matches(text: String, regex: Regex, format_type: FormatType) -> String {
    let captures = regex.captures_iter(&text);
    let mut new_text = text.clone();

    for capture in captures {
        match format_type {
            FormatType::Color => {
                new_text = regex
                    .replace(&new_text, format_type.handle_formatting(
                        &capture["text"].to_string(),
                        Some(&capture["color"].to_string())
                    ))
                    .into_owned();
            }

            _ => {
                new_text = regex
                    .replace(&new_text, format_type.handle_formatting(&capture["text"].to_string(), None))
                    .into_owned();
            }
        }
    }

    new_text
}