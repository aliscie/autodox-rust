use web_sys::{Element};
use wasm_bindgen::UnwrapThrowExt;

pub fn initial_value_handler(editor: &Element, value: &str) {
    // let doc = window().unwrap_throw().document().unwrap_throw();
    editor.set_inner_html(value);
}