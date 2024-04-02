//! Types for the table module.
//!
//! This module contains the types used by the table module.
use super::error::Result;
use serde::Serialize;
use serde_value::Value;
use std::fmt;
use yew::Html;

/// Trait for table data.
#[cfg(feature="table")]
pub trait TableData: 'static + Default + Clone + PartialOrd + Serialize {
    /// Returns the Html representation of a field. When None, the field is not rendered.
    fn get_field_as_html(&self, field_name: &str) -> Result<Html>;

    /// Returns a table value given its field name. This value is used as a sorting key for the corresponding column.
    fn get_field_as_value(&self, field_name: &str) -> Result<Value>;

    /// Returns true if the row matches the search query.
    fn matches_search(&self, needle: Option<String>) -> bool;
}

/// A column in a table.
#[cfg(feature="table")]
#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Column {
    /// The name of the column.
    pub name: String,
    /// The short name of the column.
    pub short_name: Option<String>,
    /// The data property of the column.
    pub data_property: Option<String>,
    /// Whether the column is orderable.
    pub orderable: bool,
    /// The classes of the column header.
    pub header_classes: Vec<String>,
}

#[cfg(feature="table")]
impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.short_name.as_ref().unwrap_or(&self.name))
    }
}

/// A builder for a column.
#[cfg(feature="table")]
#[derive(Default)]
pub struct ColumnBuilder {
    name: String,
    short_name: Option<String>,
    data_property: Option<String>,
    orderable: bool,
    header_classes: Vec<String>,
}

#[cfg(feature="table")]
impl ColumnBuilder {
    /// Creates a new column builder.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the column.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            short_name: None,
            data_property: None,
            orderable: false,
            header_classes: vec![],
        }
    }

    /// Builds the column.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> Column {
        Column {
            name: self.name,
            short_name: self.short_name,
            data_property: self.data_property,
            orderable: self.orderable,
            header_classes: self.header_classes,
        }
    }

    /// Sets the column as orderable.
    pub const fn orderable(mut self, orderable: bool) -> Self {
        self.orderable = orderable;
        self
    }

    /// Sets the data property of the column.
    pub fn data_property(mut self, data_property: &str) -> Self {
        self.data_property = Some(data_property.to_string());
        self
    }

    /// Sets the short name of the column.
    pub fn short_name(mut self, short_name: &str) -> Self {
        self.short_name = Some(short_name.to_string());
        self
    }

    /// Adds a class to the column header.
    pub fn header_class(mut self, class: &str) -> Self {
        self.header_classes.push(class.to_string());
        self
    }
}

/// Order of a column
#[cfg(feature="table")]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TableOrder {
    /// The column is unordered.
    Unordered = 0,
    /// The column is ordered in ascending order.
    Ascending,
    /// The column is ordered in descending order.
    Descending,
}

#[cfg(feature="table")]
impl Default for TableOrder {
    fn default() -> Self {
        Self::Unordered
    }
}

#[cfg(feature="table")]
impl TableOrder {

    /// Rotates the order.
    ///
    /// Unordered -> Ascending -> Descending -> Unordered
    pub const fn rotate(self) -> Self {
        use TableOrder::{Ascending, Descending, Unordered};
        match self {
            Unordered => Ascending,
            Ascending => Descending,
            Descending => Unordered,
        }
    }
}

/// Order state of the table.
#[cfg(feature="table")]
#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct TableState {
    /// The order of the columns.
    pub order: Vec<TableOrder>,
}
