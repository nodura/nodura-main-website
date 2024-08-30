use yew::prelude::*;

#[function_component(Hello)]
pub fn hello() -> Html {
    html! {
        <div>
            <h1>{"Hello from a separate file!"}</h1>
            <p>{"This is a Yew component in a separate file."}</p>
        </div>
    }
}
