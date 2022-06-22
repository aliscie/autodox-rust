use yew::prelude::*;

mod components;
use components::{Card};

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class="grid-container">

        <Card>
        <h1>{"text"}</h1>
        </Card>

            <div class="card">
            <button {onclick}>{ "+1" }</button>
            <h1>{ *counter }</h1>
            </div>


        <div class="static_card">
        <h1>{"Card"}</h1>
        </div>
        
        <div class="static_card bg-blur">
        <h1>{"Card"}</h1>
        </div>

        <div class="card bg-blur">
        <h1>{"Card with Gradient Background"}</h1>
        </div>

        <div class="card bg-blur">
        <h1>{"Card with Gradient Background"}</h1>
        </div>


        </div>
        
    }
}

fn main() {
    yew::start_app::<App>();
}