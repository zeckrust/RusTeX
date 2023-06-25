use super::def_constants::*;
use super::document::*;
use std::io::{Write, Error};

pub fn into_braces(string: &String) -> String
{
    let mut str_with_braces: String = String::from("{");
    str_with_braces = format!("{}{}", str_with_braces, string);
    format!("{}{}", str_with_braces, "}")
}

pub fn into_bold(string: &String) -> String
{
    format!("{}{}", DEF_BOLD, into_braces(string))
}

pub fn indent_line(doc: &Document, num_tabs: &usize) -> Result<(), Error>
{
    let tabs_str: String = "\t".repeat(*num_tabs);
    write!(&doc.file, "{}", tabs_str)
}