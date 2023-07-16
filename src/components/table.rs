use std::io::Error;
use super::document::*;
use super::item::*;
use super::container::Container;
use crate::utilities::def_syntax::*;
use crate::utilities::format::*;

pub struct Table {
    options: String,
    components: Vec<Box<dyn TableComponent>>,
    centered: bool,
    indent: usize
}

impl Table {
    pub fn new(_options: String, _centered: bool) -> Self {
        Self {
            options: _options,
            components: Vec::new(),
            centered: _centered,
            indent: 0
        }
    }

    pub fn add_component<TC: TableComponent + 'static>(&mut self, component: TC) {
        self.components.push(Box::new(component));
    }
}

impl Item for Table {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        if self.centered { write_indented_line(&doc, &self.indent, DEF_BEGIN_CENTER)? };
        let begin_tabular_str = format!("{}{}", DEF_BEGIN_TABULAR, into_braces(&self.options));
        write_indented_line(&doc, &self.indent, &begin_tabular_str)?;

        for component in &self.components {
            component.build(&doc)?;
        }

        write_indented_line(&doc, &self.indent, DEF_END_TABULAR)?;
        if self.centered { write_indented_line(&doc, &self.indent, DEF_END_CENTER)? };
        doc.add_blank_line()
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent= super_indent + 1;
        self.update_nested_indent();
    }
}

impl Container for Table {
    fn update_nested_indent(&mut self) {
        for component in &mut self.components {
            component.update_indent(&self.indent);
        }
    }
}

pub trait TableComponent: Item {}

pub struct HorizontalLine {
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

pub struct TableRow {
    content: Vec<Text>,
    indent: usize
}

impl TableRow {
    pub fn new(_content: Vec<Text>) -> Self {
        Self {
            content: _content,
            indent: 0
        }
    }
}

impl Item for TableRow {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        let mut cells: Vec<String> = Vec::new();
        for cell in &self.content {
            cells.push(cell.get_string())
        }

        let mut formatted_row: String = cells.join(" & ");
        formatted_row = format!("{} \\\\", formatted_row);
        write_indented_line(&doc, &self.indent, &formatted_row)
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent= super_indent + 1;
    }
}

impl TableComponent for TableRow {}