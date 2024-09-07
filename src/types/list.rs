use crate::MarkdownElement;
use std::{borrow::Cow, fmt};

/// The type of list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListType {
    /// An ordered list prefixes all its items using numbers.
    Ordered,
    /// An unordered list prefixes all its items using a `ListItemMarker`.
    Unordered,
}

impl Default for ListType {
    fn default() -> Self {
        Self::Unordered
    }
}

/// The marker used to indicate the start of a `ListItem`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListItemMarker {
    /// The `*` list item marker.
    Asterisk,
    /// The `-` list item marker.
    Dash,
    /// A numbered list item marker.
    ///
    /// Numbered items use a special separator char that immediately follows the
    /// item number.
    Numbered(NumberedListItemMarkerSeparator),
    /// The `+` list item marker.
    Plus,
}

impl Default for ListItemMarker {
    fn default() -> Self {
        Self::Dash
    }
}

impl fmt::Display for ListItemMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Asterisk => Cow::Borrowed("*"),
            Self::Dash => Cow::Borrowed("-"),
            Self::Numbered(separator) => Cow::Owned(format!("{}", separator)),
            Self::Plus => Cow::Borrowed("+"),
        };
        write!(f, "{}", c)
    }
}

/// The separator used to separate the list item number and the item content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NumberedListItemMarkerSeparator {
    /// The `.` separator.
    Dot,
    /// The `)` separator.
    Parenthesis,
}

impl Default for NumberedListItemMarkerSeparator {
    fn default() -> Self {
        Self::Dot
    }
}

impl fmt::Display for NumberedListItemMarkerSeparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Dot => '.',
            Self::Parenthesis => ')',
        };
        write!(f, "{}", c)
    }
}

/// An item inside a markdown list.
pub type ListItem = Box<dyn MarkdownElement>;

/// A markdown list.
///
/// # Note
///
/// The implementation does __NOT__ check if the marker matches the specified
/// list type. It is theoretically possible to create an ordered list that uses
/// dashes as its item markers. The here exposed inner fields should therefore
/// be used with care.
///
/// Please use the [builders](module.builder.html) to safely create Markdown
/// compliant documents!
#[derive(Clone, Debug, Default)]
pub struct List {
    pub items: Vec<ListItem>,
    pub marker: ListItemMarker,
    pub ty: ListType,
}

impl List {
    /// Creates a new default `List`.
    ///
    /// The list will be unordered and uses a dash marker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new empty ordered `List`.
    pub fn ordered() -> Self {
        Self {
            marker: ListItemMarker::Numbered(NumberedListItemMarkerSeparator::default()),
            ty: ListType::Ordered,
            ..Default::default()
        }
    }

    /// Creates a new empty unordered `List`.
    pub fn unordered() -> Self {
        Self {
            ty: ListType::Unordered,
            ..Default::default()
        }
    }

    /// Creates a new ordered `List` with the given items.
    pub fn ordered_with(items: Vec<ListItem>) -> Self {
        Self {
            items,
            marker: ListItemMarker::Numbered(NumberedListItemMarkerSeparator::default()),
            ty: ListType::Ordered,
        }
    }

    /// Creates a new unordered `List` with the given items.
    pub fn unordered_with(items: Vec<ListItem>) -> Self {
        Self {
            items,
            ty: ListType::Unordered,
            ..Default::default()
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, item) in self.items.iter().enumerate() {
            let marker = match self.ty {
                ListType::Ordered => format!("{}{}", idx + 1, self.marker),
                ListType::Unordered => format!("{}", self.marker),
            };
            writeln!(f, "{} {}", marker, item.render())?;
        }

        Ok(())
    }
}
