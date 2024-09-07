use crate::types::image::Image;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ImageBuilder {
    text: String,
    url: String,
    footer: bool,
}

impl ImageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn footer(mut self) -> Self {
        self.footer = true;
        self
    }

    pub fn set_footer(mut self, value: bool) -> Self {
        self.footer = value;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    pub fn build(self) -> Image {
        Image::from(self.text, self.url, self.footer)
    }
}

impl Image {
    pub fn builder() -> ImageBuilder {
        ImageBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MarkdownElement;

    #[test]
    fn test_image_builder_url() {
        assert_eq!(
            Image::builder()
                .url("https://example.com/picture.png")
                .build()
                .render(),
            "![](https://example.com/picture.png)\n"
        );
    }

    #[test]
    fn test_image_builder_url_and_text() {
        assert_eq!(
            Image::builder()
                .url("https://example.com/picture.png")
                .text("A cute picture of a sandcat")
                .build()
                .render(),
            "![A cute picture of a sandcat](https://example.com/picture.png)\n"
        );
    }

    #[test]
    fn test_image_builder_url_and_footer() {
        let image = Image::builder()
            .url("https://example.com/picture.png")
            .footer()
            .build();

        assert_eq!(image.footer, true);
        assert_eq!(image.url, "https://example.com/picture.png");
    }

    #[test]
    fn test_image_builder_url_and_set_footer() {
        assert_eq!(
            Image::builder()
                .url("https://example.com/picture.png")
                .set_footer(true)
                .build()
                .footer,
            true
        );

        assert_eq!(
            Image::builder()
                .url("https://example.com/picture.png")
                .set_footer(false)
                .build()
                .footer,
            false
        );
    }
}
