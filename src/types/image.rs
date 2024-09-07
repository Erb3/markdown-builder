use crate::traits::{AsFooter, MarkdownElement};
use std::fmt;

/// A markdown image.
#[derive(Clone, Debug, Default)]
pub struct Image {
    /// Whether the image's link should be added as a footer reference.
    pub footer: bool,
    /// The text of the image.
    pub text: String,
    /// The url of the image.
    pub url: String,
}

impl Image {
    /// Creates a new default `Image`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `Image` with the given values.
    pub fn from(text: impl Into<String>, url: impl Into<String>, footer: bool) -> Self {
        Self {
            text: text.into(),
            url: url.into(),
            footer,
        }
    }
}

impl AsFooter for Image {
    fn as_footer(&self) -> Box<dyn MarkdownElement> {
        Box::new(format!("[{}]: {}", self.text, self.url))
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.footer {
            writeln!(f, "![{}][{}]", self.text, self.text)
        } else {
            writeln!(f, "![{}]({})", self.text, self.url)
        }
    }
}
