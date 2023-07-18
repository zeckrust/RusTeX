mod components;
#[doc(hidden)]
mod utilities;

use components::document::*;
use components::item::*;
use components::container::*;
use components::table::*;
use std::fs::File;

const DOCUMENT_NAME: &str = "generated_tex/main.tex";
const DOCUMENT_CLASS: ClassType = ClassType::Report;
const FONT_SIZE: &str = "12pt";

fn main()  {
    let doc_file: File = File::create(DOCUMENT_NAME).unwrap();

    let doc_class: DocumentClass = DocumentClass::new(
        DOCUMENT_CLASS,
        vec![FONT_SIZE]
    );

    let mut doc: Document = Document::new(doc_file, doc_class);

    let packages = vec![
        Package::new(
            "babel",
            vec!["english"]
        ),
        Package::new(
            "geometry",
            vec!["margin=2.5cm"]
        ),
        Package::new(
            "fontenc",
            vec!["T1"]
        )
    ];

    doc.add_packages(packages);

    let commands = vec![
        Command::new(r"\title{Title}"),
        Command::new(r"\author{Author}"),
        Command::new(r"\date{YYYY / MM / DD}"),
        Command::new(r"\setlength{\tabcolsep}{18pt}")
    ];

    doc.add_global_commands(commands);

    doc.add_item(Command::new(r"\maketitle"));
    doc.add_item(Command::new(r"\tableofcontents"));

    let mut chapter_1 = Chapter::new("Lorem Ipsum", true);

    let mut section_1: Section = Section::new(
        "Section",
        SectionType::Section,
        true
    );

    let paragraph_1 = Text::new(
        "Lorem ipsum dolor sit amet, **consectetur** adipiscing elit. Integer congue nisi condimentum
        lacus vulputate cursus. _Curabitur_ bibendum orci ac nibh vestibulum ultrices. Aenean pulvinar
        mattis lectus, sed vehicula leo pellentesque eget. Sed sed quam sit amet nulla lacinia mollis.
        Maecenas dignissim, augue quis suscipit pellentesque, ipsum turpis facilisis eros, eu aliquam
        erat massa sit amet ex."
    );

    let mut sub_section_1: Section = Section::new(
        "SubSection",
        SectionType::SubSection,
        false
    );

    let paragraph_2 = Text::new(
        "Sed ut augue vel arcu #blue{varius accumsan}. Quisque ipsum risus, pulvinar in aliquet sodales,
        aliquet quis odio. Quisque accumsan bibendum egestas. Nullam vel est faucibus, egestas
        urna in, tempor risus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices
        posuere cubilia curae"
    );

    let figure_1 = Figure::new(
        "H",
        true,
        "lorem_ipsum.jpg",
        "scale=0.35",
        Some(Text::new("**Lorem** _Ipsum_"))
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
        "H",
        "|c|c|c|",
        true,
        Some(Text::new("Random caption"))
    );

    table_1.add_component(HorizontalLine::new());

    let table_row_1 = TableRow::new(vec![Text::new(r"\multicolumn{3}{|c|}{**Lorem Ipsum**}")]);

    table_1.add_component(table_row_1);
    table_1.add_component(HorizontalLine::new());

    let table_row_2 = TableRow::new(vec![
        Text::new("#red{First}"),
        Text::new("#violet{Second}"),
        Text::new("#teal{Third}")]
    );

    table_1.add_component(table_row_2);
    table_1.add_component(HorizontalLine::new());

    let table_row_3 = TableRow::new(vec![
        Text::new("Test 1"),
        Text::new("Test 2"),
        Text::new("Test 3")]
    );

    table_1.add_component(table_row_3);
    table_1.add_component(HorizontalLine::new());

    section_1.add_item(table_1);

    chapter_1.add_item(section_1);

    doc.add_item(chapter_1);

    doc.build();
}
