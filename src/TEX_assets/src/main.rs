extern crate web_sys;

use yew::prelude::*;

mod components;

use components::{Card, NavBar, TreeList};

use web_sys::{Element, MouseEvent, window, Document};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::console::log_1;
use editor::{Editor};

// use TEX;
// use ic_cdk_macros::import;
use ic_cdk::export::candid;
use futures::executor; // 0.3.1
use futures::executor::block_on;

use futures::{future, Future}; // 0.1.27





#[function_component(App)]
fn app() -> Html {


    let button_onclick: Callback<MouseEvent> = Callback::from(|e: MouseEvent| {
        #[ic_cdk_macros::import(canister_id = "r7inp-6aaaa-aaaaa-aaabq-cai", candid_path = "/Users/apple/Desktop/TEX/src/declarations/TEX/TEX.did")]
        struct UsersCanister;

        let future = UsersCanister::increment();
        // block_on(future);
    });


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
        <button onclick={button_onclick}>{"button"}</button>
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