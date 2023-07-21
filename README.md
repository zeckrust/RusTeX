# RusTeX

A library to make simple auto-generated  LaTeX files in Rust.

- [Quick Start](#quick-start)
- [Documentation](#documentation)
- [Implemented Features](#implemented-features)
- [Example](#example)

## Quick start

To use `RusTeX`, add the crate to your `Cargo.toml`.

```toml
[dependencies]
rustex = "0.1.0"
```

## Documentation

The documentation is accessible [here]().

## Implemented Features

### _Components_

- Package
- Command
- Chapter
- Section
- Enumerate
- Table
- Figure
- Text
- Label
- PageBreak

### _Formatting_

- Markdown **bold**
- Markdown _italic_
- Color text

## Example

A full example with the resulting PDF file is accessible in the **_example_** folder.

1. Start by creating a base `Document`

    ```rust
    const DOCUMENT_NAME: &str = "generated_tex/main.tex";
    const DOCUMENT_CLASS: ClassType = ClassType::Report;
    const FONT_SIZE: &str = "12pt";

    let doc_file: File = File::create(DOCUMENT_NAME).unwrap();

    let doc_class: DocumentClass = DocumentClass::new(
        DOCUMENT_CLASS,
        vec![FONT_SIZE]
    );

    let mut doc: Document = Document::new(doc_file, doc_class);
    ```

2. Add some `Packages`

    ```rust
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
    ```

3. Add some global `Commands`

    ```rust
    let commands = vec![
        Command::new(r"\title{Title}"),
        Command::new(r"\author{Author}"),
        Command::new(r"\date{YYYY / MM / DD}"),
        Command::new(r"\setlength{\tabcolsep}{18pt}")
    ];

    doc.add_global_commands(commands);
    ```

4. Then you can add different `Items`
    - Any `Item` can be added to a `Document`
    - Any `Item` can be added to a `Container`
    - A `Container` is an `Item`, so they can be nested
    - `Items` are built by order that they have been added

        ```rust
        let mut section_1: Section = Section::new(
            "Section",             // Section name
            SectionType::Section,  // Section type
            true,                  // Display section number
            "sec_1"                // Label
        );

        let paragraph_1 = Text::new(
            "Lorem ipsum dolor sit amet, **consectetur** adipiscing elit. Integer congue nisi condimentum
            lacus vulputate cursus. _Curabitur_ bibendum orci ac nibh vestibulum ultrices. Aenean pulvinar
            mattis lectus, sed vehicula leo pellentesque eget. Sed sed quam sit amet nulla lacinia mollis.
            Maecenas dignissim, augue quis suscipit pellentesque, ipsum turpis facilisis eros, eu aliquam
            erat massa sit amet ex."
        );

        section_1.add_item(paragraph_1);
        doc.add_item(section_1);
        ```
