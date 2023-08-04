use std::io::Error;
use super::document::*;
use super::item::*;
use super::container::Container;
use crate::utilities::def_syntax::*;
use crate::utilities::format::*;


/// An object that can contain different `TableComponents` in rows and columns.
/// Refer to `table` in LaTeX documentation for more information.
///
/// Example:
///
/// ```rust
/// let mut table_1: Table = Table::new(
///     "H",                                // Positioning
///     "|c|c|c|",                          // Options
///     true,                               // Centered
///     Some(Text::new("Random caption")),  // Caption
///     "tab_1"                             // Label
/// );
///
/// table_1.add_component(HorizontalLine::new());

/// let table_row_1 = TableRow::new(vec![Text::new(r"\multicolumn{3}{|c|}{**Lorem Ipsum**}")]);
///
/// table_1.add_component(table_row_1);
/// table_1.add_component(HorizontalLine::new());
///
/// let table_row_2 = TableRow::new(vec![
///     Text::new("#red{First}"),
///     Text::new("#violet{Second}"),
///     Text::new("#teal{Third}")]
/// );
///
/// table_1.add_component(table_row_2);
/// table_1.add_component(HorizontalLine::new());
///
/// let table_row_3 = TableRow::new(vec![
///     Text::new("Test 1"),
///     Text::new("Test 2"),
///     Text::new("Test 3")]
/// );

/// table_1.add_component(table_row_3);
/// table_1.add_component(HorizontalLine::new());
/// ```
///
/// Generated LaTeX:
///
/// ```tex
/// \begin{table}[H] \label{tab_1}
///     \centering
///     \begin{tabular}{|c|c|c|}
///         \hline
///         \multicolumn{3}{|c|}{\textbf{Lorem Ipsum}} \\
///         \hline
///         {\color{red}{First}} & {\color{violet}{Second}} & {\color{teal}{Third}} \\
///         \hline
///         Test 1 & Test 2 & Test 3 \\
///         \hline
///     \end{tabular}
///     \caption{Random caption}
/// \end{table}
/// ```
pub struct Table {
    positioning: String,
    options: String,
    components: Vec<Box<dyn TableComponent>>,
    centered: bool,
    caption: Option<Text>,
    label: String,
    indent: usize
}

impl Table {
    /// Initializes a new `Table` object
    pub fn new(_positioning: &str, _options: &str, _centered: bool,
               _caption: Option<Text>, _label: &str) -> Self {
        Self {
            positioning: String::from(_positioning),
            options: String::from(_options),
            components: Vec::new(),
            centered: _centered,
            caption: _caption,
            label: String::from(_label),
            indent: 0
        }
    }

    /// Add a `TableComponent` to the `Table`
    pub fn add_component<TC: TableComponent + 'static>(&mut self, component: TC) {
        self.components.push(Box::new(component));
    }

    fn build_header(&self, doc: &Document) -> Result<(), Error> {
        let mut begin_table_str = format!("{}{}", DEF_BEGIN_TABLE, into_brackets(&self.positioning));
        begin_table_str = format!("{} {}", begin_table_str, into_label(&self.label));
        write_indented_line(&doc, &self.indent, &begin_table_str)
    }

    fn build_tabular(&self, doc: &Document, inner_indent: &usize) -> Result<(), Error> {
        let begin_tabular_str = format!("{}{}", DEF_BEGIN_TABULAR, into_braces(&self.options));
        write_indented_line(&doc, inner_indent, &begin_tabular_str)?;

        for component in &self.components {
            component.build(&doc)?;
        }

        write_indented_line(&doc, inner_indent, DEF_END_TABULAR)
    }

    fn build_caption(&self, doc: &Document, inner_indent: &usize) -> Result<(), Error> {
        match &self.caption {
            Some(caption) => {
                let caption_str = format!("{}{}", DEF_CAPTION, into_braces(&caption.get_string()));
                write_indented_line(&doc, inner_indent, &caption_str)?;
            }
            None => {}
        }

        Ok(())
    }

    fn build_centering(&self, doc: &Document, inner_indent: &usize) -> Result<(), Error> {
        if self.centered {
            write_indented_line(&doc, inner_indent,DEF_CENTERING )?;
        }

        Ok(())
    }

    fn build_end(&self, doc: &Document) -> Result<(), Error> {
        write_indented_line(&doc, &self.indent, DEF_END_TABLE)?;
        doc.add_blank_line()
    }
}

impl Item for Table {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        let inner_indent = &(self.indent + 1);

        self.build_header(doc)?;
        self.build_centering(doc, inner_indent)?;
        self.build_tabular(doc, inner_indent)?;
        self.build_caption(doc, inner_indent)?;
        self.build_end(doc)
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent= super_indent + 1;
        self.update_nested_indent();
    }
}

impl Container for Table {
    fn update_nested_indent(&mut self) {
        for component in &mut self.components {
            component.update_indent(&(self.indent + 1));
        }
    }
}

/// A component that can be added to a `Table`
pub trait TableComponent: Item {}

/// A horizontal line that can be added to a `Table`
pub struct HorizontalLine {
    indent: usize
}

impl HorizontalLine {
    /// Initializes a new `HorizontalLine` object
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

/// A row that can be added to a `Table`
pub struct TableRow {
    content: Vec<Text>,
    indent: usize
}

impl TableRow {
    /// Initializes a new `TableRow` object
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