mod components;

use components::document::*;
use std::{fs::File, collections::LinkedList};

const DOCUMENT_NAME: &str = "tex_files/main.tex";
const DOCUMENT_CLASS: &str = "article";
const FONT_SIZE: &str = "12pt";

fn main() -> std::io::Result<()>
{
    let doc_file: File = File::create(DOCUMENT_NAME)?;

    let doc_class: DocumentClass = DocumentClass::new
    (
        DOCUMENT_CLASS.to_string(),
        LinkedList::from([FONT_SIZE.to_string()])
    );

    let mut doc: Document = Document::new(doc_file, doc_class);

    doc.build()?;

    Ok(())
}
