use std::fmt;

/// A markdown paragraph.
///
/// A paragraph is a continuous text that is visually separated from its
/// surrounding markdown elements.
#[derive(Clone, Debug)]
pub struct Paragraph {
    /// The text inside the paragraph.
    pub text: String,
}

impl Paragraph {
    /// Creates a new paragraph with the given text.
    pub fn from(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl fmt::Display for Paragraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.text)
    }
}
