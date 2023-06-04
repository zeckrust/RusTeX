use std::collections::LinkedList;
use std::fs::File;
use std::io::Write;
use super::item::*;

const DEF_PACKAGE: &str = r"\usepackage";
const DEF_DOCUMENT_CLASS: &str = r"\documentclass";

pub struct Document
{
    pub file: File,
    pub class: DocumentClass,
    pub packages: Vec<Package>,
    pub items: Vec<Box<dyn Item>>
}

impl Document
{
    pub fn add<I: Item + 'static>(&mut self, item: I)
    {
        self.items.push(Box::new(item)); // Box::new() adds it to the heap
    }

    pub fn build(&mut self)
    {
        self.build_doc_class();
        self.build_packages();
    }

    fn build_doc_class(&mut self)
    {
        let mut doc_class_str: String = format!("{}[", DEF_DOCUMENT_CLASS);

        for option in &self.class.options
        {
            doc_class_str = format!("{}{},", doc_class_str, option);
        }

        doc_class_str = format!("{}]{}", doc_class_str, into_braces(&self.class.name));
        writeln!(self.file, "{}", doc_class_str);
    }

    fn build_packages(&self)
    {
        for package in &self.packages
        {
            let mut package_str: String = format!("{}[", DEF_PACKAGE);

            for option in &package.options
            {
                package_str = format!("{}{},", package_str, option);
            }

            package_str = format!("{}]{}", package_str, into_braces(&package.name));
            writeln!(&self.file, "{}", package_str);
        }
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
    let str_with_braces: &str = "{";
    format!("{}{}", str_with_braces, string);
    format!("{}{}", str_with_braces, "}")
}