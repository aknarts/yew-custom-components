//! Table component.
//!
//! This component is a table that can be sorted by columns.
use std::fmt::Debug;
use types::{Column, TableData, TableOrder, TableState};
use yew::html;
use yew::prelude::*;

mod body;
pub mod error;
mod head;
mod macros;
pub mod types;

/// Options for the Table component.
#[cfg(feature="table")]
#[derive(Clone, Eq, PartialEq, Default)]
pub struct Options {
    /// Css classes for the unordered state.
    pub unordered_class: Option<String>,
    /// Css classes for the ascending state.
    pub ascending_class: Option<String>,
    /// Css classes for the descending state.
    pub descending_class: Option<String>,
    /// Css classes for the orderable state.
    pub orderable_classes: Vec<String>,
}

/// Properties of the Table component.
#[cfg(feature="table")]
#[derive(Properties, Clone, Eq, PartialEq, Default)]
pub struct Props<T>
where
    T: TableData + Debug,
{
    /// Columns of the table.
    pub columns: Vec<Column>,
    /// Table data
    pub data: Vec<T>,
    /// Whether the table is orderable.
    #[prop_or(false)]
    pub orderable: bool,
    /// Css classes for the table.
    #[prop_or_default]
    pub classes: Classes,
    /// Search string.
    #[prop_or_default]
    pub search: Option<String>,
    /// Options for the table.
    #[prop_or_default]
    pub options: Options,
    /// Limit of data displayed for pagination
    #[prop_or(None)]
    pub limit: Option<usize>,
    /// Page for pagination
    #[prop_or(0)]
    pub page: usize,
}

/// Messages for the Table component.
#[cfg(feature="table")]
#[derive(Debug)]
pub enum Msg<T>
where
    T: TableData + Debug,
{
    /// Sort a column.
    SortColumn(usize),
    /// Set the data of the table.
    SetData(Vec<T>),
}

/// Data for the Table component.
#[cfg(feature="table")]
#[derive(Clone, Eq, PartialEq, Default)]
pub struct Data<T>
where
    T: TableData + Debug,
{
    columns: Vec<Column>,
    data: Vec<T>,
    orderable: bool,
    state: TableState,
}

#[cfg(feature="table")]
impl<T> Reducible for Data<T>
where
    T: TableData + Debug,
{
    type Action = Msg<T>;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut new = (*self).clone();
        match action {
            Msg::SortColumn(i) => {
                use TableOrder::Unordered;
                for (j, x) in new.state.order.iter_mut().enumerate() {
                    if j == i {
                        *x = x.rotate();
                    } else {
                        *x = Unordered;
                    }
                }
            }
            Msg::SetData(data) => {
                new.data = data;
            }
        };
        new.into()
    }
}

/// Wrapper for the search string
#[cfg(feature="table")]
#[derive(Clone, Eq, PartialEq, Default)]
pub struct Search {
    /// Search string.
    pub search: Option<String>,
}

/// Pagination structure
#[cfg(feature="table")]
#[derive(Clone, Eq, PartialEq, Default)]
pub struct Pagination {
    /// Limit of data displayed for pagination
    pub limit: Option<usize>,
    /// Page for pagination
    pub page: usize,
}

/// Table component.
///
/// # Properties
///
/// * `columns` - Columns of the table.
/// * `data` - Table data.
/// * `orderable` - Whether the table is orderable.
/// * `classes` - Css classes for the table.
/// * `search` - Search string.
/// * `options` - Options for the table.
#[cfg(feature="table")]
#[function_component(Table)]
pub fn table<T>(props: &Props<T>) -> Html
where
    T: TableData + Debug,
{
    let data = props.data.clone();
    let columns = props.columns.clone();
    let column_number = props.columns.len();
    let orderable = props.orderable;
    let state = use_reducer_eq(|| Data {
        columns,
        data: vec![],
        orderable,
        state: TableState {
            order: vec![TableOrder::default(); column_number],
        },
    });

    state.dispatch(Msg::SetData(data));

    let search = Search {
        search: props.search.clone(),
    };
    let classes = props.classes.clone();
    let options = props.options.clone();

    let limit = props.limit;
    let page = props.page;

    html! {
        <ContextProvider<UseReducerHandle<Data<T>>> context={state}>
            <table class={classes!(classes)}>
                <ContextProvider<Options> context={options}>
                    <head::TableHead<T> />
                </ContextProvider<Options>>
                <ContextProvider<Search> context={search}>
                    <ContextProvider<Pagination> context={Pagination { limit, page }}>
                        <body::TableBody<T> />
                    </ContextProvider<Pagination>>
                </ContextProvider<Search>>
            </table>
        </ContextProvider<UseReducerHandle<Data<T>>>>
    }
}
