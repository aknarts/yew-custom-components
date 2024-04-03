use web_sys::Window;
use yew::prelude::*;
use yew_router::prelude::*;

mod home;
mod header;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/table")]
    Table,
    #[at("/tabs")]
    Tabs,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn window() -> Option<Window> {
    web_sys::window()
}

pub fn document() -> Option<web_sys::Document> {
    window().and_then(|w| w.document())
}

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_state(|| crate::types::theme::Theme { dark: true });

    document().and_then(|doc| {
        doc.document_element().and_then(|el| {
            if ctx.get_dark() {
                el.set_attribute("data-bs-theme", "dark").unwrap();
            } else {
                el.set_attribute("data-bs-theme", "danger").unwrap();
            };
            Some(())
        });
        Some(())
    });

    html! {
        <ContextProvider<UseStateHandle<crate::types::theme::Theme>> context={ctx}>
            <HashRouter>
                <header::Header />
                <main>
                    <div class="container mt-2">
                        <Switch<Route> render={switch} />
                    </div>
                </main>
            </HashRouter>
        </ContextProvider<UseStateHandle<crate::types::theme::Theme>>>
    }
}

#[allow(clippy::needless_pass_by_value)]
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!( <home::Home /> ),
        Route::NotFound => html!( <home::Home /> ),
        Route::Table => html!( <crate::table::TableExample /> ),
        Route::Tabs => html!( <crate::tabs::TabsExample /> )
    }
}
