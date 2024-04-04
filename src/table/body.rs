use super::types::{TableData, TableOrder};
use std::fmt::Debug;
use yew::{function_component, html, use_context, Html, Properties, UseReducerHandle};

#[cfg(feature="table")]
#[function_component(TableBody)]
pub fn table_body<T>() -> Html
where
    T: TableData + Debug,
{
    let state = use_context::<UseReducerHandle<super::Data<T>>>().unwrap();
    let pagination = use_context::<super::Pagination>().unwrap();
    let mut data = state.data.clone();
    let columns = state.columns.clone();
    let order = state.state.order.clone();
    if let Some(index) = order
        .iter()
        .enumerate()
        .filter_map(|(i, o)| match o {
            super::TableOrder::Ascending => Some(i),
            super::TableOrder::Descending => Some(i),
            _ => None,
        })
        .nth(0)
    {
        if let Some(column) = columns.get(index) {
            if let Some(data_property) = column.data_property.as_ref() {
                if let Some(ord) = order.get(index) {
                    match ord {
                        TableOrder::Unordered => {}
                        TableOrder::Ascending => {
                            data.sort_by_cached_key(|x| {
                                x.get_field_as_value(data_property).unwrap()
                            });
                        }
                        TableOrder::Descending => {
                            data.sort_by_cached_key(|x| {
                                std::cmp::Reverse(x.get_field_as_value(data_property).unwrap())
                            });
                        }
                    }
                }
            }
        }
    };

    if let Some(limit) = pagination.limit {
        data = data
            .into_iter()
            .skip(pagination.page * limit)
            .take(limit)
            .collect();
    }

    html!(<tbody>
        {for data.iter().map(|row| {
            html!(<Row<T> row={row.clone()} />)
        })}
        </tbody>)
}

#[cfg(feature="table")]
#[derive(Properties, Clone, Eq, PartialEq, Default)]
pub struct Props<T>
where
    T: TableData + Debug,
{
    pub row: T,
}

#[cfg(feature="table")]
#[function_component(Row)]
pub fn row<T>(props: &Props<T>) -> Html
where
    T: TableData + Debug,
{
    let state = use_context::<UseReducerHandle<super::Data<T>>>().unwrap();
    let columns = state.columns.clone();
    let search = use_context::<super::Search>().unwrap();
    let row = props.row.clone();

    if row.matches_search(search.search.clone()) {
        html!(<tr>
                    {
                        for columns.iter()
                            .map(|c| { c.data_property.as_ref().unwrap_or(&c.name) })
                            .map(|name| { row.get_field_as_html(name) })
                            .filter_map(std::result::Result::ok)
                            .map(|el| html! { <td>{ el }</td> })
                    }
                </tr>)
    } else {
        html!()
    }
}
