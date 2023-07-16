use std::io::{Error};
use super::document::*;
use super::item::*;
use crate::utilities::def_syntax::*;
use crate::utilities::format::*;

pub struct Table {
    options: String,
    dimensions: TableDimensions,
    components: Vec<Box<dyn TableComponent>>,
    indent: usize
}

struct TableDimensions {
    rows: usize,
    columns: usize
}

 impl TableDimensions {
    pub fn new(_rows: usize, _columns: usize) -> Self {
        Self {
            rows: _rows,
            columns: _columns
        }
    }
 }

trait TableComponent: Item {}

struct HorizontalLine {
    indent: usize
}

impl HorizontalLine {
    pub fn new() -> Self {
        Self{
            indent: 0
        }
    }
}

impl Item for HorizontalLine {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        write_indented_line(&doc, &self.indent, DEF_HORIZONTAL_LINE)
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent= super_indent + 1;
    }
}

impl TableComponent for HorizontalLine {}

struct TableRow {
    content: Vec<String>,
    indent: usize
}

impl TableRow {
    pub fn new(_content: Vec<String>) -> Self {
        Self {
            content: _content,
            indent: 0
        }
    }
}

impl Item for TableRow {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        let formatted_row: String = self.content.join(" & ");
        write_indented_line(&doc, &self.indent, &formatted_row)
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent= super_indent + 1;
    }
}

impl TableComponent for TableRow {}