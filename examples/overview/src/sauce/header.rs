use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let theme = use_context::<UseStateHandle<crate::types::theme::Theme>>().expect("no ctx found");
    let active = use_state(|| false);
    let location = use_location().unwrap();
    let route = Route::from_path(location.path(), &HashMap::new());
    let active_class = if *active {
        (Some("show"), None)
    } else {
        (None, Some("collapsed"))
    };

    let activated = *active;

    let onclick = { Callback::from(move |_| active.set(!*active)) };

    let theme_class = if (*theme).get_dark() {
        "fa-solid"
    } else {
        "fa-regular"
    };

    let onclick_theme = {
        Callback::from(move |_| {
            theme.set(crate::types::theme::Theme {
                dark: !theme.get_dark(),
            });
            theme.toggle_dark();
        })
    };

    html!(
        <>
            <nav class={classes!("navbar", "navbar-expand-lg", "sticky-top", "shadow")} aria-label="main navigation">
                <div class="container-fluid">
                    <Link<Route> to={Route::Home} classes="navbar-brand fs-2">
                        { "Yew Custom Components" }
                    </Link<Route>>
                    <div class="container-fluid">
                        <button class={classes!("navbar-toggler", active_class.1)} type="button" onclick={onclick.clone()} aria-controls="navbarSupportedContent" aria-expanded={(!activated).to_string()} aria-label="Toggle navigation">
                          <span class="navbar-toggler-icon"></span>
                        </button>
                        <div class={classes!("collapse","navbar-collapse", active_class.0)} id="navbarSupportedContent">
                            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                                <li class="nav-item">
                                    <Link<Route> classes={classes!("nav-link" , is_active(&route, &[Route::Table]))} to={Route::Table}>
                                        { "Table" }
                                    </Link<Route>>
                                </li>
                                <li class="nav-item">
                                    <Link<Route> classes={classes!("nav-link" , is_active(&route, &[Route::Tabs]))} to={Route::Tabs}>
                                        { "Tabs" }
                                    </Link<Route>>
                                </li>
                            </ul>
                            <ul class="navbar-nav">
                            {
                               html!(
                                    <li class="nav-item">
                                        <a class="nav-link" onclick={onclick_theme}>
                                            <i class={classes!(theme_class, "fa-sun")}></i>
                                        </a>
                                    </li>
                                )
                            }
                            </ul>
                        </div>
                    </div>
                </div>
            </nav>
        </>
    )
}

fn is_active(route: &Option<Route>, desired: &[Route]) -> Option<String> {
    route.as_ref().and_then(|r| {
        if desired.contains(r) {
            Some("active".to_string())
        } else {
            None
        }
    })
}
