use crate::traits::{AsFooter, MarkdownElement};
use std::fmt;

/// A markdown link.
#[derive(Clone, Debug, Default)]
pub struct Link {
    /// The text of the link.
    pub text: String,
    /// The url of the link.
    pub url: String,
    /// Whether the `Link`'s url should be rendered as a footer.
    pub footer: bool,
    /// Whether the link should be inlined (no new line).
    pub inlined: bool,
}

impl Link {
    /// Creates a new default `Link`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `Link` with the given values.
    pub fn from(
        text: impl Into<String>,
        url: impl Into<String>,
        footer: bool,
        inlined: bool,
    ) -> Self {
        Self {
            text: text.into(),
            url: url.into(),
            footer,
            inlined,
        }
    }
}

impl AsFooter for Link {
    fn as_footer(&self) -> Box<dyn MarkdownElement> {
        Box::new(format!("[{}]: {}", self.text, self.url))
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = if self.footer {
            format!("[{}][{}]", self.text, self.text)
        } else {
            format!("[{}]({})", self.text, self.url)
        };

        if self.inlined {
            write!(f, "{}", text)
        } else {
            writeln!(f, "{}", text)
        }
    }
}
