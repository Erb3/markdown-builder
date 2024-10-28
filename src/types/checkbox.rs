use std::fmt;

/// A checkbox list item.
#[derive(Clone, Debug, PartialEq)]
pub enum Checkbox {
    Checked(String),
    Unchecked(String),
}

impl Checkbox {
    /// Creates a new default checkbox item.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a mew checkbox item with the given values.
    pub fn from(text: impl Into<String>, checked: bool) -> Self {
        match checked {
            true => Checkbox::Checked(text.into()),
            false => Checkbox::Unchecked(text.into()),
        }
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Checkbox::Unchecked("".into())
    }
}

impl fmt::Display for Checkbox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Checkbox::Unchecked(text) => write!(f, "[ ] {}", text),
            Checkbox::Checked(text) => write!(f, "[x] {}", text),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MarkdownElement;

    #[test]
    fn test_checkbox_from() {
        assert_eq!(
            Checkbox::from("Eat spaghetti", false),
            Checkbox::Unchecked("Eat spaghetti".into())
        );

        assert_eq!(
            Checkbox::from("Eat spaghetti", true),
            Checkbox::Checked("Eat spaghetti".into())
        );
    }

    #[test]
    fn test_checkbox_display() {
        assert_eq!(
            Checkbox::from("Eat spaghetti", false).render(),
            "[ ] Eat spaghetti"
        );

        assert_eq!(
            Checkbox::from("Eat spaghetti", true).render(),
            "[x] Eat spaghetti"
        );
    }

    #[test]
    fn test_checkbox_default() {
        assert_eq!(Checkbox::default(), Checkbox::Unchecked(String::new()));
    }

    #[test]
    fn test_checkbox_new() {
        assert_eq!(Checkbox::new(), Checkbox::default());
    }
}
