use std::io::{Write, Error};
use super::def_constants::*;
use super::document::*;
use super::utilities::*;


pub trait Item
{
    fn build(&self, doc: &Document) -> Result<(), Error>;
    fn update_indent(&mut self, super_ident: &usize);
}

pub struct Paragraph
{
    pub text: String,
    pub ident: usize
}

impl Paragraph
{
    pub fn new(_text: String) -> Self
    {
        Self {text: _text, ident: 0}
    }

    fn indent_line(&self, doc: &Document) -> Result<(), Error>
    {
        let tabs_str: String = "\t".repeat(self.ident);
        write!(&doc.file, "{}", tabs_str)
    }
}

impl Item for Paragraph
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        let mut formatted_text = self.text.replace("  ", "");
        formatted_text = formatted_text.replace("\n", " ");
        self.indent_line(&doc)?;
        writeln!(&doc.file, "{}", formatted_text)?;
        doc.add_blank_line()
    }

    fn update_indent(&mut self, super_ident: &usize)
    {
        self.ident = super_ident + 1;
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
    items: Vec<Box<dyn Item>>,
    pub ident: usize
}

impl Enumerate
{
    pub fn new() -> Self
    {
        Self {items: Vec::new(), ident: 0}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }

    fn indent_line(&self, doc: &Document) -> Result<(), Error>
    {
        let tabs_str: String = "\t".repeat(self.ident);
        write!(&doc.file, "{}", tabs_str)
    }

    fn update_nested_indent(&mut self)
    {
        for item in &mut self.items
        {
            item.update_indent(&self.ident);
        }
    }
}

impl Item for Enumerate
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        self.indent_line(&doc)?;
        writeln!(&doc.file, "{}", DEF_BEGIN_ENUMERATE)?;
        doc.add_blank_line()?;

        for item in &self.items
        {
            self.indent_line(&doc)?;
            writeln!(&doc.file, "{}", DEF_ITEM_ENUMERATE)?;
            item.build(doc)?;
        }

        self.indent_line(&doc)?;
        writeln!(&doc.file, "{}", DEF_END_ENUMERATE)?;
        doc.add_blank_line()
    }

    fn update_indent(&mut self, super_ident: &usize)
    {
        self.ident = super_ident + 1;
        self.update_nested_indent();
    }
}

pub struct Block
{
    items: Vec<Box<dyn Item>>,
    pub ident: usize
}

impl Block
{
    pub fn new() -> Self
    {
        Self {items: Vec::new(), ident: 0}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }

    fn update_nested_indent(&mut self)
    {
        for item in &mut self.items
        {
            item.update_indent(&self.ident);
        }
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

    fn update_indent(&mut self, super_ident: &usize)
    {
        self.ident = *super_ident;
        self.update_nested_indent();
    }
}

pub struct Section
{
    name: String,
    display_num: bool,
    items: Vec<Box<dyn Item>>,
    pub ident: usize
}

impl Section
{
    pub fn new(_name: String, _display_num: bool) -> Self
    {
        Self {name: _name, display_num: _display_num, items: Vec::new(), ident: 0}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }

    fn indent_line(&self, doc: &Document) -> Result<(), Error>
    {
        let tabs_str: String = "\t".repeat(self.ident);
        write!(&doc.file, "{}", tabs_str)
    }

    fn update_nested_indent(&mut self)
    {
        for item in &mut self.items
        {
            item.update_indent(&self.ident);
        }
    }
}

impl Item for Section
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        self.indent_line(&doc)?;
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

    fn update_indent(&mut self, super_ident: &usize)
    {
        self.ident = super_ident + 1;
        self.update_nested_indent();
    }
}

pub struct SubSection
{
    name: String,
    display_num: bool,
    items: Vec<Box<dyn Item>>,
    pub ident: usize
}

impl SubSection
{
    pub fn new(_name: String, _display_num: bool) -> Self
    {
        Self {name: _name, display_num: _display_num, items: Vec::new(), ident: 0}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }

    fn indent_line(&self, doc: &Document) -> Result<(), Error>
    {
        let tabs_str: String = "\t".repeat(self.ident);
        write!(&doc.file, "{}", tabs_str)
    }

    fn update_nested_indent(&mut self)
    {
        for item in &mut self.items
        {
            item.update_indent(&self.ident);
        }
    }
}

impl Item for SubSection
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        self.indent_line(&doc)?;
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

    fn update_indent(&mut self, super_ident: &usize)
    {
        self.ident = super_ident + 1;
        self.update_nested_indent();
    }
}

pub struct SubSubSection
{
    name: String,
    display_num: bool,
    items: Vec<Box<dyn Item>>,
    pub ident: usize
}

impl SubSubSection
{
    pub fn new(_name: String, _display_num: bool) -> Self
    {
        Self {name: _name, display_num: _display_num, items: Vec::new(), ident: 0}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }

    fn indent_line(&self, doc: &Document) -> Result<(), Error>
    {
        let tabs_str: String = "\t".repeat(self.ident);
        write!(&doc.file, "{}", tabs_str)
    }

    fn update_nested_indent(&mut self)
    {
        for item in &mut self.items
        {
            item.update_indent(&self.ident);
        }
    }
}

impl Item for SubSubSection
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        self.indent_line(&doc)?;
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

    fn update_indent(&mut self, super_ident: &usize)
    {
        self.ident = super_ident + 1;
        self.update_nested_indent();
    }
}