#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;
use tousize::ToUsize;

/// The level of a header.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    /// # Panics
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
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    /// # Panics
    ///
    /// Panics if the header level is not valid (one to six inclusive).
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

    #[test]
    fn test_header_level() {
        let level = 5usize;
        assert_eq!(HeaderLevel::from(5usize), level.into());
    }

    #[test]
    #[should_panic]
    fn test_header_level_panic() {
        let _: HeaderLevel = 0usize.into();
    }

    #[test]
    fn test_header() {
        let content = "Some header content";
        let level = 5usize;
        let header = Header::from(content, level);
        assert_eq!(header.text, content);
        assert_eq!(header.level, level.into());
    }
}
