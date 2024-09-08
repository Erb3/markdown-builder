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
    pub fn from(url: impl Into<String>, text: impl Into<String>, footer: bool) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_default() {
        let image = Image::new();
        assert_eq!(image.footer, false);
        assert_eq!(image.text, "");
        assert_eq!(image.url, "");
    }

    #[test]
    fn test_image_from() {
        let image = Image::from(
            "https://example.com/picture.png",
            "A cute image of a sandcat",
            true,
        );
        assert_eq!(image.footer, true);
        assert_eq!(image.text, "A cute image of a sandcat");
        assert_eq!(image.url, "https://example.com/picture.png");
    }

    #[test]
    fn test_image_url() {
        assert_eq!(
            Image::from("https://example.com/picture.png", "", false).render(),
            "![](https://example.com/picture.png)\n"
        );
    }

    #[test]
    fn test_image_url_and_text() {
        assert_eq!(
            Image::from(
                "https://example.com/picture.png",
                "A cute picture of a sandcat",
                false
            )
            .render(),
            "![A cute picture of a sandcat](https://example.com/picture.png)\n"
        );
    }

    #[test]
    fn test_image_url_text_footer() {
        let image = Image::from(
            "https://example.com/picture.png",
            "A cute picture of a sandcat",
            true,
        );
        assert_eq!(
            image.render(),
            "![A cute picture of a sandcat][A cute picture of a sandcat]\n"
        );
        assert_eq!(
            image.as_footer().render(),
            "[A cute picture of a sandcat]: https://example.com/picture.png"
        )
    }
}
