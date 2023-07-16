mod components;
mod utilities;

use components::document::*;
use components::item::*;
use components::container::*;
use components::table::*;
use std::fs::File;

const DOCUMENT_NAME: &str = "generated_tex/main.tex";
const DOCUMENT_CLASS: ClassType = ClassType::Report;
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

    let commands = vec![
        Command::new(String::from(r"\title{Title}")),
        Command::new(String::from(r"\author{Author}")),
        Command::new(String::from(r"\date{YYYY / MM / DD}")),
        Command::new(String::from(r"\setlength{\tabcolsep}{18pt}"))
    ];

    doc.add_global_commands(commands);

    doc.add_item(Command::new(String::from(r"\maketitle")));
    doc.add_item(Command::new(String::from(r"\tableofcontents")));

    let mut chapter_1 = Chapter::new(String::from("Lorem Ipsum"), true);

    let mut section_1: Section = Section::new(
        String::from("Section"),
        SectionType::Section,
        true
    );

    let paragraph_1 = Text::new(
        String::from(
            "Lorem ipsum dolor sit amet, **consectetur** adipiscing elit. Integer congue nisi condimentum
            lacus vulputate cursus. _Curabitur_ bibendum orci ac nibh vestibulum ultrices. Aenean pulvinar
            mattis lectus, sed vehicula leo pellentesque eget. Sed sed quam sit amet nulla lacinia mollis.
            Maecenas dignissim, augue quis suscipit pellentesque, ipsum turpis facilisis eros, eu aliquam
            erat massa sit amet ex."
        )
    );

    let mut sub_section_1: Section = Section::new(
        String::from("SubSection"),
        SectionType::SubSection,
        false
    );

    let paragraph_2 = Text::new(
        String::from(
            "Sed ut augue vel arcu varius accumsan. Quisque ipsum risus, pulvinar in aliquet sodales,
            aliquet quis odio. Quisque accumsan bibendum egestas. Nullam vel est faucibus, egestas
            urna in, tempor risus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices
            posuere cubilia curae"
        )
    );

    let figure_1 = Figure::new(
        String::from("H"),
        true,
        String::from("lorem_ipsum.jpg"),
        String::from("scale=0.35"),
        Some(Text::new(String::from("**Lorem** _Ipsum_")))
    );

    let mut block_1: Block = Block::new();
    block_1.add_item(paragraph_2);
    block_1.add_item(figure_1);

    let mut enumerate_1: Enumerate = Enumerate::new();
    enumerate_1.add_item(block_1);

    sub_section_1.add_item(enumerate_1);

    section_1.add_item(paragraph_1);
    section_1.add_item(sub_section_1);

    let mut table_1: Table = Table::new(
        String::from("H"),
        String::from("|c|c|c|"),
        true,
        Some(Text::new(String::from("Random caption")))
    );

    table_1.add_component(HorizontalLine::new());
    let table_row_1 = TableRow::new(vec![Text::new(String::from(r"\multicolumn{3}{|c|}{**Lorem Ipsum**}"))]);
    table_1.add_component(table_row_1);
    table_1.add_component(HorizontalLine::new());
    let table_row_2 = TableRow::new(vec![
        Text::new(String::from("First")),
        Text::new(String::from("Second")),
        Text::new(String::from("Third"))]
    );
    table_1.add_component(table_row_2);
    table_1.add_component(HorizontalLine::new());
    let table_row_3 = TableRow::new(vec![
        Text::new(String::from("Test 1")),
        Text::new(String::from("Test 2")),
        Text::new(String::from("Test 3"))]
    );
    table_1.add_component(table_row_3);
    table_1.add_component(HorizontalLine::new());

    section_1.add_item(table_1);

    chapter_1.add_item(section_1);

    doc.add_item(chapter_1);

    doc.build()
}
