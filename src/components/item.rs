use std::io::Error;
use super::document::*;
use crate::utilities::format::*;
use crate::utilities::def_syntax::*;


pub trait Item {
    fn build(&self, doc: &Document) -> Result<(), Error>;
    fn update_indent(&mut self, super_indent: &usize);
}

pub struct Text {
    text: String,
    indent: usize
}

impl Text {
    pub fn new(_text: String) -> Self {
        Self {
            text: handle_text_format(_text),
            indent: 0
        }
    }

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

pub struct Figure {
    positioning: String,
    centered: bool,
    image_path: String,
    image_option: String,
    caption: Option<Text>,
    indent: usize
}

impl Figure {
    pub fn new(_positioning: String, _centered: bool, _image_path: String,
               _image_option: String, _caption: Option<Text>) -> Self {
        Self {
            positioning: _positioning,
            centered: _centered,
            image_path: _image_path,
            image_option: _image_option,
            caption: _caption,
            indent: 0
        }
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

    fn build_centering (&self, doc: &Document, inner_indent: &usize) -> Result<(), Error> {
        if self.centered {
            write_indented_line(&doc, inner_indent, DEF_CENTERING)?;
        }

        Ok(())
    }

    fn build_graphic (&self, doc: &Document, inner_indent: &usize) -> Result<(), Error> {
        let include_graph_str = format!("{}{}{}", DEF_INCLUDE_GRAPH,
                                        into_brackets(&self.image_option),
                                        into_braces(&self.image_path));

        write_indented_line(&doc, inner_indent, &include_graph_str)
    }
}

impl Item for Figure {
    fn build(&self, doc: &Document) -> Result<(), Error> {
        let inner_indent: &usize = &(self.indent + 1);

        let begin_figure_str = format!("{}{}", DEF_BEGIN_FIGURE, into_brackets(&self.positioning));
        write_indented_line(&doc, &self.indent, &begin_figure_str)?;

        self.build_centering(&doc, inner_indent)?;
        self.build_graphic(&doc, inner_indent)?;
        self.build_caption(&doc, inner_indent)?;

        write_indented_line(&doc, &self.indent, DEF_END_FIGURE)?;
        doc.add_blank_line()
    }

    fn update_indent(&mut self, super_indent: &usize) {
        self.indent= super_indent + 1;
    }
}