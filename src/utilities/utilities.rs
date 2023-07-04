use super::def_constants::*;
use crate::components::document::*;
use std::io::{Write, Error};
use regex::*;

pub fn into_braces(string: &String) -> String {
    let mut str_with_braces: String = String::from("{");
    str_with_braces = format!("{}{}", str_with_braces, string);
    format!("{}{}", str_with_braces, "}")
}

fn into_bold(string: &String) -> String {
    format!("{}{}", DEF_BOLD, into_braces(string))
}

fn into_italic(string: &String) -> String {
    format!("{}{}", DEF_ITALIC, into_braces(string) )
}

pub fn indent_line(doc: &Document, num_tabs: &usize) -> Result<(), Error> {
    let tabs_str: String = "\t".repeat(*num_tabs);
    write!(&doc.file, "{}", tabs_str)
}

pub fn write_indented_line(doc: &Document, num_tabs: &usize, text: &str) -> Result<(), Error> {
    indent_line(&doc, num_tabs)?;
    writeln!(&doc.file, "{}", text)
}

const BOLD_REGEX: &str = r"\*{2}(?P<text>.+?)\*{2}";
const ITALIC_REGEX: &str= r"_(?P<text>.+?)_";

pub fn handle_text_format(text: String) -> String {
    let formatted_text = handle_bold(text);
    handle_italic(formatted_text)
}

fn handle_bold(text: String) -> String {
    let bold_regex = Regex::new(BOLD_REGEX);

    match bold_regex {
        Ok(regex) => replace_bold_matches(text, regex),

        Err(error) => {
            println!("Bold Regex error: {}", error);
            text
        }
    }
}

fn replace_bold_matches(text: String, regex: Regex) -> String{
    let captures = regex.captures_iter(&text);
    let mut new_text = text.clone();

    for capture in captures {
        new_text = regex
            .replace(&new_text, into_bold(&capture["text"].to_string()))
            .into_owned();
    }

    new_text
}

fn handle_italic(text: String) -> String {
    let italic_regex = Regex::new(ITALIC_REGEX);

    match italic_regex {
        Ok(regex) => replace_italic_matches(text, regex),

        Err(error) => {
            println!("Italic Regex error: {}", error);
            text
        }
    }
}

fn replace_italic_matches(text: String, regex: Regex) -> String {
    let captures = regex.captures_iter(&text);
    let mut new_text = text.clone();

    for capture in captures {
        new_text = regex
            .replace(&new_text, into_italic(&capture["text"].to_string()))
            .into_owned();
    }

    new_text
}