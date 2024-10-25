use crate::MarkdownElement;
use std::fmt;

/// The type of list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListType {
    /// An ordered list prefixes all its items using incrementing numbers.
    Ordered,
    /// An unordered list prefixes all its items using a dash.
    Unordered,
}

impl Default for ListType {
    fn default() -> Self {
        Self::Unordered
    }
}

/// An item inside a markdown list.
pub type ListItem = Box<dyn MarkdownElement>;

/// A markdown list.
///
/// Please use the [builders](module.builder.html) to safely create Markdown
/// compliant documents!
#[derive(Clone, Debug, Default)]
pub struct List {
    pub items: Vec<ListItem>,
    pub typ: ListType,
}

impl List {
    /// Creates a new default `List`.
    ///
    /// The list will be unordered.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new empty ordered `List`.
    pub fn ordered() -> Self {
        Self {
            typ: ListType::Ordered,
            ..Default::default()
        }
    }

    /// Creates a new empty unordered `List`.
    pub fn unordered() -> Self {
        Self {
            typ: ListType::Unordered,
            ..Default::default()
        }
    }

    /// Creates a new ordered `List` with the given items.
    pub fn ordered_with(items: Vec<ListItem>) -> Self {
        Self {
            items,
            typ: ListType::Ordered,
        }
    }

    /// Creates a new unordered `List` with the given items.
    pub fn unordered_with(items: Vec<ListItem>) -> Self {
        Self {
            items,
            typ: ListType::Unordered,
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, item) in self.items.iter().enumerate() {
            let marker = match self.typ {
                ListType::Ordered => format!("{}.", idx + 1),
                ListType::Unordered => "-".into(),
            };
            writeln!(f, "{} {}", marker, item.render().trim_end_matches("\n"))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Paragraph;

    #[test]
    fn test_ordered_list_one_entry() {
        assert_eq!(
            List::ordered_with(vec![Box::new(Paragraph::from("Hello"))]).render(),
            "1. Hello\n"
        );
    }

    #[test]
    fn test_ordered_list_many_entries() {
        assert_eq!(
            List::ordered_with(vec![
                Box::new(Paragraph::from("Eat")),
                Box::new(Paragraph::from("Sleep")),
                Box::new(Paragraph::from("Debug borrow checker"))
            ])
            .render(),
            "1. Eat\n2. Sleep\n3. Debug borrow checker\n"
        );
    }

    #[test]
    fn test_unordered_list_one_entry() {
        assert_eq!(
            List::unordered_with(vec![Box::new(Paragraph::from("Hello"))]).render(),
            "- Hello\n"
        );
    }

    #[test]
    fn test_unordered_list_many_entries() {
        assert_eq!(
            List::unordered_with(vec![
                Box::new(Paragraph::from("Eat")),
                Box::new(Paragraph::from("Sleep")),
                Box::new(Paragraph::from("Debug borrow checker"))
            ])
            .render(),
            "- Eat\n- Sleep\n- Debug borrow checker\n"
        );
    }

    #[test]
    fn test_default_list() {
        let list = List::new();
        assert_eq!(list.typ, ListType::Unordered);
        assert_eq!(list.items.len(), 0);
    }

    #[test]
    fn test_empty_unordered_list() {
        let list = List::unordered();
        assert_eq!(list.typ, ListType::Unordered);
        assert_eq!(list.items.len(), 0);
    }

    #[test]
    fn test_empty_ordered_list() {
        let list = List::ordered();
        assert_eq!(list.typ, ListType::Ordered);
        assert_eq!(list.items.len(), 0);
    }
}
