use std::fmt::Debug;
use types::{Column, TableData, TableOrder, TableState};
use yew::html;
use yew::prelude::*;

mod body;
pub mod error;
mod head;
mod macros;
pub mod types;

#[cfg(feature="table")]
#[derive(Clone, Eq, PartialEq, Default)]
pub struct Options {
    pub unordered_class: Option<String>,
    pub ascending_class: Option<String>,
    pub descending_class: Option<String>,
    pub orderable_classes: Vec<String>,
}

/// Properties of the Table component.
#[cfg(feature="table")]
#[derive(Properties, Clone, Eq, PartialEq, Default)]
pub struct Props<T>
where
    T: TableData + Debug,
{
    pub columns: Vec<Column>,
    pub data: Vec<T>,
    #[prop_or(false)]
    pub orderable: bool,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub search: Option<String>,
    #[prop_or_default]
    pub options: Options,
}

#[cfg(feature="table")]
#[derive(Debug)]
pub enum Msg<T>
where
    T: TableData + Debug,
{
    SortColumn(usize),
    SetData(Vec<T>),
}

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

#[cfg(feature="table")]
#[derive(Clone, Eq, PartialEq, Default)]
pub struct Search {
    pub search: Option<String>,
}

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

    html! {
        <ContextProvider<UseReducerHandle<Data<T>>> context={state}>
            <table class={classes!(classes)}>
                <ContextProvider<Options> context={options}>
                    <head::TableHead<T> />
                </ContextProvider<Options>>
                <ContextProvider<Search> context={search}>
                    <body::TableBody<T> />
                </ContextProvider<Search>>
            </table>
        </ContextProvider<UseReducerHandle<Data<T>>>>
    }
}
