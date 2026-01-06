//! Sortable table header component
//!
//! Reusable component for creating clickable, sortable table headers.

use dioxus::prelude::*;

/// Sort direction for table columns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

impl SortDirection {
    /// Toggle between ascending and descending
    pub fn toggle(self) -> Self {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }

    /// Get the sort indicator icon
    pub fn indicator(&self) -> &'static str {
        match self {
            SortDirection::Ascending => " ▲",
            SortDirection::Descending => " ▼",
        }
    }
}

/// Sortable table header cell component
/// 
/// Renders a clickable `<th>` that shows sort direction when active.
#[component]
pub fn SortableHeader<T: Clone + PartialEq + 'static>(
    /// The column identifier
    column: T,
    /// Display label for the header
    label: String,
    /// Currently sorted column (if any)
    current_sort: Option<T>,
    /// Current sort direction
    direction: SortDirection,
    /// Callback when header is clicked - receives the column identifier
    on_sort: EventHandler<T>,
) -> Element {
    let is_active = current_sort.as_ref() == Some(&column);
    let column_for_click = column.clone();

    rsx! {
        th {
            class: if is_active { "sortable-header active" } else { "sortable-header" },
            onclick: move |_| on_sort.call(column_for_click.clone()),
            "{label}"
            if is_active {
                span { class: "sort-indicator", "{direction.indicator()}" }
            }
        }
    }
}

/// Non-sortable table header cell component (for columns like "Actions" or selection)
#[component]
pub fn StaticHeader(
    /// Display label for the header (empty string for icon columns)
    label: String,
) -> Element {
    rsx! {
        th { "{label}" }
    }
}
