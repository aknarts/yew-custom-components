//! Pagination component

use yew::{classes, function_component, html, use_state, Callback, Html, Properties};

/// Options for the pagination component
#[cfg(feature="pagination")]
#[derive(Clone, Default, PartialEq)]
pub struct Options {
    /// Show previous/next button
    pub show_prev_next: bool,
    /// List classes
    pub list_classes: Vec<String>,
    /// Item classes
    pub item_classes: Vec<String>,
    /// Link classes
    pub link_classes: Vec<String>,
    /// Active item classes
    pub active_item_classes: Vec<String>,
    /// Disabled item classes
    pub disabled_item_classes: Vec<String>,
}

impl Options {
    /// Set the show_prev_next option
    pub fn show_prev_next(mut self, show_prev_next: bool) -> Self {
        self.show_prev_next = show_prev_next;
        self
    }

    /// Set the list classes
    pub fn list_classes(mut self, list_classes: Vec<String>) -> Self {
        self.list_classes = list_classes;
        self
    }

    /// Set the item classes
    pub fn item_classes(mut self, item_classes: Vec<String>) -> Self {
        self.item_classes = item_classes;
        self
    }

    /// Set the active item classes
    pub fn active_item_classes(mut self, active_item_classes: Vec<String>) -> Self {
        self.active_item_classes = active_item_classes;
        self
    }

    /// Set the disabled item classes
    pub fn disabled_item_classes(mut self, disabled_item_classes: Vec<String>) -> Self {
        self.disabled_item_classes = disabled_item_classes;
        self
    }

    /// Set the link classes
    pub fn link_classes(mut self, link_classes: Vec<String>) -> Self {
        self.link_classes = link_classes;
        self
    }
}

/// Properties of the pagination component
#[cfg(feature="pagination")]
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    /// Total number of items
    pub total: usize,
    /// Limit per page
    pub limit: usize,
    /// Options
    #[prop_or_default]
    pub options: Options,
    /// Optional current page callback
    #[prop_or(None)]
    pub on_page: Option<Callback<usize>>,
}

/// Pagination component
#[cfg(feature="pagination")]
#[function_component(Pagination)]
pub fn pagination(props: &Props) -> Html {
    let page = use_state(|| 0usize);
    let current_page = *page;

    let options = props.options.clone();

    let handle_page = {
        let on_page = props.on_page.clone();
        Callback::from(move |id: usize| {
            page.set(id);
            if let Some(on_page) = on_page.as_ref() {
                on_page.emit(id);
            }
        })
    };

    let total_pages = props.total / props.limit;
    let total_pages = if props.total % props.limit == 0 {
        total_pages
    } else {
        total_pages + 1
    };

    html! (
        <nav>
            <ul class={classes!("pagination", options.list_classes)}>
                {
                    if options.show_prev_next {
                        let disabled_class = if current_page==0 {
                            Some(options.disabled_item_classes.clone())
                        } else {
                            None
                        };
                        html! {
                            <li class={classes!(options.item_classes.clone(), disabled_class)}><a class={classes!(options.link_classes.clone())} onclick={ let handle_page= handle_page.clone(); move |_| { handle_page.emit(current_page-1); }} href="#">{ "Previous" }</a></li>
                        }
                    } else {
                        html!()
                    }
                }
                {
                    (0..total_pages).map(|index| {
                        let class = if current_page==index {
                            Some(vec![options.active_item_classes.clone()])
                        } else {
                            None
                        };
                        html! { <li class={classes!(options.item_classes.clone(), class)}>
                                    {
                                        if current_page==index {
                                            html! { <span class={classes!(options.link_classes.clone())}>{ index+1 }</span> }
                                        } else {
                                            html! { <a class={classes!(options.link_classes.clone())} onclick={ let handle_page= handle_page.clone(); move |_| { handle_page.emit(index); }} href="#">{ index+1 }</a>}
                                        }
                                    }
                                </li> }
                    }).collect::<Html>()
                }
                {

                    if options.show_prev_next {
                        let disabled_class = if current_page==total_pages-1 {
                            Some(options.disabled_item_classes.clone())
                        } else {
                            None
                        };
                        html! {
                            <li class={classes!(options.item_classes.clone(), disabled_class)}><a class={classes!(options.link_classes.clone())} onclick={ let handle_page= handle_page.clone(); move |_| { handle_page.emit(current_page+1); }} href="#">{ "Next" }</a></li>
                        }
                    } else {
                        html!()
                    }
                }
            </ul>
        </nav>
    )

}
