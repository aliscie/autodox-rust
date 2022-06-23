use std::rc::Rc;
use std::str::SplitWhitespace;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{Element, MouseEvent, window};
use web_sys::console::log_1;

use super::draggable::Draggable;

impl Draggable {
    pub fn get_targeted(curr: Element) -> Option<Element> {
        if curr.tag_name() == "TD" {
            return Some(curr.parent_element().unwrap());
        }

        if curr.tag_name() == "TH" {
            let table_head = curr.parent_element().unwrap().parent_element().unwrap().first_element_child().unwrap();
            if table_head.tag_name() == "TR" && table_head == curr.parent_element().unwrap() {
                return Some(curr.parent_element().unwrap().parent_element().unwrap().parent_element().unwrap());
            }
        }

        return Some(curr);
    }

}

