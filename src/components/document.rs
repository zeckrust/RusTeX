use std::collections::LinkedList;
use std::fs::File;
use std::io::{Write, Error};
use super::def_constants::*;
use super::item::*;


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

    fn build_items(&mut self) -> Result<(), Error>
    {
        writeln!(&self.file, "{}\n", DEF_BEGIN_DOCUMENT)?;

        for item in &self.items
        {
            item.build(&self)?;
            self.add_blank_line()?;
        }

        writeln!(&self.file, "{}", DEF_END_DOCUMENT)
    }

    fn add_blank_line(&self) -> Result<(), Error>
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

pub fn into_braces(string: &String) -> String
{
    let mut str_with_braces: String = String::from("{");
    str_with_braces = format!("{}{}", str_with_braces, string);
    format!("{}{}", str_with_braces, "}")
}