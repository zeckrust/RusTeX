use std::io::{Write, Error};
use super::def_constants::*;
use super::item::*;
use super::utilities::*;
use super::document::*;

pub trait Container: Item
{
    fn update_nested_indent(&mut self);
}

pub struct Enumerate
{
    items: Vec<Box<dyn Item>>,
    pub indent: usize
}

impl Enumerate
{
    pub fn new() -> Self
    {
        Self {items: Vec::new(), indent: 0}
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
        indent_line(&doc, &self.indent)?;
        writeln!(&doc.file, "{}", DEF_BEGIN_ENUMERATE)?;
        doc.add_blank_line()?;

        for item in &self.items
        {
            indent_line(&doc, &(self.indent + 1))?;
            writeln!(&doc.file, "{}", DEF_ITEM_ENUMERATE)?;
            item.build(doc)?;
        }

        indent_line(&doc, &self.indent)?;
        writeln!(&doc.file, "{}", DEF_END_ENUMERATE)?;
        doc.add_blank_line()
    }

    fn update_indent(&mut self, super_indent: &usize)
    {
        self.indent = super_indent + 1;
        self.update_nested_indent();
    }
}

impl Container for Enumerate
{
    fn update_nested_indent(&mut self)
    {
        for item in &mut self.items
        {
            item.update_indent(&self.indent);
        }
    }
}

pub struct Block
{
    items: Vec<Box<dyn Item>>,
    pub indent: usize
}

impl Block
{
    pub fn new() -> Self
    {
        Self {items: Vec::new(), indent: 0}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }

    fn update_nested_indent(&mut self)
    {
        for item in &mut self.items
        {
            item.update_indent(&self.indent);
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

    fn update_indent(&mut self, super_indent: &usize)
    {
        self.indent = *super_indent;
        self.update_nested_indent();
    }
}

pub struct Section
{
    name: String,
    display_num: bool,
    items: Vec<Box<dyn Item>>,
    pub indent: usize
}

impl Section
{
    pub fn new(_name: String, _display_num: bool) -> Self
    {
        Self {name: _name, display_num: _display_num, items: Vec::new(), indent: 0}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }

    fn update_nested_indent(&mut self)
    {
        for item in &mut self.items
        {
            item.update_indent(&self.indent);
        }
    }
}

impl Item for Section
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        indent_line(&doc, &self.indent)?;
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

    fn update_indent(&mut self, super_indent: &usize)
    {
        self.indent = super_indent + 1;
        self.update_nested_indent();
    }
}

pub struct SubSection
{
    name: String,
    display_num: bool,
    items: Vec<Box<dyn Item>>,
    pub indent: usize
}

impl SubSection
{
    pub fn new(_name: String, _display_num: bool) -> Self
    {
        Self {name: _name, display_num: _display_num, items: Vec::new(), indent: 0}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }

    fn update_nested_indent(&mut self)
    {
        for item in &mut self.items
        {
            item.update_indent(&self.indent);
        }
    }
}

impl Item for SubSection
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        indent_line(&doc, &self.indent)?;
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

    fn update_indent(&mut self, super_indent: &usize)
    {
        self.indent = super_indent + 1;
        self.update_nested_indent();
    }
}

pub struct SubSubSection
{
    name: String,
    display_num: bool,
    items: Vec<Box<dyn Item>>,
    pub indent: usize
}

impl SubSubSection
{
    pub fn new(_name: String, _display_num: bool) -> Self
    {
        Self {name: _name, display_num: _display_num, items: Vec::new(), indent: 0}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item));
    }

    fn update_nested_indent(&mut self)
    {
        for item in &mut self.items
        {
            item.update_indent(&self.indent);
        }
    }
}

impl Item for SubSubSection
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        indent_line(&doc, &self.indent)?;
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

    fn update_indent(&mut self, super_indent: &usize)
    {
        self.indent = super_indent + 1;
        self.update_nested_indent();
    }
}