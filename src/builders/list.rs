use crate::types::{
    checkmark::CheckmarkItem,
    list::{List, ListItem},
};

#[derive(Clone, Debug, Default)]
pub struct ListBuilder {
    items: Vec<ListItem>,
}

impl ListBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(mut self, item: impl Into<ListItem>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Adds a checkmark using checkmark::CheckmarkItem.
    pub fn checkmark(mut self, item: impl Into<String>, checked: bool) -> Self {
        self.items.push(CheckmarkItem::from(item, checked).into());
        self
    }

    pub fn ordered(self) -> List {
        List::ordered_with(self.items)
    }

    pub fn unordered(self) -> List {
        List::unordered_with(self.items)
    }
}

impl List {
    pub fn builder() -> ListBuilder {
        ListBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MarkdownElement, Paragraph};

    #[test]
    fn test_ordered_paragraphs() {
        let list = List::builder()
            .append(Paragraph::from("Hello World"))
            .append(Paragraph::from("John doe"))
            .ordered();

        assert_eq!(list.render(), "1. Hello World\n2. John doe\n");
    }

    #[test]
    fn test_unordered_text() {
        let list = List::builder()
            .append("Hello World")
            .append("John doe")
            .unordered();

        assert_eq!(list.render(), "- Hello World\n- John doe\n");
    }

    #[test]
    fn test_unordered_checkmarks() {
        let list = List::builder()
            .checkmark("Eat spaghetti", true)
            .checkmark("Eat pizza", false)
            .checkmark("Eat kebab", true)
            .unordered();

        assert_eq!(
            list.render(),
            "- [x] Eat spaghetti\n- [ ] Eat pizza\n- [x] Eat kebab\n"
        );
    }
}
