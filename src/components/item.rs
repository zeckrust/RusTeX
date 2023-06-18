use std::io::{Write, Error};
use super::def_constants::*;
use super::document::*;
use super::utilities::*;


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
        writeln!(&doc.file, "{}", formatted_text)?;
        doc.add_blank_line()
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

// pub struct Equation
// {
//     //@TODO: Implement as Item
// }

pub struct Enumerate
{
    items: Vec<Box<dyn Item>>
}

impl Enumerate
{
    pub fn new() -> Self
    {
        Self {items: Vec::new()}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }
}

impl Item for Enumerate
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        writeln!(&doc.file, "{}", DEF_BEGIN_ENUMERATE)?;
        doc.add_blank_line()?;

        for item in &self.items
        {
            writeln!(&doc.file, "{}", DEF_ITEM_ENUMERATE)?;
            item.build(doc)?;
        }

        writeln!(&doc.file, "{}", DEF_END_ENUMERATE)?;
        doc.add_blank_line()
    }
}

pub struct Block
{
    items: Vec<Box<dyn Item>>
}

impl Block
{
    pub fn new() -> Self
    {
        Self {items: Vec::new()}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }
}

impl Item for Block
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        for item in &self.items
        {
            item.build(doc)?;
        }

        Ok(())
    }
}

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
        }

        Ok(())
    }
}