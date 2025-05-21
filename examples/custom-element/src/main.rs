use markdown_builder::Markdown;
use std::fmt;

/// Table of Contents section for Vitepress
#[derive(Clone, Debug)]
struct TableOfContents {}

impl fmt::Display for TableOfContents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[[toc]]")
    }
}

fn main() {
    let mut doc = Markdown::new();
    doc.h1("Example document with table of contents");
    doc.add(TableOfContents {});

    println!("{}", doc.render())
    // # Example document with table of contents
    //
    // [[toc]]
}
