# markdown-builder

> Opinionated Rust crate for programmatically building markdown documents.

This project is based of the [markdown-composer](https://github.com/mainrs/markdown-composer-rs) crate by [mainrs](https://github.com/mainrs).

```shell
cargo add markdown-builder
```

## Features

- Headers
- Paragraphs
  - Word wrapping
- Images
  - Footer
  - Alt text
- Links
  - Footer
- Lists
  - Ordered
  - Unordered
  - Checkmarks
- Transformations
  - Italic
  - Bold
  - Strikethrough
  - Codeblocks
  - Inline code
  - Blockquote

## Example

To fully get the gripe of markdown-builder, a good starting point is the unit tests.
Here however, is a simple example.

```rust
let mut doc = Markdown::new();

doc.image(
  ImageBuilder::new()
    .url("https://example.com/picture.png")
    .text("A cute picture of a sandcat")
    .footer()
    .build(),
);

doc.paragraph("Hello World");

println!("output: {}", doc.render())
```

## Why fork?

This project was forked out of markdown-composer due to a multitude of reasons:

- **Fun**
- Markdown-composer looked unmaintained
- Not wanting the additional settings to make it work for everyone
- Word wrapping
