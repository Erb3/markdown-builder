//! Contains builders for all Markdown content types.
//!
//! The API provided inside this module can be used to declare Markdown files in
//! an imperative style:
//!
//! ```rust
//! use markdown_builder::{Link, List, Markdown};
//!
//! let rendered = Markdown::new()
//!     .header1("Header 1")
//!     .paragraph("Some text that gets displayed")
//!     .list(List::builder()
//!         .append("Item 1")
//!         .append("Item 2")
//!         .append("Item 3")
//!         .ordered())
//!     .header2("Sub Header 1")
//!     .link(Link::builder()
//!         .text("Hello world examles")
//!         .url("https://hello.world")
//!         .build())
//!     .render();
//! ```

pub mod image;
pub mod link;
pub mod list;
