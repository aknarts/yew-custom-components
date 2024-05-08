//! Pagination component

use yew::{classes, function_component, html, use_state, Callback, Html, Properties};

/// Options for the pagination component
#[cfg(feature = "pagination")]
#[derive(Clone, PartialEq)]
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
    /// Previous text
    pub prev_text: String,
    /// Next text
    pub next_text: String,
    /// Show first/last button
    pub show_first_last: bool,
    /// First text
    pub first_text: String,
    /// Last text
    pub last_text: String,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            show_prev_next: true,
            show_first_last: false,
            first_text: String::from("First"),
            list_classes: vec![String::from("pagination")],
            item_classes: vec![String::from("page-item")],
            link_classes: vec![String::from("page-link")],
            active_item_classes: vec![String::from("active")],
            disabled_item_classes: vec![String::from("disabled")],
            prev_text: String::from("Previous"),
            next_text: String::from("Next"),
            last_text: String::from("Last"),
        }
    }
}

impl Options {
    /// Create new options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the show_prev_next option
    pub fn show_prev_next(mut self, show_prev_next: bool) -> Self {
        self.show_prev_next = show_prev_next;
        self
    }

    /// Set the show_first_last option
    pub fn show_first_last(mut self, show_first_last: bool) -> Self {
        self.show_first_last = show_first_last;
        self
    }

    /// Set the first text
    pub fn first_text(mut self, first_text: String) -> Self {
        self.first_text = first_text;
        self
    }

    /// Set the last text
    pub fn last_text(mut self, last_text: String) -> Self {
        self.last_text = last_text;
        self
    }

    /// Set the previous text
    pub fn prev_text(mut self, prev_text: String) -> Self {
        self.prev_text = prev_text;
        self
    }

    /// Set the next text
    pub fn next_text(mut self, next_text: String) -> Self {
        self.next_text = next_text;
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
#[cfg(feature = "pagination")]
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    /// Total number of items
    pub total: usize,
    /// Limit per page
    pub limit: usize,
    /// Max pages displayed
    #[prop_or_default]
    pub max_pages: Option<usize>,
    /// Options
    #[prop_or_default]
    pub options: Options,
    /// Optional current page callback
    #[prop_or(None)]
    pub on_page: Option<Callback<usize>>,
}

/// Pagination component
#[cfg(feature = "pagination")]
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

    let pages = match props.max_pages {
        None => {0..total_pages}
        Some(max) => {
            let start = if current_page < max/2 {
                0
            } else if current_page >= total_pages - max/2 {
                total_pages - max
            } else {
                current_page - max/2
            };
            let end = if start + max > total_pages {
                total_pages
            } else {
                start + max
            };
            start..end
        }
    };

    html!(
        <nav>
            <ul class={classes!("flex-wrap","pagination", options.list_classes)}>
                 {
                    if options.show_first_last {
                        let disabled_class = if current_page==0 {
                            Some(options.disabled_item_classes.clone())
                        } else {
                            None
                        };
                        html! {
                            <li class={classes!(options.item_classes.clone(), disabled_class)}><a class={classes!(options.link_classes.clone())} onclick={ let handle_page= handle_page.clone(); move |_| { handle_page.emit(0); }} href="#">{  options.first_text }</a></li>
                        }
                    } else {
                        html!()
                    }
                }
                {
                    if options.show_prev_next {
                        let disabled_class = if current_page==0 {
                            Some(options.disabled_item_classes.clone())
                        } else {
                            None
                        };
                        html! {
                            <li class={classes!(options.item_classes.clone(), disabled_class)}><a class={classes!(options.link_classes.clone())} onclick={ let handle_page= handle_page.clone(); move |_| { handle_page.emit(current_page-1); }} href="#">{  options.prev_text }</a></li>
                        }
                    } else {
                        html!()
                    }
                }
                {
                    pages.map(|index| {
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
                            <li class={classes!(options.item_classes.clone(), disabled_class)}><a class={classes!(options.link_classes.clone())} onclick={ let handle_page= handle_page.clone(); move |_| { handle_page.emit(current_page+1); }} href="#">{ options.next_text }</a></li>
                        }
                    } else {
                        html!()
                    }
                }
                {
                    if options.show_first_last {
                        let disabled_class = if current_page==total_pages-1 {
                            Some(options.disabled_item_classes.clone())
                        } else {
                            None
                        };
                        html! {
                            <li class={classes!(options.item_classes.clone(), disabled_class)}><a class={classes!(options.link_classes.clone())} onclick={ let handle_page= handle_page.clone(); move |_| { handle_page.emit(total_pages-1); }} href="#">{  options.last_text }</a></li>
                        }
                    } else {
                        html!()
                    }
                }
            </ul>
        </nav>
    )
}
