use crate::{
    traits::{AsFooter, MarkdownElement},
    types::{header::Header, link::Link, list::List, paragraph::Paragraph},
    Image,
};
use std::fmt;
use tousize::ToUsize;

/// A markdown document.
#[derive(Default)]
pub struct Markdown {
    /// The markdown elements.
    pub elements: Vec<Box<dyn MarkdownElement>>,
    /// The markdown footer elements.
    pub footers: Vec<Box<dyn MarkdownElement>>,
}

impl Markdown {
    /// Creates a new default `Markdown` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `Markdown` instance with the given elements and footers.
    pub fn with(
        elements: Vec<Box<dyn MarkdownElement>>,
        footers: Vec<Box<dyn MarkdownElement>>,
    ) -> Self {
        Self { elements, footers }
    }

    /// Adds any generic markdown element.
    pub fn add(&mut self, element: impl MarkdownElement + 'static) -> &mut Self {
        self.elements.push(Box::new(element));
        self
    }

    /// Adds a header to the document.
    ///
    /// # Arguments
    ///
    /// - `text`: The header's text.
    /// - `level`: The header's level.
    ///
    /// # Panics
    ///
    /// Panics if the header level is not valid (one to six inclusive).
    pub fn header(&mut self, text: impl Into<String>, level: impl ToUsize) -> &mut Self {
        let header = Header::from(text, level);
        self.elements.push(Box::new(header));
        self
    }

    /// Adds a header with level 1 to the document.
    ///
    /// # Arguments
    ///
    /// - `text`: The header's text.
    pub fn h1(&mut self, text: impl Into<String>) -> &mut Self {
        self.header(text, 1usize);
        self
    }

    /// Adds a header with level 2 to the document.
    ///
    /// # Arguments
    ///
    /// - `text`: The header's text.
    pub fn h2(&mut self, text: impl Into<String>) -> &mut Self {
        self.header(text, 2usize);
        self
    }

    /// Adds a header with level 3 to the document.
    ///
    /// # Arguments
    ///
    /// - `text`: The header's text.
    pub fn h3(&mut self, text: impl Into<String>) -> &mut Self {
        self.header(text, 3usize);
        self
    }

    /// Adds a header with level 4 to the document.
    ///
    /// # Arguments
    ///
    /// - `text`: The header's text.
    pub fn h4(&mut self, text: impl Into<String>) -> &mut Self {
        self.header(text, 4usize);
        self
    }

    /// Adds a header with level 5 to the document.
    ///
    /// # Arguments
    ///
    /// - `text`: The header's text.
    pub fn h5(&mut self, text: impl Into<String>) -> &mut Self {
        self.header(text, 5usize);
        self
    }

    /// Adds a header with level 6 to the document.
    ///
    /// # Arguments
    ///
    /// - `text`: The header's text.
    pub fn h6(&mut self, text: impl Into<String>) -> &mut Self {
        self.header(text, 6usize);
        self
    }

    /// Adds a list to the document.
    ///
    /// # Arguments
    ///
    /// - `list`: The list instance to add.
    pub fn list(&mut self, list: List) -> &mut Self {
        self.elements.push(Box::new(list));
        self
    }

    /// Adds a link to the document.
    ///
    /// # Arguments
    ///
    /// - `link`: The link instance to add.
    ///
    /// # Note
    ///
    /// The associated footer element is added as well if the passed link is
    /// marked as footer.
    pub fn link(&mut self, link: Link) -> &mut Self {
        if link.footer {
            self.footers.push(link.as_footer());
        }
        self.elements.push(Box::new(link));
        self
    }

    /// Adds an image to the document.
    ///
    /// ### Argument
    ///
    /// - `image`: The image instance to add.
    ///
    /// # Note
    ///
    /// The associated footer element is added as well if the passed link is
    /// marked as footer.
    pub fn image(&mut self, image: Image) -> &mut Self {
        if image.footer {
            self.footers.push(image.as_footer());
        }
        self.elements.push(Box::new(image));
        self
    }

    /// Adds a paragraph to the document.
    ///
    /// # Arguments
    ///
    /// - `text`: The paragraph's text.
    pub fn paragraph(&mut self, text: impl Into<String>) -> &mut Self {
        self.elements.push(Box::new(Paragraph::from(text)));
        self
    }

    /// Renders the markdown document to a `String`.
    ///
    /// The method does render each
    /// [element](struct.Markdown.structfield.elements) in order, followed by
    /// each [footer](struct.Markdown.structfield.footers).
    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Markdown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, element) in self.elements.iter().enumerate() {
            if index == self.elements.len() - 1 {
                write!(f, "{}", element.render())?;
            } else {
                writeln!(f, "{}", element.render())?;
            }
        }

        if !self.footers.is_empty() {
            writeln!(f, "")?;
        }

        for footer in &self.footers {
            writeln!(f, "{}", footer.render())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ImageBuilder;

    #[test]
    fn document_with_one_paragraph() {
        assert_eq!(
            Markdown::new().paragraph("Hello World").render(),
            "Hello World\n"
        );
    }

    #[test]
    fn document_with_two_paragraphs() {
        assert_eq!(
            Markdown::new()
                .paragraph("Hello World")
                .paragraph("Two paragraphs")
                .render(),
            "Hello World\n\nTwo paragraphs\n"
        );
    }

    #[test]
    fn document_with_image() {
        let mut doc = Markdown::new();
        doc.image(
            ImageBuilder::new()
                .url("https://example.com/picture.png")
                .text("A cute picture of a sandcat")
                .build(),
        );

        assert_eq!(
            doc.render(),
            "![A cute picture of a sandcat](https://example.com/picture.png)\n"
        );
    }

    #[test]
    fn document_with_image_footer() {
        let mut doc = Markdown::new();
        doc.image(
            ImageBuilder::new()
                .url("https://example.com/picture.png")
                .text("A cute picture of a sandcat")
                .footer()
                .build(),
        );

        assert_eq!(doc.render(), "![A cute picture of a sandcat][A cute picture of a sandcat]\n\n[A cute picture of a sandcat]: https://example.com/picture.png\n");
    }
}
