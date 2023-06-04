use std::collections::LinkedList;
use std::io::{Write, Error};
use super::def_constants::*;
use super::document::*;


pub trait Item
{
    fn build(&self, doc: &Document) -> Result<(), Error>;
}

pub struct Package
{
    pub name: String,
    pub options: LinkedList<String>
}

impl Item for Package
{
    fn build(&self, doc: &Document) -> Result<(), Error>
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

pub struct Table
{

}

pub struct Figure
{

}

pub struct Enumerate
{

}

pub struct Paragraph
{

}

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