use super::main::DropDown;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, Document};
use web_sys::console::log_1;
use crate::plugins::main::EditorPlugin;

impl DropDown {
    pub fn toggle_class<'a>(element: &'a Option<Element>, class: &'a str) {
        if element.is_none() {
            return;
        }
        let toggle = format!(" {}", class);
        let element = element.as_ref().unwrap();
        let mut classes = element.get_attribute("class").unwrap();
        if classes.contains(class) {
            classes = str::replace(&classes, &toggle, "");
        } else {
            classes.push_str(&toggle);
        }
        element.set_attribute("class", &classes);
    }

    pub(crate) fn drop_down(&mut self) {
        let doc = self.doc.clone();
        let element = self.editor.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let curr = doc.element_from_point(event.page_x() as f32, event.page_y() as f32).unwrap();
            if !&curr.has_attribute("drop_down_btn") {
                return;
            }
            DropDown::make_menu( &curr.as_ref(), &doc.as_ref());
        }) as Box<dyn FnMut(_)>);

        self.editor.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}
