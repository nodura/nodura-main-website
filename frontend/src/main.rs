mod components;

use yew::{function_component, html, Html};

#[function_component(App)]
fn app() -> Html {
    html! {
            <div class="p-10 max-w-sm mx-auto bg-white rounded-xl shadow-md space-y-4">
                    <h1 class="text-2xl font-bold">{"Hello, Yew with Tailwind CSS!"}</h1>
                    <p>{"This is a Yew component styled with Tailwind CSS."}</p>
            </div>
    }
}

fn main() {
    yew::ServerRenderer::<App>::new().render();
}
