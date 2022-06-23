use yew::prelude::*;

mod components;
use components::{Card, NavBar, TreeList};
use web_sys::{Element, MouseEvent, window, Document};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::console::log_1;
use editor::{Editor};

#[function_component(App)]
fn app() -> Html {
    // let counter = use_state(|| 0);
    // let onclick = {
    //     let counter = counter.clone();
    //     move |_| {
    //         let value = *counter + 1;
    //         counter.set(value);
    //     }
    // };

    let _onclick: Callback<MouseEvent> = Callback::from(|e: MouseEvent| {
        let doc = window().unwrap_throw().document().unwrap_throw();
        let body: Element = doc.query_selector("body").unwrap_throw().unwrap_throw();
        body.class_list().toggle("light-mode");
    });

    html! {
        <div>

        <NavBar>
        <span>
                <label class="switch">
                <input onclick={_onclick} type="checkbox"/>
                <span class="slider round"></span>
                </label>

            <TreeList/>
        </span>
       </NavBar>

        // <Card>
        // <h1>{"text"}</h1>
        // </Card>

            // <div class="card">
            // <button {onclick}>{ "+1" }</button>
            // <h1>{ *counter }</h1>
            // </div>

        <Editor/>
        // <div class="static_card">
        // <h1>{"Card"}</h1>
        // </div>
        //
        // <div class="static_card bg-blur">
        // <h1>{"Card"}</h1>
        // </div>
        //
        // <div class="card bg-blur">
        // <h1>{"Card with Gradient Background"}</h1>
        // </div>


        </div>
        
    }
}





fn main() {
    yew::start_app::<App>();
}