use super::types::{Column, TableData, TableState};
use super::Options;
use std::fmt::Debug;
use yew::{
    classes, function_component, html, use_context, Callback, ContextProvider, Html, Properties,
    UseReducerHandle,
};

#[cfg(feature="table")]
#[function_component(TableHead)]
pub fn table_head<T>() -> Html
where
    T: TableData + Debug,
{
    let state = use_context::<UseReducerHandle<super::Data<T>>>().unwrap();
    let sort_state = state.state.clone();
    let orderable = state.orderable;
    let columns = state.columns.clone();
    let handle_sort = {
        let dispatch = state.dispatcher().clone();
        Callback::from(move |index: usize| dispatch.dispatch(super::Msg::SortColumn(index)))
    };

    html!(
        <thead>
            <ContextProvider<TableState> context={sort_state}>
                {for columns.iter().enumerate().map(|(index, column)| {
                    html! {
                        <HeadColumn column={column.clone()} {orderable} callback={handle_sort.clone()} {index}/>
                    }
                })}
            </ContextProvider<TableState>>
        </thead>
    )
}

#[cfg(feature="table")]
#[derive(Properties, Clone, PartialEq, Default)]
pub struct ColumnProps {
    index: usize,
    callback: Callback<usize>,
    #[prop_or(false)]
    pub orderable: bool,
    #[prop_or_default]
    pub column: Column,
}

#[cfg(feature="table")]
#[function_component(HeadColumn)]
pub fn head_column(props: &ColumnProps) -> Html {
    let column = &props.column;
    let state = use_context::<TableState>().unwrap();
    let options = use_context::<Options>().unwrap();
    let cb = props.callback.clone();
    let index = props.index;

    let get_header_sorting_class = |index: usize| {
        use super::types::TableOrder::{Ascending, Descending, Unordered};

        state.order.get(index).and_then(|order| match order {
            Unordered => options.unordered_class.clone(),
            Ascending => options.ascending_class.clone(),
            Descending => options.descending_class.clone(),
        })
    };

    let th_view = |child| {
        if props.orderable && column.orderable {
            html!( <th class={classes!(column.header_classes.clone())} scope="col" onclick={move |_| { cb.emit(index) }}>{ child }</th> )
        } else {
            html!( <th class={classes!(column.header_classes.clone())} scope="col">{ child }</th> )
        }
    };

    th_view(html!(
        <span>
            { column }
            if props.orderable && column.orderable {
                <i class={classes!(options.orderable_classes.clone(), get_header_sorting_class(index))}></i>
            }
        </span>
    ))
}
