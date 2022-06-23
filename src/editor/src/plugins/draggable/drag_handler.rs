use std::rc::Rc;
use std::str::SplitWhitespace;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{Element, MouseEvent, window, Document};
use web_sys::console::log_1;

use crate::plugins::draggable::draggable::Draggable;
use std::cell::RefCell;




impl Draggable {
    fn show_icon(doc: &Document, curr: &Element) {
        let drag_ic: Element = doc.create_element("drag-icon").unwrap();
        let left = curr.get_bounding_client_rect().left() as f64 - 33 as f64;
        let top = curr.get_bounding_client_rect().top() as f64 - 5 as f64;
        let style = &format!("margin-left: 0; position: absolute; left:{}px; top:{}px", left, top);
        let icon = r#"<span drop_down_btn="true" id="dragIcon" contenteditable="false">::</span>"#;
        drag_ic.set_inner_html(&icon);
        drag_ic.set_attribute("style", style);
        drag_ic.set_attribute("id", "drag");
        drag_ic.set_attribute("drop_down_btn", "true");
        curr.insert_before(&drag_ic.clone(), curr.first_child().as_ref());
    }

    pub fn drag_handler(&mut self) {
        let drag_icon_width = self.drag_icon_width;
        let doc = self.doc.clone();
        let mut prev = 0__f64;

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let y_value = event.page_y();
            let x_value = event.page_x();
            
            let mut curr = doc.element_from_point(*&x_value as f32 + drag_icon_width, *&y_value as f32).unwrap();

            let mut dont_drag = false;
            dont_drag |= &curr.class_name() == "text_editor";
            dont_drag |= &curr.tag_name() == "BODY";
            dont_drag |= curr.id() == "myDropdown";
            if dont_drag {
                return;
            }

            //TODO store dragged element, curr, and prev in struct argument instead of local_storage
            web_sys::window().unwrap().local_storage().unwrap().unwrap().set_item("dragged_e_y", &y_value.to_string()).unwrap();
            web_sys::window().unwrap().local_storage().unwrap().unwrap().set_item("dragged_e_x", &x_value.to_string()).unwrap();
            let targeted_element = Draggable::get_targeted(curr.clone()).unwrap();
            targeted_element.set_attribute("draggable", "true");


            if prev != curr.get_bounding_client_rect().top() {
                if doc.get_element_by_id("drag") != None {
                    let prev_drag_icon = doc.query_selector("drag-icon").unwrap().unwrap();
                    let prev_element = doc.element_from_point(x_value as f32, prev as f32);
                    if prev_element != None {
                        prev_element.unwrap().remove_attribute("draggable");
                    }
                    prev_drag_icon.remove();
                    prev = curr.get_bounding_client_rect().top();
                }
                Draggable::show_icon(&doc, &curr);
            }
        }) as Box<dyn FnMut(_)>);

        self.editor.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

