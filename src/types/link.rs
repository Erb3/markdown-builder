use crate::traits::{AsFooter, MarkdownElement};
use std::fmt;

/// A markdown link.
#[derive(Clone, Debug, Default)]
pub struct Link {
    /// The url of the link.
    pub url: String,
    /// The text of the link.
    pub text: String,
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
        url: impl Into<String>,
        text: impl Into<String>,
        footer: bool,
        inlined: bool,
    ) -> Self {
        Self {
            url: url.into(),
            text: text.into(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_default() {
        let link = Link::new();
        assert_eq!(link.url, "");
        assert_eq!(link.text, "");
        assert_eq!(link.footer, false);
        assert_eq!(link.inlined, false);
    }

    #[test]
    fn test_link_from() {
        let link = Link::from("https://example.com", "example.com", false, true);
        assert_eq!(link.url, "https://example.com");
        assert_eq!(link.text, "example.com");
        assert_eq!(link.footer, false);
        assert_eq!(link.inlined, true);
    }

    #[test]
    fn test_link_url_text() {
        assert_eq!(
            Link::from("https://example.com", "example.com", false, false).render(),
            "[example.com](https://example.com)\n"
        );
    }

    #[test]
    fn test_link_url_text_inline() {
        assert_eq!(
            Link::from("https://example.com", "example.com", false, true).render(),
            "[example.com](https://example.com)"
        );
    }

    #[test]
    fn test_link_url_text_footer() {
        let link = Link::from("https://example.com", "example.com", true, false);

        assert_eq!(link.render(), "[example.com][example.com]\n");
        assert_eq!(
            link.as_footer().render(),
            "[example.com]: https://example.com"
        )
    }
}
