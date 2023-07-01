mod components;
mod utilities;

use components::document::*;
use components::item::*;
use components::container::*;
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
        },
        Package
        {
            name: String::from("fontenc"),
            options: LinkedList::from([String::from("T1")])
        }
    ]);

    doc.add_packages(packages);

    let mut section_1: Section = Section::new(
        String::from("Section"),
        SectionType::Section,
        true
    );

    let paragraph_1 = Paragraph::new(
        String::from(
            "This is a paragraph test. Lets see if
            writing on many lines still works.
            It seems like it is working!")
        );

    let mut sub_section_1: Section = Section::new(
        String::from("SubSection"),
        SectionType::SubSection,
        false
    );

    let paragraph_2 = Paragraph::new(
        String::from(
            "This is a paragraph. I can write anything.
            BLABALABLABALABALABALABALABALABALABALABA
            ABHDLBFKNSABFHISKA F fjakfbjsab jfakfjsa. Nice!")
        );

    let paragraph_3 = Paragraph::new(
        String::from(
            "This is another paragraph. I can also write anything.
            BLABALABLABALABALABALABALABALABALABALABA
            ABHDLBFKNSABFHISKA F fjakfbjsab jfakfjsa.")
        );

    let mut block_1: Block = Block::new();
    block_1.add_item(paragraph_2);
    block_1.add_item(paragraph_3);

    let mut enumerate_1: Enumerate = Enumerate::new();
    enumerate_1.add_item(block_1);

    sub_section_1.add_item(enumerate_1);

    section_1.add_item(paragraph_1);
    section_1.add_item(sub_section_1);

    doc.add_item(section_1);

    doc.build()
}
