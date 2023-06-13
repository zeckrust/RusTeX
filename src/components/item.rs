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
        formatted_text = formatted_text.replace("\n", " ");
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

// pub struct Equation
// {
//     //@TODO: Implement as Item
// }

pub struct Section
{
    name: String,
    display_num: bool,
    items: Vec<Box<dyn Item>>
}

impl Section
{
    pub fn new(_name: String, _display_num: bool) -> Self
    {
        Self {name: _name, display_num: _display_num, items: Vec::new()}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }
}

impl Item for Section
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        match self.display_num
        {
            true => writeln!(&doc.file, "{}*{}\n", DEF_SECTION, into_braces(&self.name))?,
            false => writeln!(&doc.file, "{}{}\n", DEF_SECTION, into_braces(&self.name))?
        }

        for item in &self.items
        {
            item.build(doc)?;
            doc.add_blank_line()?;
        }

        Ok(())
    }
}

pub struct SubSection
{
    name: String,
    display_num: bool,
    items: Vec<Box<dyn Item>>
}

impl SubSection
{
    pub fn new(_name: String, _display_num: bool) -> Self
    {
        Self {name: _name, display_num: _display_num, items: Vec::new()}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }
}

impl Item for SubSection
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        match self.display_num
        {
            true => writeln!(&doc.file, "{}*{}\n", DEF_SUB_SECTION, into_braces(&self.name))?,
            false => writeln!(&doc.file, "{}{}\n", DEF_SUB_SECTION, into_braces(&self.name))?
        }

        for item in &self.items
        {
            item.build(doc)?;
            doc.add_blank_line()?;
        }

        Ok(())
    }
}

pub struct SubSubSection
{
    name: String,
    display_num: bool,
    items: Vec<Box<dyn Item>>
}

impl SubSubSection
{
    pub fn new(_name: String, _display_num: bool) -> Self
    {
        Self {name: _name, display_num: _display_num, items: Vec::new()}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }
}

impl Item for SubSubSection
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        match self.display_num
        {
            true => writeln!(&doc.file, "{}*{}\n", DEF_SUB_SUB_SECTION, into_braces(&self.name))?,
            false => writeln!(&doc.file, "{}{}\n", DEF_SUB_SUB_SECTION, into_braces(&self.name))?
        }

        for item in &self.items
        {
            item.build(doc)?;
            doc.add_blank_line()?;
        }

        Ok(())
    }
}