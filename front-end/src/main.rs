mod components;

use components::Hello;
use yew::{classes, function_component, html, Html};

#[function_component]
fn App() -> Html {
    html! {
    <>
        <div class={classes!(["bg-red-500", "text-3xl"])}>
            <Hello />
        </div>
        <div>
            <p class={classes!("bg-blue-500")}>{ "Test" }</p>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
