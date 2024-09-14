use std::fmt;

/// A checkmark list item.
#[derive(Clone, Debug, Default)]
pub struct CheckmarkItem {
    /// The state of the item.
    ///
    /// `true` if the item is checked, `false` otherwise.
    pub checked: bool,
    /// The text of the checkmark item.
    pub text: String,
}

impl CheckmarkItem {
    /// Creates a new default checkmark item.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a mew checkmark item with the given values.
    pub fn from(text: impl Into<String>, checked: bool) -> Self {
        Self {
            text: text.into(),
            checked,
        }
    }
}

impl fmt::Display for CheckmarkItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let checkmark = if self.checked { "x" } else { " " };
        write!(f, "[{}] {}", checkmark, self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MarkdownElement;

    #[test]
    fn test_display() {
        assert_eq!(
            CheckmarkItem::from("Eat spaghetti", false).render(),
            "[ ] Eat spaghetti"
        );
        assert_eq!(
            CheckmarkItem::from("Eat spaghetti", true).render(),
            "[x] Eat spaghetti"
        );
    }
}
