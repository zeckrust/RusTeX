use std::collections::LinkedList;
use std::fs::File;
use std::io::{Write, Error};
use super::item::*;
use crate::utilities::def_constants::*;
use crate::utilities::utilities::*;


pub struct Document
{
    pub file: File,
    pub class: DocumentClass,
    pub packages: Vec<Package>,
    pub items: Vec<Box<dyn Item>>
}

impl Document
{
    pub fn new(doc_file: File, doc_class: DocumentClass) -> Self
    {
        Self
        {
            file: doc_file,
            class: doc_class,
            packages: Vec::new(),
            items: Vec::new()
        }
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item)); // Box::new() adds it to the heap
    }

    pub fn add_packages(&mut self, _packages: Vec<Package>)
    {
        self.packages.extend(_packages)
    }

    pub fn build(&mut self) -> Result<(), Error>
    {
        self.build_doc_class()?;
        self.build_packages()?;
        self.update_indents();
        self.build_items()
    }

    fn build_doc_class(&mut self) -> Result<(), Error>
    {
        let mut doc_class_str: String = format!("{}[", DEF_DOCUMENT_CLASS);

        for option in &self.class.options
        {
            doc_class_str = format!("{}{},", doc_class_str, option);
        }

        doc_class_str = format!("{}]{}", doc_class_str, into_braces(&self.class.name));
        writeln!(self.file, "{}", doc_class_str)?;
        self.add_blank_line()
    }

    fn build_packages(&mut self) -> Result<(), Error>
    {
        for package in &self.packages
        {
            package.build(&self)?;
        }

        self.add_blank_line()
    }

    fn update_indents(&mut self)
    {
        for item in &mut self.items
        {
            item.update_indent(&0);
        }
    }

    fn build_items(&mut self) -> Result<(), Error>
    {
        writeln!(&self.file, "{}\n", DEF_BEGIN_DOCUMENT)?;

        for item in &self.items
        {
            item.build(&self)?;
        }

        writeln!(&self.file, "{}", DEF_END_DOCUMENT)
    }

    pub fn add_blank_line(&self) -> Result<(), Error>
    {
        writeln!(&self.file, "")
    }
}

pub struct DocumentClass
{
    name: String,
    options: LinkedList<String>
}

impl DocumentClass
{
    pub fn new(class: String, options: LinkedList<String>) -> Self
    {
        Self {name: class, options: options}
    }
}

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
