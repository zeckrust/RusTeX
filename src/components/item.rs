use std::io::Error;
use super::document::*;
use crate::utilities::format::*;
use crate::utilities::def_syntax::*;


/// A component that can be added to a `Document` or a `Container`
pub trait Item {
    fn build(&self, doc: &Document) -> Result<(), Error>;
    fn update_indent(&mut self, super_indent: &usize);
}

/// An `Item` that wraps strings.
/// Can be used as paragraphs, as captions, etc.
/// Supports Markdown **bold** formatting (\*\*bold\*\*)
/// Supports Markdown _italic_ formatting (\_bold\_)
/// Supports Color text formatting (#blue{text})
///
/// Here are all the default colors:
/// - red
/// - green
/// - blue
/// - cyan
/// - magenta
/// - yellow
/// - black
/// - gray
/// - white
/// - darkgray
/// - lightgray
/// - brown
/// - lime
/// - olive
/// - orange
/// - pink
/// - purple
/// - teal
/// - violet
///
/// Other colors can be used by adding `Packages` or defining custom colors.
pub struct Text {
    text: String,
    indent: usize
}

impl Text {
    /// Initializes a new `Text` object
    pub fn new(_text: &str) -> Self {
        Self {
            text: format_text(String::from(_text)),
            indent: 0
        }
    }

    /// Gets a clone of the `Text`'s string.
    pub fn get_string(&self) -> String {
        self.text.clone()
    }
}

impl Item for Text {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        let mut formatted_text = self.text.replace("  ", "");
        formatted_text = formatted_text.replace("\n", " ");
        write_indented_line(&doc, &self.indent, formatted_text.as_str())?;
        doc.add_blank_line()
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent = super_indent + 1;
    }
}

/// An object that can display images, graphics, etc.
/// Refer to `figure` in LaTeX documentation for more information.
pub struct Figure {
    positioning: String,
    centered: bool,
    image_path: String,
    image_option: String,
    caption: Option<Text>,
    label: String,
    indent: usize
}

impl Figure {
    /// Initializes a new `Figure` object
    pub fn new(_positioning: &str, _centered: bool, _image_path: &str,
               _image_option: &str, _caption: Option<Text>, _label: &str) -> Self {
        Self {
            positioning: String::from(_positioning),
            centered: _centered,
            image_path: String::from(_image_path),
            image_option: String::from(_image_option),
            caption: _caption,
            label: String::from(_label),
            indent: 0
        }
    }

    fn build_header(&self, doc: &Document) -> Result<(), Error> {
        let mut begin_figure_str = format!("{}{}", DEF_BEGIN_FIGURE, into_brackets(&self.positioning));
        begin_figure_str = format!("{} {}", begin_figure_str, into_label(&self.label));
        write_indented_line(&doc, &self.indent, &begin_figure_str)
    }

    fn build_caption(&self, doc: &Document, inner_indent: &usize) -> Result<(), Error> {
        match &self.caption {
            Some(caption) => {
                let caption_str = format!("{}{}", DEF_CAPTION, into_braces(&caption.get_string()));
                write_indented_line(&doc, inner_indent, &caption_str)
            }
            None => Ok(())
        }
    }

    fn build_centering(&self, doc: &Document, inner_indent: &usize) -> Result<(), Error> {
        if self.centered {
            write_indented_line(&doc, inner_indent, DEF_CENTERING)?;
        }

        Ok(())
    }

    fn build_graphic(&self, doc: &Document, inner_indent: &usize) -> Result<(), Error> {
        let include_graph_str = format!("{}{}{}", DEF_INCLUDE_GRAPH,
                                        into_brackets(&self.image_option),
                                        into_braces(&self.image_path));

        write_indented_line(&doc, inner_indent, &include_graph_str)
    }

    fn build_end(&self, doc: &Document) -> Result<(), Error> {
        write_indented_line(doc, &self.indent, DEF_END_FIGURE)?;
        doc.add_blank_line()
    }
}

impl Item for Figure {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        let inner_indent: &usize = &(self.indent + 1);

        self.build_header(doc)?;
        self.build_centering(doc, inner_indent)?;
        self.build_graphic(doc, inner_indent)?;
        self.build_caption(doc, inner_indent)?;
        self.build_end(doc)
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent= super_indent + 1;
    }
}

/// An object to add any LaTeX commands to an `Item` or a `Document`.
pub struct Command {
    command: String,
    indent: usize
}

impl Command {
    /// Initializes a new `Command` object
    pub fn new(_command: &str) -> Self {
        Self {
            command: String::from(_command),
            indent: 0
        }
    }
}

impl Item for Command {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        write_indented_line(&doc, &self.indent, &self.command)
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent= super_indent + 1;
    }
}

/// An object to add a page jump to a `Document`.
pub struct PageBreak {
    indent: usize
}

impl PageBreak {
    /// Initializes a new `PageBreak` object
    pub fn new() -> Self {
        Self {
            indent: 0
        }
    }
}

impl Item for PageBreak {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        write_indented_line(&doc, &self.indent, DEF_NEW_PAGE)?;
        doc.add_blank_line()
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent= super_indent + 1;
    }

}