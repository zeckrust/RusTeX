use std::collections::LinkedList;
use std::io::{Write, Error};
use super::def_constants::*;
use super::document::*;


pub struct Package
{
    pub name: String,
    pub options: LinkedList<String>
}

impl Package
{
    pub fn build(&self, doc: &Document) -> Result<(), Error>
    {
        let mut package_str: String = format!("{}[", DEF_PACKAGE);

        for option in &self.options
        {
            package_str = format!("{}{},", package_str, option);
        }

        package_str = format!("{}]{}", package_str, into_braces(&self.name));
        writeln!(&doc.file, "{}", package_str)
    }
}

pub trait Item
{
    fn build(&self, doc: &Document) -> Result<(), Error>;
}

pub struct Paragraph
{
    pub text: String
}

impl Item for Paragraph
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        let mut formatted_text = self.text.replace("  ", "");
        formatted_text = format!("{}{}", DEF_TEXT, into_braces(&formatted_text));
        writeln!(&doc.file, "{}", formatted_text)
    }
}

// pub struct Table
// {
//     //@TODO: Implement as Item
// }

// pub struct Figure
// {
//     //@TODO: Implement as Item
// }

// pub struct Enumerate
// {
//     //@TODO: Implement as Item
// }

pub struct Section
{
    name: String,
    display_num: bool
}

pub struct SubSection
{
    name: String,
    display_num: bool
}

pub struct SubSubSection
{
    name: String,
    display_num: bool
}