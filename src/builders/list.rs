use crate::types::{
    checkmark::Checkmark,
    list::{List, ListItem},
};

#[derive(Clone, Debug, Default)]
pub struct ListBuilder {
    items: Vec<ListItem>,
    has_checkmarks: bool,
}

impl ListBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(mut self, item: impl Into<ListItem>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Adds a checkmark using [checkmark::Checkmark].
    pub fn checkmark(mut self, item: impl Into<String>, checked: bool) -> Self {
        self.items.push(Checkmark::from(item, checked).into());
        self.has_checkmarks = true;
        self
    }

    pub fn ordered(self) -> List {
        if self.items.is_empty() {
            panic!("Attempt to bulid list without contents");
        }

        if self.has_checkmarks {
            panic!("Attempt to build ordered list with checkmarks")
        }

        List::ordered_with(self.items)
    }

    pub fn unordered(self) -> List {
        if self.items.is_empty() {
            panic!("Attempt to bulid list without contents");
        }

        List::unordered_with(self.items)
    }
}

impl List {
    pub fn builder() -> ListBuilder {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ListVariant, MarkdownElement, Paragraph};

    #[test]
    fn test_ordered_paragraphs() {
        let list = List::builder()
            .append(Paragraph::from("Hello World"))
            .append(Paragraph::from("John doe"))
            .ordered();

        assert_eq!(list.variant, ListVariant::Ordered);
        // We cannot just compare the vector since because the paragraph hasn't been rendered
        assert_eq!(list.items.len(), 2);
        assert_eq!(list.render(), "1. Hello World\n2. John doe\n");
    }

    #[test]
    fn test_unordered_text() {
        let list = List::builder()
            .append("Hello World")
            .append("John doe")
            .unordered();

        assert_eq!(list.variant, ListVariant::Unordered);
        assert_eq!(list.items.len(), 2);
        assert_eq!(list.render(), "- Hello World\n- John doe\n");
    }

    #[test]
    fn test_unordered_checkmarks() {
        let list = List::builder()
            .checkmark("Eat spaghetti", true)
            .checkmark("Eat pizza", false)
            .checkmark("Eat kebab", true)
            .unordered();

        assert_eq!(list.variant, ListVariant::Unordered);
        assert_eq!(list.items.len(), 3);
        assert_eq!(
            list.render(),
            "- [x] Eat spaghetti\n- [ ] Eat pizza\n- [x] Eat kebab\n"
        );
    }

    #[test]
    #[should_panic]
    fn test_list_builder_unordered_no_elements_panic() {
        List::builder().unordered();
    }

    #[test]
    #[should_panic]
    fn test_list_builder_ordered_no_elements_panic() {
        List::builder().ordered();
    }

    #[test]
    #[should_panic]
    fn test_list_builder_ordered_checkmark_panic() {
        List::builder()
            .checkmark("Hello world", false)
            .checkmark("Checked", true)
            .ordered();
    }
}
