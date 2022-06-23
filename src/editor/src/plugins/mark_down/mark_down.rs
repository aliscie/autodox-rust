use std::convert::TryInto;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::closure::Closure;
use web_sys::{Element, Node, window};
use web_sys::console::log_1;

use super::main::MarkDown;

impl MarkDown {
    pub(crate) fn mark_down_handler(&mut self) {
        // let mut prev: Option<Element> = None;
        // let mut curr: Option<Element> = None;
        let body = self.body.clone();
        let doc = self.doc.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            // body.inner_html() =
            // let html = body.inner_html();
            // html.replace_all("# h1", "<h1> h1</h1>");
        }) as Box<dyn FnMut(_)>);

        self.editor.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref());
        self.editor.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}
