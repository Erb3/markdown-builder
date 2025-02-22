use crate::types::image::Image;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ImageBuilder {
    text: Option<String>,
    url: Option<String>,
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
        self.text = Some(text.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn build(self) -> Image {
        if self.url.is_none() {
            panic!("Attempt to build image without source URL")
        }

        if self.footer && self.text.is_none() {
            panic!("Attempt to bulid image with URL in footer without image alt-text");
        }

        Image::from(
            self.url.unwrap(),
            self.text.unwrap_or("".into()),
            self.footer,
        )
    }
}

impl Image {
    pub fn builder() -> ImageBuilder {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_builder_url_footer() {
        let image = Image::builder()
            .url("https://example.com/picture.png")
            .text("A cute picture of a sandcat")
            .footer()
            .build();

        assert_eq!(image.footer, true);
        assert_eq!(image.url, "https://example.com/picture.png");
        assert_eq!(image.text, "A cute picture of a sandcat");
    }

    #[test]
    fn test_image_builder_url_set_footer() {
        let footer = Image::builder()
            .url("https://example.com/picture.png")
            .text("A cute picture of a sandcat")
            .set_footer(true)
            .build();

        assert_eq!(footer.footer, true);
        assert_eq!(footer.url, "https://example.com/picture.png");
        assert_eq!(footer.text, "A cute picture of a sandcat");

        let no_footer = Image::builder()
            .url("https://example.com/picture.png")
            .text("A cute picture of a sandcat")
            .set_footer(false)
            .build();

        assert_eq!(no_footer.footer, false);
        assert_eq!(no_footer.url, "https://example.com/picture.png");
        assert_eq!(no_footer.text, "A cute picture of a sandcat");
    }

    #[test]
    #[should_panic]
    fn test_image_builder_no_url_panic() {
        Image::builder().text("Hello world").build();
    }

    #[test]
    #[should_panic]
    fn test_image_builder_footer_no_alt_text_panic() {
        Image::builder()
            .url("https://example.com/picture.png")
            .footer()
            .build();
    }
}
