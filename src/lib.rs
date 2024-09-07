//! `markdown-builder` helps you to create Markdown documents using Rust code.
//!
//! The main API involves around using builders to create structured Markdown
//! elements that can be combined together into a Markdown document
//!
//! ```rust
//! use markdown_builder::{Link, Markdown};
//!
//! let link = Link::builder()
//!     .text("Hello World")
//!     .url("https://hello.world")
//!     .build();
//!
//! let mut doc = Markdown::new();
//! doc.header1("First level header")
//!     .paragraph("Some standalone paragraph")
//!     .link(link)
//!     .paragraph("Some other paragraph");
//!
//! let rendered = doc.render();
//! ```

pub mod builders;
pub mod extensions;
pub mod traits;
pub mod transforms;
pub mod types;

pub use crate::{
    builders::{image::ImageBuilder, link::LinkBuilder, list::ListBuilder},
    traits::{AsFooter, MarkdownElement},
    transforms::{BlockQuote, Bold, CodeBlock, Inline, Italic},
    types::{
        header::{Header, HeaderLevel},
        image::Image,
        link::Link,
        list::{List, ListItem, ListItemMarker, ListType, NumberedListItemMarkerSeparator},
        markdown::Markdown,
        paragraph::Paragraph,
    },
};

#[cfg(feature = "extension-github")]
pub use crate::extensions::github::{CheckmarkItem, Strikethrough};
