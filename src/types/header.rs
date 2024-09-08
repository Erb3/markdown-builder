use std::fmt;
use tousize::ToUsize;

/// The level of a header.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeaderLevel(usize);

impl Default for HeaderLevel {
    /// Returns the default header level (1).
    fn default() -> Self {
        HeaderLevel(1)
    }
}

impl HeaderLevel {
    /// Creates a new default header level of 1.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new header level.
    ///
    /// Panics if the header level is not valid (one to six inclusive).
    pub fn from(level: impl ToUsize) -> Self {
        let level = level.to_usize();
        assert!((1..=6).contains(&level));
        Self(level)
    }
}

impl<T> From<T> for HeaderLevel
where
    T: ToUsize,
{
    fn from(value: T) -> Self {
        Self::from(value)
    }
}

/// A markdown header.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Header {
    /// The header text.
    pub text: String,
    /// The header level.
    pub level: HeaderLevel,
}

impl Header {
    /// Creates a new empty header with a level of 1.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new header.
    ///
    /// Panics if the header level is not valid (one to six inclusive).
    /// Lower level means more important header.
    pub fn from(text: impl Into<String>, level: impl Into<HeaderLevel>) -> Self {
        Self {
            text: text.into(),
            level: level.into(),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}", "#".repeat(self.level.0), self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::{Header, HeaderLevel};
    use crate::MarkdownElement;

    #[test]
    fn test_header_level() {
        assert_eq!(HeaderLevel::from(5usize), 5usize.into());
    }

    #[test]
    #[should_panic]
    fn test_header_level_panic_lowball() {
        HeaderLevel::from(0usize);
    }

    #[test]
    #[should_panic]
    fn test_header_level_panic_highball() {
        HeaderLevel::from(7usize);
    }

    #[test]
    fn test_header_of_all_sizes() {
        assert_eq!(Header::from("A header", 1usize).render(), "# A header\n");
        assert_eq!(Header::from("A header", 2usize).render(), "## A header\n");
        assert_eq!(Header::from("A header", 3usize).render(), "### A header\n");
        assert_eq!(Header::from("A header", 4usize).render(), "#### A header\n");
        assert_eq!(
            Header::from("A header", 5usize).render(),
            "##### A header\n"
        );
        assert_eq!(
            Header::from("A header", 6usize).render(),
            "###### A header\n"
        );
    }

    #[test]
    fn test_header_default() {
        assert_eq!(Header::new().level, 1usize.into());
        assert_eq!(Header::new().text, "");
    }
}
