mod components;

use components::document::*;
use std::fs::File;

const DOCUMENT_NAME: &str = "tex_files/main.tex";
const DOCUMENT_CLASS: &str = "article";
const FONT_SIZE: &str = "12pt";

fn main() -> std::io::Result<()>
{
    let doc_file: File = File::create(DOCUMENT_NAME)?;

    let doc_class: DocumentClass = DocumentClass::new
    (
        DOCUMENT_CLASS.to_string(),
        [FONT_SIZE.to_string()].into()
    );

    let mut doc: Document = Document
    {
        file: doc_file,
        class: doc_class,
        packages: Vec::new(),
        items: Vec::new()
    };

    doc.build();

    Ok(())
}
