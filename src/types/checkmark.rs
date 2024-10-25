use std::fmt;

/// A checkmark list item.
#[derive(Clone, Debug, PartialEq)]
pub enum Checkmark {
    Checked(String),
    Unchecked(String),
}

impl Checkmark {
    /// Creates a new default checkmark item.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a mew checkmark item with the given values.
    pub fn from(text: impl Into<String>, checked: bool) -> Self {
        match checked {
            true => Checkmark::Checked(text.into()),
            false => Checkmark::Unchecked(text.into()),
        }
    }
}

impl Default for Checkmark {
    fn default() -> Self {
        Checkmark::Unchecked("".into())
    }
}

impl fmt::Display for Checkmark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Checkmark::Unchecked(text) => write!(f, "[ ] {}", text),
            Checkmark::Checked(text) => write!(f, "[x] {}", text),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MarkdownElement;

    #[test]
    fn test_checkmark_from() {
        assert_eq!(
            Checkmark::from("Eat spaghetti", false),
            Checkmark::Unchecked("Eat spaghetti".into())
        );

        assert_eq!(
            Checkmark::from("Eat spaghetti", true),
            Checkmark::Checked("Eat spaghetti".into())
        );
    }

    #[test]
    fn test_checkmark_display() {
        assert_eq!(
            Checkmark::from("Eat spaghetti", false).render(),
            "[ ] Eat spaghetti"
        );

        assert_eq!(
            Checkmark::from("Eat spaghetti", true).render(),
            "[x] Eat spaghetti"
        );
    }

    #[test]
    fn test_checkmark_default() {
        assert_eq!(Checkmark::default(), Checkmark::Unchecked(String::new()));
    }

    #[test]
    fn test_checkmark_new() {
        assert_eq!(Checkmark::new(), Checkmark::default());
    }
}
