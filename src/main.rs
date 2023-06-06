mod components;

use components::document::*;
use components::item::*;
use std::{fs::File, collections::LinkedList};

const DOCUMENT_NAME: &str = "tex_files/main.tex";
const DOCUMENT_CLASS: &str = "article";
const FONT_SIZE: &str = "12pt";

fn main() -> std::io::Result<()>
{
    let doc_file: File = File::create(DOCUMENT_NAME)?;

    let doc_class: DocumentClass = DocumentClass::new(
        DOCUMENT_CLASS.to_string(),
        LinkedList::from([FONT_SIZE.to_string()])
    );

    let mut doc: Document = Document::new(doc_file, doc_class);

    let packages = Vec::<Package>::from(
    [
        Package
        {
            name: String::from("babel"),
            options: LinkedList::from([String::from("french")])
        },
        Package
        {
            name: String::from("geometry"),
            options: LinkedList::from([String::from("margin=2.5cm")])
        }
    ]);

    doc.add_packages(packages);

    let paragraph_1 = Paragraph
    {
        text: String::from(
            "This is a paragraphe test. Lets see if
             writing on many lines still works.
             It seems like it is working!")
    };
    doc.add_item(paragraph_1);

    doc.build()
}
