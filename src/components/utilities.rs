use super::def_constants::*;

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