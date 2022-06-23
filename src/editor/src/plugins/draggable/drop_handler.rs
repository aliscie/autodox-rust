use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{Element, MouseEvent, window, Document};
use web_sys::console::log_1;

use super::draggable::Draggable;


impl Draggable {
    fn wrap_element(tag: &str, element: Element) -> Option<Element> {
        let doc = window().unwrap_throw().document().unwrap_throw();
        let wrapped: Element = doc.create_element(tag).unwrap();
        wrapped.set_inner_html(&element.inner_html());
        return Some(wrapped);
    }


    pub fn render_drop(curr_targeted: Element, targeted_element_targeted: Element, doc: Document) {
        let mut dragged = targeted_element_targeted.clone();
        let mut dragged_over = curr_targeted.clone();

        let is_drag_on = dragged_over.get_attribute("drag_on");
        let drag_below = dragged_over.get_attribute("drag_below");
        let drag_right = dragged_over.get_attribute("drag_right");
        let targeted_tag_name = dragged.tag_name();
        let tag_name = dragged_over.tag_name();
        let parent_tag_name = dragged_over.parent_element().unwrap().tag_name();

        if is_drag_on != None && tag_name == "LI" && parent_tag_name == "OL" {
            if targeted_tag_name != "LI" && parent_tag_name == "OL" {
                dragged = Draggable::wrap_element("li", dragged).unwrap();
                targeted_element_targeted.remove()
            }
        }

        if is_drag_on != None {
            dragged_over.append_child(&dragged).unwrap();
            curr_targeted.remove_attribute("drag_on").unwrap();
            targeted_element_targeted.remove_attribute("horizontal").unwrap();
            curr_targeted.remove_attribute("horizontal").unwrap();
        }
        if drag_below != None {
            dragged_over.parent_node().unwrap().insert_before(&dragged, dragged_over.next_sibling().as_ref()).unwrap();
            curr_targeted.remove_attribute("drag_below").unwrap();
            targeted_element_targeted.remove_attribute("horizontal").unwrap();
            curr_targeted.remove_attribute("horizontal").unwrap();
        }

        if drag_right != None {
            targeted_element_targeted.set_attribute("horizontal", "true").unwrap();
            curr_targeted.set_attribute("horizontal", "true").unwrap();
            dragged_over.parent_node().unwrap().insert_before(&dragged, dragged_over.next_sibling().as_ref()).unwrap();
            curr_targeted.remove_attribute("drag_right").unwrap();
        }
    }


    pub fn drop_handler(&mut self) {
        let drag_icon_width = 20 as f32;
        let doc = self.doc.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::DragEvent| {
            event.prevent_default();
            let dropped_y = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("dragged_e_y").unwrap().unwrap();
            let dropped_x = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("dragged_e_x").unwrap().unwrap();
            let mut dropped_element = Draggable::get_element_from_point(dropped_x.parse::<f32>().unwrap() + drag_icon_width, dropped_y.parse::<f32>().unwrap()).unwrap();

            let mut y_value = event.page_y();
            let mut x_value = event.page_x();
            let curr = Draggable::get_element_from_point(x_value as f32 + drag_icon_width, y_value as f32).unwrap();
            let curr_targeted = Draggable::get_targeted(curr.clone()).unwrap();
            let targeted_element = Draggable::get_targeted(dropped_element.clone()).unwrap_throw();
            let targeted_element_targeted = Draggable::get_targeted(targeted_element.clone()).unwrap_throw();

            if curr_targeted != targeted_element_targeted {
                Draggable::render_drop(curr_targeted.clone(), targeted_element_targeted.clone(), doc.clone());
            }
        }) as Box<dyn FnMut(_)>);

        self.editor.add_event_listener_with_callback("drop", &closure.as_ref().unchecked_ref());
        closure.forget();
    }
}