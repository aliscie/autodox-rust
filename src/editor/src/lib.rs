use wasm_bindgen::{UnwrapThrowExt, JsCast};
use web_sys::{Element, window, Document, MouseEvent};
use yew::prelude::*;
use initial_value::initial_value;
use initial_value_handler::initial_value_handler;

mod plugins;

use plugins::{Draggable, TableFormula, DropDown, MarkDown, Mention, KeyboardActions};

use std::cell::RefCell;
use std::rc::Rc;
use web_sys::console::log_1;
use wasm_bindgen::closure::Closure;


// use database::get_posts;


mod initial_value_handler;
mod initial_value;

// use nav_bar::NavBar;
// use tree_list::TreeList;
// use extend_triats::log;



#[function_component(Editor)]
pub fn editor() -> Html {
    let html = initial_value();
    let my_text_handle = use_state(|| "".to_string());
    let my_text = (*my_text_handle).clone();
    let doc: Document = window().unwrap_throw().document().unwrap_throw();


    use_effect_with_deps(move |_my_text| {
        let editor: Rc<Element> = Rc::new(doc.query_selector(".text_editor").unwrap_throw().unwrap_throw());
        initial_value_handler(&editor, html);
        Draggable::new(editor.clone());
        TableFormula::new(editor.clone());
        DropDown::new(editor.clone());
        MarkDown::new(editor.clone());
        Mention::new(editor.clone());
        KeyboardActions::new(editor.clone());

        || {}
    }, my_text);


    let _onkeydown: Callback<KeyboardEvent> = Callback::from(|e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
        }
    });

    let _onclick: Callback<MouseEvent> = Callback::from(|e: MouseEvent| {
        let doc = window().unwrap_throw().document().unwrap_throw();
        let body: Element = doc.query_selector("body").unwrap_throw().unwrap_throw();
        body.class_list().toggle("dark-mode");
    });

    // a function that return a list of posts from database
    // let posts = get_posts();

    // let post = &posts[0];
    // let title = &post.title;

    // log_1(&format!("title is :{:#?}", title).into());


    html! {
        <div
        id="1"
        class="text_editor"
        contenteditable="true"/>
    }
}