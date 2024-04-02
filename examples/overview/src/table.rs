use yew::{Callback, classes, function_component, Html, html, TargetCast, use_reducer, use_state};
use serde::Serialize;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew_custom_components::table::{Options, Table};
use yew_custom_components::table::types::{ColumnBuilder, TableData};

#[function_component(TableExample)]
pub fn table_example() -> Html {
    let data = use_reducer(crate::types::mock_data::Data::default);
    let search_term = use_state(|| None::<String>);
    let new_id = use_state(|| 0);
    let new_name = use_state(|| None::<String>);
    let new_value = use_state(|| 0);
    let search = (*search_term).as_ref().cloned();
    let id = *new_id;
    let name = (*new_name).clone();
    let value = *new_value;

    let columns = vec![
        ColumnBuilder::new("id").orderable(false).short_name("ID").data_property("id").header_class("user-select-none").build(),
        ColumnBuilder::new("name").orderable(true).short_name("Name").data_property("name").header_class("user-select-none").build(),
        ColumnBuilder::new("value").orderable(true).short_name("Value").data_property("value").header_class("user-select-none").build(),
    ];

    let options = Options {
        unordered_class: Some("fa-sort".to_string()),
        ascending_class: Some("fa-sort-up".to_string()),
        descending_class: Some("fa-sort-down".to_string()),
        orderable_classes: vec!["mx-1".to_string(), "fa-solid".to_string()],
    };

    let mock_data = (*data).clone();

    let mut table_data = Vec::new();
    for (id, name, value) in mock_data.data {
        table_data.push(TableLine {
            id,
            name,
            value,
        })
    }

    let oninput_search = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input.value().is_empty() {
                search_term.set(None);
            } else {
                search_term.set(Some(input.value()));
            }
        })
    };

    let oninput_id = {
        let old_id = id;
        let id = new_id.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input.value().is_empty() {
                id.set(0);
            } else {
                if let Ok(v) = input.value().parse::<i32>() {
                    id.set(v);
                } else {
                    id.set(old_id);
                }
            }
        })
    };

    let oninput_value = {
        let old_value = value;
        let value = new_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input.value().is_empty() {
                value.set(0);
            } else {
                if let Ok(v) = input.value().parse::<i32>() {
                    value.set(v);
                } else {
                    value.set(old_value);
                }
            }
        })
    };

    let oninput_name = {
        let name = new_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input.value().is_empty() {
                name.set(None);
            } else {
                name.set(Some(input.value()));
            }
        })
    };

    let onclick = {
        let dispatcher = data.dispatcher().clone();
        let id = new_id.clone();
        let name = new_name.clone();
        let value = new_value.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(n) = (*name).clone() {
                dispatcher.dispatch(crate::types::mock_data::DataActions::AddData((*id, n, *value as i64)));
                id.set(0);
                name.set(None);
                value.set(0);
            }
        })
    };

    let onclick_random = {
        let dispatcher = data.dispatcher().clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            dispatcher.dispatch(crate::types::mock_data::DataActions::RandomizeData);
        })
    };


    html!(
        <>
            <h1>{"Table Example"}</h1>
            <div class="flex-grow-1 p-2 input-group mb-2">
              <span class="input-group-text">{"id"}</span>
              <input type="text" oninput={oninput_id} class="form-control" value={format!("{}", id)}/>
              <span class="input-group-text">{"name"}</span>
              <input type="text" oninput={oninput_name} class="form-control" value={name.unwrap_or_default()} />
              <span class="input-group-text">{"value"}</span>
              <input type="text" oninput={oninput_value} class="form-control" value={format!("{}", value)}/>
              <button type="button" {onclick} class="btn btn-primary">{"Add"}</button>
            </div>
            <div class="flex-grow-1 p-2 input-group mb-2">
              <button type="button" onclick={onclick_random} class="btn btn-danger">{"Randomize"}</button>
            </div>
            <div class="flex-grow-1 p-2 input-group mb-2">
                <span class="input-group-text">
                    <i class="fas fa-search"></i>
                </span>
                <input class="form-control" type="text" id="search" placeholder="Search" oninput={oninput_search} />
            </div>
            <Table<TableLine> options={options.clone()} search={search.clone()} classes={classes!("table", "table-hover")} columns={columns.clone()} data={table_data.clone()} orderable={true}/>
        </>
    )
}

#[derive(Clone, Serialize, Debug, Default)]
struct TableLine {
    pub id: i32,
    pub name: String,
    pub value: i64,
}

impl PartialEq<Self> for TableLine {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}

impl PartialOrd for TableLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl TableData for TableLine {
    fn get_field_as_html(&self, field_name: &str) -> yew_custom_components::table::error::Result<Html> {
        match field_name {
            "id" => Ok(html! { self.id }),
            "name" => Ok(html! { self.name.clone() }),
            "value" => Ok(html! { self.value }),
            _ => Ok(html! {}),
        }
    }

    fn get_field_as_value(&self, field_name: &str) -> yew_custom_components::table::error::Result<serde_value::Value> {
        match field_name {
            "id" => Ok(serde_value::Value::I32(self.id)),
            "name" => Ok(serde_value::Value::String(self.name.clone())),
            "value" => Ok(serde_value::Value::I64(self.value)),
            _ => Ok(serde_value::to_value(()).unwrap()),
        }
    }

    fn matches_search(&self, needle: Option<String>) -> bool {
        match needle {
            Some(needle) => self.name.to_lowercase().contains(&needle.to_lowercase()),
            None => true,
        }
    }
}
