use std::convert::TryInto;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::closure::Closure;
use web_sys::{Element, Node, window, HtmlElement};
use web_sys::console::log_1;

use super::main::KeyboardActions;

impl KeyboardActions {
    pub(crate) fn place_holder(&mut self) {
        // let doc = self.doc.clone();
        // let mut prev: Option<Element> = None;
        // let mut curr: Option<Element> = None;
        //
        // let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        //     prev = curr.clone();
        //     curr = Some(doc.element_from_point(event.page_x() as f32, event.page_y() as f32).unwrap());
        //     if &curr.as_ref().unwrap().text_content().unwrap() == "::" {
        //         curr.as_ref().unwrap().set_attribute("empty", "true");
        //     }
        //     prev.as_ref().unwrap().remove_attribute("empty");
        // }) as Box<dyn FnMut(_)>);
        //
        // // self.editor.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        // self.editor.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        // closure.forget();
    }

    pub(crate) fn keyboard_actions_handler(&mut self) {
        let doc = self.doc.clone();
        let mut prev: Option<Element> = None;
        let mut curr: Option<Element> = None;

        // let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        //     if ["Enter", "Tab"].contains(&event.key().as_str()) {
        //         event.prevent_default();
        //
        //         let range = window().unwrap_throw().get_selection().unwrap().unwrap();
        //         let range = range.get_range_at(0).unwrap();
        //         curr = range.common_ancestor_container().unwrap().parent_element();
        //
        //         curr.as_ref().unwrap().insert_adjacent_html("afterend", r#"<p new_element="true" empty="true"></P>"#);
        //         let new_el = doc.query_selector("[new_element]").unwrap().unwrap();
        //         let new_el = new_el as HtmlElement;
        //         // new_el.focus();
        //     }
        // }) as Box<dyn FnMut(_)>);
        //
        // self.editor.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        // // self.editor.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        //
        // closure.forget();
    }
}
