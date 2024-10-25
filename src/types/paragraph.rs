use std::fmt;

/// A markdown paragraph.
///
/// A paragraph is a continuous text that is visually separated from its
/// surrounding markdown elements. Word wrapped at 80 characters.

#[derive(Clone, Debug)]
pub struct Paragraph {
    /// The text inside the paragraph.
    pub text: String,
}

impl Paragraph {
    /// Creates a new paragraph with the given text.
    pub fn from(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl fmt::Display for Paragraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current_size = 0;

        for (index, word) in self.text.split(" ").enumerate() {
            if current_size + word.len() > 80 {
                current_size = 0;
                write!(f, "\n{}", word)?;
            } else {
                current_size += word.len();
                if index == 0 {
                    write!(f, "{}", word,)?;
                } else {
                    write!(f, " {}", word,)?;
                };
            }
        }

        writeln!(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Paragraph::from("Hello world").to_string(), "Hello world\n");
    }

    #[test]
    fn test_word_wrapping() {
        assert_eq!(
			Paragraph::from(
				"Markdown Builder is a Rustlang crate by Erb3, which lets you create markdown documents. It now supports word wrapping!"
			).to_string(),
			"Markdown Builder is a Rustlang crate by Erb3, which lets you create markdown documents. It now\nsupports word wrapping!\n"
		);
    }
}
