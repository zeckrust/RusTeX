use std::fs::File;
use std::io::{Write, Error};
use super::item::*;
use crate::utilities::def_syntax::*;
use crate::utilities::format::*;


pub struct Document {
    file: File,
    class: DocumentClass,
    packages: Vec<Package>,
    commands: Vec<Command>,
    items: Vec<Box<dyn Item>>
}

impl Document {
    pub fn new(doc_file: File, doc_class: DocumentClass) -> Self {
        Self {
            file: doc_file,
            class: doc_class,
            packages: Vec::new(),
            commands: Vec::new(),
            items: Vec::new()
        }
    }

    pub fn get_file(&self) -> &File {
        &self.file
    }

    pub fn add_item<I: Item + 'static>(&mut self, item: I) {
        self.items.push(Box::new(item));
    }

    pub fn add_packages(&mut self, _packages: Vec<Package>) {
        self.packages.extend(_packages);
    }

    pub fn add_global_commands(&mut self, _commands: Vec<Command>) {
        self.commands.extend(_commands);
    }

    pub fn build(&mut self) {
        self.update_indents();
        self.build_doc_class().unwrap();
        self.build_packages().unwrap();
        self.build_commands().unwrap();
        self.build_items().unwrap();
    }

    fn build_doc_class(&mut self) -> Result<(), Error> {
        let options_str = self.class.options.join(", ");
        let mut doc_class_str = format!("{}{}", DEF_DOCUMENT_CLASS, into_brackets(&options_str));
        doc_class_str = format!("{}{}", doc_class_str, into_braces(&self.class._type.to_str()));

        writeln!(self.file, "{}", doc_class_str)?;
        self.add_blank_line()
    }

    fn build_packages(&mut self) -> Result<(), Error> {
        self.build_default_packages()?;

        writeln!(&self.file, "{}", USER_PACKAGES_COMMENT)?;
        for package in &self.packages {
            package.build(&self)?;
        }

        self.add_blank_line()
    }

    fn build_default_packages(&mut self) -> Result<(), Error> {
        writeln!(&self.file, "{}", DEFAULT_PACKAGES_COMMENT)?;
        writeln!(&self.file, "{}", DEFAULT_FLOAT_PACKAGE)?;
        writeln!(&self.file, "{}", DEFAULT_GRAPHICX_PACKAGE)?;
        self.add_blank_line()
    }

    fn build_commands(&mut self) -> Result<(), Error> {
        writeln!(&self.file, "{}", USER_GLOBAL_COMMANDS_COMMENT)?;
        for command in &self.commands {
            command.build(&self)?;
        }

        self.add_blank_line()
    }

    fn update_indents(&mut self) {
        for item in &mut self.items {
            item.update_indent(&0);
        }
    }

    fn build_items(&mut self) -> Result<(), Error> {
        writeln!(&self.file, "{}\n", DEF_BEGIN_DOCUMENT)?;

        for item in &self.items {
            item.build(&self)?;
        }

        writeln!(&self.file, "{}", DEF_END_DOCUMENT)
    }

    pub fn add_blank_line(&self) -> Result<(), Error> {
        writeln!(&self.file, "")
    }
}

pub struct DocumentClass {
    _type: ClassType,
    options: Vec<String>
}

impl DocumentClass {
    pub fn new(class_type: ClassType, _options: Vec<&str>) -> Self {
        Self {
            _type: class_type,
            options: _options
                .iter()
                .map(|&option| String::from(option))
                .collect()
        }
    }
}

pub enum ClassType {
    Article,
    Report,
    Book,
    Memoir,
    Letter,
    Beamer
}

impl ClassType {
    pub fn to_str(&self) -> String {
        match self {
            ClassType::Article => String::from("article"),
            ClassType::Report => String::from("report"),
            ClassType::Book => String::from("book"),
            ClassType::Memoir => String::from("memoir"),
            ClassType::Letter => String::from("letter"),
            ClassType::Beamer => String::from("beamer"),
        }
    }
}

pub struct Package {
    name: String,
    options: Vec<String>
}

impl Package {
    pub fn new(_name: &str, _options: Vec<&str>) -> Self {
        Self {
            name: String::from(_name),
            options: _options
                .iter()
                .map(|&option| String::from(option))
                .collect()
        }
    }

    pub fn build(&self, doc: &Document) -> Result<(), Error> {
        let options_str = self.options.join(", ");
        let mut package_str: String = format!("{}{}", DEF_PACKAGE, into_brackets(&options_str));
        package_str = format!("{}{}", package_str, into_braces(&self.name));

        writeln!(&doc.file, "{}", package_str)
    }
}
