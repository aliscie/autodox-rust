use super::main::DropDown;
use web_sys::console::log_1;
use wasm_bindgen::closure::Closure;
use web_sys::{Element, MouseEvent, window, Document};
use crate::plugins::main::EditorPlugin;
use wasm_bindgen::{UnwrapThrowExt, JsCast};
// use extend_traits::get_target_element::*;

//TODO
// make reusable extend_traits workspace

pub trait MyNewTrait {
    fn target_element<'a>(&self)-> Option<Element>;

}

impl MyNewTrait for MouseEvent {
    fn target_element(&self) -> Option<Element> {
        let doc = window().unwrap_throw().document().unwrap_throw();
        let x = self.page_x();
        let y = self.page_y();
        doc.element_from_point(x as f32, y as f32)
    }
}


impl DropDown {
    pub(crate) fn click_away(&mut self) {
        let doc = self.doc.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let curr = event.target_element().unwrap();
            DropDown::remove_menu(&curr.as_ref(), &doc.as_ref());
        }) as Box<dyn FnMut(_)>);

        self.body.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}
