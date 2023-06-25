use std::io::{Write, Error};
use super::document::*;
use super::utilities::*;


pub trait Item
{
    fn build(&self, doc: &Document) -> Result<(), Error>;
    fn update_indent(&mut self, super_indent: &usize);
}

pub struct Paragraph
{
    pub text: String,
    pub indent: usize
}

impl Paragraph
{
    pub fn new(_text: String) -> Self
    {
        Self {text: _text, indent: 0}
    }
}

impl Item for Paragraph
{
    fn build(&self, doc: &Document) -> Result<(), Error>
    {
        let mut formatted_text = self.text.replace("  ", "");
        formatted_text = formatted_text.replace("\n", " ");
        indent_line(&doc, &self.indent)?;
        writeln!(&doc.file, "{}", formatted_text)?;
        doc.add_blank_line()
    }

    fn update_indent(&mut self, super_indent: &usize)
    {
        self.indent = super_indent + 1;
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
