use yew::{function_component, html, Html};


#[function_component(Home)]
pub fn home() -> Html {
    html!(
        <div class="container pb-0">
            <div class="row">
                <div class="col-12">
                    <h1 class="display-1">{"Examples"}</h1>
                    <p class="lead">{"Select a component in the top panel"}</p>
                </div>
            </div>
        </div>
    )
}
