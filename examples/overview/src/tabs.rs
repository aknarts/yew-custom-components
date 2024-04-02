use yew::{
    function_component, html, Html,
};

use yew_custom_components::tabs::Tabs;

#[function_component(TabsExample)]
pub fn tabs_example() -> Html {

    html!(<>
        <Tabs tabs={vec![String::from("Tab 1"), String::from("Tab 2")]}>
            <div>{"Content 1"}</div>
            <div>{"Content 2"}</div>
        </Tabs>
    </>)
}
