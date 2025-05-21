# Example: Custom element

This example creates a custom Markdown element called `TableOfContents`, and adds it to the document. The important part
to notice is implementing `std::fmt::Display` to satisfy `document.add()`.

```shell
$ cargo run
# Example document with table of contents

[[toc]]
```
