//! Markdown extensions defined by GitHub.
//!
//! The full specification can be found at https://github.github.com/gfm/.

use std::fmt;

/// A checkmark list item.
#[derive(Clone, Debug, Default)]
pub struct CheckmarkItem<'a> {
    /// The state of the item.
    ///
    /// `True` if the item is checked, `false` otherwise.
    pub checked: bool,
    /// The text of the checkmark item.
    pub text: &'a str,
}

impl<'a> CheckmarkItem<'a> {
    /// Creates a new default checkmark item.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a mew checkmark item with the given values.
    pub fn from(text: &'a str, checked: bool) -> Self {
        Self { text, checked }
    }
}

impl fmt::Display for CheckmarkItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let checkmark = if self.checked { "x" } else { " " };
        write!(f, "[{}] {}", checkmark, self.text)
    }
}

/// An extension trait for strikethrough transformations.
pub trait Strikethrough {
    /// Transforms the given text to be strikethrough.
    ///
    /// # Example
    ///
    /// ```rust
    /// use markdown_builder::extensions::github::Strikethrough;
    ///
    /// let text = "text";
    /// let striked = text.to_strikethrough();
    /// assert_eq!(striked, "~text~");
    /// ```
    fn to_strikethrough(&self) -> String;
}

impl<T> Strikethrough for T
where
    T: AsRef<str>,
{
    fn to_strikethrough(&self) -> String {
        format!("~{}~", self.as_ref())
    }
}
