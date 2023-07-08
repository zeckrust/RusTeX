use std::io::{Write, Error};
use super::item::*;
use super::document::*;
use crate::utilities::def_constants::*;
use crate::utilities::format::*;

pub trait Container: Item {
    fn update_nested_indent(&mut self);
}

pub struct Enumerate {
    items: Vec<Box<dyn Item>>,
    indent: usize
}

impl Enumerate {
    pub fn new() -> Self {
        Self {items: Vec::new(), indent: 0}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I) {
        self.items.push(Box::new(item));
    }
}

impl Item for Enumerate {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        write_indented_line(&doc, &self.indent, DEF_BEGIN_ENUMERATE)?;
        doc.add_blank_line()?;

        for item in &self.items {
            write_indented_line(&doc, &(self.indent + 1), DEF_ITEM_ENUMERATE)?;
            item.build(doc)?;
        }

        write_indented_line(&doc, &self.indent, DEF_END_ENUMERATE)?;
        doc.add_blank_line()
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent = super_indent + 1;
        self.update_nested_indent();
    }
}

impl Container for Enumerate {
    fn update_nested_indent(&mut self) {
        for item in &mut self.items {
            item.update_indent(&self.indent);
        }
    }
}

pub struct Block {
    items: Vec<Box<dyn Item>>,
    indent: usize
}

impl Block {
    pub fn new() -> Self {
        Self {items: Vec::new(), indent: 0}
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I) {
        self.items.push(Box::new(item));
    }
}

impl Item for Block {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        for item in &self.items {
            item.build(doc)?;
        }

        Ok(())
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent = *super_indent;
        self.update_nested_indent();
    }
}

impl Container for Block {

    fn update_nested_indent(&mut self) {
        for item in &mut self.items {
            item.update_indent(&self.indent);
        }
    }
}

pub enum SectionType {
    Section,
    SubSection,
    SubSubSection
}

impl SectionType {
    pub fn get_def(&self) -> &str {
        match self {
            SectionType::Section => DEF_SECTION,
            SectionType::SubSection => DEF_SUB_SECTION,
            SectionType::SubSubSection => DEF_SUB_SUB_SECTION
        }
    }
}

pub struct Section {
    name: String,
    sec_type: SectionType,
    display_num: bool,
    items: Vec<Box<dyn Item>>,
    indent: usize
}

impl Section {
    pub fn new(_name: String, _sec_type: SectionType, _display_num: bool) -> Self {
        Self {
            name: _name,
            sec_type: _sec_type,
            display_num: _display_num,
            items: Vec::new(),
            indent: 0
        }
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I) {
        self.items.push(Box::new(item));
    }
}

impl Item for Section {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        indent_line(&doc, &self.indent)?;
        match self.display_num {
            true => writeln!(&doc.file, "{}*{}\n", self.sec_type.get_def(), into_braces(&self.name))?,
            false => writeln!(&doc.file, "{}{}\n", self.sec_type.get_def(), into_braces(&self.name))?
        }

        for item in &self.items {
            item.build(doc)?;
        }

        Ok(())
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent = super_indent + 1;
        self.update_nested_indent();
    }
}

impl Container for Section {
    fn update_nested_indent(&mut self) {
        for item in &mut self.items {
            item.update_indent(&self.indent);
        }
    }
}