//! `markdown-builder` helps you to create Markdown documents using Rust code.
//!
//! The main API involves around using builders to create structured Markdown
//! elements that can be combined together into a Markdown document

pub mod builders;
pub mod traits;
pub mod transforms;
pub mod types;

pub use crate::{
    builders::{image::ImageBuilder, link::LinkBuilder, list::ListBuilder},
    traits::{AsFooter, MarkdownElement},
    transforms::{BlockQuote, Bold, CodeBlock, Inline, Italic, Strikethrough},
    types::{
        header::{Header, HeaderLevel},
        image::Image,
        link::Link,
        list::{List, ListItem, ListVariant},
        markdown::Markdown,
        paragraph::Paragraph,
    },
};
