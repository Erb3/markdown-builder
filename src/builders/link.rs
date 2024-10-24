use crate::types::link::Link;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LinkBuilder {
    text: Option<String>,
    url: Option<String>,
    footer: bool,
    inlined: bool,
}

impl LinkBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn footer(mut self) -> Self {
        self.footer = true;
        self
    }

    pub fn set_footer(mut self, value: bool) -> Self {
        self.footer = value;
        self
    }

    pub fn inlined(mut self) -> Self {
        self.inlined = true;
        self
    }

    pub fn set_inlined(mut self, value: bool) -> Self {
        self.inlined = value;
        self
    }

    pub fn build(self) -> Link {
        if self.url.is_none() {
            panic!("Attempt to build link without target URL")
        }

        if self.text.is_none() {
            panic!("Attempt to build link without text")
        }

        Link::from(
            self.url.unwrap(),
            self.text.unwrap(),
            self.footer,
            self.inlined,
        )
    }
}

impl Link {
    pub fn builder() -> LinkBuilder {
        LinkBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_builder_text_url() {
        let link = Link::builder()
            .url("https://www.rust-lang.org/")
            .text("A cool website")
            .build();

        assert_eq!(link.url, "https://www.rust-lang.org/");
        assert_eq!(link.text, "A cool website");
        assert_eq!(link.footer, false);
        assert_eq!(link.inlined, false);
    }

    #[test]
    fn test_link_builder_text_url_footer() {
        let link = Link::builder()
            .url("https://www.rust-lang.org/")
            .text("A cool website")
            .footer()
            .build();

        assert_eq!(link.url, "https://www.rust-lang.org/");
        assert_eq!(link.text, "A cool website");
        assert_eq!(link.footer, true);
        assert_eq!(link.inlined, false);
    }

    #[test]
    fn test_link_builder_text_url_set_footer() {
        let link = Link::builder()
            .url("https://www.rust-lang.org/")
            .text("A cool website")
            .set_footer(true)
            .build();

        assert_eq!(link.url, "https://www.rust-lang.org/");
        assert_eq!(link.text, "A cool website");
        assert_eq!(link.footer, true);
        assert_eq!(link.inlined, false);

        let link_footerless = Link::builder()
            .url("https://www.rust-lang.org/")
            .text("A cool website")
            .set_footer(false)
            .build();

        assert_eq!(link_footerless.url, "https://www.rust-lang.org/");
        assert_eq!(link_footerless.text, "A cool website");
        assert_eq!(link_footerless.footer, false);
        assert_eq!(link_footerless.inlined, false);
    }

    #[test]
    fn test_link_builder_text_url_inlined() {
        let link = Link::builder()
            .url("https://www.rust-lang.org/")
            .text("A cool website")
            .inlined()
            .build();

        assert_eq!(link.url, "https://www.rust-lang.org/");
        assert_eq!(link.text, "A cool website");
        assert_eq!(link.footer, false);
        assert_eq!(link.inlined, true);
    }

    #[test]
    fn test_link_builder_text_url_set_inlined() {
        let link_inlined = Link::builder()
            .url("https://www.rust-lang.org/")
            .text("A cool website")
            .set_footer(true)
            .set_inlined(true)
            .build();

        assert_eq!(link_inlined.url, "https://www.rust-lang.org/");
        assert_eq!(link_inlined.text, "A cool website");
        assert_eq!(link_inlined.footer, true);
        assert_eq!(link_inlined.inlined, true);

        let link = Link::builder()
            .url("https://www.rust-lang.org/")
            .text("A cool website")
            .set_footer(false)
            .set_inlined(false)
            .build();

        assert_eq!(link.url, "https://www.rust-lang.org/");
        assert_eq!(link.text, "A cool website");
        assert_eq!(link.footer, false);
        assert_eq!(link.inlined, false);
    }

    #[test]
    #[should_panic]
    fn test_link_builder_no_url_panic() {
        Link::builder().url("https://www.rust-lang.org/").build();
    }

    #[test]
    #[should_panic]
    fn test_link_builder_no_text_panic() {
        Link::builder().text("A cool website").build();
    }
}
