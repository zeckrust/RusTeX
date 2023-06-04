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

    pub fn add<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item)); // Box::new() adds it to the heap
    }

    pub fn build(&mut self) -> Result<(), Error>
    {
        self.build_doc_class()?;
        self.build_packages()?;
        Ok(())
    }

    fn build_doc_class(&mut self) -> Result<(), Error>
    {
        let mut doc_class_str: String = format!("{}[", DEF_DOCUMENT_CLASS);

        for option in &self.class.options
        {
            doc_class_str = format!("{}{},", doc_class_str, option);
        }

        doc_class_str = format!("{}]{}", doc_class_str, into_braces(&self.class.name));
        writeln!(self.file, "{}", doc_class_str)
    }

    fn build_packages(&self) -> Result<(), Error>
    {
        for package in &self.packages
        {
            package.build(&self)?;
        }

        Ok(())
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