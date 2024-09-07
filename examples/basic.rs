use markdown_builder::{
    builders::list::ListBuilder, extensions::github::CheckmarkItem, types::markdown::Markdown, Link,
};

fn main() {
    let mut md = Markdown::new();
    for _ in 1..=10 {
        md.header1("hello").list(
            ListBuilder::new()
                .add("first")
                .add("second")
                .add(CheckmarkItem {
                    text: "Buy groceries",
                    checked: true,
                })
                .ordered(),
        );
        let link = Link::builder()
            .footer(true)
            .text("Hello")
            .inlined()
            .url("https://hello.world")
            .build();
        md.link(link);
    }
    println!("{}", md);
}
