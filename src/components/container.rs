use std::io::{Write, Error};
use super::item::*;
use super::document::*;
use crate::utilities::def_syntax::*;
use crate::utilities::format::*;

/// An object that can contain `Items`.
/// A `Container` is an `Item`, so it can be nested.
pub trait Container: Item {
    fn update_nested_indent(&mut self);
}

/// An object that can list `Items`.
/// Refer to `enumerate` in LaTeX documentation for more information.
pub struct Enumerate {
    items: Vec<Box<dyn Item>>,
    label: String,
    indent: usize
}

impl Enumerate {
    /// Initializes a new `Enumerate` object
    pub fn new(_label: &str) -> Self {
        Self {
            items: Vec::new(),
            label: String::from(_label),
            indent: 0
        }
    }

    /// Adds an `Item` to this `Container`
    pub fn add_item<I: Item + 'static>(&mut self, item: I) {
        self.items.push(Box::new(item));
    }

    fn build_header(&self, doc: &Document) -> Result<(), Error> {
        let begin_enumerate_str = format!("{} {}", DEF_BEGIN_ENUMERATE, into_label(&self.label));
        write_indented_line(&doc, &self.indent, &begin_enumerate_str)?;
        doc.add_blank_line()
    }

    fn build_items(&self, doc: &Document) -> Result<(), Error> {
        for item in &self.items {
            write_indented_line(&doc, &(self.indent + 1), DEF_ITEM_ENUMERATE)?;
            item.build(doc)?;
        }

        Ok(())
    }

    fn build_end(&self, doc: &Document) -> Result<(), Error> {
        write_indented_line(&doc, &self.indent, DEF_END_ENUMERATE)?;
        doc.add_blank_line()
    }
}

impl Item for Enumerate {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        self.build_header(doc)?;
        self.build_items(doc)?;
        self.build_end(doc)
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

/// A transparent object that contains `Items`
/// A `Block` object is not displayed. Its only purpose is to group other `Items`.
/// Example: A `Block` can be composed of a `Text` and a `Figure` and be added to an `Enumerate`.
/// This way, the text and the figure will be placed under the same `\item` in the enumerate.
pub struct Block {
    items: Vec<Box<dyn Item>>,
    indent: usize
}

impl Block {
    /// Initializes a new `Block` object
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            indent: 0
        }
    }

    /// Adds an `Item` to this `Container`
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

/// Can be passed to a `Section` component to define its type.
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

/// A `Container` object that contains `Items` and displays them in a _section_.
/// Refer to `section` in LaTeX documentation for more information.
pub struct Section {
    name: String,
    sec_type: SectionType,
    display_num: bool,
    items: Vec<Box<dyn Item>>,
    label: String,
    indent: usize
}

impl Section {
    /// Initializes a new `Section` object
    pub fn new(_name: &str, _sec_type: SectionType, _display_num: bool, _label: &str) -> Self {
        Self {
            name: String::from(_name),
            sec_type: _sec_type,
            display_num: _display_num,
            items: Vec::new(),
            label: String::from(_label),
            indent: 0
        }
    }

    /// Adds an `Item` to this `Container`
    pub fn add_item<I: Item + 'static>(&mut self, item: I) {
        self.items.push(Box::new(item));
    }

    fn build_header(&self, doc: &Document) -> Result<(), Error> {
        indent_line(&doc, &self.indent)?;

        match self.display_num {
            true => write!(doc.get_file(), "{}{}", self.sec_type.get_def(), into_braces(&self.name))?,
            false => write!(doc.get_file(), "{}*{}", self.sec_type.get_def(), into_braces(&self.name))?
        }

        writeln!(doc.get_file(), " {}", into_label(&self.label))?;
        doc.add_blank_line()
    }

    fn build_items(&self, doc: &Document) -> Result<(), Error> {
        for item in &self.items {
            item.build(doc)?;
        }

        Ok(())
    }

}

impl Item for Section {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        self.build_header(doc)?;
        self.build_items(doc)
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

/// A `Container` object that contains `Items` and displays them in a _chapter_.
/// Refer to `chapter` in LaTeX documentation for more information.
pub struct Chapter {
    name: String,
    display_num: bool,
    items: Vec<Box<dyn Item>>,
    label: String,
    indent: usize
}

impl Chapter {
    /// Initializes a new `Chapter` object
    pub fn new(_name: &str, _display_num: bool, _label: &str) -> Self {
        Self {
            name: String::from(_name),
            display_num: _display_num,
            items: Vec::new(),
            label: String::from(_label),
            indent: 0
        }
    }

    /// Adds an `Item` to this `Container`
    pub fn add_item<I: Item + 'static>(&mut self, item: I) {
        self.items.push(Box::new(item));
    }

    fn build_header(&self, doc: &Document) -> Result<(), Error> {
        indent_line(&doc, &self.indent)?;

        match self.display_num {
            true => write!(doc.get_file(), "{}{}", DEF_CHAPTER, into_braces(&self.name))?,
            false => write!(doc.get_file(), "{}*{}", DEF_CHAPTER, into_braces(&self.name))?
        }

        writeln!(doc.get_file(), " {}", into_label(&self.label))?;
        doc.add_blank_line()
    }

    fn build_items(&self, doc: &Document) -> Result<(), Error> {
        for item in &self.items {
            item.build(doc)?;
        }

        Ok(())
    }
}

impl Item for Chapter {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        self.build_header(doc)?;
        self.build_items(doc)
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent = super_indent + 1;
        self.update_nested_indent();
    }
}

impl Container for Chapter {
    fn update_nested_indent(&mut self) {
        for item in &mut self.items {
            item.update_indent(&self.indent);
        }
    }
}