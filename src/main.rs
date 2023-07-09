mod components;
mod utilities;

use components::document::*;
use components::item::*;
use components::container::*;
use std::fs::File;

const DOCUMENT_NAME: &str = "generated_tex/main.tex";
const DOCUMENT_CLASS: ClassType = ClassType::Article;
const FONT_SIZE: &str = "12pt";

fn main() -> std::io::Result<()> {
    let doc_file: File = File::create(DOCUMENT_NAME)?;

    let doc_class: DocumentClass = DocumentClass::new(
        DOCUMENT_CLASS,
        vec![FONT_SIZE.to_string()]
    );

    let mut doc: Document = Document::new(doc_file, doc_class);

    let packages = vec![
        Package::new(
            String::from("babel"),
            vec![String::from("french")]
        ),
        Package::new(
            String::from("geometry"),
            vec!["margin=2.5cm".to_string()]
        ),
        Package::new(
            String::from("fontenc"),
            vec![String::from("T1")]
        )
    ];

    doc.add_packages(packages);

    let mut section_1: Section = Section::new(
        String::from("Section"),
        SectionType::Section,
        true
    );

    let paragraph_1 = Text::new(
        String::from(
            "This is a **paragraph** test. Lets **see** if
            writing on many _lines_ still works.
            It _seems_ like it is working!"
        )
    );

    let mut sub_section_1: Section = Section::new(
        String::from("SubSection"),
        SectionType::SubSection,
        false
    );

    let paragraph_2 = Text::new(
        String::from(
            "This is a paragraph. I can write anything.
            BLABALABLABALABALABALABALABALABALABALABA
            ABHDLBFKNSABFHISKA F fjakfbjsab jfakfjsa. Nice!"
        )
    );

    let figure_1 = Figure::new(
        String::from("H"),
        true,
        String::from("itachi_sasuke.jpg"),
        String::from("scale=0.2"),
        Some(Text::new(String::from("**Itachi** and **Sasuke**")))
    );

    let mut block_1: Block = Block::new();
    block_1.add_item(paragraph_2);
    block_1.add_item(figure_1);

    let mut enumerate_1: Enumerate = Enumerate::new();
    enumerate_1.add_item(block_1);

    sub_section_1.add_item(enumerate_1);

    section_1.add_item(paragraph_1);
    section_1.add_item(sub_section_1);

    doc.add_item(section_1);

    doc.build()
}
